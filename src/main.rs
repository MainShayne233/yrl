#[macro_use]
extern crate lalrpop_util;

use crate::ast::{Declaration, DeclarationType, Grammar, Node, NodeExpression};
use crate::preprocess::*;
use pretty_assertions::assert_eq;
use std::fs;

lalrpop_mod!(pub grammar);

pub mod ast;
pub mod preprocess;

fn parse_grammar(input: &str) -> Grammar {
    grammar::GrammarParser::new()
        .parse(&strip_extra(input))
        .unwrap()
}

#[test]
fn test_nonterminals_parser() {
    let input = r#"
Nonterminals
  grammar expr_list
  expr container_expr block_expr access_expr
  number
  .

Terminals
  paren_identifier do_identifier block_identifier
  'true' 'false' 'nil' 'do' eol ';' ',' '.'
  '(' ')' '[' ']' '{' '}' '<<' '>>' '%{}' '%'
  .

Rootsymbol grammar. % sample past code comment

%% Two shift/reduce conflicts coming from call_args_parens and
%% one coming from empty_paren on stab.
Expect 3.

Left       5 do.
Right     10 stab_op_eol.     %% ->
Left      20 ','.
Left      40 in_match_op_eol. %% <-, \\ (allowed in matches along =)
Nonassoc 300 unary_op_eol.    %% +, -, !, ^, not, ~~~

%%% MAIN FLOW OF EXPRESSIONS

expr -> matched_expr : '$1'.
grammar -> eoe : {'__block__', meta_from_token('$1'), []}.
grammar -> '$empty' : {'__block__', [], []}.
expr_list -> expr_list eoe expr : ['$1', '$2' | cool('$3')].
matched_expr -> matched_expr matched_op_expr : build_op(element(1, '$2'), '$1', element(2, '$2')).
assoc_expr -> dot_identifier : build_identifier('$1', nil).
access_expr -> 'true' : handle_literal(?id('$1'), '$1').
access_expr -> bin_string : build_bin_string('$1', delimiter(<<$">>)).
access_expr -> atom : handle_literal(?exprs('$1'), '$1', delimiter(<<$:>>)).
access_expr -> atom : handle_literal(?exprs('$1'), '$1', delimiter(<<$'>>)).
% Also used by maps and structs
"#;

    let grammar = parse_grammar(input);
    assert_eq!(
        grammar.nodes,
        vec![
            Node {
                lhs: "expr".to_string(),
                rhs: vec!["matched_expr".to_string()],
                expressions: vec![NodeExpression::Charlist {
                    value: "\'$1\'".to_string()
                }]
            },
            Node {
                lhs: "grammar".to_string(),
                rhs: vec!["eoe".to_string()],
                expressions: vec![NodeExpression::Tuple {
                    values: Box::new(vec![
                        NodeExpression::Charlist {
                            value: "'__block__'".to_string()
                        },
                        NodeExpression::FunctionCall {
                            name: "meta_from_token".to_string(),
                            args: Box::new(vec![NodeExpression::Charlist {
                                value: "'$1'".to_string()
                            }])
                        },
                        NodeExpression::List {
                            values: Box::new(vec![])
                        }
                    ])
                }]
            },
            Node {
                lhs: "grammar".to_string(),
                rhs: vec!["\'$empty\'".to_string(),],
                expressions: vec![NodeExpression::Tuple {
                    values: Box::new(vec![
                        NodeExpression::Charlist {
                            value: "\'__block__\'".to_string(),
                        },
                        NodeExpression::List {
                            values: Box::new(vec![])
                        },
                        NodeExpression::List {
                            values: Box::new(vec![])
                        },
                    ]),
                },]
            },
            Node {
                lhs: "expr_list".to_string(),
                rhs: vec![
                    "expr_list".to_string(),
                    "eoe".to_string(),
                    "expr".to_string(),
                ],
                expressions: vec![NodeExpression::HeadTailList {
                    head: Box::new(vec![
                        NodeExpression::Charlist {
                            value: "\'$1\'".to_string(),
                        },
                        NodeExpression::Charlist {
                            value: "\'$2\'".to_string(),
                        },
                    ]),
                    tail: Box::new(NodeExpression::FunctionCall {
                        name: "cool".to_string(),
                        args: Box::new(vec![NodeExpression::Charlist {
                            value: "\'$3\'".to_string(),
                        }]),
                    }),
                },]
            },
            Node {
                lhs: "matched_expr".to_string(),
                rhs: vec!["matched_expr".to_string(), "matched_op_expr".to_string(),],
                expressions: vec![NodeExpression::FunctionCall {
                    name: "build_op".to_string(),
                    args: Box::new(vec![
                        NodeExpression::FunctionCall {
                            name: "element".to_string(),
                            args: Box::new(vec![
                                NodeExpression::Integer { value: 1 },
                                NodeExpression::Charlist {
                                    value: "\'$2\'".to_string(),
                                },
                            ]),
                        },
                        NodeExpression::Charlist {
                            value: "\'$1\'".to_string(),
                        },
                        NodeExpression::FunctionCall {
                            name: "element".to_string(),
                            args: Box::new(vec![
                                NodeExpression::Integer { value: 2 },
                                NodeExpression::Charlist {
                                    value: "\'$2\'".to_string(),
                                },
                            ]),
                        },
                    ]),
                },]
            },
            Node {
                lhs: "assoc_expr".to_string(),
                rhs: vec!["dot_identifier".to_string(),],
                expressions: vec![NodeExpression::FunctionCall {
                    name: "build_identifier".to_string(),
                    args: Box::new(vec![
                        NodeExpression::Charlist {
                            value: "\'$1\'".to_string(),
                        },
                        NodeExpression::Atom {
                            value: "nil".to_string(),
                        },
                    ]),
                },],
            },
            Node {
                lhs: "access_expr".to_string(),
                rhs: vec!["\'true\'".to_string(),],
                expressions: vec![NodeExpression::FunctionCall {
                    name: "handle_literal".to_string(),
                    args: Box::new(vec![
                        NodeExpression::MacroCall {
                            name: "id".to_string(),
                            args: Box::new(vec![NodeExpression::Charlist {
                                value: "\'$1\'".to_string(),
                            }]),
                        },
                        NodeExpression::Charlist {
                            value: "\'$1\'".to_string(),
                        },
                    ]),
                },],
            },
            Node {
                lhs: "access_expr".to_string(),
                rhs: vec!["bin_string".to_string(),],
                expressions: vec![NodeExpression::FunctionCall {
                    name: "build_bin_string".to_string(),
                    args: Box::new(vec![
                        NodeExpression::Charlist {
                            value: "\'$1\'".to_string(),
                        },
                        NodeExpression::FunctionCall {
                            name: "delimiter".to_string(),
                            args: Box::new(vec![NodeExpression::Binary {
                                value: "\"".to_string(),
                            }]),
                        },
                    ]),
                },],
            },
            Node {
                lhs: "access_expr".to_string(),
                rhs: vec!["atom".to_string(),],
                expressions: vec![NodeExpression::FunctionCall {
                    name: "handle_literal".to_string(),
                    args: Box::new(vec![
                        NodeExpression::MacroCall {
                            name: "exprs".to_string(),
                            args: Box::new(vec![NodeExpression::Charlist {
                                value: "\'$1\'".to_string()
                            },]),
                        },
                        NodeExpression::Charlist {
                            value: "\'$1\'".to_string(),
                        },
                        NodeExpression::FunctionCall {
                            name: "delimiter".to_string(),
                            args: Box::new(vec![NodeExpression::Binary {
                                value: ":".to_string(),
                            }]),
                        },
                    ]),
                },],
            },
            Node {
                lhs: "access_expr".to_string(),
                rhs: vec!["atom".to_string(),],
                expressions: vec![NodeExpression::FunctionCall {
                    name: "handle_literal".to_string(),
                    args: Box::new(vec![
                        NodeExpression::MacroCall {
                            name: "exprs".to_string(),
                            args: Box::new(vec![NodeExpression::Charlist {
                                value: "\'$1\'".to_string()
                            },]),
                        },
                        NodeExpression::Charlist {
                            value: "\'$1\'".to_string(),
                        },
                        NodeExpression::FunctionCall {
                            name: "delimiter".to_string(),
                            args: Box::new(vec![NodeExpression::Binary {
                                value: "'".to_string(),
                            }]),
                        },
                    ]),
                },],
            },
        ]
    );
    assert_eq!(
        grammar.declarations,
        vec![
            Declaration(
                DeclarationType::Nonterminals,
                vec![
                    String::from("grammar"),
                    String::from("expr_list"),
                    String::from("expr"),
                    String::from("container_expr"),
                    String::from("block_expr"),
                    String::from("access_expr"),
                    String::from("number")
                ]
            ),
            Declaration(
                DeclarationType::Terminals,
                vec![
                    String::from("paren_identifier"),
                    String::from("do_identifier"),
                    String::from("block_identifier"),
                    String::from("'true'"),
                    String::from("'false'"),
                    String::from("'nil'"),
                    String::from("'do'"),
                    String::from("eol"),
                    String::from("';'"),
                    String::from("','"),
                    String::from("'.'"),
                    String::from("'('"),
                    String::from("')'"),
                    String::from("'['"),
                    String::from("']'"),
                    String::from("'{'"),
                    String::from("'}'"),
                    String::from("'<<'"),
                    String::from("'>>'"),
                    String::from("'%{}'"),
                    String::from("'%'"),
                ]
            ),
            Declaration(DeclarationType::Rootsymbol, vec![String::from("grammar")]),
            Declaration(DeclarationType::Expect, vec![String::from("3")]),
            Declaration(
                DeclarationType::Left,
                vec![String::from("5"), String::from("do")]
            ),
            Declaration(
                DeclarationType::Right,
                vec![String::from("10"), String::from("stab_op_eol")]
            ),
            Declaration(
                DeclarationType::Left,
                vec![String::from("20"), String::from("','")]
            ),
            Declaration(
                DeclarationType::Left,
                vec![String::from("40"), String::from("in_match_op_eol")]
            ),
            Declaration(
                DeclarationType::Nonassoc,
                vec![String::from("300"), String::from("unary_op_eol")]
            ),
        ]
    )
}

// #[test]
fn test_parse_elixir_grammar() {
    let source = fs::read_to_string("test_data/elixir_parser.yrl").unwrap();
    parse_grammar(&source);
    assert_eq!(true, true);
}
