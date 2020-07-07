use crate::environment::Environment;
use crate::stmt::{Expr, Stmt};
use crate::token::{Literal, Token, TokenKind};

pub fn interpret(statements: Vec<Stmt>, environment: Option<Environment>) -> Environment {
    Interpreter {
        environment: environment.unwrap_or(Environment::new()),
    }
    .interpret(statements)
}

struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    fn interpret(mut self, statements: Vec<Stmt>) -> Environment {
        for statement in statements {
            self.interpret_statement(&statement);
        }

        self.environment
    }

    fn interpret_statement(&mut self, statement: &Stmt) {
        match statement {
            Stmt::Expression { expr } => {
                self.interpret_expression(expr);
            }
            Stmt::Print { expr } => {
                println!("{}", self.interpret_expression(expr));
            }
            Stmt::Var { name, initializer } => {
                if let Some(initializer) = initializer {
                    let value = self.interpret_expression(initializer);
                    self.environment.define(name.name(), Some(value));
                } else {
                    self.environment.define(name.name(), None);
                }
            }
            Stmt::Block { statements } => {
                self.environment.push();
                for statement in statements {
                    self.interpret_statement(statement);
                }
                self.environment.pop();
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                if self.interpret_expression(condition).is_truthy() {
                    self.interpret_statement(then_branch);
                } else if let Some(else_branch) = else_branch {
                    self.interpret_statement(else_branch);
                }
            }
            Stmt::While { body, condition } => {
                while self.interpret_expression(condition.clone()).is_truthy() {
                    self.interpret_statement(body);
                }
            }
        }
    }

    fn interpret_expression(&mut self, expr: &Expr) -> Literal {
        match expr {
            Expr::Literal { val } => val.clone(),
            Expr::Grouping { expr } => self.interpret_expression(expr),
            Expr::Variable { name } => self.environment.get(&name.name()).unwrap_or(Literal::Nil),
            Expr::Unary { operator, right } => {
                let right = self.interpret_expression(right);
                match operator.kind {
                    TokenKind::Bang => Literal::Bool(!right.is_truthy()),
                    TokenKind::Minus => {
                        match right {
                            Literal::Number(num) => Literal::Number(-num),
                            _ => Literal::Nil, // TODO: fail here
                        }
                    }
                    _ => Literal::Nil, // TODO: fail here
                }
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = self.interpret_expression(left);
                let right = self.interpret_expression(right);
                match (&operator.kind, left, right) {
                    (TokenKind::Minus, Literal::Number(left), Literal::Number(right)) => {
                        Literal::Number(left - right)
                    }
                    (TokenKind::Star, Literal::Number(left), Literal::Number(right)) => {
                        Literal::Number(left * right)
                    }

                    // Plus is overloaded so we handle a few cases
                    (TokenKind::Plus, Literal::Number(left), Literal::Number(right)) => {
                        Literal::Number(left + right)
                    }
                    (TokenKind::Plus, Literal::Str(mut left), Literal::Str(right)) => {
                        left.push_str(&right);
                        Literal::Str(left)
                    }
                    (TokenKind::Plus, left, Literal::Str(right)) => {
                        let mut left = left.to_string();
                        left.push_str(&right);
                        Literal::Str(left)
                    }
                    (TokenKind::Plus, Literal::Str(mut left), right) => {
                        left.push_str(&right.to_string());
                        Literal::Str(left)
                    }

                    (TokenKind::Greater, Literal::Number(left), Literal::Number(right)) => {
                        Literal::Bool(left > right)
                    }
                    (TokenKind::GreaterEqual, Literal::Number(left), Literal::Number(right)) => {
                        Literal::Bool(left >= right)
                    }
                    (TokenKind::Less, Literal::Number(left), Literal::Number(right)) => {
                        Literal::Bool(left < right)
                    }
                    (TokenKind::LessEqual, Literal::Number(left), Literal::Number(right)) => {
                        Literal::Bool(left <= right)
                    }

                    (TokenKind::BangEqual, left, right) => Literal::Bool(left != right),
                    (TokenKind::EqualEqual, left, right) => Literal::Bool(left == right),

                    (operator, left, right) => {
                        println!(
                            "warning: operator {:?} cannot be applied to values of type {} and {}",
                            operator,
                            left.kind_name(),
                            right.kind_name()
                        );
                        Literal::Nil
                    }
                }
            }
            Expr::Assign { name, value } => {
                let value = self.interpret_expression(value);
                if let Err(_) = self
                    .environment
                    // TODO: every value is stored twice!!!!! oof
                    .assign(name.name(), value.clone())
                {
                    println!("variable {} not declared", name.name())
                }
                value
            }
            Expr::Logical {
                left,
                operator,
                right,
            } => {
                let left = self.interpret_expression(left);
                if let Token {
                    kind: TokenKind::Or,
                    ..
                } = operator
                {
                    if left.is_truthy() {
                        return left;
                    }
                } else {
                    if !left.is_truthy() {
                        return left;
                    }
                }

                self.interpret_expression(right)
            }
        }
    }
}
