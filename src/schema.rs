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
