mod upload;
mod meta;
use warp::{Filter};

#[tokio::main]
async fn main() {
    println!("Starting server");

    let root = warp::path!();
    let serve_frontend = root.and(warp::fs::dir("static"));

    let download_file = root.and(warp::path!("api" / "files")
        .and(warp::fs::dir("files")));

    let upload_file = root.and(warp::path!("api" / "files")
        .and(warp::post())
        .and(warp::multipart::form().max_length(u64::MAX)))
        .and_then(upload::upload_file);

    let meta_file = root.and(warp::path!("api" / "meta" / String))
        // .and(warp::get())
        .and_then(meta::meta_for_file);


    let serve=meta_file
        .or(upload_file)
        .or(download_file)
        .or(serve_frontend);

    warp::serve(serve).run(([127,0,0,1], 8081)).await;
}

