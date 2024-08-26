use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};

/// Simple grep implementation
#[derive(Parser)]
#[command(version="0.1.11", about="A simple grep implementation by chagelo", long_about = None)]
struct Args {
    #[command(subcommand)]
    option: Option<Options>,

    pattern: Option<String>,    

    // Files to search
    file: Vec<PathBuf>,
}

#[repr(align(2))]
#[derive(Subcommand)]
enum Options {
    /// Pattern Syntax
    Pattern {
        /// lists test values
        #[arg(short, long)]
        extended_regexp: bool,

        #[arg(short, long)]
        fixed_strings: bool,

        #[arg(short, long)]
        basic_regexp: bool,

        #[arg(short, long)]
        perl_regexp: bool,
    },
    Test {
        /// lists test values
        #[arg(short, long)]
        extended_regexp: bool,

        #[arg(short, long)]
        fixed_strings: bool,

        #[arg(short, long)]
        basic_regexp: bool,

        #[arg(short, long)]
        perl_regexp: bool,
    },
}

fn main() {
    let _args = Args::parse();
}
