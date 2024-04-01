use super::parse::Hand;
use super::parse::Player;
use rusqlite::{Connection, Result};

fn get_name(player: &Option<Player>) -> String {
    if let Some(player) = &player {
        player.name.clone()
    } else {
        String::from("NULL")
    }
}

fn get_card(card: &Option<String>) -> String {
    if let Some(card) = &card {
        String::from(card)
    } else {
        String::from("NULL")
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
            String::from("NULL"),
            String::from("NULL"),
            String::from("NULL"),
        ]
    }
}

pub struct HandDB {
    connection: Connection,
}

impl HandDB {
    pub fn new() -> Result<Self> {
        let connection = Connection::open("data/hand.db")?;
        Result::Ok(HandDB { connection })
    }

    pub fn insert(&self, hand: &Hand) {
        let hand_table_query = format!(
            "INSERT INTO Hand VALUES ({}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {})",
            hand.id,
            hand.date.timestamp(),
            hand.table_name,
            hand.table_size,
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
    }
}
