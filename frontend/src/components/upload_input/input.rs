#[path = "./drop_zone.rs"]
mod drop_zone;
#[path = "./field.rs"]
mod field;

use web_sys::File;
use yew::{html, Callback, Component, Html, Properties};

use drop_zone::{DropZone, FileListIterator};
use field::Field;

pub struct UploadInput {
    files: Vec<File>,
    zipped: bool,
}

pub enum Msg {
    Files(FileListIterator),
    Remove(String),
    Zip(bool),
    Submit,
}

#[derive(Properties, Clone, PartialEq)]
pub struct UploadInputProps {
    pub upload_files_cb: Callback<Vec<File>>,
}

impl Component for UploadInput {
    type Message = Msg;

    type Properties = UploadInputProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        UploadInput {
            files: vec![],
            zipped: false,
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let file_dropped_cb: Callback<FileListIterator> = ctx.link().callback(|fl| Msg::Files(fl));
        let remove_file_cb: Callback<String> = ctx.link().callback(|fl| Msg::Remove(fl));
        let zip_toggle_cb: Callback<bool> = ctx.link().callback(|fl| Msg::Zip(fl));
        let onclick = ctx
            .link()
            .callback(|_: yew::events::MouseEvent| Msg::Submit);

        let selected_files: Vec<String> = self.files.iter().map(|f| f.name()).collect();
        html! {
            <div>
                <DropZone {file_dropped_cb}>
                    <Field {selected_files} {remove_file_cb} {zip_toggle_cb} />
                    <button {onclick}> {"Click"} </button>
                </DropZone>
            </div>
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Files(fl) => {
                let file_names: Vec<String> = self.files.iter().map(|f| f.name()).collect();
                for file in fl {
                    if !file_names.contains(&file.name()) {
                        self.files.push(file);
                    }
                }
                true
            }
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
            Msg::Submit => {
                _ctx.props().upload_files_cb.emit(self.files.clone());
                self.files.clear();
                true
            }
        }
    }
}
