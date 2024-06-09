use crate::db::schema;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::action)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Action {
  pub id: i32,
  pub player: String,
  pub hand: i64,
  pub kind: String,
  pub moment: String,
  pub sequence: i32,
  pub amount1: f32,
  pub amount2: f32,
  pub allin: bool,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::blind)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Blind {
  pub id: i32,
  pub player: String,
  pub hand: i64,
  pub amount: f32,
  pub kind: String,
}

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = schema::hand)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Hand {
  pub id: i64,
  pub content: String,
  pub real_money: bool,
  pub time: i64,
  pub table_name: String,
  pub table_size: i32,
  pub winner: String,
  pub pot: f32,
  pub player1: String,
  pub player2: String,
  pub player3: String,
  pub player4: String,
  pub player5: String,
  pub player6: String,
  pub player7: String,
  pub player8: String,
  pub player9: String,
  pub card1: String,
  pub card2: String,
  pub card3: String,
  pub card4: String,
  pub card5: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::holeCard)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct HoldCard {
  pub id: i32,
  pub hand: i64,
  pub player: String,
  pub card1: String,
  pub card2: String,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = schema::player)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Player {
  pub name: String,
  pub real_money: bool,
  pub vpip: f32,
  pub pfr: f32,
  pub af: f32,
  pub pre_3bet: f32,
  pub fold_pre_3bet: f32,
  pub cbet: f32,
  pub fold_cbet: f32,
  pub squeeze: f32,
  pub nb_hand: f32,
  pub nb_can_pre_3bet: f32,
  pub nb_can_fold_pre_3bet: f32,
  pub nb_can_cbet: f32,
  pub nb_can_fold_cbet: f32,
  pub nb_can_squeeze: f32,
  pub nb_call: f32,
  pub nb_bet: f32,
  pub nb_raise: f32,
}

impl Player {
  pub fn new(name: &str) -> Self {
    Player {
      name: String::from(name),
      real_money: false,
      vpip: 0.0,
      pfr: 0.0,
      af: 0.0,
      nb_hand: 0.0,
      nb_call: 0.0,
      nb_bet: 0.0,
      nb_raise: 0.0,
      pre_3bet: 0.0,
      fold_pre_3bet: 0.0,
      cbet: 0.0,
      fold_cbet: 0.0,
      squeeze: 0.0,
      nb_can_pre_3bet: 0.0,
      nb_can_fold_pre_3bet: 0.0,
      nb_can_cbet: 0.0,
      nb_can_fold_cbet: 0.0,
      nb_can_squeeze: 0.0,
    }
  }
}
