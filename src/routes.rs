use axum::routing::get;
use axum::Router;
use std::sync::Arc;
use mongodb::Database;
use crate::handlers::{create_book, get_books, get_book_by_id, update_book, delete_book};
use tower_http::trace::TraceLayer;

pub fn create_router(db: Database) -> Router {
    let db = Arc::new(db);
    Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/books", get(get_books).post(create_book))
        .route("/books/{id}", get(get_book_by_id).put(update_book).delete(delete_book))
        .layer(TraceLayer::new_for_http())
        .with_state(db)
}