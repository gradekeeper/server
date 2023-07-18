use std::sync::Arc;
use axum::http::StatusCode;
use axum::{Extension, Json};
use cuid2::cuid;
use diesel::{insert_into, QueryDsl, RunQueryDsl, SelectableHelper};
use serde::{Deserialize,Serialize};
use time::{OffsetDateTime, PrimitiveDateTime};
use crate::errors::{AppError};
use crate::models::StudyBlock;
use crate::routes::api::auth::callback::Session;
use crate::schema::study_block::dsl::study_block;
use crate::ServerState;


pub async fn create_block(
                          Extension(user): Extension<Arc<Session>>,
                          Extension(state): Extension<Arc<ServerState>>,
                          Json(payload): Json<CreateBlock>)
    -> Result<Json<StudyBlock>, AppError> {
    let con = &mut state.db_pool.get().unwrap();
    let id = cuid2::create_id();
    let block = StudyBlock {
        end_date: payload.end_date,
        start_date: payload.start_date,
        id: id.clone(),
        name: payload.name,
        user_id: user.id.clone(),
    };

    insert_into(study_block).values(block).execute(con).or_else(|e|Err(AppError{
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
        description: format!("Could not create study block: {}", e)
    }))?;

    let block = study_block.find(id)
        .select(StudyBlock::as_select())
        .first(con)
        .or_else(|e|Err(AppError{
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
        description: format!("Could not load created study block: {}", e)
    }))?;

    Ok(Json(block))
}

#[serde(rename_all="camelCase")]
#[derive(Deserialize, Serialize)]
pub struct CreateBlock{
    pub end_date: PrimitiveDateTime,
    pub start_date: PrimitiveDateTime,
    pub name: String
}