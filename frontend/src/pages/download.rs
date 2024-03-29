#[path = "../components/file.rs"]
mod file;
#[path = "../components/file_not_found.rs"]
mod file_not_found;

use std::rc::Rc;

use file::{File, FileProps};
use file_not_found::FileNotFound;
use shared::FileMeta;
use yew::prelude::*;
use yew::props;
use yew::suspense::use_future;
use yew::Properties;

#[derive(Properties, PartialEq)]
pub struct DownloadProps {
    pub file_id: AttrValue,
}

#[function_component(Overview)]
fn overview(p: &DownloadProps) -> HtmlResult {
    let fid = Rc::new(p.file_id.clone());
    let fetch = use_future(|| async {
        let gg = fid;
        reqwest::get(format!("api/meta?id={}", gg)).await?.json::<FileMeta>().await
        }
    )?;

    let file_info = match *fetch {
        Ok(ref res) => {
            let file_props = props!(FileProps {
                file_dl_link: String::from(&res.file_link),
                file_name: String::from(&res.file_name),
                file_size: res.file_size,
                file_uploaded: res.file_uploaded,
            });
            html! { <File ..file_props /> }
        }
        Err(ref failure) => {
            eprintln!("{}", failure.to_string());
            html! {
                <FileNotFound />
            }
        }
    };

    Ok(file_info)
}

#[function_component(Download)]
pub fn download(p: &DownloadProps) -> Html {
    let fallback = html! {<div> {"Loading"} </div>};
    let props = props! {
        DownloadProps { file_id: &p.file_id }
    };

    html! {
        <div>
            <Suspense {fallback}>
                <Overview ..props />
            </Suspense >
        </div>
    }
}
