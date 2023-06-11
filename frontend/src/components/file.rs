use yew::prelude::*;
use yew::Properties;

#[derive(Properties, PartialEq)]
pub struct FileProps {
    pub file_name: AttrValue,
    pub file_size: i32,
    pub file_uploaded: i32,
    pub file_dl_link: AttrValue,
}

#[function_component(File)]
pub fn file(props: &FileProps) -> Html {
    html! {
        <div>
            <h2> {"File name: "} { &props.file_name } </h2>
            <h2> {"File size: "} { &props.file_size } </h2>
            <h2> {"File uploaded: "} { &props.file_uploaded } </h2>
            <a href={&props.file_dl_link}> {"Download"} </a>
        </div>
    }
}
