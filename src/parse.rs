#![allow(unused_imports)]
#![allow(dead_code)]
use chrono::{DateTime, NaiveDateTime};
use regex::Regex;
use std::env;
use std::fs;
use std::io::{self, stdin, BufReader, BufWriter, Stdout};
use std::str::Lines;

pub fn parse() {
    let filepath = "/home/noah/Games/poker_logs/PokerZhyte/HH20240326 Cornelia III - $0.01-$0.02 - USD No Limit Hold'em.txt";
    let mut filecontent = fs::read_to_string(filepath).expect("Invalid file");
    filecontent = filecontent.replace('\r', "");
    filecontent = filecontent.replace('\u{feff}', "");

    let hands = split_hands_content(&filecontent);
    for hand in hands {
        println!("{}", &hand[0..50]);
    }
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

    // extract date
    let re = Regex::new(r"\[(.*?)\]").unwrap();
    let first_line = lines.next().unwrap();
    let capture = re.captures(first_line).unwrap();
    let date_string = capture[0].to_string();
    // TODO : set offset to ET
    let date = NaiveDateTime::parse_from_str(&date_string, "[%Y/%m/%d %H:%M:%S ET]");
    match date {
        Ok(date) => hand.date = date,
        Err(e) => println!("Error parsing date: {}", e),
    }

    start(&mut hand, &mut lines);
    preflop(&mut hand, &mut lines);
    flop(&mut hand, &mut lines);
    turn(&mut hand, &mut lines);
    river(&mut hand, &mut lines);
    // TODO : showdown and summary
    hand
}

fn start(hand: &mut Hand, lines: &mut Lines) {
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
            let player = Player {
                name: split.next().unwrap().to_string(),
                position: position as u8,
            };
            hand.players[position - 1] = player;
        } else if line == "*** HOLE CARDS ***" {
            return;
        }
    }
}

fn preflop(hand: &mut Hand, lines: &mut Lines) {
    while let Some(line) = lines.next() {
        let mut split = line.split_whitespace();
        if line.starts_with("Dealt to ") {
            // user cards
            split.next().unwrap();
            split.next().unwrap();
            let username = split.next().unwrap();
            let player = hand.get_player(username);

            hand.players_card[player.position as usize - 1][0] =
                split.next().unwrap().replace('[', "");
            hand.players_card[player.position as usize - 1][1] =
                split.next().unwrap().replace(']', "");
        } else if Action::is_action(line) {
            // actions
            hand.preflop
                .push(Action::get_action(hand, split.collect::<Vec<&str>>()));
        } else if line.starts_with("Uncalled bet") {
            hand.end = End::extract_end(hand, line, lines.next().unwrap());
        } else if line.starts_with("*** FLOP ***") {
            // TODO : add flop cards
            return;
        }
    }
}

// TODO : refactor to avoid duplicate
fn flop(hand: &mut Hand, lines: &mut Lines) {
    while let Some(line) = lines.next() {
        if Action::is_action(line) {
            hand.flop
                .push(Action::get_action(hand, line.split_whitespace().collect()));
        } else if line.starts_with("Uncalled bet") {
            hand.end = End::extract_end(hand, line, lines.next().unwrap());
        } else if line.starts_with("*** TURN ***") {
            // TODO : add turn cards
            return;
        }
    }
}

fn turn(hand: &mut Hand, lines: &mut Lines) {
    while let Some(line) = lines.next() {
        if Action::is_action(line) {
            hand.turn
                .push(Action::get_action(hand, line.split_whitespace().collect()));
        } else if line.starts_with("Uncalled bet") {
            hand.end = End::extract_end(hand, line, lines.next().unwrap());
        } else if line.starts_with("*** RIVER ***") {
            // TODO : add river cards
            break;
        }
    }
}

fn river(hand: &mut Hand, lines: &mut Lines) {
    while let Some(line) = lines.next() {
        if Action::is_action(line) {
            hand.river
                .push(Action::get_action(hand, line.split_whitespace().collect()));
        } else if line.starts_with("Uncalled bet") {
            hand.end = End::extract_end(hand, line, lines.next().unwrap());
        } else if line.starts_with("*** SHOW DOWN ***") {
            // TODO : add river cards
            return;
        }
    }
}

#[derive(Default, Debug, PartialEq)]
struct Hand {
    content: String,
    table: String, // TODO: empty
    date: NaiveDateTime,
    players: [Player; 6],
    players_card: [[String; 2]; 6],
    small_blind: Blind,
    big_blind: Blind,
    preflop: Vec<Action>,
    flop: Vec<Action>,
    turn: Vec<Action>,
    river: Vec<Action>,
    end: End,
}

