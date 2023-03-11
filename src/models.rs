use mongodb::bson::{oid::ObjectId, Bson, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Admin {
    pub _id: ObjectId,
    pub name: String,
    pub email: String,
    pub password: String,
    pub blocked: bool,
    pub permissions: Vec<String>,
    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub _id: ObjectId,
    pub name: String,
    pub email: String,
    pub password: String,
    pub blocked: bool,
    pub deleted: Option<DateTime>,
    pub status: String,
    pub company: ObjectId,
    pub verified: bool,
    pub permissions: Vec<String>,
    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub _id: ObjectId,
    pub title: String,
    pub description: String,
    pub blocked: bool,
    pub companies: Vec<ObjectId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Point {
    pub _id: ObjectId,
    pub id: Option<i64>,
    pub location: Option<Bson>,
    pub attributes: Option<Vec<Bson>>,
    pub layer: ObjectId,
    pub user: Option<ObjectId>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Photo {
    pub _id: ObjectId,
    pub uri: String,
    pub location: Option<Bson>,
    pub shot_on: i64,
    pub project: ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Layer {
    pub _id: ObjectId,
    pub title: String,
    pub icon: String,
    pub attributes: Option<Vec<Bson>>,
    pub user: ObjectId,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Company {
    pub _id: ObjectId,
    pub name: String,
    pub email: String,
    pub document: String,
    pub blocked: bool,
    pub deleted: Option<DateTime>,
    #[serde(rename = "type")]
    pub kind: String,
}

pub fn get_env(variable: &str) -> String {
    let error_message: String = format!("{} must be set.", variable);
    return std::env::var(variable).expect(error_message.as_str());
}
