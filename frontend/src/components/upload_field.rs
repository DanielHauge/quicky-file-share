use yew::{prelude::*, classes};

#[function_component(UploadField)]
pub fn upload_field() -> Html {

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

    let ondragover={
        move |e:html::ondragover::Event| e.prevent_default() 
    };

    let ondrop = {
        let drag_class_c = drag_class.clone();
        move |e:html::ondrop::Event| {
            e.prevent_default();
            drag_class_c.set("");
        }
    };



    html! {
        <label ondragenter={ondragenter} ondragleave={ondragleave} ondragover={ondragover} ondrop={ondrop} for="images" class={classes!(classes.as_ref())}>
            <span class="drop-title">{"Drop files here"}</span>
            {"or"}
            <input type="file" multiple=true id="images" required=true />
        </label>
    }
}