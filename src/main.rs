// main file
#![feature(assert_matches)]
#![feature(box_syntax, box_patterns)]
#![feature(arbitrary_enum_discriminant)]

mod code;
mod err;
mod lex;
mod parse;
mod res;
mod spv;
mod typer;
mod util;
use code::*;
use lex::*;
use parse::*;
use res::*;
use spv::*;
use typer::*;
use util::Pass;

fn main() {
    let mut args = std::env::args();
    let p_name = args.next().unwrap();
    let in_file = args
        .next()
        .expect(&format!("{}: Need at least one file", p_name));
    let s = std::fs::read_to_string(in_file).unwrap();
    let handler = err::ErrHandler::new(&s);
    let toks = lex(&s);
    // eprintln!("{:#?}", toks);

    let mut tree = vec![];
    tree.extend_from_slice(&parse(&handler, toks));

    // eprintln!("{:#?}", tree);
    let tree = Resolver::res(&handler, tree);
    // eprintln!("{:#?}", tree);
    let tree = Typer::res(&handler, tree);
    // eprintln!("{:#?}", tree);
    let mut gen = GenLLVM::new("hello");

    for i in tree {
        gen.gen_tld(&i);
    }

    gen.dump();

    // use std::io::Write;

    // let mut ws = GenSPV::new();

    // for tld in tree {
    //     ws.gen_tld(&tld);
    // }

    // let ws = ws.pack();

    // let mut bs = vec![];

    // for w in ws {
    //     let b = w.to_le_bytes();
    //     bs.push(b[0]);
    //     bs.push(b[1]);
    //     bs.push(b[2]);
    //     bs.push(b[3]);
    // }

    // std::io::stdout().write(&bs).unwrap();
}
