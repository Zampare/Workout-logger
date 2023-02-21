extern crate diesel;
mod ops;
mod db;
mod schema;
mod models;

use ops::lifts_ops::log_lift;
use db::establish_connection; 


#[get("/")]
async fn write_to_database() -> impl Responder{
    let conn  = establish_connection();
    
    log_lift(&mut conn, set);
}