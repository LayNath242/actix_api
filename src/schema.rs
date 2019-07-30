table! {
    roles (id) {
        id -> Int4,
        title -> Varchar,
        created_at -> Nullable<Timestamp>,
    }
}

table! {
    users (id) {
        id -> Uuid,
        fullname -> Varchar,
        email -> Varchar,
        password -> Varchar,
        avatar -> Nullable<Varchar>,
        biography -> Nullable<Varchar>,
        created_at -> Timestamp,
        role_id -> Int4,
    }
}

joinable!(users -> roles (role_id));

allow_tables_to_appear_in_same_query!(
    roles,
    users,
);
