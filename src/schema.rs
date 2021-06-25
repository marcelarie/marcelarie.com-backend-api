table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        description -> Text,
        body -> Text,
        created_at -> Timestamp,
        published -> Bool,
    }
}

table! {
    posts_by_year (id) {
        id -> Int4,
        year -> Nullable<Int2>,
        post_id -> Int4,
    }
}

joinable!(posts_by_year -> posts (post_id));

allow_tables_to_appear_in_same_query!(
    posts,
    posts_by_year,
);
