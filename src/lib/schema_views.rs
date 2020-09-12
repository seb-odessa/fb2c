
table! {
    genres_view (id) {
        id -> Integer,
        code -> Text,
        name -> Text,
        group -> Text,
    }
}

table! {
    titles_view (id) {
        id -> Integer,
        title -> Text,
    }
}

table! {
    authors_view (id) {
        id -> Integer,
        book_id -> Integer,
        first_name -> Text,
        middle_name -> Text,
        last_name -> Text,
        nickname -> Text,
        uuid -> Text,
    }
}
