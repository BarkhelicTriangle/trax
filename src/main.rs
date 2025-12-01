mod decode;
mod sink_interfacing;
mod tui;

fn main() 
{
    // _stream must live as long as the sink
    let stream_handle = rodio::OutputStreamBuilder::open_default_stream()
            .expect("open default audio stream");
    let sink = rodio::Sink::connect_new(&stream_handle.mixer());

    tui::repl_ui(&sink);

    // The sound plays in a separate thread. This call will block the current thread until the sink
    // has finished playing all its queued sounds.
    sink.stop();
    sink.sleep_until_end();
}