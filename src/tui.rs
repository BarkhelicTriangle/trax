use crate::decode;
use std::io::{self, Write};

fn command_handler(sink: &rodio::Sink, cmd: &str)
{
    let cmd_param = cmd.split_once(' ').unwrap_or(("", "")).1;

    match cmd.split_ascii_whitespace().nth(0).unwrap_or("")
    {
        ":+" => 
        {
            if cmd_param == "" { return; }
            let source_file = std::fs::File::open( &cmd_param).expect("Couldn't open file");
            let decoder = decode::create_decoder_for_file(source_file).expect("Couldn't decode");
            sink.append(decoder);
        },
        _ => return
    };
}

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

        // We need to check for quit first to leave the tui
        if input == ":q" || input == ":quit" { return; }
        command_handler(&sink, &input);
    }
}