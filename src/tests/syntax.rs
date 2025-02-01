#[cfg(test)]
mod tests {
    use crate::{matcha::*, parser::*, scanner::*, source::*, statement::*, token::*};

    mod math {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_parses_simple_expressions() {
            let tokens = Scanner {
                source: Source::new("1 + 1;"),
            }
            .scan()
            .unwrap();
            let parser_result = Parser::new(tokens).parse().unwrap();

            assert_eq!(
                parser_result,
                vec![Statement::Expression(Expression::Binary(
                    BinaryExpression {
                        left: Box::new(Expression::Literal(LiteralExpression {
                            value: Token {
                                token_type: TokenType::Integer,
                                lexeme: "1",
                                line: 1,
                                position: 1,
                                literal: Some(Literal::Number(NumberLiteral::Integer(1)))
                            }
                        })),
                        operator: Token {
                            token_type: TokenType::Plus,
                            lexeme: "+",
                            line: 1,
                            position: 3,
                            literal: None
                        },
                        right: Box::new(Expression::Literal(LiteralExpression {
                            value: Token {
                                token_type: TokenType::Integer,
                                lexeme: "1",
                                line: 1,
                                position: 5,
                                literal: Some(Literal::Number(NumberLiteral::Integer(1)))
                            }
                        })),
                    }
                ))]
            );
        }

        #[test]
        fn it_parses_multiple_expressions() {
            let tokens = Scanner {
                source: Source::new("1 + 1 + 5;"),
            }
            .scan()
            .unwrap();
            let parser_result = Parser::new(tokens).parse().unwrap();

            assert_eq!(
                parser_result,
                vec![Statement::Expression(Expression::Binary(
                    BinaryExpression {
                        left: Box::new(Expression::Binary(BinaryExpression {
                            left: Box::new(Expression::Literal(LiteralExpression {
                                value: Token {
                                    token_type: TokenType::Integer,
                                    lexeme: "1",
                                    line: 1,
                                    position: 1,
                                    literal: Some(Literal::Number(NumberLiteral::Integer(1)))
                                }
                            })),
                            operator: Token {
                                token_type: TokenType::Plus,
                                lexeme: "+",
                                line: 1,
                                position: 3,
                                literal: None
                            },
                            right: Box::new(Expression::Literal(LiteralExpression {
                                value: Token {
                                    token_type: TokenType::Integer,
                                    lexeme: "1",
                                    line: 1,
                                    position: 5,
                                    literal: Some(Literal::Number(NumberLiteral::Integer(1)))
                                }
                            })),
                        })),
                        operator: Token {
                            token_type: TokenType::Plus,
                            lexeme: "+",
                            line: 1,
                            position: 7,
                            literal: None
                        },
                        right: Box::new(Expression::Literal(LiteralExpression {
                            value: Token {
                                token_type: TokenType::Integer,
                                lexeme: "5",
                                line: 1,
                                position: 9,
                                literal: Some(Literal::Number(NumberLiteral::Integer(5)))
                            }
                        })),
                    }
                ))]
            );
        }

