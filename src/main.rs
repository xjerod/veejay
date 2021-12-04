use std::env;
use std::ffi::CString;

use ffmpeg::sys::av_dump_format;
fn main() {
    ffmpeg::init().unwrap();

    let path = &env::args().nth(1).expect("missing file");

    match ffmpeg::format::input(path) {
        Ok(mut context) => {
            let uri = CString::new(path.as_str()).unwrap();

            unsafe { av_dump_format(context.as_mut_ptr(), 0, uri.as_ptr(), 0) }

            let video_stream = context.streams().best(ffmpeg::media::Type::Video).unwrap();
            println!("Best video stream index: {}", video_stream.index());

            let audio_stream = context.streams().best(ffmpeg::media::Type::Audio).unwrap();
            println!("Best audio stream index: {}", audio_stream.index());

            println!(
                "duration (seconds): {:.2}",
                context.duration() as f64 / f64::from(ffmpeg::ffi::AV_TIME_BASE)
            );

            println!("frames: {}", video_stream.frames());
            println!(
                "video.width: {}",
                video_stream.codec().decoder().video().unwrap().width()
            );
            println!(
                "video.height: {}",
                video_stream.codec().decoder().video().unwrap().height()
            );
        }

        Err(error) => println!("error: {}", error),
    }
}
