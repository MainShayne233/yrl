#[macro_use]
extern crate lalrpop_util;

use crate::ast::{Declaration, DeclarationType};

lalrpop_mod!(pub grammar);

pub mod ast;

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
"#;

    let grammar = grammar::GrammarParser::new().parse(input).unwrap();
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
            )
        ]
    )
}
