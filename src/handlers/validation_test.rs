use axum::Json;
use serde::Deserialize;
use serde_json::{json, Value};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct TestRequest {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8))]
    pub password: String,
}

pub async fn validation_test(
    Json(payload): Json<TestRequest>,
) -> Json<Value> {
    if let Err(errors) = payload.validate() {
        return Json(json!({
            "success": false,
            "message": "Validation failed",
            "errors": errors
        }));
    }

    Json(json!({
        "success":true,
        "message": "Validation passed"
    }))
}