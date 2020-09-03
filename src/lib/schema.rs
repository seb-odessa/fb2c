table! {
    authors (id) {
        id -> Integer,
        first_name -> Nullable<Text>,
        middle_name -> Nullable<Text>,
        last_name -> Nullable<Text>,
        nickname -> Nullable<Text>,
        lib_id -> Nullable<Text>,
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
    emails,
    homepages,
);
