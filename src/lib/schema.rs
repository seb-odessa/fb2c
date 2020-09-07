table! {
    archives (id) {
        id -> Integer,
        zip_name -> Text,
        zip_path -> Text,
        zip_md5 -> Text,
    }
}

table! {
    authors (id) {
        id -> Integer,
        first_name -> Text,
        middle_name -> Text,
        last_name -> Text,
        nickname -> Text,
        uuid -> Text,
    }
}

table! {
    books (id) {
        id -> Integer,
        book_title -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    archives,
    authors,
    books,
);
