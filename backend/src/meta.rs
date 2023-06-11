use serde_json;
use shared::{FileMeta, FileType};
use warp::{
    hyper::Response,
    reject::{self, Reject},
    Rejection, Reply,
};

fn fetch_meta_for_file(id: String) -> Option<FileMeta> {
    // TODO -> Lookup actual file meta
    let result = FileMeta {
        file_link: String::from("123"),
        file_name: id,
        file_size: 123456,
        file_type: FileType::ASCII,
        file_uploaded: 123455,
    };
    Some(result)
}

#[derive(Debug)]
struct ParsingError;
impl Reject for ParsingError {}

fn json_response(fm: FileMeta) -> Result<impl Reply, Rejection> {
    match serde_json::to_string(&fm) {
        Ok(json_string) => Ok(Response::new(json_string)),
        Err(_) => Err(reject::custom(ParsingError)),
    }
}

pub async fn meta_for_file(id: Option<String>) -> Result<impl Reply, Rejection> {
    match id {
        None => Err(warp::reject()),
        Some(f_id) => match fetch_meta_for_file(f_id) {
            Some(fm) => json_response(fm),
            None => Err(warp::reject()),
        },
    }
}
