#![allow(dead_code)]
use super::parse::{Action, Hand, Player};
use rusqlite::{Connection, Result};

fn get_name(player: &Option<Player>) -> String {
    if let Some(player) = &player {
        player.name.clone()
    } else {
        String::from("null")
    }
}

fn get_card(card: &Option<String>) -> String {
    if let Some(card) = &card {
        String::from(card)
    } else {
        String::from("null")
    }
}

fn get_card_flop(cards: &Option<[String; 3]>) -> [String; 3] {
    if let Some(cards) = &cards {
        [
            String::from(&cards[0]),
            String::from(&cards[1]),
            String::from(&cards[2]),
        ]
    } else {
        [
            String::from("null"),
            String::from("null"),
            String::from("null"),
        ]
    }
}

fn generate_action_query(action: &Action, moment: &str, i: u32, id: u64) -> String {
    let mut kind = String::new();
    let mut amount1 = 0.;
    let mut amount2 = 0.;
    let mut is_allin = false;
    let mut player_name = String::new();
    match action {
        Action::Call(player, amount, allin) => {
            kind.push_str("call");
            amount1 = *amount;
            is_allin = *allin;
            player_name.push_str(&player.name);
        }
        Action::Bet(player, amount, allin) => {
            kind.push_str("bet");
            amount1 = *amount;
            is_allin = *allin;
            player_name.push_str(&player.name);
        }
        Action::Raise(player, a1, a2, allin) => {
            kind.push_str("raise");
            amount1 = *a1;
            amount2 = *a2;
            is_allin = *allin;
            player_name.push_str(&player.name);
        }
        Action::Check(player) => {
            kind.push_str("check");
            player_name.push_str(&player.name);
        }
        Action::Fold(player) => {
            kind.push_str("fold");
            player_name.push_str(&player.name);
        }
        Action::Leave(player) => {
            kind.push_str("leave");
            player_name.push_str(&player.name);
        }
        Action::UncalledBet(player, amount) => {
            kind.push_str("uncalledbet");
            player_name.push_str(&player.name);
        }
    }
    format!(
        "INSERT INTO Blind VALUES ({}, {}, {}, {}, {}, {}, {}, {})",
        player_name, id, kind, moment, i, amount1, amount2, is_allin
    )
}

pub struct HandDB {
    connection: Connection,
}

impl HandDB {
    pub fn new() -> Result<Self> {
        let connection = Connection::open("data/hands.db")?;
        Result::Ok(HandDB { connection })
    }

