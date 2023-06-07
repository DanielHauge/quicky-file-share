use warp::{multipart::FormData, reply::Response, Reply, Rejection};

pub struct File {
    data: FormData
}

impl warp::Reply for File {
    fn into_response(self) -> warp::reply::Response {
        Response::new(format!("Thanks!, {:?}", self.data).into())
    }
}

/// Downloads a file given a string
/// TODO: This function is a placeholder
pub async fn upload_file(form: FormData) -> Result<impl Reply, Rejection> {
    let file = File{
        data: form
    };
    Ok(file)
}