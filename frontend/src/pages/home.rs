use yew::prelude::*;
#[path = "../components/button.rs"] mod button;
#[path = "../components/upload_field.rs"] mod upload_field;

use upload_field::UploadField;
// use button::Button;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div>

            <UploadField />
        </div>
    }
}