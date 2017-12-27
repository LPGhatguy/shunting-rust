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

pub fn parse_expression(mut tokens: &[Token]) -> Option<AstNode> {
    let mut operator_stack: Vec<ShuntOperator> = Vec::new();
    let mut operand_stack: Vec<AstNode> = Vec::new();

    loop {
        let token = match tokens.first() {
            Some(token) => token,
            None => break,
        };

        tokens = &tokens[1..];

        match *token {
            Token::Constant(value) => {
                operand_stack.push(AstNode::Constant {
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
                        let top_operator = match operator_stack.last() {
                            Some(operator) => operator,
                            None => break,
                        };

                        if operator.precedence() >= top_operator.precedence() {
                            break;
                        }
                    }

                    let top_operator = operator_stack.pop().unwrap();
                    let right = Box::new(operand_stack.pop().unwrap());
                    let left = Box::new(operand_stack.pop().unwrap());

                    operand_stack.push(AstNode::BinaryOperator {
                        kind: top_operator.to_binary_operator().unwrap(),
                        left,
                        right,
                    });
                }

                operator_stack.push(operator);
            },
            _ => unimplemented!(),
        }
    }

    while !operator_stack.is_empty() {
        let top_operator = operator_stack.pop().unwrap();
        let right = Box::new(operand_stack.pop().unwrap());
        let left = Box::new(operand_stack.pop().unwrap());

        operand_stack.push(AstNode::BinaryOperator {
            kind: top_operator.to_binary_operator().unwrap(),
            left,
            right,
        });
    }

    if operand_stack.is_empty() {
        None
    } else {
        Some(operand_stack.pop().unwrap())
    }
}
