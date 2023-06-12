

use web_sys::File;
use yew::{html, Callback, Component, Html, Properties};


pub struct Task{
    file: File
}

pub enum Msg {
    Done,
}

#[derive(Properties, Clone, PartialEq)]
pub struct TaskProps {
    pub file: File,
    pub done_cb: Callback<File>
}

impl Component for Task {
    type Message = Msg;

    type Properties = TaskProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Task {
            file: _ctx.props().file.clone()
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| Msg::Done); 

        html! {
            <div {onclick}>
                { ctx.props().file.name() }
            </div>
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Done => {
                ctx.props().done_cb.emit(self.file.clone());
                true
            }
        }
    }
}
