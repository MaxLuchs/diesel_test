use diesel_test::db::DAO;
use dotenv::*;

pub fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    dotenv().ok().unwrap();
    let dao = DAO::new(std::env::var("DATABASE_URL")?);
    for (animal, zoo, species) in dao.get_all_animal_data()? {
        println!("Hallo, ich heisse {name} und bin ein(e) {tier} und lebe im Zoo \"{zoo_name}\" auf der \"{zoo_ort}\".", name = animal.name, tier = species.name, zoo_name = zoo.name, zoo_ort = zoo.street)
    }
    Ok(())
}