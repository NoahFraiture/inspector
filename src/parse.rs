use chrono::{DateTime, FixedOffset, NaiveDateTime};
use regex::Regex;
use std::fs;
use std::str::Lines;

pub fn parse(filepath: &str) -> Vec<Hand> {
    let mut filecontent = fs::read_to_string(filepath).expect("Invalid file");
    filecontent = filecontent.replace('\r', "");
    filecontent = filecontent.replace('\u{feff}', "");

    let hands_content = split_hands_content(&filecontent);
    let mut hands: Vec<Hand> = vec![];
    for hand in hands_content {
        hands.push(parse_hand(&hand));
    }
    hands
}

fn split_hands_content(content: &str) -> Vec<String> {
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
    if !current_hand.is_empty() {
        hands.push(current_hand);
    }
    hands
}

// Suppose that the table is 6 person
// Can use regex in future
fn parse_hand(hand_txt: &str) -> Hand {
    let mut hand = Hand {
        content: hand_txt.to_string(),
        ..Default::default()
    };
    let mut lines = hand_txt.lines();

    start(&mut hand, &mut lines);
    preflop(&mut hand, &mut lines);
    flop(&mut hand, &mut lines);
    turn(&mut hand, &mut lines);
    river(&mut hand, &mut lines);
    // TODO : showdown and summary
    hand
}

fn start(hand: &mut Hand, lines: &mut Lines) {
    let first_line = lines.next().unwrap();

    // extract id
    let re = Regex::new(r"#(\d+)").unwrap();
    let capture_id = re.captures(first_line).unwrap();
    let mut chars = capture_id[0].chars();
    chars.next();
    hand.id = chars.as_str().parse::<u64>().unwrap();

    // extract date
    let re = Regex::new(r"\[(.*?)\]").unwrap();
    let capture_date = re.captures(first_line).unwrap();
    let date_string = capture_date[0].to_string();
    // TODO : set offset to ET
    let date = NaiveDateTime::parse_from_str(&date_string, "[%Y/%m/%d %H:%M:%S ET]");
    let offset = FixedOffset::east_opt(5 * 3600).unwrap();
    hand.date = DateTime::<FixedOffset>::from_naive_utc_and_offset(date.unwrap(), offset);

    let second_line = lines.next().unwrap();

    // extract table name
    let re = Regex::new(r"'([^']*)'").unwrap();
    let capture_table_name = re.captures(second_line).unwrap();
    let mut chars = capture_table_name[0].chars();
    chars.next(); // remove chars ''
    chars.next_back(); // remove chars ''
    hand.table_name = chars.as_str().to_string();

    // extract button position to latter shift and get actual position of the players
    let re = Regex::new(r"#(\d+)").unwrap();
    let capture_button_position = re.captures(second_line).unwrap();
    let mut chars = capture_button_position[0].chars();
    chars.next();
    hand.button_position = chars.as_str().parse::<u8>().unwrap();

    // extract table size
    let re = Regex::new(r"(\d+)-max").unwrap();
    let capture_table_size = re.captures(second_line).unwrap();
    let mut chars = capture_table_size[0].chars();
    chars.next_back();
    chars.next_back();
    chars.next_back();
    chars.next_back();
    hand.table_size = chars.as_str().parse::<u8>().unwrap();

    // not very optimized but very easy
    for line in lines {
        let mut split = line.split_whitespace();
        if line.contains("posts small blind") {
            hand.small_blind = Blind {
                player: hand.get_player(split.next().unwrap()),
                amount: split
                    .last()
                    .unwrap()
                    .replace('$', "")
                    .parse::<f64>()
                    .unwrap(),
            };
        } else if line.contains("posts big blind") {
            hand.big_blind = Blind {
                player: hand.get_player(split.next().unwrap()),
                amount: split
                    .last()
                    .unwrap()
                    .replace('$', "")
                    .parse::<f64>()
                    .unwrap(),
            };
        } else if line.starts_with("Seat ") {
            split.next(); // "Seat"
            let position = split
                .next()
                .unwrap()
                .replace(':', "")
                .parse::<usize>()
                .unwrap();
            let name = split.next().unwrap().to_string();
            let mut bank = split.next().unwrap().to_string();
            bank = bank.replace(['$', '('], "");
            let player = Player {
                name,
                position: position as u8,
                bank: bank.parse::<f64>().unwrap(),
            };
            hand.players[position - 1] = Some(player);
        } else if line == "*** HOLE CARDS ***" {
            return;
        }
    }
}

