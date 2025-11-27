use std::{fs::File, io::BufReader};


pub fn create_decoder_for_file(p_file: File) -> Result<rodio::Decoder<BufReader<File>>, rodio::decoder::DecoderError>
{
    // the default settings are fine for now; making this its own function
    // to make things easier going foward if i have to fuck around w/ settings
    rodio::Decoder::try_from(p_file)
}