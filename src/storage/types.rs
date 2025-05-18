use diesel::{AsChangeset, Identifiable, Insertable, Queryable};

use crate::utils;

use super::schema;

#[derive(Debug, Clone, Identifiable, Queryable)]
#[diesel(table_name = schema::users)]
pub struct User {
    pub id: i32,
    pub user_id: String,
    pub email: String,
    pub name: String,
    pub password: String,
    pub balance_in_rs: f64,
    pub created_at: time::PrimitiveDateTime,
    pub last_modified_at: time::PrimitiveDateTime,
}

#[derive(Debug, Clone, Identifiable, Queryable, Insertable)]
#[diesel(table_name = schema::transactions)]
pub struct Transaction {
    pub id: i32,
    pub transaction_id: String,
    pub sender_id: String,
    pub recipient_id: String,
    pub amount_in_rs: f64,
    pub description: Option<String>,
    pub created_at: time::PrimitiveDateTime,
    pub status: String,
    pub updated_at: time::PrimitiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = schema::users)]
pub struct UserNew {
    pub user_id: String,
    pub email: String,
    pub name: String,
    pub password: String,
    pub balance_in_rs: f64,
}

#[derive(Insertable)]
#[diesel(table_name = schema::transactions)]
pub struct NewTransaction {
    pub transaction_id: String,
    pub sender_id: String,
    pub recipient_id: String,
    pub amount_in_rs: f64,
    pub description: Option<String>,
    pub created_at: time::PrimitiveDateTime,
    pub status: String,
    pub updated_at: time::PrimitiveDateTime,
}

#[derive(Clone, Debug, AsChangeset)]
#[diesel(table_name = schema::users)]
pub struct UserUpdateInternal {
    pub name: Option<String>,
    pub balance_in_rs: Option<f64>,
    pub last_modified_at: time::PrimitiveDateTime,
}

impl UserUpdateInternal {
    pub fn new(name: Option<String>, amount_in_rs: Option<f64>) -> Self {
        let last_modified_at = utils::datetime::now();
        Self {
            name,
            balance_in_rs: amount_in_rs,
            last_modified_at,
        }
    }
}
