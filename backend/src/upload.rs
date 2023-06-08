use futures::{TryStreamExt, StreamExt};
use bytes::{Buf};
use tokio::task;
use std::{fs};
use warp::{
    multipart::{FormData},
    Rejection, Reply, reply::Response
};
pub struct File {
    data: FormData
}

impl warp::Reply for File {
    fn into_response(self) -> warp::reply::Response {
        Response::new(format!("Thanks!, {:?}", self.data).into())
    }
}

pub async fn upload_file(form: FormData) -> Result<impl Reply, Rejection> {
    task::spawn(async move {
        let mut parts = form.into_stream();
        while let Some(Ok(p)) = parts.next().await {
            let filename = p.filename().unwrap_or("unknown");
            let filepath = format!("files/{}", filename);
            fs::create_dir_all("files").unwrap();
            save_part_to_file(&filepath, p).await.expect("save error");
        }
    });
    Ok("Upload successful!")
}

async fn save_part_to_file(path: &str, part: warp::multipart::Part) -> Result<(), std::io::Error> {
    let data = part.stream()
        .try_fold(Vec::new(), |mut acc, buf| async move {
            acc.extend_from_slice(buf.chunk());
            Ok(acc)
        })
        .await.expect("folding error");
    std::fs::write(path, data)
}