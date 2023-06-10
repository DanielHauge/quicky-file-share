#[path = "../components/upload_drop_zone.rs"] mod upload_drop_zone;
#[path = "../components/upload_field.rs"] mod upload_field;


use web_sys::File;
use yew::{Component, Html, html, Callback};
use upload_drop_zone::UploadZone;
use upload_field::UploadField;


use crate::home::upload::upload_drop_zone::FileListIterator;


pub struct Upload {
    files: Vec<File>,
    zipped: bool
}

pub enum Msg {
    Files(FileListIterator),
    Remove(String),
    Zip(bool)
}


impl Component for Upload {
    type Message = Msg;

    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Upload { files: vec![], zipped: false }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let file_dropped_cb: Callback<FileListIterator> = ctx.link().callback(|fl| Msg::Files(fl));
        let remove_file_cb: Callback<String> = ctx.link().callback(|fl| Msg::Remove(fl));
        let zip_toggle_cb: Callback<bool> = ctx.link().callback(|fl| Msg::Zip(fl));
            
        let selected_files: Vec<String> = self.files.iter().map(|f| f.name()).collect();
        html! {
            <UploadZone {file_dropped_cb}>
                <UploadField {selected_files} {remove_file_cb} {zip_toggle_cb} />
            </UploadZone>
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Files(fl) => {
                let file_names: Vec<String> = self.files.iter().map(|f| f.name()).collect();
                for file in fl {
                    if !file_names.contains(&file.name()){
                        self.files.push(file);
                    }
                }
                true
            },
            Msg::Remove(file_for_removal) => {
                match self.files.iter().position(|f| f.name() == file_for_removal) {
                    None => false,
                    Some(i) => {
                        self.files.remove(i);
                        true
                    }
                }
            }
            Msg::Zip(z) => {
                self.zipped = z;
                false
            }

        }
    }
}