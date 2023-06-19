
use futures::StreamExt;
use wasm_bindgen::prelude::*;
use web_sys::{File, console};




pub async fn upload_file(file : File) {
    //Check the file list from the input
    console::log_1(&"Hello using web-sys".into());

    // let file_reader = web_sys::FileReader::new().unwrap();

    // let fr_c = file_reader.clone();
    // // let fr_g = file_reader.clone();

    // // create onLoadEnd callback
    // let onloadend_cb = Closure::wrap(Box::new(move |_e: web_sys::ProgressEvent| {
    //     let array = js_sys::Uint8Array::new(&fr_c.result().unwrap());
        
        
    //     let str = array.to_string().as_string().unwrap();
        
    //     // log::info!("total: {}", _e.total());
    //     // console::log_1(&JsValue::from_str(&format!("total: {}/{}", _e.loaded(),_e.total())));

    //     let len = str.len() as u32;
    //     console::log_1(&JsValue::from_str(&format!("size: {}", len)));
    //     console::log_1(&JsValue::from_str(&format!("content: {}", str)));
    // }) as Box<dyn Fn(web_sys::ProgressEvent)>);

    // let onpg_cb = Closure::wrap(Box::new(move |_e: web_sys::ProgressEvent| {
    //     let result = &fr_g.result().expect("Result exists");

    //     match result.as_string(){
              
    //         Some(str) =>  console::log_1(&JsValue::from_str(&format!("content: {}", str))),
    //         None => console::log_1(&JsValue::from_str(&format!("content: {}", "None"))),
    //     }
    //     console::log_1(&JsValue::from_str(&format!("progress: {}/{}", _e.loaded(),_e.total())));

    // }) as Box<dyn Fn(web_sys::ProgressEvent)>);

    let mut stream = wasm_streams::ReadableStream::from_raw(file.stream().unchecked_into()).into_stream();
    while let Some(Ok(chunk)) = stream.next().await {
        let array = js_sys::Uint8Array::new(&chunk);
        let str = array.to_string().as_string().unwrap();
        let len = str.len() as u32;
        console::log_1(&JsValue::from_str(&format!("size: {}", len)));
        console::log_1(&JsValue::from_str(&format!("content: {}", str)));
        console::log_1(&JsValue::from_str(&format!("something")));

    }

    // file_reader.set_onloadend(Some(onloadend_cb.as_ref().unchecked_ref()));
    
    // // file_reader.set_onprogress(Some(onpg_cb.as_ref().unchecked_ref()));
    // let f_size = file.size() as i32;

    // let file_slices_1 = &file.slice_with_i32_and_i32(0, f_size/2).unwrap();
    // let file_slices_2 = &file.slice_with_i32_and_i32(f_size/2, f_size).unwrap();
    
    // file_reader.read_as_array_buffer(file_slices_1).expect("blob not readable");
    // file_reader.read_as_array_buffer().expect("blob not readable");
    // file_reader.read_as_array_buffer(&file.slice_with_i32(60).unwrap()).expect("blob not readable");
    // onloadend_cb.forget();
    // onpg_cb.forget();

}