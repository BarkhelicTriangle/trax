use std::fs::File;

// this should return a Result type for the UI to work with
// also copying an entire file just to use this function is bad
pub fn append_file_to_sink(p_sink: &rodio::Sink, p_file: File) -> Result<(), rodio::decoder::DecoderError>
{
    let decoder = rodio::Decoder::try_from(p_file)?;
    p_sink.append(decoder);
    Ok(())
}