fn preflop(hand: &mut Hand, lines: &mut Lines) {
    while let Some(line) = lines.next() {
        println!("preflop");
        let mut split = line.split_whitespace();
        if line.starts_with("Dealt to ") {
            // user cards
            split.next().unwrap();
            split.next().unwrap();
            let username = split.next().unwrap();
            let player = hand.get_player(username);

            hand.players_card[player.position as usize - 1] = Some([String::new(), String::new()]);
            hand.players_card[player.position as usize - 1]
                .as_mut()
                .unwrap()[0] = split.next().unwrap().replace('[', "");
            hand.players_card[player.position as usize - 1]
                .as_mut()
                .unwrap()[1] = split.next().unwrap().replace(']', "");
        } else if Action::is_action(line) {
            // actions
            hand.preflop
                .push(Action::get_action(hand, split.collect::<Vec<&str>>()));
        } else if line.starts_with("Uncalled bet") {
            hand.end = End::extract_end(hand, line, lines.next().unwrap());
        } else if line.starts_with("*** FLOP ***") {
            let re = Regex::new(r"\[(.. .. ..)\]").unwrap();
            let capture_card = re.captures(line).unwrap();
            let mut chars = capture_card[0].chars();
            chars.next(); // remove chars [
            chars.next_back(); // remove chars ]
            let c = chars.as_str().to_string();
            let cards = c.split_whitespace();
            let mut hand_cards: [String; 3] = [String::new(), String::new(), String::new()];
            for (i, card) in cards.enumerate() {
                hand_cards[i] = card.to_string();
            }
            hand.flop_card = Some(hand_cards);
            return;
        }
    }
}

// TODO : refactor to avoid duplicate
fn flop(hand: &mut Hand, lines: &mut Lines) {
    while let Some(line) = lines.next() {
        println!("flop");
        if Action::is_action(line) {
            println!("line={}", line);
            hand.flop
                .push(Action::get_action(hand, line.split_whitespace().collect()));
        } else if line.starts_with("Uncalled bet") {
            hand.end = End::extract_end(hand, line, lines.next().unwrap());
        } else if line.starts_with("*** TURN ***") {
            let re = Regex::new(r"\[(..)\]").unwrap();
            let capture_card = re.captures(line).unwrap();
            let mut chars = capture_card[0].chars();
            chars.next(); // remove chars [
            chars.next_back(); // remove chars ]
            hand.turn_card = Some(chars.as_str().to_string());
            return;
        }
    }
}

fn turn(hand: &mut Hand, lines: &mut Lines) {
    while let Some(line) = lines.next() {
        println!("turn");
        if Action::is_action(line) {
            hand.turn
                .push(Action::get_action(hand, line.split_whitespace().collect()));
        } else if line.starts_with("Uncalled bet") {
            hand.end = End::extract_end(hand, line, lines.next().unwrap());
        } else if line.starts_with("*** RIVER ***") {
            let re = Regex::new(r"\[(..)\]").unwrap();
            let capture_card = re.captures(line).unwrap();
            let mut chars = capture_card[1].chars();
            chars.next(); // remove chars [
            chars.next_back(); // remove chars ]
            hand.river_card = Some(chars.as_str().to_string());
            return;
        }
    }
}

