use yew::prelude::*;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <>
            <p> {"Not found"} </p>
        </>
    }
}
