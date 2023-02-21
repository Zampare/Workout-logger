// @generated automatically by Diesel CLI.

diesel::table! {
    weighttraining (id) {
        id -> Int4,
        lift -> Varchar,
        weight -> Int4,
        reps -> Int4,
        rpe -> Nullable<Int4>,
        notes -> Nullable<Text>,
    }
}
