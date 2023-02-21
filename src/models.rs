use diesel::prelude::*;
use crate::schema::weighttraining;

#[derive(Queryable, Debug)]
pub struct Set {
    pub id: i32,
    pub lift: String,
    pub weight: i32,
    pub reps: i32,
    pub rpe: i32,
    pub notes: String,
}

#[derive(Insertable)]
#[diesel(table_name = weighttraining)]
pub struct LogSet<'a>{
    pub lift: &'a str,
    pub weight: &'a i32,
    pub reps: &'a i32,
    pub rpe: &'a i32,
    pub notes: &'a str,
}