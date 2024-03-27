#![allow(unused_imports)]
#![allow(dead_code)]
use chrono::DateTime;
use std::env;
use std::fs;
use std::io::{self, stdin, BufReader, BufWriter, Stdout};

pub fn parse() {
    let filepath = "/home/noah/Games/poker_logs/PokerZhyte/HH20240326 Cornelia III - $0.01-$0.02 - USD No Limit Hold'em.txt";
    let filecontent = fs::read_to_string(filepath).expect("Invalid file");

    let hands = get_hands(&filecontent);
    for hand in hands {
        println!("{}", &hand[0..50]);
    }
}

fn get_hands(content: &str) -> Vec<String> {
    let mut current_hand = String::new();
    let mut hands = Vec::new();
    for line in content.lines() {
        if line.starts_with("PokerStars Hand") && !current_hand.is_empty() {
            hands.push(current_hand);
            current_hand = String::new();
        }

        current_hand.push_str(line);
        current_hand.push('\n');
    }
    hands
}

// Suppose that the table is 6 person
// Can use regex in future
fn parse_hand<'a>(hand_txt: &'a str, username: &'a str) -> Hand<'a> {
    let mut hand = Hand::<'a> {
        content: hand_txt,
        username,
        ..Default::default()
    };
    let mut lines = hand_txt.lines();
    let mut date_str = lines.next().unwrap().split('[');
    let day_str = date_str.next().unwrap();
    let hour_str = date_str.next().unwrap();
    let mut date_string = String::new();
    date_string.push_str(day_str);
    date_string.push_str(hour_str);

    hand.date = DateTime::parse_from_str(&date_string, "%Y-%m-%d %H:%M:%S").unwrap(); // ET hour

    for line in lines {
        let mut split = line.split_whitespace();
        if line.contains("posts small blind") {
            hand.small_blind = Blind {
                player: Player::new(split.next().unwrap()),
                amount: split
                    .last()
                    .unwrap()
                    .replace('$', "")
                    .parse::<f64>()
                    .unwrap(),
            };
        } else if line.contains("posts big blind") {
            hand.big_blind = Blind {
                player: Player::new(split.next().unwrap()),
                amount: split
                    .last()
                    .unwrap()
                    .replace('$', "")
                    .parse::<f64>()
                    .unwrap(),
            };
        } else if line.starts_with("Seat ") {
            let position = split
                .next()
                .unwrap()
                .replace(':', "")
                .parse::<usize>()
                .unwrap();
            let player = Player::new(split.next().unwrap());
            if player.name == username {
                hand.player_position = position as u8 - 1;
            }
            hand.players[position - 1] = player;
        } else if line.starts_with("Dealt to ") {
            split.next().unwrap();
            split.next().unwrap();
            split.next().unwrap();
            hand.players_card[hand.player_position as usize][0] =
                split.next().unwrap().replace('[', "");
            hand.players_card[hand.player_position as usize][1] =
                split.next().unwrap().replace(']', "");
        }
    }
    hand
}

#[derive(Default)]
struct Hand<'a> {
    content: &'a str,
    table: &'a str,
    username: &'a str,
    date: DateTime<chrono::FixedOffset>,
    player_position: u8,
    players: [Player<'a>; 6],
    players_card: [[String; 2]; 6],
    small_blind: Blind<'a>,
    big_blind: Blind<'a>,
    preflop: Vec<Action<'a>>,
    flop: Vec<Action<'a>>,
    turn: Vec<Action<'a>>,
    river: Vec<Action<'a>>,
    total_pot: f64,
    rake: f64,
    winner: Win,
    show: Vec<Player<'a>>,
}

enum Action<'a> {
    Fold(&'a Player<'a>),
    Call(&'a Player<'a>, f64),
    Bet(&'a Player<'a>, f64),
    Raise(&'a Player<'a>, f64, f64), // raise from .. to ..
}

#[derive(Default)]
struct Player<'a> {
    name: &'a str,
}

impl<'a> Player<'a> {
    fn new(name: &'a str) -> Self {
        Self { name }
    }
}

#[derive(Default)]
struct Blind<'a> {
    player: Player<'a>,
    amount: f64,
}

#[derive(Default)]
struct Win {
    winner: String,
    amount: f64,
}
