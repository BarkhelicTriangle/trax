use std::fs::File;

// this should return a Result type for the UI to work with
// also copying an entire file just to use this function is bad
pub fn append_file_to_sink(p_sink: &rodio::Sink, p_file: File)
{
    let decoder = rodio::Decoder::try_from(p_file).unwrap();
    p_sink.append(decoder);
}