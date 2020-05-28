table! {
    books (id) {
        id -> Int4,
        book_uid -> Varchar,
        title -> Varchar,
        author -> Varchar,
        year -> Int2,
    }
}

table! {
    ratings (id) {
        id -> Int4,
        user_id -> Int4,
        book_id -> Int4,
        score -> Float8,
    }
}

table! {
    users (id) {
        id -> Int4,
        address -> Varchar,
        age -> Nullable<Int2>,
    }
}

joinable!(ratings -> books (book_id));
joinable!(ratings -> users (user_id));

allow_tables_to_appear_in_same_query!(
    books,
    ratings,
    users,
);
