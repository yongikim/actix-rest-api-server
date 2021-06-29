table! {
    recipes (id) {
        id -> Int4,
        title -> Varchar,
        making_time -> Varchar,
        serves -> Varchar,
        ingredients -> Varchar,
        cost -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
