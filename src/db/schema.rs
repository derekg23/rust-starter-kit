use diesel::table;

table! {
  users (id) {
    id -> Int4,
    name -> Text,
    email -> Text,
    created_at -> Timestamp,
    updated_at -> Timestamp,
  }
}

