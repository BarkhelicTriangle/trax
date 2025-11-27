use crate::decode;
use std::io::{self, Write};

pub fn repl_ui(sink: &rodio::Sink) 
{
    let stdin = io::stdin();
    let mut input = String::new();

    loop 
    {
        print!(">");
        io::stdout().flush().unwrap();
        input.clear();
        stdin.read_line(&mut input).unwrap();
        input = input.strip_suffix('\n').unwrap().to_string();
        // create source from file
        let source_file = std::fs::File::open(&mut *input).expect("Couldn't open file");
        let decoder = decode::create_decoder_for_file(source_file).expect("Couldn't decode");
        sink.append(decoder);
    }
}