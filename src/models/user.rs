use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ObjectId,

    pub email: String,

    pub password_hash: String,

    pub role: UserRole,

    pub status: UserStatus,

    pub email_verified: bool,

    pub created_at: mongodb::bson::DateTime,

    pub updated_at: mongodb::bson::DateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UserRole {
    SuperAdmin,
    Admin,
    Lecturr,
    Staff,
    Student,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UserStatus {
    Active,
    Suspended,
    Disabled,
}