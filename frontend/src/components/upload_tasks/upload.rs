
use futures_util::StreamExt;
use js_sys::{Boolean, Uint8Array, Error};
use reqwest::Request;
use wasm_bindgen::JsValue;
use serde::{Serialize, Deserialize};
use web_sys::{File, console};

use web_sys::ReadableStreamByobReader;
use wasm_bindgen_futures::JsFuture;


#[path = "./byob_stream.rs"]
mod byob_stream;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReadResult {
    done: bool,
    value: Vec<u8>,
}


pub async fn upload_file(file : File) -> Result<String, Error> {

    console::log_1(&"Hello using web-sys".into());
    let stream = file.stream();
    let reader = ReadableStreamByobReader::new(&stream).ok().unwrap();
    
    let mut done = true;
    let mut bytes = Vec::new();
    while done {
        let gg = JsFuture::from(reader.read_with_array_buffer_view(&js_sys::Uint8Array::new(&JsValue::from(4000)))).await?;
        let res: ReadResult = serde_wasm_bindgen::from_value(gg).expect("failed to parse readable stream read result");
        console::log_1(&res.value.len().into());
        bytes.extend_from_slice(&res.value);

        done = !res.done;
    }

    
    let yo = reqwest::Body::from(bytes);
    
    return Ok(String::from("file uploaded."))

}
