
use web_sys::{File, console};
use yew::{html, Callback, Component, Html, Properties, suspense::{use_future}, function_component, HtmlResult, BaseComponent};

#[path = "./upload.rs"]
mod upload;

pub struct BaseUploadTask{
    file: File,
    // Progress: i32
}

pub enum Msg {
    // Progress(i32),
    Done,
}

#[function_component(WithAsyncUpload)]
pub fn upload_task<T>(props: &TaskProps) -> HtmlResult where T: Component<Properties = TaskProps>  {
    let file = props.file.clone();
    console::log_1(&"New task:".into());

    let future = use_future(|| async move {
        console::log_1(&"task start?:".into());
        let fil = upload::upload_file(file).await;
        console::log_1(&"task done?:".into());

    });

    let p = props.clone();

    Ok(html! {<T ..p />})
}

#[derive(Properties, Clone, PartialEq)]
pub struct TaskProps {
    pub file: File,
    pub done_cb: Callback<File>
}

impl Component for BaseUploadTask {
    type Message = Msg;

    type Properties = TaskProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        
        BaseUploadTask {
            file: _ctx.props().file.clone(),
            // Progress: 0
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
            },
        }
    }
}

pub type UploadTask = WithAsyncUpload<BaseUploadTask>;