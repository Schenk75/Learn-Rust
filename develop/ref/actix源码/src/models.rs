use serde::{Deserialize, Serialize};
use crate::schema::*;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub title: String,
    pub date_created: String
}

#[derive(Debug, Insertable)]
#[table_name = "product"]
pub struct PostProduct<'a> {
    pub name: &'a str,
    pub title: &'a str,
    pub date_created: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductJson {
    pub name: String,
    pub title: String,
}