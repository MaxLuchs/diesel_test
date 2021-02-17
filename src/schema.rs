table! {
    animals (id) {
        id -> Int4,
        name -> Varchar,
        species -> Varchar,
        zoo_id -> Int4,
        age -> Int4,
    }
}

table! {
    species (name) {
        name -> Varchar,
    }
}

table! {
    zoos (id) {
        id -> Int4,
        street -> Varchar,
        gps -> Nullable<Varchar>,
        name -> Varchar,
    }
}

joinable!(animals -> species (species));
joinable!(animals -> zoos (zoo_id));

allow_tables_to_appear_in_same_query!(
    animals,
    species,
    zoos,
);
