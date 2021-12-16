use std::ffi::CString;

use ffmpeg::format::input;
use ffmpeg::sys::av_dump_format;

pub struct Clip {
    path: String,
    start_index: i64,
    end_index: i64,
}

impl Clip {
    pub fn new(path: &str) -> Self {
        Clip {
            path: String::from(path),
            start_index: 0,
            end_index: input(&path.to_owned()).unwrap().duration(),
        }
    }

    pub fn dump_format(&mut self) {
        let url = CString::new(self.path.as_str()).unwrap();
        unsafe {
            av_dump_format(
                input(&self.path.to_owned()).unwrap().as_mut_ptr(),
                0,
                url.as_ptr(),
                0,
            )
        }
    }

    pub fn video_stream_index(&self) -> usize {
        self.stream_index(ffmpeg::media::Type::Video)
    }

    pub fn audio_stream_index(&self) -> usize {
        self.stream_index(ffmpeg::media::Type::Audio)
    }

    pub fn stream_index(&self, t: ffmpeg::media::Type) -> usize {
        match input(&self.path.to_owned()).unwrap().streams().best(t) {
            Some(stream) => stream.index(),
            _ => usize::MAX,
        }
    }

    pub fn get_start_index(&self) -> i64 {
        self.start_index
    }

    pub fn get_end_index(&self) -> i64 {
        self.end_index
    }
}
