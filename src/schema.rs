table! {
    users (id) {
        id -> Uuid,
        email -> Text,
        name -> Text,
        created_at -> Timestamptz,
    }
}
