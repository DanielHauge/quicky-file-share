
use yew::{html, Html, Properties, Children, function_component};



#[derive(Properties, Clone, PartialEq)]
pub struct TasksProps {
    pub children: Children
}

#[function_component(Tasks)]
pub fn upload_field(props: &TasksProps) -> Html {
    html! {
        <li>
            { props.children.clone() }
        </li>
    }
}
