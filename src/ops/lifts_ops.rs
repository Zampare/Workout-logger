use crate::models::{Set, LogSet};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use axum::prelude::*;
use std::net::SocketAddr;

pub fn log_lift(conn: &mut PgConnection, set: LogSet){
    use crate::schema::weighttraining::dsl::*;

    let new_set = LogSet{
        lift: &set.lift,
        weight: &set.weight,
        reps: &set.reps,
        rpe: &set.rpe,
        notes: &set.notes,
    };

    diesel::insert_into(weighttraining)
        .values(&new_set)
        .execute(conn)
        .expect("Error logging new set");
}