use std::{error::Error, str::Bytes};

use wasm_bindgen::JsValue;
use web_sys::{ReadableStreamByobReader, File};
use futures_core::{stream::Stream, stream::TryStream};

#[derive(Debug)]
pub struct ByobStream {
    reader: ReadableStreamByobReader,
    done: bool,
}



// futures_core::stream::TryStream + Send + Sync + 'static

impl ByobStream {
    pub fn from_file(file: File) -> Result<ByobStream, JsValue> {
        let stream = file.stream();
        let new_reader = ReadableStreamByobReader::new(&stream);
        match new_reader {
            Ok(reader) => Ok(ByobStream {
                reader,
                done: false,
            }),
            Err(e) => Err(e),
        }
    }
}

impl Stream for ByobStream {
    type Item = Result<String, String>;

    fn poll_next(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        todo!()
    }
}



unsafe impl Send for ByobStream {
    
}

unsafe impl Sync for ByobStream {

}
