use axum::{extract::{State, Path}, Json, http::StatusCode};
use mongodb::bson::{Bson, doc, oid::ObjectId};
use mongodb::Database;
use crate::models::{Book, NewBook, BookDto};
use serde_json::json;
use std::sync::Arc;
use futures::stream::TryStreamExt;

pub async fn get_books(
    State(db): State<Arc<Database>>,
) -> Result<(StatusCode, Json<Vec<BookDto>>), (StatusCode, String)> {
    println!("LOG: GET /books called");
    let coll = db.collection::<Book>("books");
    let mut cursor = coll.find(doc! {}).await.map_err(|e| {
        println!("LOG: GET /books error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    let mut books = Vec::new();
    while let Some(book) = cursor.try_next().await.map_err(|e| {
        println!("LOG: GET /books cursor error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })? {
        books.push(BookDto::from(book));
    }

    println!("LOG: GET /books success, returned {} books", books.len());
    Ok((StatusCode::OK, Json(books)))
}

pub async fn create_book(
    State(db): State<Arc<Database>>,
    Json(payload): Json<NewBook>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, String)> {
    println!("LOG: POST /books called with payload: {:?}", payload);
    let coll = db.collection::<NewBook>("books");

    // check duplicate name
    match coll.find_one(doc! { "name": &payload.name }).await {
        Ok(Some(_)) => {
            println!("LOG: POST /books conflict: name already exists: {}", payload.name);
            return Err((StatusCode::CONFLICT, "Book name already exists".to_string()));
        }
        Ok(None) => {}
        Err(e) => {
            println!("LOG: POST /books find_one error: {}", e);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
        }
    }

    let insert_result = coll
        .insert_one(payload)
        .await
        .map_err(|e| {
            println!("LOG: POST /books error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    let id_str = match insert_result.inserted_id {
        Bson::ObjectId(oid) => oid.to_hex(),
        other => other.to_string(),
    };
    println!("LOG: POST /books success, inserted id: {}", id_str);
    Ok((StatusCode::CREATED, Json(json!({ "id": id_str }))))
}

pub async fn get_book_by_id(
    State(db): State<Arc<Database>>,
    Path(id): Path<String>,
) -> Result<(StatusCode, Json<BookDto>), (StatusCode, String)> {
    println!("LOG: GET /books/{} called", id);
    let coll = db.collection::<Book>("books");

    let oid = ObjectId::parse_str(&id).map_err(|_| {
        println!("LOG: GET /books/{} invalid id", id);
        (StatusCode::BAD_REQUEST, "Invalid ID".to_string())
    })?;

    let found = coll
        .find_one(doc! { "_id": oid })
        .await
        .map_err(|e| {
            println!("LOG: GET /books/{} error: {}", id, e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    match found {
        Some(book) => {
            println!("LOG: GET /books/{} success", id);
            Ok((StatusCode::OK, Json(BookDto::from(book))))
        }
        None => {
            println!("LOG: GET /books/{} not found", id);
            Err((StatusCode::NOT_FOUND, "Book not found".to_string()))
        }
    }
}

pub async fn update_book(
    State(db): State<Arc<Database>>,
    Path(id): Path<String>,
    Json(payload): Json<NewBook>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, String)> {
    println!("LOG: PUT /books/{} called with payload: {:?}", id, payload);

    let coll = db.collection::<Book>("books");

    let oid = ObjectId::parse_str(&id).map_err(|_| {
        println!("LOG: PUT /books/{} invalid id", id);
        (StatusCode::BAD_REQUEST, "Invalid ID".to_string())
    })?;

    // check existence
    match coll.find_one(doc! { "_id": &oid }).await {
        Ok(Some(found)) => {
            println!("LOG: PUT /books/{} found before update: {:?}", id, found);
        }
        Ok(None) => {
            println!("LOG: PUT /books/{} not found (pre-check)", id);
            return Err((StatusCode::NOT_FOUND, "Book not found".to_string()));
        }
        Err(e) => {
            println!("LOG: PUT /books/{} find_one error: {}", id, e);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
        }
    }

    let update_doc = doc! {
        "$set": { "name": &payload.name, "author": &payload.author, "year": payload.year }
    };

    let res = coll.update_one(doc! { "_id": &oid }, update_doc).await.map_err(|e| {
        println!("LOG: PUT /books/{} update error: {}", id, e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    if res.matched_count == 0 {
        println!("LOG: PUT /books/{} not found", id);
        return Err((StatusCode::NOT_FOUND, "Book not found".to_string()));
    }

    println!("LOG: PUT /books/{} success, modified_count={}", id, res.modified_count);
    Ok((StatusCode::OK, Json(json!({ "message": "Updated" }))))
}

pub async fn delete_book(
    State(db): State<Arc<Database>>,
    Path(id): Path<String>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, String)> {
    println!("LOG: DELETE /books/{} called", id);
    let coll = db.collection::<Book>("books");
    let oid = ObjectId::parse_str(&id).map_err(|_| {
        println!("LOG: DELETE /books/{} invalid id", id);
        (StatusCode::BAD_REQUEST, "Invalid ID".to_string())
    })?;
    let delete_result = coll.delete_one(doc! { "_id": oid }).await.map_err(|e| {
        println!("LOG: DELETE /books/{} error: {}", id, e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;
    if delete_result.deleted_count == 0 {
        println!("LOG: DELETE /books/{} not found", id);
        return Err((StatusCode::NOT_FOUND, "Book not found".to_string()));
    }
    println!("LOG: DELETE /books/{} success", id);
    Ok((StatusCode::OK, Json(json!({ "message": "Deleted" }))))
}
