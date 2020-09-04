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

table! {
    emails (id) {
        id -> Integer,
        owner -> Integer,
        email -> Text,
    }
}

table! {
    homepages (id) {
        id -> Integer,
        owner -> Integer,
        homepage -> Text,
    }
}

joinable!(emails -> authors (owner));
joinable!(homepages -> authors (owner));

allow_tables_to_appear_in_same_query!(
    authors,
    books,
    emails,
    homepages,
);
