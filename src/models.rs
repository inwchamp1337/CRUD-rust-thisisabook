use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

/// DB model
#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub author: String,
    pub year: i32,
}

/// API model - สำหรับ request
#[derive(Debug, Serialize, Deserialize)]
pub struct NewBook {
    pub name: String,
    pub author: String,
    pub year: i32,
}

/// API model - สำหรับ response ที่มี id (เป็น string)
#[derive(Debug, Serialize, Deserialize)]
pub struct BookDto {
    pub id: String,
    pub name: String,
    pub author: String,
    pub year: i32,
}

/// แปลงจาก DB model เป็น API model
impl From<Book> for BookDto {
    fn from(book: Book) -> Self {
        Self {
            id: book.id.map(|oid| oid.to_hex()).unwrap_or_default(),
            name: book.name,
            author: book.author,
            year: book.year,
        }
    }
}