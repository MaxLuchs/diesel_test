use diesel::*;
use std::env::var;
use diesel_test::models::Animal;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel_test::schema;
use std::error::Error;
use dotenv::*;

fn main() -> std::result::Result<(), Box<dyn Error>> {
    dotenv().ok().unwrap();
    let db_url = var("DATABASE_URL").expect("No DB URL");
    let con = PgConnection::establish(&db_url).expect("No connection!");
    diesel::insert_into(schema::animals::table).values(Animal {
        id: 22i32,
        name: Some("Esell".to_string()),
        age: Some(11i32),
        atype: Some("dd".to_string()),
    }).execute(&con);
    use schema::animals;
    let x = animals::table.select(animals::name).filter(animals::name.eq("Esell")).load::<Option<String>>(&con);
    println!("{:?}", x);
    return Ok(());
}


