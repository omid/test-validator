table! {
    use diesel::sql_types::*;

    users (id) {
        id -> Text,
        name -> Text,
        email -> Text,
        numbers -> Jsonb,
    }
}