fn river(hand: &mut Hand, lines: &mut Lines) {
    while let Some(line) = lines.next() {
        println!("river");
        if Action::is_action(line) {
            hand.river
                .push(Action::get_action(hand, line.split_whitespace().collect()));
        } else if line.starts_with("Uncalled bet") {
            hand.end = End::extract_end(hand, line, lines.next().unwrap());
        } else if line.starts_with("*** SHOW DOWN ***") {
            return;
        }
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct Hand {
    pub content: String,
    pub id: u64, // u32 is too small
    pub date: DateTime<FixedOffset>,
    pub table_name: String,
    pub table_size: u8,
    pub button_position: u8, // usefull to shift position and guess real position
    pub players: [Option<Player>; 9],
    pub small_blind: Blind,
    pub big_blind: Blind,
    end: End,
    pub players_card: [Option<[String; 2]>; 9],
    preflop: Vec<Action>,
    flop: Vec<Action>,
    turn: Vec<Action>,
    river: Vec<Action>,
    pub flop_card: Option<[String; 3]>,
    pub turn_card: Option<String>,
    pub river_card: Option<String>,
}

impl Hand {
    fn get_player(&self, name: &str) -> Player {
        let trimed_name = name.trim_end_matches(':');
        for player in &self.players {
            match player {
                Some(player) => {
                    if player.name == trimed_name {
                        return player.clone();
                    }
                }
                None => continue,
            }
        }
        panic!("player not found")
    }
}

#[derive(Default, Debug, PartialEq)]
struct End {
    pot: f64,
    winner: Player,
    uncalled_bet: f64, // value of the bet where every one fold. WARN: consider remove
}

impl End {
    fn extract_end(hand: &Hand, first_line: &str, second_line: &str) -> Self {
        let mut first_split = first_line.split_whitespace();
        let mut second_split = second_line.split_whitespace();
        first_split.next(); // "Uncalled"
        first_split.next(); // "bet"
        let uncalled_bet = first_split
            .next()
            .unwrap()
            .replace(['$', '(', ')'], "")
            .parse::<f64>()
            .unwrap();
        let winner = hand.get_player(second_split.next().unwrap());
        second_split.next(); // "collected"
        let pot = second_split
            .next()
            .unwrap()
            .replace('$', "")
            .parse::<f64>()
            .unwrap();
        End {
            pot,
            winner,
            uncalled_bet,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Action {
    Call(Player, f64),
    Bet(Player, f64),
    Raise(Player, f64, f64), // raise from .. to ..
    Check(Player),
    Fold(Player),
    Leave(Player),
}

impl Action {
    fn is_action(line: &str) -> bool {
        line.contains("calls")
            || line.contains("bets")
            || line.contains("raises")
            || line.contains("check")
            || line.contains("folds")
            || line.contains("leaves")
    }

    fn get_action(hand: &Hand, line: Vec<&str>) -> Self {
        match line[1] {
            "calls" => Action::Call(
                hand.get_player(&line[0].replace(':', "")),
                line[2].replace('$', "").parse::<f64>().unwrap(),
            ),
            "bets" => Action::Bet(
                hand.get_player(&line[0].replace(':', "")),
                line[2].replace('$', "").parse::<f64>().unwrap(),
            ),
            "raises" => Action::Raise(
                hand.get_player(&line[0].replace(':', "")),
                line[2].replace('$', "").parse::<f64>().unwrap(),
                line[4].replace('$', "").parse::<f64>().unwrap(),
            ),
            "checks" => Action::Check(hand.get_player(&line[0].replace(':', ""))),
            "folds" => Action::Fold(hand.get_player(&line[0].replace(':', ""))),
            "leaves" => Action::Leave(hand.get_player(&line[0].replace(':', ""))),
            _ => panic!("Unknow action"),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Player {
    pub name: String,
    position: u8,
    bank: f64,
}

#[derive(Default, Debug, PartialEq)]
pub struct Blind {
    player: Player,
    amount: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parsing() {
        let filepath = "test/test_hands.txt";
        let mut filecontent = fs::read_to_string(filepath).expect("Invalid file");
        filecontent = filecontent.replace('\r', "");
        filecontent = filecontent.replace('\u{feff}', "");
        let hands_content = split_hands_content(&filecontent);
        let actual_hand = parse_hand(&hands_content[0]);

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
        let expected_hand = Hand {
            flop_card: Some(["Qh".to_string(), "9s".to_string(), "3d".to_string()]),
            turn_card: Some("6s".to_string()),
            river_card: None,
            content: hands_content[0].clone(),
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
                uncalled_bet: 0.18,
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
                Action::Call(players[4].clone().unwrap(), 0.02),
                Action::Fold(players[5].clone().unwrap()),
                Action::Fold(players[0].clone().unwrap()),
                Action::Fold(players[1].clone().unwrap()),
                Action::Call(players[2].clone().unwrap(), 0.01),
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
                Action::Bet(players[4].clone().unwrap(), 0.18),
                Action::Fold(players[2].clone().unwrap()),
                Action::Fold(players[3].clone().unwrap()),
            ],
            river: vec![],
        };

        assert_eq!(actual_hand, expected_hand);
    }
}
