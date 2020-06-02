#![feature(try_blocks)]
#[macro_use]
extern crate lazy_static;

use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::input_stream::InputStream;
use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::tree::{ErrorNode, ParseTreeListener, TerminalNode};

mod csvlexer;
mod csvlistener;
mod csvparser;
use csvlexer::CSVLexer;
use csvlistener::CSVListener;
use csvparser::*;

struct Listener;

impl ParseTreeListener for Listener {
    fn visit_terminal(&mut self, _node: &TerminalNode) {}
    fn visit_error_node(&mut self, _node: &ErrorNode) {}
    fn enter_every_rule(&mut self, _ctx: &dyn ParserRuleContext) {}
    fn exit_every_rule(&mut self, _ctx: &dyn ParserRuleContext) {}
}

impl CSVListener for Listener {
    fn enter_csvFile(&mut self, _ctx: &CsvFileContext) {}
    fn exit_csvFile(&mut self, _ctx: &CsvFileContext) {}
    fn enter_hdr(&mut self, _ctx: &HdrContext) {}
    fn exit_hdr(&mut self, _ctx: &HdrContext) {}
    fn enter_row(&mut self, _ctx: &RowContext) {}
    fn exit_row(&mut self, _ctx: &RowContext) {}
    fn enter_field(&mut self, _ctx: &FieldContext) {}
    fn exit_field(&mut self, _ctx: &FieldContext) {}
}

fn main() {
    let input = String::from(
        "This, is, a, header
This, is, a, row
",
    );
    let lexer = CSVLexer::new(Box::new(InputStream::new(input)));
    let token_source = CommonTokenStream::new(lexer);
    let mut parser = CSVParser::new(Box::new(token_source));
    parser.add_parse_listener(Box::new(Listener {}));
    let result = parser.csvFile();
    assert!(result.is_ok());
}
