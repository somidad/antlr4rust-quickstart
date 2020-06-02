#![feature(try_blocks)]
#[macro_use]
extern crate lazy_static;

use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::input_stream::InputStream;
use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::tree::{ErrorNode, ParseTree, ParseTreeListener, TerminalNode};

mod csvlexer;
mod csvlistener;
mod csvparser;
use csvlexer::CSVLexer;
use csvlistener::CSVListener;
use csvparser::*;

type Row = Vec<String>;

#[derive(Debug)]
struct CSV {
    header: Row,
    rows: Vec<Row>,
}

struct Listener {
    csv: Box<CSV>,
    add_to_header: bool,
    row_to_add: Vec<String>,
}

impl ParseTreeListener for Listener {
    fn visit_terminal(&mut self, _node: &TerminalNode) {}
    fn visit_error_node(&mut self, _node: &ErrorNode) {}
    fn enter_every_rule(&mut self, _ctx: &dyn ParserRuleContext) {}
    fn exit_every_rule(&mut self, _ctx: &dyn ParserRuleContext) {}
}

impl CSVListener for Listener {
    fn enter_csvFile(&mut self, _ctx: &CsvFileContext) {}
    fn exit_csvFile(&mut self, _ctx: &CsvFileContext) {}
    fn enter_hdr(&mut self, _ctx: &HdrContext) {
        self.add_to_header = true;
    }
    fn exit_hdr(&mut self, _ctx: &HdrContext) {
        self.csv.header = self.row_to_add.to_vec();
        self.row_to_add.clear();
        self.add_to_header = false;
    }
    fn enter_row(&mut self, _ctx: &RowContext) {}
    fn exit_row(&mut self, _ctx: &RowContext) {
        if self.add_to_header {
            return;
        }
        self.csv.rows.push(self.row_to_add.to_vec());
        self.row_to_add.clear();
    }
    fn enter_field(&mut self, _ctx: &FieldContext) {}
    fn exit_field(&mut self, _ctx: &FieldContext) {
        self.row_to_add.push(_ctx.get_text());
    }
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
    let listener_id = parser.add_parse_listener(Box::new(Listener {
        csv: Box::new(CSV {
            header: Row::new(),
            rows: Vec::new(),
        }),
        add_to_header: false,
        row_to_add: Row::new(),
    }));
    let result = parser.csvFile();
    assert!(result.is_ok());
    let listener = parser.remove_parse_listener(listener_id);
    let csv = listener.csv;
    println!("{:#?}", csv);
}
