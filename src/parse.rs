use crate::stmt::{Expr, Stmt};
use crate::token::{Token, Literal, TokenKind};

pub fn parse(tokens: Vec<Token>) -> Vec<Stmt> {
    Parser { tokens, current: 0 }.parse()
}

pub struct ParseErr {
    token: Token,
    message: String,
}

#[derive(Debug)]
struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

macro_rules! check {
    ($self:ident, $($p:pat),+) => {
        match $self.peek() {
            $(Token { kind: $p, .. }) |+ => true,
            _ => false,
        }
    }; 
}

macro_rules! eat {
    ($self:ident, $($p:pat),+) => {
        match $self.peek() {
            $(Token { kind: $p, .. }) |+ => Some($self.advance()),
            _ => None,
        }
    };
}

macro_rules! did_eat {
    ($self:ident, $($p:pat),+) => {
        match $self.peek() {
            $(Token { kind: $p, .. }) |+ => { $self.advance(); true },
            _ => false,
        }
    };
}

macro_rules! consume {
    ($self:ident, $p:pat, $message:literal) => {
        if let Some(tok) = eat!($self, $p) {
            Ok(tok)
        } else {
            Err(ParseErr {
                message: $message.to_string(),
                token: $self.peek(),
            })
        }
    };
}

impl Parser {
    fn parse(&mut self) -> Vec<Stmt> {
        let mut statments = Vec::new();

        while !self.is_at_end() {
            match self.declaration() {
                Ok(statement) => statments.push(statement),
                Err(ParseErr {
                    message,
                    ..
                }) => {
                    self.synchronize();
                    println!("parse error: {}", message)
                },
            }
        }

        statments
    }

