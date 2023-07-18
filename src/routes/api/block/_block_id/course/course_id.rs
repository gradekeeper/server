use std::sync::Arc;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::{Extension, Json};
use axum::response::{IntoResponse, Response};
use diesel::{delete, ExpressionMethods, QueryDsl, RunQueryDsl};
use log::info;
use serde::{Serialize, Deserialize};
use crate::errors::AppError;
use crate::schema::course::dsl::course;
use crate::schema::course::id;
use crate::ServerState;

#[derive(Serialize)]
pub struct GetCourse {

}
#[derive(Deserialize)]
pub struct UpdateCourse{}

pub async fn update_course(Path((_block_id, _course_id)): Path<(String, String)>, Json(_update_course): Json<UpdateCourse>) -> StatusCode {
    StatusCode::NOT_FOUND
}
pub async fn delete_course(Path((_block_id, _course_id)): Path<(String, String)>, Extension(state): Extension<Arc<ServerState>>) -> Result<Response,AppError> {
    let con = &mut state.db_pool.get().unwrap();
    let result = delete(course.filter(id.eq(_course_id))).execute(con).or_else(|e|AppError::database_ise(e).into())?;

    (result == 1).then(||StatusCode::OK.into_response()).ok_or(AppError::resource_not_found())
}
pub async fn get_course(Path((_block_id, _course_id)): Path<(String, String)>) -> (StatusCode, Json<GetCourse>) {
    (StatusCode::OK, Json(GetCourse{}))
}