pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

#[derive(Debug)]
pub enum DBErrorType {
  Connection,
  Insert,
  Select,
}

#[derive(Debug)]
pub struct DBError {
  t: DBErrorType,
  msg: String,
}

impl DBError {
  pub fn err(t: DBErrorType, e: impl std::string::ToString) -> Self {
    DBError {
      t,
      msg: format!("Error : {}", e.to_string()),
    }
  }
}

pub fn establish_connection() -> Result<SqliteConnection, DBError> {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  SqliteConnection::establish(&database_url).map_err(|e| DBError::err(DBErrorType::Connection, e))
}

pub fn insert_hand(
  conn: &mut SqliteConnection,
  hand: &models::Hand,
) -> Result<models::Hand, DBError> {
  diesel::insert_into(schema::hand::table)
    .values(hand)
    .returning(models::Hand::as_returning())
    .get_result(conn)
    .map_err(|e| DBError::err(DBErrorType::Insert, e))
}

pub fn show_hands(conn: &mut SqliteConnection) -> Result<(), DBError> {
  use crate::db::models::Hand;
  use crate::db::schema::hand::dsl::*;
  let result = hand
    .filter(time.gt(1))
    .select(Hand::as_select())
    .load(conn)
    .map_err(|e| DBError::err(DBErrorType::Select, e))?;

  for h in result {
    println!("h : {:#?}", h);
  }
  Ok(())
}
