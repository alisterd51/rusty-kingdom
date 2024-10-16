// @generated automatically by Diesel CLI.

diesel::table! {
    buildings (id) {
        id -> Int4,
        name -> Varchar,
        level -> Int4,
        fortress_id -> Int4,
    }
}

diesel::table! {
    fortresses (id) {
        id -> Int4,
        gold -> Int4,
        food -> Int4,
        wood -> Int4,
        energy -> Int4,
    }
}

diesel::joinable!(buildings -> fortresses (fortress_id));

diesel::allow_tables_to_appear_in_same_query!(buildings, fortresses);
