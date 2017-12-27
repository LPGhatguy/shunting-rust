use lexer::{OperatorKind, Token};

#[derive(Debug, PartialEq, Eq)]
pub enum AstNode {
    Constant {
        value: i64,
    },
    BinaryOperator {
        kind: OperatorKind,
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    UnaryOperator {
        kind: OperatorKind,
        value: Box<AstNode>,
    },
}

pub fn parse_expression(mut tokens: &[Token]) -> Option<AstNode> {
    let mut operator_stack: Vec<OperatorKind> = Vec::new();
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
                        kind: top_operator,
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
            kind: top_operator,
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