    pub fn insert(&self, hand: &Hand) -> Result<()> {
        let hand_query = format!(
            "INSERT INTO Hand VALUES ({}, {}, \"{}\", {}, \"{}\", {}, \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\")",
            hand.id,
            hand.date.timestamp(),
            hand.table_name,
            hand.table_size,
            hand.end.winner.name,
            hand.end.pot,
            get_name(&hand.players[0]),
            get_name(&hand.players[1]),
            get_name(&hand.players[2]),
            get_name(&hand.players[3]),
            get_name(&hand.players[4]),
            get_name(&hand.players[5]),
            get_name(&hand.players[6]),
            get_name(&hand.players[7]),
            get_name(&hand.players[8]),
            get_card_flop(&hand.flop_card)[0],
            get_card_flop(&hand.flop_card)[1],
            get_card_flop(&hand.flop_card)[2],
            get_card(&hand.turn_card),
            get_card(&hand.river_card),
        );
        let result = self.connection.execute(&hand_query, ());
        match result {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        let small_blind_query = format!(
            "INSERT INTO Blind VALUES ({}, {}, {}, {})",
            hand.small_blind.player.name, hand.id, hand.small_blind.amount, "small",
        );
        let result = self.connection.execute(&small_blind_query, ());
        match result {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        let big_blind_query = format!(
            "INSERT INTO Blind VALUES ({}, {}, {}, {})",
            hand.big_blind.player.name, hand.id, hand.big_blind.amount, "big",
        );
        let result = self.connection.execute(&big_blind_query, ());
        match result {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        // WARN: may restart player
        for i in 0..9 {
            if let Some(player) = &hand.players[i] {
                // Player table
                let player_query = format!("INSERT INTO Player Values ({})", player.name);
                match self.connection.execute(&player_query, ()) {
                    Ok(_) => {}
                    Err(e) => return Err(e),
                }

                // HoldCard table
                if let Some(cards) = &hand.players_card[i] {
                    // HoldCard table
                    let hole_card_query = format!(
                        "INSERT INTO HoleCard Values ({}, {}, {}, {})",
                        hand.id, player.name, cards[0], cards[1]
                    );
                    match self.connection.execute(&hole_card_query, ()) {
                        Ok(_) => {}
                        Err(e) => return Err(e),
                    }
                }

                // TODO: next tables
            }
        }

        for (i, action) in hand.preflop.iter().enumerate() {
            let query = generate_action_query(action, "preflop", 0, hand.id);
        }
        for (i, action) in hand.flop.iter().enumerate() {
            let query = generate_action_query(action, "flop", 0, hand.id);
        }
        for (i, action) in hand.turn.iter().enumerate() {
            let query = generate_action_query(action, "turn", 0, hand.id);
        }
        for (i, action) in hand.river.iter().enumerate() {
            let query = generate_action_query(action, "river", 0, hand.id);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::parse::*;
    use chrono::{DateTime, FixedOffset, NaiveDateTime};
    use pretty_assertions::assert_eq;

    fn init_hand_real_fold() -> Hand {
        let players = [
            Some(Player {
                name: "sidneivl".to_string(),
                position: 1,
                bank: 3.24,
            }),
            Some(Player {
                name: "Savva08".to_string(),
                position: 2,
                bank: 1.96,
            }),
            Some(Player {
                name: "captelie52".to_string(),
                position: 3,
                bank: 0.70,
            }),
            Some(Player {
                name: "PokerZhyte".to_string(),
                position: 4,
                bank: 2.,
            }),
            Some(Player {
                name: "alencarbrasil19".to_string(),
                position: 5,
                bank: 1.59,
            }),
            Some(Player {
                name: "Cazunga".to_string(),
                position: 6,
                bank: 2.,
            }),
            None,
            None,
            None,
        ];
        let naive_date =
            NaiveDateTime::parse_from_str("[2024/03/26 17:02:04 ET]", "[%Y/%m/%d %H:%M:%S ET]");
        let offset = FixedOffset::east_opt(5 * 3600).unwrap();
        let date = DateTime::<FixedOffset>::from_naive_utc_and_offset(naive_date.unwrap(), offset);
        Hand {
            small_limit: 0.01,
            big_limit: 0.02,
            flop_card: Some(["Qh".to_string(), "9s".to_string(), "3d".to_string()]),
            turn_card: Some("6s".to_string()),
            river_card: None,
            // NOTE : since it's not used, set to empty is easier
            content: String::new(),
            id: 249638850870,
            date,
            table_name: "Ostara III".to_string(),
            table_size: 6,
            button_position: 2,
            players: players.clone(),
            small_blind: Blind {
                player: players[2].clone().unwrap(),
                amount: 0.01,
            },
            big_blind: Blind {
                player: players[3].clone().unwrap(),
                amount: 0.02,
            },
            end: End {
                pot: 0.06,
                winner: players[4].clone().unwrap(),
            },
            players_card: [
                None,
                None,
                None,
                Some(["2c".to_string(), "7d".to_string()]),
                None,
                None,
                None,
                None,
                None,
            ],
            preflop: vec![
                Action::Call(players[4].clone().unwrap(), 0.02, false),
                Action::Fold(players[5].clone().unwrap()),
                Action::Fold(players[0].clone().unwrap()),
                Action::Fold(players[1].clone().unwrap()),
                Action::Call(players[2].clone().unwrap(), 0.01, false),
                Action::Check(players[3].clone().unwrap()),
            ],
            flop: vec![
                Action::Check(players[2].clone().unwrap()),
                Action::Check(players[3].clone().unwrap()),
                Action::Check(players[4].clone().unwrap()),
            ],
            turn: vec![
                Action::Check(players[2].clone().unwrap()),
                Action::Check(players[3].clone().unwrap()),
                Action::Bet(players[4].clone().unwrap(), 0.18, false),
                Action::Fold(players[2].clone().unwrap()),
                Action::Fold(players[3].clone().unwrap()),
                Action::UncalledBet(players[4].clone().unwrap(), 0.18),
            ],
            river: vec![],
        }
    }

    #[test]
    fn test_insert_real_fold() {}
}