    fn declaration(&mut self) -> Result<Stmt, ParseErr> {
        if eat!(self, TokenKind::Var).is_some() {
            self.var_declaration()
        } else {
            self.statement()
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt, ParseErr> {
        let name = consume!(self, TokenKind::Identifier, "Expect variable name.")?;

        let initializer = if eat!(self, TokenKind::Equal).is_some() {
            Some(self.expression()?)
        } else {
            None
        };

        consume!(self, TokenKind::Semicolon, "Expect ';' after variable declaration.")?;

        Ok(Stmt::Var { name, initializer })
    }

    fn statement(&mut self) -> Result<Stmt, ParseErr> {
        if did_eat!(self, TokenKind::For) {
            self.for_statement()
        } else if did_eat!(self, TokenKind::If) {
            self.if_statement()
        } else if did_eat!(self, TokenKind::Print) {
            self.print_statement()
        } else if did_eat!(self, TokenKind::While) {
            self.while_statement()
        } else if did_eat!(self, TokenKind::LeftBrace) {
            Ok(Stmt::Block{ statements: self.block()? })
        } else {
            self.expression_statement()
        }
    }

    /// for statements are de-sugared into while loops
    fn for_statement(&mut self) -> Result<Stmt, ParseErr> {
        consume!(self, TokenKind::LeftParen, "Expect '(' after 'for'.")?;

        let initializer = if did_eat!(self, TokenKind::Semicolon) {
            None
        } else if did_eat!(self, TokenKind::Var) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        let condition = if check!(self, TokenKind::Semicolon) {
            Expr::Literal { val: Literal::Bool(true) }
        } else {
            self.expression()?
        };
        consume!(self, TokenKind::Semicolon, "Expect ';' after loop condition.")?;

        let increment = if check!(self, TokenKind::RightParen) {
            None
        } else {
            Some(self.expression()?)
        };
        consume!(self, TokenKind::RightParen, "Expect ')' after for clauses.")?;

        let mut body = self.statement()?;
        if let Some(increment) = increment {
            body = Stmt::Block {
                statements: vec![body, Stmt::Expression { expr: increment }]
            };
        }

        body = Stmt::While {
            body: Box::new(body),
            condition,
        };

        if let Some(initializer) = initializer {
            body = Stmt::Block {
                statements: vec![initializer, body]
            }
        }

        Ok(body)
    }

    fn if_statement(&mut self) -> Result<Stmt, ParseErr> {
        consume!(self, TokenKind::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        consume!(self, TokenKind::RightParen, "Expect ')' after if condition.")?;

        let then_branch = Box::new(self.statement()?);
        let else_branch = if eat!(self, TokenKind::Else).is_some() {
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Ok(Stmt::If {
            condition,
            else_branch,
            then_branch,
        })
    }

    fn print_statement(&mut self) -> Result<Stmt, ParseErr> {
        let expr = self.expression()?;
        consume!(self, TokenKind::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Print { expr })
    }

    fn while_statement(&mut self) -> Result<Stmt, ParseErr> {
        consume!(self, TokenKind::LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        consume!(self, TokenKind::RightParen, "Expect ')' after while condition.")?;
        let body = self.statement()?;
        Ok(Stmt::While{ condition, body: Box::new(body) })
    }

    fn block(&mut self) -> Result<Vec<Stmt>, ParseErr> {
        let mut statements = Vec::new();
        while !check!(self, TokenKind::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        consume!(self, TokenKind::RightBrace, "Expect '}' after block.")?;
        Ok(statements)
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParseErr> {
        let expr = self.expression()?;
        consume!(self, TokenKind::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Expression { expr })
    }

    fn expression(&mut self) -> Result<Expr, ParseErr> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, ParseErr> {
        let expr = self.or()?;
        if let Some(equals) = eat!(self, TokenKind::Equal) {
            let value = self.assignment()?;

            if let Expr::Variable { name } = expr {
                return Ok(Expr::Assign { name, value: Box::new(value) });
            }

            return Err(ParseErr {
                token: equals,
                message: "Invalid assignment target.".to_string(),
            });
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, ParseErr> {
        let mut expr = self.and()?;
        while let Some(operator) = eat!(self, TokenKind::Or) {
            let right = self.and()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, ParseErr> {
        let mut expr = self.equality()?;
        while let Some(operator) = eat!(self, TokenKind::And) {
            let right = self.equality()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, ParseErr> {
        let mut expr = self.comparison()?;
        while let Some(operator) = eat!(self, TokenKind::BangEqual, TokenKind::EqualEqual) {
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParseErr> {
        let mut expr = self.addition()?;
        use TokenKind::*;
        while let Some(operator) = eat!(self, Greater, GreaterEqual, Less, LessEqual) {
            let right = self.addition()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr) 
    }

    fn addition(&mut self) -> Result<Expr, ParseErr> {
        let mut expr = self.multiplication()?;
        while let Some(operator) = eat!(self, TokenKind::Minus, TokenKind::Plus) {
            let right = self.multiplication()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn multiplication(&mut self) -> Result<Expr, ParseErr> {
        let mut expr = self.unary()?;
        while let Some(operator) = eat!(self, TokenKind::Slash, TokenKind::Star) {
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParseErr> {
        if let Some(operator) = eat!(self, TokenKind::Bang, TokenKind::Minus) {
            let right = self.unary()?;
            Ok(Expr::Unary { operator, right: Box::new(right) })
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr, ParseErr> {
        let tok = self.advance();
        use TokenKind::*;
        match tok.kind {
            False => Ok(Expr::Literal { val: Literal::Bool(false) }),
            True => Ok(Expr::Literal { val: Literal::Bool(true) }),
            Nil => Ok(Expr::Literal { val: Literal::Nil }),
            Number => Ok(Expr::Literal { val: Literal::Number(0f64) }),
            TokenKind::String => Ok(Expr::Literal { val: Literal::Str("".to_string()) }),
            LeftParen => {
                let expr = self.expression()?;
                consume!(self, TokenKind::RightParen, "Expect ')' after expression.")?;
                Ok(Expr::Grouping { expr: Box::new(expr) })
            }
            Identifier => Ok(Expr::Variable { name: tok }),
            _ => Err(ParseErr {
                token: tok,
                message: "Expect expression.".to_string(),
            }),
        }
    }

    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if let TokenKind::Semicolon = self.previous().kind {
                return;
            }

            match self.peek().kind {
                TokenKind::Class |
                TokenKind::Fun |
                TokenKind::Var |
                TokenKind::For |
                TokenKind::If |
                TokenKind::While |
                TokenKind::Print |
                TokenKind::Return => return,
                _ => {
                    self.advance();
                },
            }
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.peek()
    }

    fn is_at_end(&self) -> bool {
        if let Token {
            kind: TokenKind::Eof,
            ..
        } = self.peek()
        {
            true
        } else {
            false
        }
    }

    fn peek(&self) -> Token {
        self.peek_nth(0)
    }

    fn previous(&self) -> Token {
        self.peek_nth(-1)
    }

    fn peek_nth(&self, n: i16) -> Token {
        self.tokens[((self.current as i16) + n) as usize].clone() // TODO: avoidable??
    }
}
