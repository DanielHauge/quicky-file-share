
use log::log;
use web_sys::{File, FileReader, Event, console};
use yew::{html, Callback, Component, Html, Properties, suspense::use_future};

#[path = "./upload.rs"]
mod upload;

pub struct Task{
    file: File,
    // Progress: i32
}

pub enum Msg {
    // Progress(i32),
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
        
        let file = _ctx.props().file.clone();

        // upload::upload_file(file);
        

        Task {
            file: _ctx.props().file.clone(),
            // Progress: 0
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| Msg::Done); 

        // JavaScript Stream -> Rust Stream , done pretty unsafe and badly right here, look away please xD
        
        // upload::gg(self.file.clone());
        
        // let result = fil.unwrap();


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
            },
            // Msg::Progress(p) => {
            //     self.Progress = p;
            //     true
            // }
        }
    }
}
