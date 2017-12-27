use lexer::{Operator, Token};

#[derive(Debug, PartialEq, Eq)]
pub enum ShuntOperator {
    Plus,
    Minus,
    Times,
    Divide,
    Exponent,
    UnaryPlus,
    UnaryMinus,
    OpenParen,
    CloseParen,
}

#[derive(Debug, PartialEq, Eq)]
pub enum BinaryOperatorKind {
    Plus,
    Minus,
    Times,
    Divide,
    Exponent,
}

#[derive(Debug, PartialEq, Eq)]
pub enum UnaryOperatorKind {
    Plus,
    Minus,
}

impl ShuntOperator {
    pub fn from_lex_operator(operator: &Operator) -> ShuntOperator {
        match *operator {
            Operator::Plus => ShuntOperator::Plus,
            Operator::Minus => ShuntOperator::Minus,
            Operator::Times => ShuntOperator::Times,
            Operator::Divide => ShuntOperator::Divide,
            Operator::Exponent => ShuntOperator::Exponent,
        }
    }

    pub fn precedence(&self) -> u8 {
        match *self {
            ShuntOperator::Plus | ShuntOperator::Minus => 1,
            ShuntOperator::Times | ShuntOperator::Divide => 2,
            ShuntOperator::Exponent => 3,
            ShuntOperator::UnaryPlus | ShuntOperator::UnaryMinus => 254,
            _ => 0,
        }
    }

    pub fn to_binary_operator(&self) -> Option<BinaryOperatorKind> {
        match *self {
            ShuntOperator::Plus => Some(BinaryOperatorKind::Plus),
            ShuntOperator::Minus => Some(BinaryOperatorKind::Minus),
            ShuntOperator::Times => Some(BinaryOperatorKind::Times),
            ShuntOperator::Divide => Some(BinaryOperatorKind::Divide),
            ShuntOperator::Exponent => Some(BinaryOperatorKind::Exponent),
            _ => None,
        }
    }

    pub fn to_unary_operator(&self) -> Option<UnaryOperatorKind> {
        match *self {
            ShuntOperator::UnaryPlus => Some(UnaryOperatorKind::Plus),
            ShuntOperator::UnaryMinus => Some(UnaryOperatorKind::Minus),
            _ => None,
        }
    }

    pub fn is_unary(&self) -> bool {
        match *self {
            ShuntOperator::UnaryPlus | ShuntOperator::UnaryMinus => true,
            _ => false,
        }
    }

    pub fn is_left_associative(&self) -> bool {
        match *self {
            ShuntOperator::Exponent => false,
            _ => true,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum AstNode {
    Constant {
        value: i64,
    },
    BinaryOperator {
        kind: BinaryOperatorKind,
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    UnaryOperator {
        kind: UnaryOperatorKind,
        value: Box<AstNode>,
    },
}

struct ShuntingState {
    pub operator_stack: Vec<ShuntOperator>,
    pub operand_stack: Vec<AstNode>,
    pub last_was_operator: bool,
}

impl ShuntingState {
    pub fn new() -> ShuntingState {
        ShuntingState {
            operator_stack: Vec::new(),
            operand_stack: Vec::new(),
            last_was_operator: true,
        }
    }

    pub fn clear_one_operator(&mut self) {
        let top_operator = match self.operator_stack.pop() {
            Some(v) => v,
            None => {
                eprintln!("Tried to pop operator, but couldn't!");
                return;
            },
        };

        let right = match self.operand_stack.pop() {
            Some(v) => Box::new(v),
            None => {
                eprintln!("Tried to pop right operand, but couldn't!");
                return;
            },
        };

        if top_operator.is_unary() {
            self.operand_stack.push(AstNode::UnaryOperator {
                kind: top_operator.to_unary_operator().unwrap(),
                value: right,
            });
        } else {
            let left = match self.operand_stack.pop() {
                Some(v) => Box::new(v),
                None => {
                    eprintln!("Tried to pop left operand, but couldn't!");
                    return;
                },
            };

            self.operand_stack.push(AstNode::BinaryOperator {
                kind: top_operator.to_binary_operator().unwrap(),
                left,
                right,
            });
        }
    }
}

pub fn parse_expression(mut tokens: &[Token]) -> Option<AstNode> {
    let mut state = ShuntingState::new();

    loop {
        let token = match tokens.first() {
            Some(token) => token,
            None => break,
        };

        tokens = &tokens[1..];

        match *token {
            Token::Constant(value) => {
                state.operand_stack.push(AstNode::Constant {
                    value
                });

                state.last_was_operator = false;
            },
            Token::Operator(lex_operator) => {
                let operator = match lex_operator {
                    Operator::Plus => {
                        match state.last_was_operator {
                            true => ShuntOperator::UnaryPlus,
                            false => ShuntOperator::Plus,
                        }
                    },
                    Operator::Minus => {
                        match state.last_was_operator {
                            true => ShuntOperator::UnaryMinus,
                            false => ShuntOperator::Minus,
                        }
                    },
                    _ => ShuntOperator::from_lex_operator(&lex_operator),
                };

                if !operator.is_unary() {
                    if operator.is_left_associative() {
                        // Clear all operators of higher precedence
                        loop {
                            match state.operator_stack.last() {
                                Some(top_operator) => {
                                    if operator.precedence() > top_operator.precedence() {
                                        break;
                                    }
                                },
                                None => break,
                            }

                            state.clear_one_operator();
                        }
                    } else {
                        // Clear all operators of higher or equal precedence
                        loop {
                            match state.operator_stack.last() {
                                Some(top_operator) => {
                                    if operator.precedence() >= top_operator.precedence() {
                                        break;
                                    }
                                },
                                None => break,
                            }

                            state.clear_one_operator();
                        }
                    }
                }

                state.operator_stack.push(operator);
                state.last_was_operator = true;
            },
            Token::OpenParen => {
                state.operator_stack.push(ShuntOperator::OpenParen);
                state.last_was_operator = true;
            },
            Token::CloseParen => {
                // clear up until we reach an OpenParen, then pop that too
                loop {
                    match state.operator_stack.last() {
                        Some(&ShuntOperator::OpenParen) => {
                            state.operator_stack.pop();
                            break;
                        },
                        Some(_) => {},
                        None => break,
                    }

                    state.clear_one_operator();
                }

                state.last_was_operator = false;
            },
        }
    }

    while !state.operator_stack.is_empty() {
        state.clear_one_operator();
    }

    match state.operand_stack.is_empty() {
        true => None,
        false => Some(state.operand_stack.pop().unwrap()),
    }
}
