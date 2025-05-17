use diesel::{Identifiable, Insertable, Queryable};

use super::schema;

#[derive(Debug, Clone, Identifiable, Queryable, Insertable)]
#[diesel(table_name = schema::users)]
pub struct User {
    pub id: i32,
    pub user_id: String,
    pub email: String,
    pub name: String,
    pub password: String,
    pub balance_in_rs: i64,
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
    pub amount_in_rs: i64,
    pub description: Option<String>,
    pub created_at: time::PrimitiveDateTime,
    pub status: String,
    pub updated_at: time::PrimitiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = schema::users)]
pub struct NewUser<'a> {
    pub user_id: &'a str,
    pub email: &'a str,
    pub name: &'a str,
    pub password: &'a str,
    pub balance_in_rs: i64,
    pub created_at: time::PrimitiveDateTime,
    pub last_modified_at: time::PrimitiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = schema::transactions)]
pub struct NewTransaction<'a> {
    pub transaction_id: &'a str,
    pub sender_id: &'a str,
    pub recipient_id: &'a str,
    pub amount_in_rs: i64,
    pub description: Option<&'a str>,
    pub created_at: time::PrimitiveDateTime,
    pub status: &'a str,
    pub updated_at: time::PrimitiveDateTime,
}
