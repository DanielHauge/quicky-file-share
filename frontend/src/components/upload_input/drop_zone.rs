use web_sys::{DragEvent, File, FileList};
use yew::prelude::*;

pub struct FileListIterator {
    files: FileList,
    index: u32,
}

impl FileListIterator {
    fn from(drag_event: DragEvent) -> FileListIterator {
        let list = drag_event.data_transfer().unwrap().files().unwrap();
        FileListIterator {
            files: list,
            index: 0,
        }
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

#[derive(Properties, Clone, PartialEq)]
pub struct DropZoneProps {
    pub file_dropped_cb: Callback<FileListIterator>,
    pub children: Children,
}

#[function_component(DropZone)]
pub fn upload_field(props: &DropZoneProps) -> Html {
    let ondragover = { move |e: html::ondragover::Event| e.prevent_default() };

    let ondrop = {
        let cb = props.file_dropped_cb.clone();
        move |e: DragEvent| {
            e.prevent_default();
            let iterator = FileListIterator::from(e);
            cb.emit(iterator)
        }
    };

    html! {
        <div ondrop={ondrop} ondragover={ondragover} >
            { props.children.clone() }
        </div>
    }
}
