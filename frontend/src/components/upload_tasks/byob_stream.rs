use std::{error::Error, str::Bytes};
use async_stream::stream;
use js_sys::Uint8Array;
use serde::{Serialize, Deserialize};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{ReadableStreamByobReader, File};
use futures_core::{stream::Stream, stream::TryStream};

// use crate::home::upload_input::drop_zone::upload::ReadResult;

#[derive(Debug)]
pub struct ByobStream {
    reader: ReadableStreamByobReader,
    done: bool,
    buffer: Uint8Array,
}

#[derive(Debug, Serialize, Deserialize)]
struct ReadResult {
    done: bool,
    value: Vec<u8>,
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
                buffer: js_sys::Uint8Array::new(&JsValue::from(4000))
            }),
            Err(e) => Err(e),
        }
    }
}


pub fn to_stream(bs: ByobStream) -> impl Stream<Item = Result<Vec<u8>, String>> {
    stream! {
        let mut done = false;
        while !done {
            let future = JsFuture::from(bs.reader.read_with_array_buffer_view(&bs.buffer)).await;
            // match gg
            match future {
                Ok(gg) => {
                    let res: ReadResult = serde_wasm_bindgen::from_value(gg).expect("failed to parse readable stream read result");
                    yield Ok(res.value);
                    done = res.done;
                },
                Err(e) => {
                    // console.error!
                    done = true;
                }
            }

        }
    }
} 





unsafe impl Send for ByobStream {
    
}

unsafe impl Sync for ByobStream {

}
