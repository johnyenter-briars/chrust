use std::io::Cursor;

use actix_web::HttpResponse;
use actix_web::http::header::ContentType;

use crate::board::coordinate::Coordinate;
use crate::state::programstate::ProgramState;

#[derive(Serialize, Deserialize)]
pub struct MoveOptionsResponse {
    pub options: Vec<Coordinate>,
}
impl MoveOptionsResponse {
    pub fn ok(options: Vec<Coordinate>) -> HttpResponse {
        HttpResponse::Ok().content_type(ContentType::json()).body(
            MoveOptionsResponse  {
                options
            }
            .as_serde_string(),
        )
    }

    pub fn as_serde_string(self) -> String {
        serde_json::to_string(&self).expect("Unable to parse response object!")
    }
}


#[derive(Serialize, Deserialize)]
pub struct ValidateResponse {
    pub is_valid: bool,
}

impl ValidateResponse {
    pub fn ok(is_valid: bool) -> HttpResponse {
        HttpResponse::Ok().content_type(ContentType::json()).body(
            ValidateResponse {
                is_valid
            }
            .as_serde_string(),
        )
    }

    pub fn as_serde_string(self) -> String {
        serde_json::to_string(&self).expect("Unable to parse response object!")
    }
}

impl SettingsResponse {
    pub fn ok(program_state: ProgramState) -> HttpResponse {
        HttpResponse::Ok().content_type(ContentType::json()).body(
            SettingsResponse  {
                program_state
            }
            .as_serde_string(),
        )
    }

    pub fn as_serde_string(self) -> String {
        serde_json::to_string(&self).expect("Unable to parse response object!")
    }
}


#[derive(Serialize, Deserialize)]
pub struct SettingsResponse {
    pub program_state: ProgramState
}

