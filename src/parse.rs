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
pub struct Hand {
    content: String,
    id: u64, // u32 is too small
    date: DateTime<FixedOffset>,
    table_name: String,
    table_size: u8,
    button_position: u8, // usefull to shift position and guess real position
    players: [Player; 6],
    small_blind: Blind,
    big_blind: Blind,
    end: End,
    players_card: [[String; 2]; 6],
    preflop: Vec<Action>,
    flop: Vec<Action>,
    turn: Vec<Action>,
    river: Vec<Action>,
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
    bank: f64,
}

#[derive(Default, Debug, PartialEq)]
struct Blind {
    player: Player,
    amount: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_start() {
        let mut input = "PokerStars Hand #249638850870:  Hold'em No Limit ($0.01/$0.02 USD) - 2024/03/26 22:02:04 CET [2024/03/26 17:02:04 ET]
Table 'Ostara III' 6-max Seat #2 is the button
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

        let naive_date =
            NaiveDateTime::parse_from_str("[2024/03/26 17:02:04 ET]", "[%Y/%m/%d %H:%M:%S ET]");
        let offset = FixedOffset::east_opt(5 * 3600).unwrap();
        let date = DateTime::<FixedOffset>::from_naive_utc_and_offset(naive_date.unwrap(), offset);
        let expected_hand = Hand {
            id: 249638850870,
            date,
            table_name: "Ostara III".to_string(),
            table_size: 6,
            button_position: 2,
            small_blind: Blind {
                player: Player {
                    name: "captelie52".to_string(),
                    position: 3,
                    bank: 0.7,
                },
                amount: 0.01,
            },
            big_blind: Blind {
                player: Player {
                    name: "PokerZhyte".to_string(),
                    position: 4,
                    bank: 2.,
                },
                amount: 0.02,
            },
            players: [
                Player {
                    name: "sidneivl".to_string(),
                    position: 1,
                    bank: 3.24,
                },
                Player {
                    name: "Savva08".to_string(),
                    position: 2,
                    bank: 1.96,
                },
                Player {
                    name: "captelie52".to_string(),
                    position: 3,
                    bank: 0.7,
                },
                Player {
                    name: "PokerZhyte".to_string(),
                    position: 4,
                    bank: 2.0,
                },
                Player {
                    name: "alencarbrasil19".to_string(),
                    position: 5,
                    bank: 1.59,
                },
                Player {
                    name: "Cazunga".to_string(),
                    position: 6,
                    bank: 2.0,
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
        let players = [
            Player {
                name: "sidneivl".to_string(),
                position: 1,
                bank: 3.24,
            },
            Player {
                name: "Savva08".to_string(),
                position: 2,
                bank: 1.96,
            },
            Player {
                name: "captelie52".to_string(),
                position: 3,
                bank: 0.7,
            },
            Player {
                name: "PokerZhyte".to_string(),
                position: 4,
                bank: 2.0,
            },
            Player {
                name: "alencarbrasil19".to_string(),
                position: 5,
                bank: 1.59,
            },
            Player {
                name: "Cazunga".to_string(),
                position: 6,
                bank: 2.0,
            },
        ];
        let mut actual_hand = Hand {
            small_blind: Blind {
                player: players[2].clone(),
                amount: 0.01,
            },
            big_blind: Blind {
                player: players[3].clone(),
                amount: 0.02,
            },
            players: players.clone(),
            ..Default::default()
        };
        preflop(&mut actual_hand, &mut input);
        let expected_hand = Hand {
            small_blind: Blind {
                player: players[2].clone(),
                amount: 0.01,
            },
            big_blind: Blind {
                player: players[3].clone(),
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
        let players = [
            Player {
                name: "sidneivl".to_string(),
                position: 1,
                bank: 3.24,
            },
            Player {
                name: "Savva08".to_string(),
                position: 2,
                bank: 1.96,
            },
            Player {
                name: "captelie52".to_string(),
                position: 3,
                bank: 0.7,
            },
            Player {
                name: "PokerZhyte".to_string(),
                position: 4,
                bank: 2.0,
            },
            Player {
                name: "alencarbrasil19".to_string(),
                position: 5,
                bank: 1.59,
            },
            Player {
                name: "Cazunga".to_string(),
                position: 6,
                bank: 2.0,
            },
        ];
        let mut actual_hand = Hand {
            small_blind: Blind {
                player: players[3].clone(),
                amount: 0.1,
            },
            big_blind: Blind {
                player: players[4].clone(),
                amount: 0.2,
            },
            players: players.clone(),
            ..Default::default()
        };
        flop(&mut actual_hand, &mut input);
        let expected_hand = Hand {
            small_blind: Blind {
                player: players[3].clone(),
                amount: 0.1,
            },
            big_blind: Blind {
                player: players[4].clone(),
                amount: 0.2,
            },
            players: players.clone(),
            flop: vec![
                Action::Bet(players[1].clone(), 0.07),
                Action::Fold(players[2].clone()),
            ],
            end: End {
                winner: players[1].clone(),
                pot: 0.14,
                uncalled_bet: 0.07,
            },
            ..Default::default()
        };

        assert_eq!(actual_hand, expected_hand);
    }

    #[test]
    fn test_parsing() {
        let filepath = "test/test_hands.txt";
        let mut filecontent = fs::read_to_string(filepath).expect("Invalid file");
        filecontent = filecontent.replace('\r', "");
        filecontent = filecontent.replace('\u{feff}', "");
        let hands_content = split_hands_content(&filecontent);
        let actual_hand = parse_hand(&hands_content[0]);

        let players = [
            Player {
                name: "sidneivl".to_string(),
                position: 1,
                bank: 3.24,
            },
            Player {
                name: "Savva08".to_string(),
                position: 2,
                bank: 1.96,
            },
            Player {
                name: "captelie52".to_string(),
                position: 3,
                bank: 0.70,
            },
            Player {
                name: "PokerZhyte".to_string(),
                position: 4,
                bank: 2.,
            },
            Player {
                name: "alencarbrasil19".to_string(),
                position: 5,
                bank: 1.59,
            },
            Player {
                name: "Cazunga".to_string(),
                position: 6,
                bank: 2.,
            },
        ];
        let naive_date =
            NaiveDateTime::parse_from_str("[2024/03/26 17:02:04 ET]", "[%Y/%m/%d %H:%M:%S ET]");
        let offset = FixedOffset::east_opt(5 * 3600).unwrap();
        let date = DateTime::<FixedOffset>::from_naive_utc_and_offset(naive_date.unwrap(), offset);
        let expected_hand = Hand {
            content: hands_content[0].clone(),
            id: 249638850870,
            date,
            table_name: "Ostara III".to_string(),
            table_size: 6,
            button_position: 2,
            players: players.clone(),
            small_blind: Blind {
                player: players[2].clone(),
                amount: 0.01,
            },
            big_blind: Blind {
                player: players[3].clone(),
                amount: 0.02,
            },
            end: End {
                pot: 0.06,
                winner: players[4].clone(),
                uncalled_bet: 0.18,
            },
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
        let filepath = "test/test_hands.txt";
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
