
use js_sys::{Boolean, Uint8Array};
use wasm_bindgen::JsValue;
// use futures::StreamExt;
// use wasm_bindgen::prelude::*;
use web_sys::{File, console, ReadableStream};
// use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::ReadableStreamByobReader;
use web_sys::FileReader;
use wasm_bindgen_futures::JsFuture;

pub struct ReadResult {
    done: Boolean,
    value: Uint8Array,
}


pub async fn upload_file(file : File) -> String {
    //Check the file list from the input
    console::log_1(&"Hello using web-sys".into());

    let stream = file.stream();
    let reader = ReadableStreamByobReader::new(&stream).ok().unwrap();
    
    match JsFuture::from(reader.read_with_array_buffer_view(&js_sys::Uint8Array::new(&JsValue::from(4000)))).await {
        Ok(result) => console::log_1(&result),
        Err(error) => console::log_1(&error),
    }

    match JsFuture::from(reader.read_with_array_buffer_view(&js_sys::Uint8Array::new(&JsValue::from(4000)))).await {
        Ok(result) => console::log_1(&result),
        Err(error) => console::log_1(&error),
    }




    
    return String::from("file uploaded.")

}