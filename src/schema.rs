table! {
    appointments (id) {
        id -> Int4,
        title -> Varchar,
        place -> Varchar,
        begins -> Timestamp,
        ends -> Timestamp,
        description -> Varchar,
    }
}

table! {
    belongs (gid, uid) {
        gid -> Int4,
        uid -> Int4,
    }
}

table! {
    groups (gid) {
        gid -> Int4,
        title -> Varchar,
    }
}

table! {
    participates (aid, uid) {
        aid -> Int4,
        gid -> Int4,
        uid -> Int4,
    }
}

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

joinable!(belongs -> groups (gid));
joinable!(belongs -> users (uid));
joinable!(participates -> appointments (aid));
joinable!(participates -> groups (gid));
joinable!(participates -> users (uid));

allow_tables_to_appear_in_same_query!(
    appointments,
    belongs,
    groups,
    participates,
    users,
);
