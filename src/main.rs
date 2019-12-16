#![feature(try_trait)]

use crate::lexer::Lexer;
use crate::token::TokenType;
use clap::{crate_authors, crate_description, crate_name, crate_version};
use clap::{App, Arg, SubCommand};
use std::error::Error;
use std::fs;
use std::path::Path;

mod lexer;
mod token;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(
            SubCommand::with_name("run")
                .about("Run scy program")
                .arg(Arg::with_name("FILE").required(true).index(1)),
        )
        .get_matches();

    let matches = matches.subcommand_matches("run").unwrap();
    let file_path = matches.value_of("FILE").unwrap();
    let canonical_path = Path::new(file_path).with_extension("scy").canonicalize()?;
    let content = fs::read_to_string(canonical_path)?;

    let mut lexer = Lexer::new(&content);

    let mut token = lexer.read();
    while token.token_type != TokenType::EOF {
        println!("{:?}", token);
        token = lexer.read();
    }

    Ok(())
}
