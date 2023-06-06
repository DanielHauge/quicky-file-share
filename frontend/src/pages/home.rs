use yew::prelude::*;
#[path = "../components/button.rs"] mod button;

use button::Button;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
            <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h1>{ "Home!" }</h1>
            <Button />
        </>
    }
}