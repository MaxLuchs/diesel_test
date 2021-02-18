use std::env::var;
use diesel_test::models::Animal;
use std::error::Error;
use dotenv::*;
use diesel_test::db::DAO;

fn main() -> std::result::Result<(), Box<dyn Error>> {
    dotenv().ok().unwrap();
    let db_url = var("DATABASE_URL").expect("No DB URL");
    let dao = DAO::new(db_url);
    let mut animals: Vec<Animal> = dao.get_all_animals();
    println!("animals : {:?}", animals);

    animals = dao.get_animals_by_name("Esel");
    println!("animals : {:?}", animals);

    let mut animal = dao.get_first_animal(1);
    println!("1st animal : {:?}", animal);

    dao.insert_animal("Emil".to_owned(), 32, "Zoo Dortmund".to_owned(), "Kuh".to_owned());

    Ok(())
}


