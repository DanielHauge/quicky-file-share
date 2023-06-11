use web_sys::File;
use yew::prelude::*;
#[path = "../components/button.rs"]
mod button;
#[path = "../components/upload_input/input.rs"]
mod upload_input;

use upload_input::UploadInput;
use yew_router::prelude::use_navigator;

use crate::Route;

#[function_component(Home)]
pub fn home() -> Html {
    let navigator = use_navigator().expect("navigator to work");
    let onclick = Callback::from(move |_| {
        navigator.push(&Route::Download {
            id: String::from("helol"),
        })
    });
    let upload_files_cb: Callback<Vec<File>> = Callback::from(move |_| ());
    html! {
        <div>
            <button {onclick}>{ "Test" }</button>
            <UploadInput {upload_files_cb} />
        </div>
    }
}
