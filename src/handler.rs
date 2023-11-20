use serde::{Serialize, Deserialize};
use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    http::Response, 
    Json,
};
use serde_json::json;

use crate::{schema::NearBySearchOptions, AppState};


enum PlaceType {
    Cafe,
    Restaurant
  }
  

  #[derive(Default)]
  struct GetPlacesSearchNearByOptions{
    latitude: f64,
    longitude:f64,
    radius:u16,
    r#type:String,

}


const GOOGLE_PLACE_NEAR_BY_SEARCH_URL: &str =
    "https://places.googleapis.com/v1/places:searchNearby";

    pub async fn get_places_search_near_by(
            State(data): State<Arc<AppState>>,
        ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    enum PlaceType {
        CAFE,
        RESTAURANT
    }
    #[derive(Serialize)]
    struct LocationRestriction {
        circle: Circle
      }
      
      #[derive(Serialize)]
      struct Circle {
        center: Center,
        radius: f64
      } 
      #[derive(Serialize)]
      struct Center {
        latitude: f64,
        longitude: f64
      }
      
    
      
      struct RequestBody  {
        included_types: Vec<PlaceType>,
        maxResultCount: u8,
        locationRestriction:  Vec<String>,
      }

      let body = LocationRestriction {
        circle: Circle {
          center: Center {
            latitude: 37.5,
            longitude: 127.0  
          },
          radius: 500.0
        }
      };

    let request_body = serde_json::to_string(&body).unwrap();

    let resp = data.client.post("https://places.googleapis.com/v1/places:searchNearby")
    .json(&request_body)
    .send()
    .await
    .unwrap();
  
    #[derive(Serialize)]
    #[derive(Deserialize)] 
    struct PlacesResponse{

  }
  // 응답 본문 역직렬화
//   let places: PlacesResponse = resp.json().await.unwrap();
//   let places_value: serde_json::Value = serde_json::to_value(places).unwrap();
//   let axum_response = Response::new(Json(places_value));
//   Ok(axum_response)

    const MESSAGE: &str = "Simple CRUD API with Rust, SQLX, Postgres,and Axum";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Ok(Json(json_response))
  // Axum 응답으로 반환


}



// pub async fn place_near_by_search_handler(
//     opts: Option<Query<NearBySearchOptions>>,
//     State(data): State<Arc<AppState>>,
// ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
//     let Query(opts) = opts.unwrap_or_default();

//     const GOOGLE_PLACE_NEAR_BY_SEARCH_URL: &str =
//         "https://places.googleapis.com/v1/places:searchNearby";

//         let client = reqwest::Client::new();
//         let request_body = {};

      
//         let resp = client.post("https://places.googleapis.com/v1/places:searchNearby")
//             .json(&request_body)
//             .send()
//             .await;
    
//     let json_response = serde_json::json!({
//         "status": "success",
//         "message": MESSAGE
//     });

//     return Json({})
// }

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
