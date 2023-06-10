#[path = "../components/upload_field.rs"] mod upload_field;

use log::info;
use yew::{prelude::*, classes};
use web_sys::{DragEvent, FileList, File};
use upload_field::UploadField;

pub struct FileListIterator {
    files: FileList,
    index: u32,
}

impl FileListIterator {
    fn from(drag_event: DragEvent) -> FileListIterator{
        let list = drag_event.data_transfer().unwrap().files().unwrap();
        FileListIterator{files: list, index: 0}
    }
}


impl Iterator for FileListIterator {
    type Item = File;

    fn next(&mut self) -> Option<Self::Item> {
        
        match self.files.get(self.index) {
            None => None,
            Some(item) => {
                self.index += 1;
                Some(item)
            }
        }
    }
}


#[function_component(UploadZone)]
pub fn upload_field() -> Html {

    let ondragover={ move |e:html::ondragover::Event| e.prevent_default() };

    let ondrop = {
        move |e:DragEvent| {
            e.prevent_default();
            let iterator = FileListIterator::from(e);
            for file in iterator {
                info!("ondrop {:?}", file.name());
            }
        }
    };

    html! {
        <div ondrop={ondrop} ondragover={ondragover} >
            <UploadField />
        </div>
    }
}