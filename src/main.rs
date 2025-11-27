const SONG_PATH: &str = "./songs/renard - intensive care unit.mp3";

//fn append_to_sink()

fn main() 
{
    // _stream must live as long as the sink
    let stream_handle = rodio::OutputStreamBuilder::open_default_stream()
            .expect("open default audio stream");
    let sink = rodio::Sink::connect_new(&stream_handle.mixer());

    // create source from file
    let source = rodio::Decoder::try_from(
        std::fs::File::open(SONG_PATH).expect("Couldn't open file"))
        .expect("Couldn't decode");
    sink.append(source);

    // The sound plays in a separate thread. This call will block the current thread until the sink
    // has finished playing all its queued sounds.
    sink.sleep_until_end();
}
