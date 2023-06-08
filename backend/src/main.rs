mod upload;
mod meta;

use std::collections::HashMap;
use warp::{Filter};


#[tokio::main]
async fn main() {
    println!("Starting server");

    let serve_frontend = warp::fs::dir("../frontend/dist");
    
    let download_file = warp::path("files")
        .and(warp::fs::dir("files"));

    let upload_file = warp::path!("files")
        .and(warp::post())
        .and(warp::multipart::form().max_length(u64::MAX))
        .and_then(upload::upload_file);

    let meta_file = warp::path!("api" / "meta")
        .and(warp::query::<HashMap<String, String>>())
        .map(|hm: HashMap<String, String>| {
            match hm.get("id") {
                None => None,
                Some(s) => Some(s.to_owned())
            }
        })
        .and_then(meta::meta_for_file);

    let not_found = warp::fs::file("../frontend/dist/index.html").map(|e| {
        println!("not found");
        e
    });

    let serve=meta_file
        .or(download_file)
        .or(upload_file)
        .or(serve_frontend)
        .or(not_found);

    warp::serve(serve).run(([127,0,0,1], 8080)).await;
}

