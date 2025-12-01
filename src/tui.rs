use crate::decode;
use crate::sink_interfacing;
use std::{fs::File, io::{self, Write}};

fn command_handler(sink: &rodio::Sink, cmd: &str)
{
    let cmd_param = cmd.split_once(' ').unwrap_or(("", "")).1;

    match cmd.split_ascii_whitespace().nth(0).unwrap_or("")
    {
        ":+" => 
        {
            if cmd_param == "" { return; }
            let source_file = std::fs::File::open( &cmd_param).expect("Couldn't open file");

            sink_interfacing::append_file_to_sink(sink, source_file);
        },
        ":+b" =>
        {
            if cmd_param == "" { return; }

            // code duplication; wouldn't be an unnecessary abstraction to adding a song its own function (pref. outside this file)
            let directory = std::path::Path::new(&cmd_param);
            if !directory.is_dir() || !directory.exists() { return; }
            for file in directory.read_dir().unwrap()
            {
                if let Ok(file) = file
                {
                    if file.file_name() == ".DS_Store" {continue;}
                    sink_interfacing::append_file_to_sink(sink,File::open(file.path()).unwrap());
                } 
            }
        }
        _ => return
    };
}

pub fn repl_ui(sink: &rodio::Sink) 
{
    let stdin = io::stdin();
    let mut input = String::new();

    loop 
    {
        io::stdout().flush().unwrap();
        input.clear();
        stdin.read_line(&mut input).unwrap();
        input = input.strip_suffix('\n').unwrap().to_string();

        // We need to check for quit first to leave the tui
        if input == ":q" || input == ":quit" { return; }
        command_handler(&sink, &input);
    }
}