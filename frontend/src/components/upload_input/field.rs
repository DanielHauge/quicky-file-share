#[path = "./file_li.rs"]
mod file_li;

use file_li::FileLi;
use yew::{classes, prelude::*};

#[derive(Properties, Clone, PartialEq)]
pub struct FieldProps {
    pub selected_files: Vec<String>,
    pub zip_toggle_cb: Callback<bool>,
    pub remove_file_cb: Callback<String>,
}

#[function_component(Field)]
pub fn upload_field(props: &FieldProps) -> Html {
    let drag_class = use_state(|| "");
    let classes = ["drop-container", *drag_class];

    let ondragenter = {
        let drag_class_c = drag_class.clone();
        move |_| drag_class_c.set("drag-hover")
    };

    let ondragleave = {
        let drag_class_c = drag_class.clone();
        move |_| drag_class_c.set("")
    };

    let ondrop = {
        let drag_class_c = drag_class.clone();
        move |_| drag_class_c.set("")
    };

    let cbs = props
        .selected_files
        .iter()
        .map(|_| props.remove_file_cb.clone());
    let combined = props.selected_files.iter().map(|f| f.clone()).zip(cbs);

    let lis = combined
        .map(|(f, cb)| html! { <FileLi selected_file={f} remove_file_cb={cb} /> })
        .collect::<Html>();

    html! {
        <div {ondragenter} {ondragleave} {ondrop} class={classes!(classes.as_ref())}>
            <span class="drop-title">{"Drop files here"}</span>
                {"or"}
            <input type="file" multiple=true id="images" value="" title=" " required=true />
            <div>
            { if props.selected_files.is_empty() {
                html! {
                    {"No files selected" }
                }
            } else {
                html! {
                    <ul>
                        {lis}
                    </ul>
                }
            } }
            </div>
        </div>
    }
}
