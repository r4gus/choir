table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        password_hash -> Varchar,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        street -> Nullable<Varchar>,
        house_number -> Nullable<Varchar>,
        zip -> Nullable<Varchar>,
        city -> Nullable<Varchar>,
        phone -> Nullable<Varchar>,
        is_admin -> Bool,
    }
}