impl Hand {
    fn get_player(&self, name: &str) -> Player {
        let trimed_name = name.trim_end_matches(':');
        for player in &self.players {
            if player.name == trimed_name {
                return player.clone();
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
struct Player {
    name: String,
    position: u8,
}

#[derive(Default, Debug, PartialEq)]
struct Blind {
    player: Player,
    amount: f64,
}

#[derive(Default)]
struct Win {
    winner: Player,
    amount: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_start() {
        let mut input = "Table 'Ostara III' 6-max Seat #2 is the button
Seat 1: sidneivl ($3.24 in chips) 
Seat 2: Savva08 ($1.96 in chips) 
Seat 3: captelie52 ($0.70 in chips) 
Seat 4: PokerZhyte ($2 in chips) 
Seat 5: alencarbrasil19 ($1.59 in chips) 
Seat 6: Cazunga ($2 in chips) 
captelie52: posts small blind $0.01
PokerZhyte: posts big blind $0.02"
            .lines();
        let mut actual_hand = Hand::default();
        start(&mut actual_hand, &mut input);
        let expected_hand = Hand {
            small_blind: Blind {
                player: Player {
                    name: "captelie52".to_string(),
                    position: 3,
                },
                amount: 0.01,
            },
            big_blind: Blind {
                player: Player {
                    name: "PokerZhyte".to_string(),
                    position: 4,
                },
                amount: 0.02,
            },
            players: [
                Player {
                    name: "sidneivl".to_string(),
                    position: 1,
                },
                Player {
                    name: "Savva08".to_string(),
                    position: 2,
                },
                Player {
                    name: "captelie52".to_string(),
                    position: 3,
                },
                Player {
                    name: "PokerZhyte".to_string(),
                    position: 4,
                },
                Player {
                    name: "alencarbrasil19".to_string(),
                    position: 5,
                },
                Player {
                    name: "Cazunga".to_string(),
                    position: 6,
                },
            ],
            ..Default::default()
        };
        assert_eq!(actual_hand, expected_hand);
    }

    #[test]
    fn test_preflop() {
        // TODO : add uncalled bet
        let mut input = "Dealt to PokerZhyte [2c 7d]
alencarbrasil19: calls $0.02
Cazunga: folds 
sidneivl: folds 
Savva08: folds 
captelie52: calls $0.01
PokerZhyte: checks"
            .lines();
        let mut actual_hand = Hand {
            small_blind: Blind {
                player: Player {
                    name: "captelie52".to_string(),
                    position: 3,
                },
                amount: 0.01,
            },
            big_blind: Blind {
                player: Player {
                    name: "PokerZhyte".to_string(),
                    position: 4,
                },
                amount: 0.02,
            },
            players: [
                Player {
                    name: "sidneivl".to_string(),
                    position: 1,
                },
                Player {
                    name: "Savva08".to_string(),
                    position: 2,
                },
                Player {
                    name: "captelie52".to_string(),
                    position: 3,
                },
                Player {
                    name: "PokerZhyte".to_string(),
                    position: 4,
                },
                Player {
                    name: "alencarbrasil19".to_string(),
                    position: 5,
                },
                Player {
                    name: "Cazunga".to_string(),
                    position: 6,
                },
            ],
            ..Default::default()
        };
        preflop(&mut actual_hand, &mut input);
        let players = [
            Player {
                name: "sidneivl".to_string(),
                position: 1,
            },
            Player {
                name: "Savva08".to_string(),
                position: 2,
            },
            Player {
                name: "captelie52".to_string(),
                position: 3,
            },
            Player {
                name: "PokerZhyte".to_string(),
                position: 4,
            },
            Player {
                name: "alencarbrasil19".to_string(),
                position: 5,
            },
            Player {
                name: "Cazunga".to_string(),
                position: 6,
            },
        ];
        let expected_hand = Hand {
            small_blind: Blind {
                player: Player {
                    name: "captelie52".to_string(),
                    position: 3,
                },
                amount: 0.01,
            },
            big_blind: Blind {
                player: Player {
                    name: "PokerZhyte".to_string(),
                    position: 4,
                },
                amount: 0.02,
            },
            players: players.clone(),
            players_card: [
                [String::new(), String::new()],
                [String::new(), String::new()],
                [String::new(), String::new()],
                [String::from("2c"), String::from("7d")],
                [String::new(), String::new()],
                [String::new(), String::new()],
            ],
            preflop: vec![
                Action::Call(players[4].clone(), 0.02),
                Action::Fold(players[5].clone()),
                Action::Fold(players[0].clone()),
                Action::Fold(players[1].clone()),
                Action::Call(players[2].clone(), 0.01),
                Action::Check(players[3].clone()),
            ],
            ..Default::default()
        };

        assert_eq!(actual_hand, expected_hand);
    }

    #[test]
    fn test_flop() {
        let mut input = "Savva08: bets $0.07
captelie52: folds 
Uncalled bet ($0.07) returned to Savva08
Savva08 collected $0.14 from pot
Savva08: doesn't show hand"
            .lines();
        let mut actual_hand = Hand {
            small_blind: Blind {
                player: Player {
                    name: String::from("PokerZhyte"),
                    position: 4,
                },
                amount: 0.1,
            },
            big_blind: Blind {
                player: Player {
                    name: String::from("alencarbrasil19"),
                    position: 5,
                },
                amount: 0.2,
            },
            players: [
                Player {
                    name: String::from("sidneivl"),
                    position: 1,
                },
                Player {
                    name: String::from("Savva08"),
                    position: 2,
                },
                Player {
                    name: String::from("captelie52"),
                    position: 3,
                },
                Player {
                    name: String::from("PokerZhyte"),
                    position: 4,
                },
                Player {
                    name: String::from("alencarbrasil19"),
                    position: 5,
                },
                Player {
                    name: String::from("Cazunga"),
                    position: 6,
                },
            ],
            ..Default::default()
        };
        flop(&mut actual_hand, &mut input);
        let expected_hand = Hand {
            small_blind: Blind {
                player: Player {
                    name: String::from("PokerZhyte"),
                    position: 4,
                },
                amount: 0.1,
            },
            big_blind: Blind {
                player: Player {
                    name: String::from("alencarbrasil19"),
                    position: 5,
                },
                amount: 0.2,
            },
            players: [
                Player {
                    name: "sidneivl".to_string(),
                    position: 1,
                },
                Player {
                    name: "Savva08".to_string(),
                    position: 2,
                },
                Player {
                    name: "captelie52".to_string(),
                    position: 3,
                },
                Player {
                    name: "PokerZhyte".to_string(),
                    position: 4,
                },
                Player {
                    name: "alencarbrasil19".to_string(),
                    position: 5,
                },
                Player {
                    name: "Cazunga".to_string(),
                    position: 6,
                },
            ],
            flop: vec![
                Action::Bet(
                    Player {
                        name: "Savva08".to_string(),
                        position: 2,
                    },
                    0.07,
                ),
                Action::Fold(Player {
                    name: "captelie52".to_string(),
                    position: 3,
                }),
            ],
            end: End {
                winner: Player {
                    name: "Savva08".to_string(),
                    position: 2,
                },
                pot: 0.14,
                uncalled_bet: 0.07,
            },
            ..Default::default()
        };

        assert_eq!(actual_hand, expected_hand);
    }

    #[test]
    fn test_parsing() {
        let filepath = "test_hands.txt";
        let mut filecontent = fs::read_to_string(filepath).expect("Invalid file");
        filecontent = filecontent.replace('\r', "");
        filecontent = filecontent.replace('\u{feff}', "");
        let hands_content = split_hands_content(&filecontent);
        let actual_hand = parse_hand(&hands_content[0]);

        let players = [
            Player {
                name: "sidneivl".to_string(),
                position: 1,
            },
            Player {
                name: "Savva08".to_string(),
                position: 2,
            },
            Player {
                name: "captelie52".to_string(),
                position: 3,
            },
            Player {
                name: "PokerZhyte".to_string(),
                position: 4,
            },
            Player {
                name: "alencarbrasil19".to_string(),
                position: 5,
            },
            Player {
                name: "Cazunga".to_string(),
                position: 6,
            },
        ];
        let expected_hand = Hand {
            table: "".to_string(),
            content: hands_content[0].clone(),
            small_blind: Blind {
                player: players[2].clone(),
                amount: 0.01,
            },
            big_blind: Blind {
                player: players[3].clone(),
                amount: 0.02,
            },
            date: NaiveDateTime::parse_from_str("2024/03/26 17:02:04", "%Y/%m/%d %H:%M:%S")
                .unwrap(),
            end: End {
                pot: 0.06,
                winner: players[4].clone(),
                uncalled_bet: 0.18,
            },
            players: players.clone(),
            players_card: [
                ["".to_string(), "".to_string()],
                ["".to_string(), "".to_string()],
                ["".to_string(), "".to_string()],
                ["2c".to_string(), "7d".to_string()],
                ["".to_string(), "".to_string()],
                ["".to_string(), "".to_string()],
            ],
            preflop: vec![
                Action::Call(players[4].clone(), 0.02),
                Action::Fold(players[5].clone()),
                Action::Fold(players[0].clone()),
                Action::Fold(players[1].clone()),
                Action::Call(players[2].clone(), 0.01),
                Action::Check(players[3].clone()),
            ],
            flop: vec![
                Action::Check(players[2].clone()),
                Action::Check(players[3].clone()),
                Action::Check(players[4].clone()),
            ],
            turn: vec![
                Action::Check(players[2].clone()),
                Action::Check(players[3].clone()),
                Action::Bet(players[4].clone(), 0.18),
                Action::Fold(players[2].clone()),
                Action::Fold(players[3].clone()),
            ],
            river: vec![],
        };

        assert_eq!(actual_hand, expected_hand);
    }

    #[test]
    fn test_split_hands_content() {
        let filepath = "test_hands.txt";
        let filecontent = fs::read_to_string(filepath).expect("Invalid file");
        let actual_hands: Vec<String> = split_hands_content(&filecontent);

        let expected_hands: [&str; 2] = ["PokerStars Hand #249638850870:  Hold'em No Limit ($0.01/$0.02 USD) - 2024/03/26 22:02:04 CET [2024/03/26 17:02:04 ET]
Table 'Ostara III' 6-max Seat #2 is the button
Seat 1: sidneivl ($3.24 in chips) 
Seat 2: Savva08 ($1.96 in chips) 
Seat 3: captelie52 ($0.70 in chips) 
Seat 4: PokerZhyte ($2 in chips) 
Seat 5: alencarbrasil19 ($1.59 in chips) 
Seat 6: Cazunga ($2 in chips) 
captelie52: posts small blind $0.01
PokerZhyte: posts big blind $0.02
*** HOLE CARDS ***
Dealt to PokerZhyte [2c 7d]
alencarbrasil19: calls $0.02
Cazunga: folds 
sidneivl: folds 
Savva08: folds 
captelie52: calls $0.01
PokerZhyte: checks 
*** FLOP *** [Qh 9s 3d]
captelie52: checks 
PokerZhyte: checks 
alencarbrasil19: checks 
*** TURN *** [Qh 9s 3d] [6s]
captelie52: checks 
PokerZhyte: checks 
alencarbrasil19: bets $0.18
captelie52: folds 
PokerZhyte: folds 
Uncalled bet ($0.18) returned to alencarbrasil19
alencarbrasil19 collected $0.06 from pot
alencarbrasil19: doesn't show hand 
*** SUMMARY ***
Total pot $0.06 | Rake $0 
Board [Qh 9s 3d 6s]
Seat 1: sidneivl folded before Flop (didn't bet)
Seat 2: Savva08 (button) folded before Flop (didn't bet)
Seat 3: captelie52 (small blind) folded on the Turn
Seat 4: PokerZhyte (big blind) folded on the Turn
Seat 5: alencarbrasil19 collected ($0.06)
Seat 6: Cazunga folded before Flop (didn't bet)\n\n\n\n",
        "PokerStars Hand #249638867813:  Hold'em No Limit ($0.01/$0.02 USD) - 2024/03/26 22:02:57 CET [2024/03/26 17:02:57 ET]
Table 'Ostara III' 6-max Seat #3 is the button
Seat 1: sidneivl ($3.24 in chips) 
Seat 2: Savva08 ($1.96 in chips) 
Seat 3: captelie52 ($0.68 in chips) 
Seat 4: PokerZhyte ($2 in chips) 
Seat 5: alencarbrasil19 ($1.63 in chips) 
Seat 6: Cazunga ($2 in chips) 
PokerZhyte: posts small blind $0.01
alencarbrasil19: posts big blind $0.02
*** HOLE CARDS ***
Dealt to PokerZhyte [4c 6h]
Cazunga: folds 
sidneivl: folds 
Savva08: raises $0.04 to $0.06
captelie52: calls $0.06
PokerZhyte: folds 
alencarbrasil19: folds 
*** FLOP *** [Th 5h Jc]
Savva08: bets $0.07
captelie52: folds 
Uncalled bet ($0.07) returned to Savva08
Savva08 collected $0.14 from pot
Savva08: doesn't show hand 
*** SUMMARY ***
Total pot $0.15 | Rake $0.01 
Board [Th 5h Jc]
Seat 1: sidneivl folded before Flop (didn't bet)
Seat 2: Savva08 collected ($0.14)
Seat 3: captelie52 (button) folded on the Flop
Seat 4: PokerZhyte (small blind) folded before Flop
Seat 5: alencarbrasil19 (big blind) folded before Flop
Seat 6: Cazunga folded before Flop (didn't bet)\n"];

        assert_eq!(
            expected_hands
                .iter()
                .map(|x| x.trim_start_matches('\u{feff}').to_owned())
                .collect::<Vec<_>>(),
            actual_hands
                .iter()
                .map(|x| x.trim_start_matches('\u{feff}').to_owned())
                .collect::<Vec<_>>()
        );
    }
}
