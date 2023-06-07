use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::Route;

#[function_component(FileNotFound)]
pub fn file_not_found() -> Html {
    let navigator = use_navigator().expect("navigator to work");
    let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <>
            <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h1>{ "File was not found:" }</h1>
            <button {onclick}>{ "Test" }</button>
        </>
    }
}