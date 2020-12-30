table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        password_hash -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        street -> Varchar,
        house_number -> Varchar,
        zip -> Varchar,
        city -> Varchar,
        phone -> Varchar,
        is_admin -> Bool,
        verified -> Bool,
    }
}
