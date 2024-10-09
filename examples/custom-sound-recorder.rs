use {
    sfml::audio::{SoundRecorder, SoundRecorderDriver},
    std::{error::Error, fs::File, io::Write},
};

struct FileRecorder {
    file: File,
}

impl SoundRecorder for FileRecorder {
    fn on_process_samples(&mut self, data: &[i16]) -> bool {
        match self.file.write_all(unsafe {
            std::slice::from_raw_parts(data.as_ptr() as *const u8, data.len() * 2)
        }) {
            Ok(_) => true,
            Err(e) => {
                eprintln!("Error writing to file: {e}");
                false
            }
        }
    }
}

impl FileRecorder {
    fn create(path: &str) -> std::io::Result<Self> {
        let file = File::create(path)?;
        Ok(Self { file })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut fr = FileRecorder::create("hello.pcm")?;
    let mut recorder = SoundRecorderDriver::new(&mut fr);
    recorder.start(44_100)?;
    let mut left = 5000;
    while left > 0 {
        std::thread::sleep(std::time::Duration::from_millis(100));
        left -= 100;
        print!("You have {left} left to record\r");
        let _ = std::io::stdout().flush();
    }
    Ok(())
}
