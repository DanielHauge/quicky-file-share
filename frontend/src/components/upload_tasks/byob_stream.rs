use std::error::Error;

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



impl TryStream for ByobStream {
    type Ok = JsValue;
    type Error = JsValue;

    fn try_poll_next(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Result<Self::Ok, Self::Error>>> {
        todo!()
    }
}


unsafe impl Send for ByobStream {
    
}

unsafe impl Sync for ByobStream {

}