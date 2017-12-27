use lexer::{Operator, Token};

#[derive(Debug, PartialEq, Eq)]
pub enum ShuntOperator {
    Plus,
    Minus,
    Times,
    Divide,
    // UnaryPlus,
    // UnaryMinus,
    OpenParen,
    CloseParen,
}

#[derive(Debug, PartialEq, Eq)]
pub enum BinaryOperatorKind {
    Plus,
    Minus,
    Times,
    Divide,
}

impl ShuntOperator {
    pub fn precedence(&self) -> u8 {
        match *self {
            ShuntOperator::Plus | ShuntOperator::Minus => 1,
            ShuntOperator::Times | ShuntOperator::Divide => 2,
            _ => 0,
        }
    }

    pub fn to_binary_operator(&self) -> Option<BinaryOperatorKind> {
        match *self {
            ShuntOperator::Plus => Some(BinaryOperatorKind::Plus),
            ShuntOperator::Minus => Some(BinaryOperatorKind::Minus),
            ShuntOperator::Times => Some(BinaryOperatorKind::Times),
            ShuntOperator::Divide => Some(BinaryOperatorKind::Divide),
            _ => None,
        }
    }

    pub fn is_left_associative(&self) -> bool {
        match *self {
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
    // UnaryOperator {
    //     kind: UnaryOperatorKind,
    //     value: Box<AstNode>,
    // },
}

struct ShuntingState {
    pub operator_stack: Vec<ShuntOperator>,
    pub operand_stack: Vec<AstNode>,
}

impl ShuntingState {
    pub fn new() -> ShuntingState {
        ShuntingState {
            operator_stack: Vec::new(),
            operand_stack: Vec::new(),
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
            },
            Token::Operator(operator) => {
                let operator = match operator {
                    Operator::Plus => ShuntOperator::Plus,
                    Operator::Minus => ShuntOperator::Minus,
                    Operator::Times => ShuntOperator::Times,
                    Operator::Divide => ShuntOperator::Divide,
                };

                // clear all operators of higher precedence from the stack
                loop {
                    {
                        let top_operator = match state.operator_stack.last() {
                            Some(operator) => operator,
                            None => break,
                        };

                        if operator.precedence() >= top_operator.precedence() {
                            break;
                        }
                    }

                    let top_operator = state.operator_stack.pop().unwrap();
                    let right = Box::new(state.operand_stack.pop().unwrap());
                    let left = Box::new(state.operand_stack.pop().unwrap());

                    state.operand_stack.push(AstNode::BinaryOperator {
                        kind: top_operator.to_binary_operator().unwrap(),
                        left,
                        right,
                    });
                }

                state.operator_stack.push(operator);
            },
            Token::OpenParen => {
                state.operator_stack.push(ShuntOperator::OpenParen);
            },
            Token::CloseParen => {

            },
        }
    }

    while !state.operator_stack.is_empty() {
        let top_operator = state.operator_stack.pop().unwrap();
        let right = Box::new(state.operand_stack.pop().unwrap());
        let left = Box::new(state.operand_stack.pop().unwrap());

        state.operand_stack.push(AstNode::BinaryOperator {
            kind: top_operator.to_binary_operator().unwrap(),
            left,
            right,
        });
    }

    if state.operand_stack.is_empty() {
        None
    } else {
        Some(state.operand_stack.pop().unwrap())
    }
}
