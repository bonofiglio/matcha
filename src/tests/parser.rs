#[cfg(test)]
mod tests {
    use crate::{matcha::*, parser::*, scanner::*, source::*, statement::*, token::*};

    mod numeric_operators {
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
                            }
                        })),
                        operator: Token {
                            token_type: TokenType::Plus,
                            lexeme: "+",
                            line: 1,
                            position: 3,
                        },
                        right: Box::new(Expression::Literal(LiteralExpression {
                            value: Token {
                                token_type: TokenType::Integer,
                                lexeme: "1",
                                line: 1,
                                position: 5,
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
                                }
                            })),
                            operator: Token {
                                token_type: TokenType::Plus,
                                lexeme: "+",
                                line: 1,
                                position: 3,
                            },
                            right: Box::new(Expression::Literal(LiteralExpression {
                                value: Token {
                                    token_type: TokenType::Integer,
                                    lexeme: "1",
                                    line: 1,
                                    position: 5,
                                }
                            })),
                        })),
                        operator: Token {
                            token_type: TokenType::Plus,
                            lexeme: "+",
                            line: 1,
                            position: 7,
                        },
                        right: Box::new(Expression::Literal(LiteralExpression {
                            value: Token {
                                token_type: TokenType::Integer,
                                lexeme: "5",
                                line: 1,
                                position: 9,
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
                                    },
                                })),
                                operator: Token {
                                    token_type: TokenType::Star,
                                    lexeme: "*",
                                    line: 1,
                                    position: 3,
                                },
                                right: Box::new(Expression::Literal(LiteralExpression {
                                    value: Token {
                                        token_type: TokenType::Integer,
                                        lexeme: "2",
                                        line: 1,
                                        position: 5,
                                    },
                                })),
                            })),
                            operator: Token {
                                token_type: TokenType::Plus,
                                lexeme: "+",
                                line: 1,
                                position: 7,
                            },
                            right: Box::new(Expression::Binary(BinaryExpression {
                                left: Box::new(Expression::Literal(LiteralExpression {
                                    value: Token {
                                        token_type: TokenType::Integer,
                                        lexeme: "3",
                                        line: 1,
                                        position: 9,
                                    },
                                })),
                                operator: Token {
                                    token_type: TokenType::Slash,
                                    lexeme: "/",
                                    line: 1,
                                    position: 11,
                                },
                                right: Box::new(Expression::Literal(LiteralExpression {
                                    value: Token {
                                        token_type: TokenType::Integer,
                                        lexeme: "4",
                                        line: 1,
                                        position: 13,
                                    },
                                })),
                            })),
                        })),
                        operator: Token {
                            token_type: TokenType::Minus,
                            lexeme: "-",
                            line: 1,
                            position: 15,
                        },
                        right: Box::new(Expression::Binary(BinaryExpression {
                            left: Box::new(Expression::Literal(LiteralExpression {
                                value: Token {
                                    token_type: TokenType::Integer,
                                    lexeme: "5",
                                    line: 1,
                                    position: 17,
                                },
                            })),
                            operator: Token {
                                token_type: TokenType::Star,
                                lexeme: "*",
                                line: 1,
                                position: 19,
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
                                                        },
                                                    }
                                                )),
                                                operator: Token {
                                                    token_type: TokenType::Minus,
                                                    lexeme: "-",
                                                    line: 1,
                                                    position: 25,
                                                },
                                                right: Box::new(Expression::Literal(
                                                    LiteralExpression {
                                                        value: Token {
                                                            token_type: TokenType::Integer,
                                                            lexeme: "7",
                                                            line: 1,
                                                            position: 27,
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
                                                        },
                                                    }
                                                )),
                                                operator: Token {
                                                    token_type: TokenType::Plus,
                                                    lexeme: "+",
                                                    line: 1,
                                                    position: 35,
                                                },
                                                right: Box::new(Expression::Literal(
                                                    LiteralExpression {
                                                        value: Token {
                                                            token_type: TokenType::Integer,
                                                            lexeme: "9",
                                                            line: 1,
                                                            position: 37,
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
                            }
                        })),
                        operator: Token {
                            token_type: TokenType::Star,
                            lexeme: "*",
                            line: 2,
                            position: 3,
                        },
                        right: Box::new(Expression::Literal(LiteralExpression {
                            value: Token {
                                token_type: TokenType::Integer,
                                lexeme: "2",
                                line: 2,
                                position: 5,
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
                            }
                        })),
                        operator: Token {
                            token_type: TokenType::Slash,
                            lexeme: "/",
                            line: 3,
                            position: 7,
                        },
                        right: Box::new(Expression::Literal(LiteralExpression {
                            value: Token {
                                token_type: TokenType::Integer,
                                lexeme: "4",
                                line: 3,
                                position: 9,
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
                                }
                            })),
                            operator: Token {
                                token_type: TokenType::Plus,
                                lexeme: "+",
                                line: 4,
                                position: 2,
                            },
                            right: Box::new(Expression::Literal(LiteralExpression {
                                value: Token {
                                    token_type: TokenType::Integer,
                                    lexeme: "6",
                                    line: 4,
                                    position: 3,
                                }
                            })),
                        })),
                        operator: Token {
                            token_type: TokenType::Minus,
                            lexeme: "-",
                            line: 4,
                            position: 4,
                        },
                        right: Box::new(Expression::Literal(LiteralExpression {
                            value: Token {
                                token_type: TokenType::Integer,
                                lexeme: "2",
                                line: 4,
                                position: 5,
                            }
                        })),
                    }))
                ]
            );
        }
    }

    mod variables {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_parses_a_single_variable_declaration() {
            let tokens = Scanner {
                source: Source::new("my_variable := 15;"),
            }
            .scan()
            .unwrap();

            let parser_result = Parser::new(tokens).parse().unwrap();

            assert_eq!(
                parser_result,
                vec![Statement::VariableDeclaration(VariableDeclaration {
                    identifier: Token {
                        token_type: TokenType::Identifier,
                        line: 1,
                        position: 1,
                        lexeme: "my_variable",
                    },
                    initializer: Expression::Literal(LiteralExpression {
                        value: Token {
                            token_type: TokenType::Integer,
                            lexeme: "15",
                            line: 1,
                            position: 16,
                        }
                    }),
                    r#type: None
                })]
            );
        }

        #[test]
        fn it_parses_multiple_variable_declarations() {
            let tokens = Scanner {
                source: Source::new(
                    r#"a_number := 1;
                       string := "abc";
                    "#,
                ),
            }
            .scan()
            .unwrap();

            let parser_result = Parser::new(tokens).parse().unwrap();

            assert_eq!(
                parser_result,
                vec![
                    Statement::VariableDeclaration(VariableDeclaration {
                        identifier: Token {
                            token_type: TokenType::Identifier,
                            line: 1,
                            position: 1,
                            lexeme: "a_number",
                        },
                        initializer: Expression::Literal(LiteralExpression {
                            value: Token {
                                token_type: TokenType::Integer,
                                lexeme: "1",
                                line: 1,
                                position: 13,
                            }
                        }),
                        r#type: None
                    }),
                    Statement::VariableDeclaration(VariableDeclaration {
                        identifier: Token {
                            token_type: TokenType::Identifier,
                            line: 2,
                            position: 24,
                            lexeme: "string",
                        },
                        initializer: Expression::Literal(LiteralExpression {
                            value: Token {
                                token_type: TokenType::String,
                                lexeme: "\"abc\"",
                                line: 2,
                                position: 34,
                            }
                        }),
                        r#type: None
                    })
                ]
            );
        }

        #[test]
        fn it_parses_a_declaration_with_an_explicit_type() {
            let tokens = Scanner {
                source: Source::new("my_variable : i32 = 15;"),
            }
            .scan()
            .unwrap();

            let parser_result = Parser::new(tokens).parse().unwrap();

            assert_eq!(
                parser_result,
                vec![Statement::VariableDeclaration(VariableDeclaration {
                    identifier: Token {
                        token_type: TokenType::Identifier,
                        line: 1,
                        position: 1,
                        lexeme: "my_variable",
                    },
                    initializer: Expression::Literal(LiteralExpression {
                        value: Token {
                            token_type: TokenType::Integer,
                            lexeme: "15",
                            line: 1,
                            position: 21,
                        }
                    }),
                    r#type: Some(Token {
                        token_type: TokenType::Identifier,
                        lexeme: "i32",
                        line: 1,
                        position: 15,
                    })
                })]
            );
        }

        #[test]
        fn it_parses_many_declarations_with_mixed_typing() {
            let tokens = Scanner {
                source: Source::new(
                    "var1 : i32 = 1; var2:a_type=2;
var3 := 16;
var_4: u64 = var1;
var_5 :u =
    var_4;",
                ),
            }
            .scan()
            .unwrap();

            let parser_result = Parser::new(tokens).parse().unwrap();

            assert_eq!(
                parser_result,
                vec![
                    Statement::VariableDeclaration(VariableDeclaration {
                        identifier: Token {
                            token_type: TokenType::Identifier,
                            line: 1,
                            position: 1,
                            lexeme: "var1",
                        },
                        r#type: Some(Token {
                            token_type: TokenType::Identifier,
                            lexeme: "i32",
                            line: 1,
                            position: 8,
                        }),
                        initializer: Expression::Literal(LiteralExpression {
                            value: Token {
                                token_type: TokenType::Integer,
                                lexeme: "1",
                                line: 1,
                                position: 14,
                            }
                        }),
                    }),
                    Statement::VariableDeclaration(VariableDeclaration {
                        identifier: Token {
                            token_type: TokenType::Identifier,
                            line: 1,
                            position: 17,
                            lexeme: "var2",
                        },
                        r#type: Some(Token {
                            token_type: TokenType::Identifier,
                            lexeme: "a_type",
                            line: 1,
                            position: 22,
                        }),
                        initializer: Expression::Literal(LiteralExpression {
                            value: Token {
                                token_type: TokenType::Integer,
                                lexeme: "2",
                                line: 1,
                                position: 29,
                            }
                        }),
                    }),
                    Statement::VariableDeclaration(VariableDeclaration {
                        identifier: Token {
                            token_type: TokenType::Identifier,
                            line: 2,
                            position: 1,
                            lexeme: "var3",
                        },
                        r#type: None,
                        initializer: Expression::Literal(LiteralExpression {
                            value: Token {
                                token_type: TokenType::Integer,
                                lexeme: "16",
                                line: 2,
                                position: 9,
                            }
                        }),
                    }),
                    Statement::VariableDeclaration(VariableDeclaration {
                        identifier: Token {
                            token_type: TokenType::Identifier,
                            line: 3,
                            position: 1,
                            lexeme: "var_4",
                        },
                        r#type: Some(Token {
                            token_type: TokenType::Identifier,
                            lexeme: "u64",
                            line: 3,
                            position: 8,
                        }),
                        initializer: Expression::Variable(VariableExpression {
                            value: Token {
                                token_type: TokenType::Identifier,
                                lexeme: "var1",
                                line: 3,
                                position: 14,
                            }
                        }),
                    }),
                    Statement::VariableDeclaration(VariableDeclaration {
                        identifier: Token {
                            token_type: TokenType::Identifier,
                            line: 4,
                            position: 1,
                            lexeme: "var_5",
                        },
                        r#type: Some(Token {
                            token_type: TokenType::Identifier,
                            lexeme: "u",
                            line: 4,
                            position: 8,
                        }),
                        initializer: Expression::Variable(VariableExpression {
                            value: Token {
                                token_type: TokenType::Identifier,
                                lexeme: "var_4",
                                line: 5,
                                position: 5,
                            }
                        }),
                    })
                ]
            );
        }

        #[test]
        fn it_parses_a_single_variable_assignment() {
            let tokens = Scanner {
                source: Source::new("my_variable = 15;"),
            }
            .scan()
            .unwrap();

            let parser_result = Parser::new(tokens).parse().unwrap();

            assert_eq!(
                parser_result,
                vec![Statement::Expression(Expression::Assignment(
                    AssignmentExpression {
                        identifier: Token {
                            token_type: TokenType::Identifier,
                            line: 1,
                            position: 1,
                            lexeme: "my_variable",
                        },
                        value: Box::new(Expression::Literal(LiteralExpression {
                            value: Token {
                                token_type: TokenType::Integer,
                                lexeme: "15",
                                line: 1,
                                position: 15,
                            }
                        }))
                    }
                ))]
            );
        }

        #[test]
        fn it_parses_a_multiple_variable_assignments() {
            let tokens = Scanner {
                source: Source::new("var1 = 15;var2=3; var3= 4;"),
            }
            .scan()
            .unwrap();

            let parser_result = Parser::new(tokens).parse().unwrap();

            assert_eq!(
                parser_result,
                vec![
                    Statement::Expression(Expression::Assignment(AssignmentExpression {
                        identifier: Token {
                            token_type: TokenType::Identifier,
                            line: 1,
                            position: 1,
                            lexeme: "var1",
                        },
                        value: Box::new(Expression::Literal(LiteralExpression {
                            value: Token {
                                token_type: TokenType::Integer,
                                lexeme: "15",
                                line: 1,
                                position: 8,
                            }
                        }))
                    })),
                    Statement::Expression(Expression::Assignment(AssignmentExpression {
                        identifier: Token {
                            token_type: TokenType::Identifier,
                            line: 1,
                            position: 11,
                            lexeme: "var2",
                        },
                        value: Box::new(Expression::Literal(LiteralExpression {
                            value: Token {
                                token_type: TokenType::Integer,
                                lexeme: "3",
                                line: 1,
                                position: 16,
                            }
                        }))
                    })),
                    Statement::Expression(Expression::Assignment(AssignmentExpression {
                        identifier: Token {
                            token_type: TokenType::Identifier,
                            line: 1,
                            position: 19,
                            lexeme: "var3",
                        },
                        value: Box::new(Expression::Literal(LiteralExpression {
                            value: Token {
                                token_type: TokenType::Integer,
                                lexeme: "4",
                                line: 1,
                                position: 25,
                            }
                        }))
                    })),
                ]
            );
        }
    }
}
