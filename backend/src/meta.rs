use warp::{Reply, Rejection, hyper::Response};
use shared::{FileMeta,FileType};
use serde_json;

pub async fn meta_for_file(_id: String) -> Result<impl Reply, Rejection> {
    // TODO -> Lookup actual file meta.
    let result = FileMeta {
        file_link: String::from("123"),
        file_name: String::from("test_name"),
        file_size: 123456,
        file_type: FileType::ASCII,
        file_uploaded: 123455
    };
    let gg = serde_json::to_string(&result).unwrap();
    let resp = Response::new(gg);
    Ok(resp)
}