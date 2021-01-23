table! {
    archives (id) {
        id -> Integer,
        arch_name -> Text,
        arch_home -> Text,
        arch_size -> BigInt,
        arch_uuid -> Text,
        arch_done -> Bool,
    }
}

table! {
    author_links (id) {
        id -> Integer,
        book_id -> Integer,
        author_id -> Integer,
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
        arch_id -> Integer,
        book_file -> Text,
        book_zip_size -> BigInt,
        book_size -> BigInt,
        book_crc32 -> BigInt,
        book_offset -> BigInt,
    }
}

table! {
    genre_groups (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    genre_links (id) {
        id -> Integer,
        book_id -> Integer,
        genre_id -> Integer,
    }
}

table! {
    genre_names (id) {
        id -> Integer,
        group_id -> Integer,
        code -> Text,
        name -> Text,
    }
}

table! {
    genre_synonyms (id) {
        id -> Integer,
        code -> Text,
        synonym_id -> Integer,
    }
}

table! {
    genres (id) {
        id -> Integer,
        genre_name -> Text,
    }
}

table! {
    title_links (id) {
        id -> Integer,
        book_id -> Integer,
        title_id -> Integer,
    }
}

table! {
    titles (id) {
        id -> Integer,
        book_title -> Text,
    }
}

joinable!(author_links -> authors (author_id));
joinable!(author_links -> books (book_id));
joinable!(books -> archives (arch_id));
joinable!(genre_links -> books (book_id));
joinable!(genre_links -> genres (genre_id));
joinable!(genre_names -> genre_groups (group_id));
joinable!(genre_synonyms -> genre_names (synonym_id));
joinable!(title_links -> books (book_id));
joinable!(title_links -> titles (title_id));

allow_tables_to_appear_in_same_query!(
    archives,
    author_links,
    authors,
    books,
    genre_groups,
    genre_links,
    genre_names,
    genre_synonyms,
    genres,
    title_links,
    titles,
);
