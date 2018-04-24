table! {
    users (id) {
        id -> Int4,
        email -> Text,
        username -> Text,
        password -> Text,
        created_at -> Timestamp,
    }
}
table! {
    theme (id) {
        id -> Int4,
        user_id -> Int4,
        category -> Text,
        status -> Int4,
        title -> Text,
        content -> Text,
        view_count -> Int4,
        comment_count -> Int4,
        created_at -> Timestamp,
    }
}