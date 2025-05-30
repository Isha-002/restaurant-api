use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Type;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone, Type)]
#[sqlx(type_name = "role")]
#[sqlx(rename_all = "lowercase")]
#[allow(non_camel_case_types)]
pub enum Role {
    customer,
    restaurant_owner,
    banned_user,
    admin,
}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Account {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub phone_number: String,
    pub role: Role,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewAccount {
    pub name: String,
    pub email: String,
    pub password: String,
    pub phone_number: String,
    pub role: Role,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Login {
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session {
    pub exp: DateTime<Utc>,
    pub account_id: Uuid,
    pub nbf: DateTime<Utc>
}