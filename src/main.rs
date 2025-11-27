mod decode;
const SONG_PATH: &str = "./songs/renard - intensive care unit.mp3";

fn main() 
{
    // _stream must live as long as the sink
    let stream_handle = rodio::OutputStreamBuilder::open_default_stream()
            .expect("open default audio stream");
    let sink = rodio::Sink::connect_new(&stream_handle.mixer());

    // create source from file
    let source_file = std::fs::File::open(SONG_PATH).expect("Couldn't open file");
    let decoder = decode::create_decoder_for_file(source_file).expect("Couldn't decode");
    sink.append(decoder);

    // The sound plays in a separate thread. This call will block the current thread until the sink
    // has finished playing all its queued sounds.
    sink.sleep_until_end();
}
