use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::action)]
// #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Action {
  pub id: i64,
  pub player: String,
  pub hand: i64,
  pub kind: String,
  pub moment: String,
  pub sequence: i64,
  pub amount1: i64,
  pub amount2: i64,
  pub allin: bool,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::blind)]
// #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Blind {
  pub id: i64,
  pub player: String,
  pub hand: i64,
  pub amount: i64,
  pub kind: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::hand)]
// #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Hand {
  pub id: i64,
  pub time: i64,
  pub table_name: String,
  pub table_size: i64,
  pub winner: String,
  pub pot: i64,
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
#[diesel(table_name = crate::schema::holeCard)]
// #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct HoldCard {
  pub id: i64,
  pub hand: i64,
  pub player: String,
  pub card1: String,
  pub card2: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::player)]
// #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Player {
  pub name: String,
  pub vpip: f64,
  pub pfr: f64,
  pub af: f64,
  pub pre_3bet: f64,
  pub fold_pre_3bet: f64,
  pub cbet: f64,
  pub fold_cbet: f64,
  pub squeeze: f64,
  pub nb_hand: f64,
  pub nb_can_pre_3bet: f64,
  pub nb_can_fold_pre_3bet: f64,
  pub nb_can_cbet: f64,
  pub nb_can_fold_cbet: f64,
  pub nb_can_squeeze: f64,
  pub nb_call: f64,
  pub nb_bet: f64,
  pub nb_raise: f64,
}

impl Player {
  pub fn new() -> Self {
    Player {
      name: String::from(name),
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
