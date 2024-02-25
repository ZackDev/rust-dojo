use clap::{self, arg, Parser};

fn main() {
    let arg1 = arg!(-a --arg1 "ARG1");
    let arg2 = arg!(-b --arg2 "ARG2");
}