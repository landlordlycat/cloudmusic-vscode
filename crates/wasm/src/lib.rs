use std::io::Cursor;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    Ok(())
}

#[wasm_bindgen]
pub struct Player {
    volume: f32,
    sink: Option<rodio::Sink>,
    #[allow(dead_code)]
    stream: Option<rodio::OutputStream>,
}

#[wasm_bindgen]
impl Player {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            volume: 0.0,
            sink: None,
            stream: None,
        }
    }

    #[wasm_bindgen]
    pub fn load(&mut self, data: &[u8]) -> bool {
        self.sink = None;
        self.stream = None;
        if let Ok((stream, handle)) = rodio::OutputStream::try_default() {
            if let Ok(sink) = rodio::Sink::try_new(&handle) {
                sink.set_volume(self.volume);
                let cur = Cursor::new(data.to_owned());
                let decoder = rodio::Decoder::new(cur).unwrap();
                sink.append(decoder);
                self.sink = Some(sink);
                self.stream = Some(stream);
                return true;
            }
        }
        false
    }

    #[wasm_bindgen]
    pub fn play(&mut self) -> bool {
        if let Some(ref sink) = self.sink {
            if sink.empty() {
                return false;
            } else {
                sink.play();
            }
        }
        true
    }

    #[wasm_bindgen]
    pub fn pause(&mut self) {
        if let Some(ref sink) = self.sink {
            sink.pause();
        }
    }

    #[wasm_bindgen]
    pub fn stop(&mut self) {
        if let Some(ref sink) = self.sink {
            sink.stop();
            self.sink = None;
            self.stream = None;
        }
    }

    #[wasm_bindgen]
    pub fn set_volume(&mut self, level: f32) {
        if let Some(ref sink) = self.sink {
            let level = level / 100.0;
            sink.set_volume(level);
            self.volume = level;
        }
    }

    #[wasm_bindgen]
    pub fn empty(&self) -> bool {
        if let Some(ref sink) = self.sink {
            sink.empty()
        } else {
            true
        }
    }
}