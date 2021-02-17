use diesel::*;
use crate::schema::*;

#[derive(Insertable, Queryable, Debug)]
#[table_name = "animals"]
pub struct Animal {
    pub id: i32,
    pub name: String,
    pub species: String,
    pub zoo_id: i32,
    pub age: i32,
}

#[derive(Insertable, Queryable, Debug)]
#[table_name = "zoos"]
pub struct Zoo {
    pub id: i32,
    pub street: String,
    pub gps: Option<String>,
    pub name: String,
}

#[derive(Insertable, Queryable, Debug)]
#[table_name = "species"]
pub struct Species {
    pub name: String,
}