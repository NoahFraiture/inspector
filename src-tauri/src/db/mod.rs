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

pub fn insert_actions(
  conn: &mut SqliteConnection,
  actions: &Vec<models::Action>,
) -> Result<(), DBError> {
  for action in actions {
    diesel::insert_into(schema::action::table)
      .values(action)
      .returning(models::Action::as_returning())
      .get_result(conn)
      .map_err(|e| DBError::err(DBErrorType::Insert, e))?;
  }
  Ok(())
}

pub fn insert_blind(
  conn: &mut SqliteConnection,
  blind: &models::Blind,
) -> Result<models::Blind, DBError> {
  diesel::insert_into(schema::blind::table)
    .values(blind)
    .returning(models::Blind::as_returning())
    .get_result(conn)
    .map_err(|e| DBError::err(DBErrorType::Insert, e))
}

pub fn insert_hole_cards(
  conn: &mut SqliteConnection,
  hole_cards: &Vec<models::HoleCard>,
) -> Result<(), DBError> {
  for hole_card in hole_cards {
    diesel::insert_into(schema::holeCard::table)
      .values(hole_card)
      .returning(models::HoleCard::as_returning())
      .get_result(conn)
      .map_err(|e| DBError::err(DBErrorType::Insert, e))?;
  }
  Ok(())
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

pub fn get_players(
  conn: &mut SqliteConnection,
  names: Vec<&str>,
) -> Result<Vec<models::Player>, DBError> {
  use crate::db::models::Player;
  use crate::db::schema::player::dsl::{name, player};

  let result = player
    .filter(name.eq_any(names))
    .select(Player::as_select())
    .load(conn)
    .map_err(|e| DBError::err(DBErrorType::Select, e))?;

  Ok(result)
}
