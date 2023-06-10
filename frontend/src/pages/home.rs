use yew::prelude::*;
#[path = "../components/button.rs"] mod button;
#[path = "../components/upload_drop_zone.rs"] mod upload_drop_zone;

use upload_drop_zone::UploadZone;
use yew_router::prelude::use_navigator;

use crate::Route;

#[function_component(Home)]
pub fn home() -> Html {
    let navigator = use_navigator().expect("navigator to work");
    let onclick = Callback::from(move |_| navigator.push(&Route::Download { id: String::from("helol") }));
    html! {
        <div>
            <button {onclick}>{ "Test" }</button>
            <UploadZone />
        </div>
    }
}