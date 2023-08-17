use reqwest::Error;

use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{schema::NearBySearchOptions, AppState};

pub async fn place_near_by_search_handler(
    opts: Option<Query<NearBySearchOptions>>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(opts) = opts.unwrap_or_default();

    const GOOGLE_PLACE_NEAR_BY_SEARCH_URL: &str =
        "https://maps.googleapis.com/maps/api/place/nearbysearch/json";

    let mut queries = vec![];

    match opts.location {
        Some(value) => {
            queries.push(("location", value));
        }
        None => {
            return send_400_error("location is required");
        }
    }

    match opts.radius {
        Some(value) => {
            queries.push(("radius", value.to_string()));
        }
        None => {
            return send_400_error("radius is required");
        }
    }

    match opts.keyword {
        Some(value) => {
            queries.push(("keyword", value));
        }
        None => {
            return send_400_error("keyword is required");
        }
    }

    match opts.r#type {
        Some(value) => {
            queries.push(("type", value));
        }
        None => {
            return send_400_error("type is required");
        }
    }

    queries.push(("key",std::env::var("GOOGLE_MAPS_API_KEY").expect("GOOGLE_MAPS_API_KEY must be set")))

    let response = reqwest::get(GOOGLE_PLACE_NEAR_BY_SEARCH_URL).await?;


        match response.status().as_u16() {
        200..=299 => {
        let body = response.text().await?;
        println!("Success! Body:\n{}", body);
        }
        ..=599 => {
         status = response.status();
         error_message = response.text().await?;
        !("Error {}: {}", status, error_message);
        }
         => {
        !("Unexpected status code: {}", response.status());
        }
        }

    println!("Status: {}", response.status());

    return Ok(());

    // let offset = (opts.page.unwrap_or(1) - 1) * limit;
    // if opts.location == 0 {

    // }
}

// pub async fn health_checker_handler() -> impl IntoResponse {
//     const MESSAGE: &str = "Simple CRUD API with Rust, SQLX, Postgres,and Axum";

//     let json_response = serde_json::json!({
//         "status": "success",
//         "message": MESSAGE
//     });

//     Json(json_response)
// }

// pub async fn note_list_handler(
//     opts: Option<Query<FilterOptions>>,
//     State(data): State<Arc<AppState>>,
// ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
//     let Query(opts) = opts.unwrap_or_default();

//     let limit = opts.limit.unwrap_or(10);
//     let offset = (opts.page.unwrap_or(1) - 1) * limit;

//     let query_result = sqlx::query_as!(
//         NoteModel,
//         "SELECT * FROM notes ORDER by id LIMIT $1 OFFSET $2",
//         limit as i32,
//         offset as i32
//     )
//     .fetch_all(&data.db)
//     .await;

//     if query_result.is_err() {
//         let error_response = serde_json::json!({
//             "status": "fail",
//             "message": "Something bad happened while fetching all note items",
//         });
//         return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
//     }

//     let notes = query_result.unwrap();

//     let json_response = serde_json::json!({
//         "status": "success",
//         "results": notes.len(),
//         "notes": notes
//     });
//     Ok(Json(json_response))
// }

// pub async fn create_note_handler(
//     State(data): State<Arc<AppState>>,
//     Json(body): Json<CreateNoteSchema>,
// ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
//     let query_result = sqlx::query_as!(
//         NoteModel,
//         "INSERT INTO notes (title,content,category) VALUES ($1, $2, $3) RETURNING *",
//         body.title.to_string(),
//         body.content.to_string(),
//         body.category.to_owned().unwrap_or("".to_string())
//     )
//     .fetch_one(&data.db)
//     .await;

//     match query_result {
//         Ok(note) => {
//             let note_response = json!({"status": "success","data": json!({
//                 "note": note
//             })});

//             return Ok((StatusCode::CREATED, Json(note_response)));
//         }
//         Err(e) => {
//             if e.to_string()
//                 .contains("duplicate key value violates unique constraint")
//             {
//                 let error_response = serde_json::json!({
//                     "status": "fail",
//                     "message": "Note with that title already exists",
//                 });
//                 return Err((StatusCode::CONFLICT, Json(error_response)));
//             }
//             return Err((
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 Json(json!({"status": "error","message": format!("{:?}", e)})),
//             ));
//         }
//     }
// }

// pub async fn get_note_handler(
//     Path(id): Path<uuid::Uuid>,
//     State(data): State<Arc<AppState>>,
// ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
//     let query_result = sqlx::query_as!(NoteModel, "SELECT * FROM notes WHERE id = $1", id)
//         .fetch_one(&data.db)
//         .await;

//     match query_result {
//         Ok(note) => {
//             let note_response = serde_json::json!({"status": "success","data": serde_json::json!({
//                 "note": note
//             })});

//             return Ok(Json(note_response));
//         }
//         Err(_) => {
//             let error_response = serde_json::json!({
//                 "status": "fail",
//                 "message": format!("Note with ID: {} not found", id)
//             });
//             return Err((StatusCode::NOT_FOUND, Json(error_response)));
//         }
//     }
// }

// pub async fn edit_note_handler(
//     Path(id): Path<uuid::Uuid>,
//     State(data): State<Arc<AppState>>,
//     Json(body): Json<UpdateNoteSchema>,
// ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
//     let query_result = sqlx::query_as!(NoteModel, "SELECT * FROM notes WHERE id = $1", id)
//         .fetch_one(&data.db)
//         .await;

//     if query_result.is_err() {
//         let error_response = serde_json::json!({
//             "status": "fail",
//             "message": format!("Note with ID: {} not found", id)
//         });
//         return Err((StatusCode::NOT_FOUND, Json(error_response)));
//     }

//     let now = chrono::Utc::now();
//     let note = query_result.unwrap();

//     let query_result = sqlx::query_as!(
//         NoteModel,
//         "UPDATE notes SET title = $1, content = $2, category = $3, published = $4, updated_at = $5 WHERE id = $6 RETURNING *",
//         body.title.to_owned().unwrap_or(note.title),
//         body.content.to_owned().unwrap_or(note.content),
//         body.category.to_owned().unwrap_or(note.category.unwrap()),
//         body.published.unwrap_or(note.published.unwrap()),
//         now,
//         id
//     )
//     .fetch_one(&data.db)
//     .await
//     ;

//     match query_result {
//         Ok(note) => {
//             let note_response = serde_json::json!({"status": "success","data": serde_json::json!({
//                 "note": note
//             })});

//             return Ok(Json(note_response));
//         }
//         Err(err) => {
//             return Err((
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 Json(json!({"status": "error","message": format!("{:?}", err)})),
//             ));
//         }
//     }
// }

// pub async fn delete_note_handler(
//     Path(id): Path<uuid::Uuid>,
//     State(data): State<Arc<AppState>>,
// ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
//     let rows_affected = sqlx::query!("DELETE FROM notes  WHERE id = $1", id)
//         .execute(&data.db)
//         .await
//         .unwrap()
//         .rows_affected();

//     if rows_affected == 0 {
//         let error_response = serde_json::json!({
//             "status": "fail",
//             "message": format!("Note with ID: {} not found", id)
//         });
//         return Err((StatusCode::NOT_FOUND, Json(error_response)));
//     }

//     Ok(StatusCode::NO_CONTENT)
// }

fn send_400_error(message: &str) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    let error_response = json!({
        "status": "fail",
        "message": format!("{}", message)
    });
    Err((StatusCode::BAD_REQUEST, Json(error_response)))
}
