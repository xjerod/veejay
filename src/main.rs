use std::env;

use ffmpeg::format::input;

mod clip;
use clip::Clip;

fn main() {
    ffmpeg::init().unwrap();

    let path = env::args().nth(1).expect("missing file");

    let mut clip = Clip::new(&path);

    clip.dump_format();

    let video_index = clip.video_stream_index();
    let audio_index = clip.audio_stream_index();

    for (_stream, pkt) in input(&path).unwrap().packets() {
        if pkt.stream().eq(&video_index) {
            println!(
                "video stream: {:#?} pts: {:#?}",
                pkt.stream(),
                pkt.pts().unwrap_or(-1)
            )
        }

        if pkt.stream().eq(&audio_index) {
            println!(
                "audio stream: {:#?} pts: {:#?}",
                pkt.stream(),
                pkt.pts().unwrap_or(-1)
            )
        }
    }

    println!("start index {}", clip.get_start_index());
    println!("end index {}", clip.get_end_index());
    println!("video stream index {}", video_index);
    println!("audio stream index {}", audio_index);
}
