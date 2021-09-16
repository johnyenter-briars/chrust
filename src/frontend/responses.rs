use std::io::Cursor;

use rocket::http::ContentType;
use rocket::response::Responder;
use rocket::response;
use rocket::response::Response;
use rocket::request::Request;
use crate::board::coordinate::Coordinate;

#[derive(Serialize, Deserialize)]
pub struct MoveOptionsResponse {
	pub options: Vec<Coordinate>
}

impl<'r> Responder<'r, 'static> for MoveOptionsResponse {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
		let response_string = serde_json::to_string(&self).unwrap();
        Response::build()
            .sized_body(response_string.len(), Cursor::new(response_string))
            .ok()
    }
}

#[derive(Serialize, Deserialize)]
pub struct ValidateResponse {
    pub is_valid: bool
}

impl<'r> Responder<'r, 'static> for ValidateResponse {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
		let response_string = serde_json::to_string(&self).unwrap();
        Response::build()
            .sized_body(response_string.len(), Cursor::new(response_string))
            .ok()
    }
}