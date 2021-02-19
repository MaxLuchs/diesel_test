use diesel::pg::PgConnection;
use std::error::Error;
use diesel::{ConnectionResult, Connection, ExpressionMethods, QueryDsl, RunQueryDsl};
use crate::models::{Animal, Zoo, Species};
use crate::schema::*;
use eyre::*;

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

    pub fn insert_animal(&self, name: String, age: i32, zoo_name: String, species: String) -> Result<usize, Box<dyn Error>> {
        let species: Species = species::table.find(species).first::<Species>(&self.con)?;
        let zoo: Zoo = zoos::table.filter(zoos::name.eq(zoo_name)).first(&self.con)?;
        let result = diesel::insert_into(animals::table).values((animals::name.eq(name), animals::species.eq(species.name), animals::zoo_id.eq(&zoo.id), animals::age.eq(age))).execute(&self.con)?;
        Ok(result)
    }

    pub fn get_first_animal(&self, zoo_id: i32) -> Option<Animal> {
        let result = animals::table.filter(animals::zoo_id.eq(zoo_id)).first::<Animal>(&self.con);
        return result.ok();
    }

    pub fn delete_animal_by_name(&self, name: String) -> Result<usize, Box<dyn Error>> {
        Ok(diesel::delete(animals::table.filter(animals::name.eq(&name))).execute(&self.con)?)
    }

    pub fn update_zoo_address(&self, zoo_name: String, new_address: String) -> Result<usize, Box<dyn Error>> {
        let res = diesel::update(zoos::table.filter(zoos::name.eq(zoo_name))).set(zoos::street.eq(new_address)).execute(&self.con)?;
        Ok(res)
    }

    pub fn get_all_animal_data(&self) -> Result<Vec<(Animal, Zoo)>, Box<dyn Error>> {
        let animals = animals::table
            .inner_join(zoos::table)
            .load::<(Animal, Zoo)>(&self.con)?;
        //println!("get_all_animals : {:?}", &animals);
        return Ok(animals);
    }
}

pub fn get_con(db_url: String) -> Result<ConnectionResult<PgConnection>, Box<dyn Error>> {
    let con = PgConnection::establish(&db_url);
    return Ok(con);
}