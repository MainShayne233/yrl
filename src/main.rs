#[macro_use]
extern crate lalrpop_util;

use crate::ast::{Declaration, DeclarationType, Grammar};

lalrpop_mod!(pub grammar);

pub mod ast;

#[test]
fn test_nonterminals_parser() {
    let input = r#"
Nonterminals
  grammar expr_list
  expr container_expr block_expr access_expr
  call_args_parens_expr call_args_parens_base call_args_parens parens_call
  number
  .

Terminals
  identifier kw_identifier kw_identifier_safe kw_identifier_unsafe bracket_identifier
  paren_identifier do_identifier block_identifier
  .
"#;

    let program = grammar::GrammarParser::new().parse(input).unwrap();
    assert_eq!(
        program,
        Grammar {
            declarations: vec![
                Declaration(
                    DeclarationType::Nonterminals,
                    vec![
                        String::from("grammar"),
                        String::from("expr_list"),
                        String::from("expr"),
                        String::from("container_expr"),
                        String::from("block_expr"),
                        String::from("access_expr"),
                        String::from("call_args_parens_expr"),
                        String::from("call_args_parens_base"),
                        String::from("call_args_parens"),
                        String::from("parens_call"),
                        String::from("number")
                    ]
                ),
                Declaration(
                    DeclarationType::Terminals,
                    vec![
                        String::from("identifier"),
                        String::from("kw_identifier"),
                        String::from("kw_identifier_safe"),
                        String::from("kw_identifier_unsafe"),
                        String::from("bracket_identifier"),
                        String::from("paren_identifier"),
                        String::from("do_identifier"),
                        String::from("block_identifier")
                    ]
                )
            ]
        }
    )
}
