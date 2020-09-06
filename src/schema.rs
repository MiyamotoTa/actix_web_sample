table! {
    users (id) {
        id -> Unsigned<Bigint>,
        name -> Varchar,
        email -> Varchar,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}
