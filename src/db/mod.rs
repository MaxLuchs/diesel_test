use diesel::pg::PgConnection;
use std::error::Error;
use diesel::{ConnectionResult, Connection, ExpressionMethods, QueryDsl, RunQueryDsl};
use crate::models::Animal;
use crate::schema::*;

pub struct DAO {
    con: PgConnection
}

impl DAO {
    pub fn new(db_url: String) -> DAO {
        return DAO {
            con: PgConnection::establish(&db_url).unwrap()
        };
    }

    pub fn get_all_animals(&self) -> Vec<Animal> {
        return animals::table.load::<Animal>(&self.con).unwrap();
    }

    pub fn get_animals_by_name(&self, name: &str) -> Vec<Animal> {
        return animals::table.filter(animals::name.eq(name)).load::<Animal>(&self.con).unwrap();
    }
}

pub fn get_con(db_url: String) -> Result<ConnectionResult<PgConnection>, Box<dyn Error>> {
    let con = PgConnection::establish(&db_url);
    return Ok(con);
}