        #[test]
        fn it_respects_the_order_of_operations() {
            let tokens = Scanner {
                source: Source::new("1 * 2 + 3 / 4 - 5 * ((6 - 7) / (8 + 9));"),
            }
            .scan()
            .unwrap();
            let parser_result = Parser::new(tokens).parse().unwrap();

            assert_eq!(
                parser_result,
                vec![Statement::Expression(Expression::Binary(
                    BinaryExpression {
                        left: Box::new(Expression::Binary(BinaryExpression {
                            left: Box::new(Expression::Binary(BinaryExpression {
                                left: Box::new(Expression::Literal(LiteralExpression {
                                    value: Token {
                                        token_type: TokenType::Integer,
                                        lexeme: "1",
                                        line: 1,
                                        position: 1,
                                        literal: Some(Literal::Number(NumberLiteral::Integer(1))),
                                    },
                                })),
                                operator: Token {
                                    token_type: TokenType::Star,
                                    lexeme: "*",
                                    line: 1,
                                    position: 3,
                                    literal: None,
                                },
                                right: Box::new(Expression::Literal(LiteralExpression {
                                    value: Token {
                                        token_type: TokenType::Integer,
                                        lexeme: "2",
                                        line: 1,
                                        position: 5,
                                        literal: Some(Literal::Number(NumberLiteral::Integer(2))),
                                    },
                                })),
                            })),
                            operator: Token {
                                token_type: TokenType::Plus,
                                lexeme: "+",
                                line: 1,
                                position: 7,
                                literal: None,
                            },
                            right: Box::new(Expression::Binary(BinaryExpression {
                                left: Box::new(Expression::Literal(LiteralExpression {
                                    value: Token {
                                        token_type: TokenType::Integer,
                                        lexeme: "3",
                                        line: 1,
                                        position: 9,
                                        literal: Some(Literal::Number(NumberLiteral::Integer(3))),
                                    },
                                })),
                                operator: Token {
                                    token_type: TokenType::Slash,
                                    lexeme: "/",
                                    line: 1,
                                    position: 11,
                                    literal: None,
                                },
                                right: Box::new(Expression::Literal(LiteralExpression {
                                    value: Token {
                                        token_type: TokenType::Integer,
                                        lexeme: "4",
                                        line: 1,
                                        position: 13,
                                        literal: Some(Literal::Number(NumberLiteral::Integer(4))),
                                    },
                                })),
                            })),
                        })),
                        operator: Token {
                            token_type: TokenType::Minus,
                            lexeme: "-",
                            line: 1,
                            position: 15,
                            literal: None,
                        },
                        right: Box::new(Expression::Binary(BinaryExpression {
                            left: Box::new(Expression::Literal(LiteralExpression {
                                value: Token {
                                    token_type: TokenType::Integer,
                                    lexeme: "5",
                                    line: 1,
                                    position: 17,
                                    literal: Some(Literal::Number(NumberLiteral::Integer(5))),
                                },
                            })),
                            operator: Token {
                                token_type: TokenType::Star,
                                lexeme: "*",
                                line: 1,
                                position: 19,
                                literal: None,
                            },
                            right: Box::new(Expression::Grouping(GroupingExpression {
                                expression: Box::new(Expression::Binary(BinaryExpression {
                                    left: Box::new(Expression::Grouping(GroupingExpression {
                                        expression: Box::new(Expression::Binary(
                                            BinaryExpression {
                                                left: Box::new(Expression::Literal(
                                                    LiteralExpression {
                                                        value: Token {
                                                            token_type: TokenType::Integer,
                                                            lexeme: "6",
                                                            line: 1,
                                                            position: 23,
                                                            literal: Some(Literal::Number(
                                                                NumberLiteral::Integer(6),
                                                            )),
                                                        },
                                                    }
                                                )),
                                                operator: Token {
                                                    token_type: TokenType::Minus,
                                                    lexeme: "-",
                                                    line: 1,
                                                    position: 25,
                                                    literal: None,
                                                },
                                                right: Box::new(Expression::Literal(
                                                    LiteralExpression {
                                                        value: Token {
                                                            token_type: TokenType::Integer,
                                                            lexeme: "7",
                                                            line: 1,
                                                            position: 27,
                                                            literal: Some(Literal::Number(
                                                                NumberLiteral::Integer(7),
                                                            )),
                                                        },
                                                    }
                                                )),
                                            }
                                        )),
                                    })),
                                    operator: Token {
                                        token_type: TokenType::Slash,
                                        lexeme: "/",
                                        line: 1,
                                        position: 30,
                                        literal: None,
                                    },
                                    right: Box::new(Expression::Grouping(GroupingExpression {
                                        expression: Box::new(Expression::Binary(
                                            BinaryExpression {
                                                left: Box::new(Expression::Literal(
                                                    LiteralExpression {
                                                        value: Token {
                                                            token_type: TokenType::Integer,
                                                            lexeme: "8",
                                                            line: 1,
                                                            position: 33,
                                                            literal: Some(Literal::Number(
                                                                NumberLiteral::Integer(8),
                                                            )),
                                                        },
                                                    }
                                                )),
                                                operator: Token {
                                                    token_type: TokenType::Plus,
                                                    lexeme: "+",
                                                    line: 1,
                                                    position: 35,
                                                    literal: None,
                                                },
                                                right: Box::new(Expression::Literal(
                                                    LiteralExpression {
                                                        value: Token {
                                                            token_type: TokenType::Integer,
                                                            lexeme: "9",
                                                            line: 1,
                                                            position: 37,
                                                            literal: Some(Literal::Number(
                                                                NumberLiteral::Integer(9),
                                                            ))
                                                        },
                                                    }
                                                )),
                                            }
                                        )),
                                    })),
                                })),
                            })),
                        })),
                    },
                ))]
            );
        }

