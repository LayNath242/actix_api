table! {
    categories (id) {
        id -> Int4,
        title -> Varchar,
        user_id -> Int4,
        created_at -> Nullable<Timestamp>,
    }
}

table! {
    comments (id) {
        id -> Int4,
        descriton -> Nullable<Varchar>,
        user_id -> Int4,
        created_at -> Nullable<Timestamp>,
    }
}

table! {
    courses (id) {
        id -> Int4,
        title -> Varchar,
        price -> Float8,
        thumbnail -> Nullable<Varchar>,
        video_url -> Varchar,
        description -> Nullable<Varchar>,
        cat_id -> Int4,
        created_at -> Nullable<Timestamp>,
    }
}

table! {
    question_answer (id) {
        id -> Int4,
        title -> Nullable<Varchar>,
        descriton -> Nullable<Varchar>,
        user_id -> Int4,
        created_at -> Nullable<Timestamp>,
    }
}

table! {
    roles (id) {
        id -> Int4,
        title -> Varchar,
        created_at -> Nullable<Timestamp>,
    }
}

table! {
    users (id) {
        id -> Int4,
        fullname -> Varchar,
        email -> Varchar,
        password -> Varchar,
        avatar -> Nullable<Varchar>,
        biography -> Nullable<Varchar>,
        created_at -> Timestamp,
        role_id -> Int4,
    }
}

joinable!(categories -> users (user_id));
joinable!(comments -> users (user_id));
joinable!(courses -> categories (cat_id));
joinable!(question_answer -> users (user_id));
joinable!(users -> roles (role_id));

allow_tables_to_appear_in_same_query!(
    categories,
    comments,
    courses,
    question_answer,
    roles,
    users,
);
