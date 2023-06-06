use yew::prelude::*;
use yew::Properties;


#[derive(Properties, PartialEq)]
pub struct FileProps {
    pub file_id: AttrValue,
}


#[function_component(File)]
pub fn file(props: &FileProps) -> Html {
    html! {
        <>
            // <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h1>{ "File:" } { props.file_id.clone() }</h1>
        </>
    }
}