        #[test]
        fn it_works_with_multiple_lines() {
            let tokens = Scanner {
                source: Source::new(
                    "
1 * 2;
    3 / 4;
5+6-2;",
                ),
            }
            .scan()
            .unwrap();

            let parser_result = Parser::new(tokens).parse().unwrap();

            assert_eq!(
                parser_result,
                vec![
                    Statement::Expression(Expression::Binary(BinaryExpression {
                        left: Box::new(Expression::Literal(LiteralExpression {
                            value: Token {
                                token_type: TokenType::Integer,
                                lexeme: "1",
                                line: 2,
                                position: 1,
                                literal: Some(Literal::Number(NumberLiteral::Integer(1)))
                            }
                        })),
                        operator: Token {
                            token_type: TokenType::Star,
                            lexeme: "*",
                            line: 2,
                            position: 3,
                            literal: None
                        },
                        right: Box::new(Expression::Literal(LiteralExpression {
                            value: Token {
                                token_type: TokenType::Integer,
                                lexeme: "2",
                                line: 2,
                                position: 5,
                                literal: Some(Literal::Number(NumberLiteral::Integer(2)))
                            }
                        })),
                    })),
                    Statement::Expression(Expression::Binary(BinaryExpression {
                        left: Box::new(Expression::Literal(LiteralExpression {
                            value: Token {
                                token_type: TokenType::Integer,
                                lexeme: "3",
                                line: 3,
                                position: 5,
                                literal: Some(Literal::Number(NumberLiteral::Integer(3)))
                            }
                        })),
                        operator: Token {
                            token_type: TokenType::Slash,
                            lexeme: "/",
                            line: 3,
                            position: 7,
                            literal: None
                        },
                        right: Box::new(Expression::Literal(LiteralExpression {
                            value: Token {
                                token_type: TokenType::Integer,
                                lexeme: "4",
                                line: 3,
                                position: 9,
                                literal: Some(Literal::Number(NumberLiteral::Integer(4)))
                            }
                        })),
                    })),
                    Statement::Expression(Expression::Binary(BinaryExpression {
                        left: Box::new(Expression::Binary(BinaryExpression {
                            left: Box::new(Expression::Literal(LiteralExpression {
                                value: Token {
                                    token_type: TokenType::Integer,
                                    lexeme: "5",
                                    line: 4,
                                    position: 1,
                                    literal: Some(Literal::Number(NumberLiteral::Integer(5)))
                                }
                            })),
                            operator: Token {
                                token_type: TokenType::Plus,
                                lexeme: "+",
                                line: 4,
                                position: 2,
                                literal: None
                            },
                            right: Box::new(Expression::Literal(LiteralExpression {
                                value: Token {
                                    token_type: TokenType::Integer,
                                    lexeme: "6",
                                    line: 4,
                                    position: 3,
                                    literal: Some(Literal::Number(NumberLiteral::Integer(6)))
                                }
                            })),
                        })),
                        operator: Token {
                            token_type: TokenType::Minus,
                            lexeme: "-",
                            line: 4,
                            position: 4,
                            literal: None
                        },
                        right: Box::new(Expression::Literal(LiteralExpression {
                            value: Token {
                                token_type: TokenType::Integer,
                                lexeme: "2",
                                line: 4,
                                position: 5,
                                literal: Some(Literal::Number(NumberLiteral::Integer(2)))
                            }
                        })),
                    }))
                ]
            );
        }
    }
}
