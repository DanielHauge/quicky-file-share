use std::vec;

use web_sys::File;
use yew::prelude::*;

#[path = "../components/upload_input/input.rs"]
mod upload_input;
#[path = "../components/upload_tasks/tasks.rs"]
mod upload_tasks;
#[path = "../components/upload_tasks/task.rs"]
mod upload_task;


use upload_input::UploadInput;
use upload_tasks::Tasks;
use upload_task::Task;

pub enum Msg {
    UploadStart(Vec<File>),
    UploadDone(File),

}


pub struct Home {
    upload_tasks: Vec<File>,
    upload_recents: Vec<String> //TODO A more appropriate ds
}

impl Component for Home {
    type Message = Msg;

    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Home {
            upload_recents: vec![],
            upload_tasks: vec![],
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
            
        let upload_files_cb: Callback<Vec<File>> = ctx.link().callback(|fl| Msg::UploadStart(fl));
        let upload_complete: Callback<File> = ctx.link().callback(|f| Msg::UploadDone(f));

        html! {
            <div>
                <UploadInput upload_files_cb={&upload_files_cb} />
                <Tasks > 
                    { self.upload_tasks.iter().map(move |f|  {
                        html!{ <Task file={f.clone()} done_cb={upload_complete.clone()} /> }
                    }
                    ).collect::<Html>() }
                </Tasks>
        </div>
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UploadStart(f) => {
                for i in f {
                    self.upload_tasks.push(i)
                }
                true
            },
            Msg::UploadDone(f) => {
                self.upload_recents.push(f.name());
                true
            }
        }
    }
}

