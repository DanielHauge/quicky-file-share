use yew::prelude::*;
#[path = "../components/button.rs"] mod button;
#[path = "../components/upload_field.rs"] mod upload_field;

use upload_field::UploadField;
use yew_router::prelude::use_navigator;

use crate::Route;

#[function_component(Home)]
pub fn home() -> Html {
    let navigator = use_navigator().expect("navigator to work");
    let onclick = Callback::from(move |_| navigator.push(&Route::Download { id: String::from("helol") }));
    html! {
        <div>
            <button {onclick}>{ "Test" }</button>
            <UploadField />
        </div>
    }
}