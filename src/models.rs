use diesel::*;
use crate::schema::animals;

#[derive(Insertable, Queryable, Debug)]
#[table_name="animals"]
pub struct Animal {
    pub id: i32,
    pub name: Option<String>,
    pub atype: Option<String>,
    pub age: Option<i32>,
}