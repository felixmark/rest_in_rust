// @generated automatically by Diesel CLI.

diesel::table! {
    notes (id) {
        id -> Uuid,
        timestamp -> Timestamp,
        content -> Text,
    }
}
