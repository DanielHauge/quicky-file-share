
use futures::StreamExt;
// use wasm_bindgen::prelude::*;
use web_sys::{File, console, ReadableStream};
// use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::ReadableStreamByobReader;
use wasm_bindgen_futures::JsFuture;

pub async fn upload_file(file : File) -> String {
    //Check the file list from the input
    console::log_1(&"Hello using web-sys".into());

    let stream = file.stream();
    let reader = ReadableStreamByobReader::new(&stream).ok().unwrap();
    let mut view: [u8;2500] = [0;2500];
    let promise = reader.read_with_u8_array(view.as_mut_slice());
    match JsFuture::from(promise).await {
        Ok(result) => console::log_1(&result),
        Err(error) => console::log_1(&error),
    }
        
    
    return String::from("file uploaded.")

}