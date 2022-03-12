mod brainfuck;

use std::fs::File;
use std::io::prelude::*;
use clap::Parser;
use brainfuck::interpreting;

#[derive(Parser, Debug)]
#[clap(version = "0.1.0", about = "RBFI - Rust BrainFuck Interpreter, and it is simply my version of BrainFuck interpreter.")]
struct Args {
   file: String,
}

fn main() {
   let args = Args::parse();

   let mut cells: [u8; 32768] = [0; 32768];
   let mut p = 0;

   let mut buffer = String::new();
   let mut f = File::open(args.file).unwrap();
   f.read_to_string(&mut buffer).unwrap();

   let mut i = 0;
   while i < buffer.len() {

      match buffer.as_bytes()[i] as char {
         '+' => cells[p] = interpreting::add_to_cell(cells[p]),
         '-' => cells[p] = interpreting::subtract_from_cell(cells[p]),
         '>' => p = interpreting::pointer_right(p),
         '<' => p = interpreting::pointer_left(p),
         '.' => interpreting::output_cell(cells[p]),
         ',' => cells[p] = interpreting::take_input(),
         '[' => i = interpreting::open_loop(&buffer, i, cells[p]),
         ']' => i = interpreting::close_loop(&buffer, i) - 1,
         _ => (),
      }
      
      i += 1;
   }
   println!();
}