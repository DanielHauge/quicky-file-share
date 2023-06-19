
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct FileLiProps {
    pub selected_file: String,
    pub remove_file_cb: Box<Callback<String>>,
}

#[function_component(FileLi)]
pub fn upload_field(props: &FileLiProps) -> Html {
    html! {
        <li>
            { props.selected_file.clone() }
            <i onclick={
                let cb = props.remove_file_cb.clone();
                let s = props.selected_file.clone();
                move |_| {
                    let ss = s.clone();
                    cb.emit(String::from(ss))
                }
            } class="material-icons">{"close"}</i>
        </li>
    }
}
