use mongodb::{
    bson::{doc, oid::ObjectId, DateTime},
    Database,
};

use serde::Deserialize;
use validator::Validate;

use crate::{
    models::user::{User, UserRole, UserStatus},
    services::password_service::hash_password,
};

const USERS_COLLECTION: &str = "users";

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8))]
    pub password: String,

    pub role: UserRole,
}

pub async fn create_user(
    database: &Database,
    request: CreateUserRequest,
) -> Result<User, String> {
    request.validate().map_err(|error| error.to_string())?;

    let email = request.email.trim().to_lowercase();

    let existing_user = find_user_by_email(
        database,
        &email,
    ).await.map_err(|error| error.to_string())?;

    if existing_user.is_some() {
        return Err("A user with this email already exists".to_string());
    }

    let password_hash = hash_password(
        &request.password,
    );

    let now = DateTime::now();

    let user = User {
        id: ObjectId::new(),
        email,
        password_hash: password_hash?,
        role: request.role,
        status: UserStatus::Active,
        email_verified: false,
        created_at: now,
        updated_at: now,
    };

    let collection = database.collection::<User>(USERS_COLLECTION);

    collection.insert_one(&user).await.map_err(|error| error.to_string())?;

    Ok(user)
}

pub async fn find_user_by_email(
    database: &Database,
    email: &str,
) -> mongodb::error::Result<Option<User>> {
    let collection = database.collection::<User>(USERS_COLLECTION);

    collection.find_one(doc! {
        "email": email
    }).await

}

pub async fn find_user_by_id(
    database: &Database,
    id: &ObjectId,
) -> mongodb::error::Result<Option<User>> {
    let collection = database.collection::<User>(USERS_COLLECTION);

    collection.find_one(doc! {
        "_id": id
    }).await
}