#[macro_use]
extern crate lalrpop_util;

use crate::ast::Grammar;

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
"#;

    let program = grammar::GrammarParser::new().parse(input).unwrap();
    assert_eq!(
        program,
        Grammar {
            nonterminals: vec![
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
        }
    )
}
