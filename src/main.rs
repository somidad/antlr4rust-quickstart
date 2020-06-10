#![feature(try_blocks)]
#[macro_use]
extern crate lazy_static;

use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::input_stream::InputStream;
use antlr_rust::tree::{ParseTree, ParseTreeListener};

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
}

impl Listener {
    fn hdr(&self, ctx: &HdrContextAll) -> Row {
        let row_ctx = ctx.row().unwrap();
        self.row(&row_ctx)
    }

    fn row(&self, ctx: &RowContextAll) -> Row {
        let mut row = Row::new();
        let field_ctx_list = ctx.field_all();
        for (_i, field_ctx) in field_ctx_list.iter().enumerate() {
            let field = self.field(&field_ctx);
            row.push(field);
        }
        row
    }

    fn field(&self, ctx: &FieldContextAll) -> String {
        ctx.get_text()
    }
}

impl ParseTreeListener for Listener {}

impl CSVListener for Listener {
    fn exit_csvFile(&mut self, ctx: &CsvFileContext) {
        let hdr_ctx = ctx.hdr().unwrap();
        let header = self.hdr(&hdr_ctx);
        self.csv.header = header;
        let row_ctx_list = ctx.row_all();
        for (_i, row_ctx) in row_ctx_list.iter().enumerate() {
            let row = self.row(&row_ctx);
            self.csv.rows.push(row);
        }
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
    }));
    let result = parser.csvFile();
    assert!(result.is_ok());
    let listener = parser.remove_parse_listener(listener_id);
    let csv = listener.csv;
    println!("{:#?}", csv);
}
