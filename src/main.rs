use clap::Parser;
use colored::Colorize;
use regex::Regex;
use std::io::{self, BufRead};

#[derive(Parser, Debug)]
#[clap(version)]
/// Colorize lines by pattern.
///
/// Colorize reads lines from stdin and colorizes the matching lines
/// according to the set regular expression pattern. 
/// 
/// Supported colors:
///   red (default),
///   blue, green, yellow, purple, cyan, black, white 
///
/// Patterns and colors can be specifed more than once to use multiple lines. 
/// Eg: 
///
/// $ colorize -p error -c yellow -p debug -c green
///
struct Args {

    /// Color to use (default: red)
    #[clap(short, long, value_parser)]
    color: Vec<String>,
    
    /// Regular expression pattern to match lines
    #[clap(short, long, value_parser)]
    pattern: Vec<String>,
}


fn main() -> io::Result<()> {

    let args = Args::parse();

    let patterns: Vec<Regex> = args.pattern.iter().map(
        |p| Regex::new(&p).unwrap()
    ).collect();

    let mut stdin = io::stdin().lock().lines();

    while let Some(line) = stdin.next() {
        let last = line.unwrap();

        if last.len() == 0 {
            break;
        }

        let index = patterns.iter().enumerate()
            .find_map(|(i, r)| {
                r.is_match(&last).then_some(i)
            }
        );
        
        match index {
            None => println!("{}", last),
            Some(i) => {
                match args.color.get(i) {
                    None => println!("{}", last.red()),
                    Some(color) => println!("{}", last.color(color.to_string())),
                }
            }
        }
    }

    Ok(())
}

