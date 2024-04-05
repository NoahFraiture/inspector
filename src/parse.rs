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

pub fn split_hands_content(content: &str) -> Vec<String> {
    let mut current_hand = String::new();
    let mut hands = Vec::new();
    for line in content.lines() {
        if line.starts_with("PokerStars ") && !current_hand.is_empty() {
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
    let mut next;
    next = preflop(&mut hand, &mut lines);
    if next {
        next = flop(&mut hand, &mut lines);
    }
    if next {
        next = turn(&mut hand, &mut lines);
    }
    if next {
        next = river(&mut hand, &mut lines);
    }
    if next {
        showdown(&mut hand, &mut lines);
    }
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
    let date = NaiveDateTime::parse_from_str(&date_string, "[%Y/%m/%d %H:%M:%S ET]");
    let offset = FixedOffset::east_opt(5 * 3600).unwrap();
    hand.date = DateTime::<FixedOffset>::from_naive_utc_and_offset(date.unwrap(), offset);

    // extract limits
    // NOTE: may be useless since we create blind object later
    let re = Regex::new(r"\(\$?(\d+\.)?\d+\/\$?(\d+\.)?\d+( USD)?\)").unwrap();
    let capture_limites = re.captures(first_line).unwrap();
    let limits_str = capture_limites[0].to_string();
    let mut chars = limits_str.chars();
    chars.next();
    chars.next_back();
    let mut limits = chars.as_str().split('/');
    let small_limit_str = limits.next().unwrap();
    let big_limit_str = limits.next().unwrap();

    hand.small_limit = small_limit_str.replace('$', "").parse::<f64>().unwrap();
    hand.big_limit = big_limit_str
        .replace('$', "")
        .replace(" USD", "")
        .parse::<f64>()
        .unwrap();

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

fn preflop(hand: &mut Hand, lines: &mut Lines) -> bool {
    let line = lines.next().unwrap();
    let mut split = line.split_whitespace();
    if !line.starts_with("Dealt to ") {
        panic!("Not preflop");
    }
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

    for line in lines {
        if line.starts_with("*** SUMMARY ***") {
            return false;
        }
        if line.starts_with("*** FLOP ***") {
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
            return true;
        }

        if Action::is_action(line) {
            // actions
            hand.preflop.push(Action::get_action(
                hand,
                line.split_whitespace().collect::<Vec<&str>>(),
            ));
        } else if line.contains("collected") {
            hand.end = End::extract_end(hand, line)
        }
    }
    false
}

fn flop(hand: &mut Hand, lines: &mut Lines) -> bool {
    for line in lines {
        if line.starts_with("*** SUMMARY ***") {
            return false;
        }
        if line.starts_with("*** TURN ***") {
            let re = Regex::new(r"\[(..)\]").unwrap();
            let capture_card = re.captures(line).unwrap();
            let mut chars = capture_card[0].chars();
            chars.next(); // remove chars [
            chars.next_back(); // remove chars ]
            hand.turn_card = Some(chars.as_str().to_string());
            return true;
        }

        if Action::is_action(line) {
            hand.flop
                .push(Action::get_action(hand, line.split_whitespace().collect()));
        } else if line.contains("collected") {
            hand.end = End::extract_end(hand, line)
        }
    }
    false
}

fn turn(hand: &mut Hand, lines: &mut Lines) -> bool {
    for line in lines {
        if line.starts_with("*** SUMMARY ***") {
            return false;
        }
        if line.starts_with("*** RIVER ***") {
            let re = Regex::new(r"\[(..)\]").unwrap();
            let capture_card = re.captures(line).unwrap();
            hand.river_card = Some(capture_card[1].to_string().replace(['[', ']'], ""));
            return true;
        }
        if Action::is_action(line) {
            hand.turn
                .push(Action::get_action(hand, line.split_whitespace().collect()));
        } else if line.contains("collected") {
            hand.end = End::extract_end(hand, line)
        }
    }
    false
}

fn river(hand: &mut Hand, lines: &mut Lines) -> bool {
    for line in lines {
        if line.starts_with("*** SUMMARY ***") {
            return false;
        }
        if line.starts_with("*** SHOW DOWN ***") {
            return true;
        }
        if Action::is_action(line) {
            hand.river
                .push(Action::get_action(hand, line.split_whitespace().collect()));
        } else if line.contains("collected") {
            hand.end = End::extract_end(hand, line)
        }
    }
    false
}

fn showdown(hand: &mut Hand, lines: &mut Lines) {
    for line in lines {
        if line.starts_with("*** SUMMARY ***") {
            return;
        }
        if line.contains("shows") {
            let mut split = line.split_whitespace();
            let player_name = split.next().unwrap().replace(':', "");
            let player = hand.get_player(player_name.as_str());
            let re = Regex::new(r"\[(.. ..)\]").unwrap();
            let capture_card = re.captures(line).unwrap();
            let cards_str = capture_card[0].replace(['[', ']'], "");
            let mut cards = cards_str.split_whitespace();
            hand.players_card[player.position as usize - 1] = Some([
                cards.next().unwrap().to_string(),
                cards.next().unwrap().to_string(),
            ]);
        } else if line.contains("collected") {
            hand.end = End::extract_end(hand, line)
        }
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct Hand {
    pub content: String,
    pub id: u64, // u32 is too small
    pub date: DateTime<FixedOffset>,
    pub small_limit: f64,
    pub big_limit: f64,
    pub table_name: String,
    pub table_size: u8,
    pub button_position: u8, // usefull to shift position and guess real position
    pub players: [Option<Player>; 9],
    pub small_blind: Blind,
    pub big_blind: Blind,
    pub end: End, // NOTE: not used
    pub players_card: [Option<[String; 2]>; 9],
    pub preflop: Vec<Action>,
    pub flop: Vec<Action>,
    pub turn: Vec<Action>,
    pub river: Vec<Action>,
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
        panic!("player not found : {:#?}", name)
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct End {
    pub pot: f64,
    pub winner: Player,
}

impl End {
    fn extract_end(hand: &Hand, line: &str) -> Self {
        let mut split = line.split_whitespace();
        let winner = hand.get_player(split.next().unwrap());
        split.next(); // "collected"
        let pot = split
            .next()
            .unwrap()
            .replace('$', "")
            .parse::<f64>()
            .unwrap();
        End { pot, winner }
    }
}

#[derive(Debug, PartialEq)]
pub enum Action {
    Call(Player, f64, bool),
    /// is all-in
    Bet(Player, f64, bool),
    Raise(Player, f64, f64, bool), // raise from .. to ..
    Check(Player),
    Fold(Player),
    Leave(Player),
    UncalledBet(Player, f64),
}

impl Action {
    fn is_action(line: &str) -> bool {
        line.contains("calls")
            || line.contains("bets")
            || line.contains("raises")
            || line.contains("check")
            || line.contains("folds")
            || line.contains("leaves")
            || line.contains("Uncalled bet")
    }

    fn get_action(hand: &Hand, line: Vec<&str>) -> Self {
        match line[1] {
            "calls" => Action::Call(
                hand.get_player(&line[0].replace(':', "")),
                line[2].replace('$', "").parse::<f64>().unwrap(),
                line.contains(&"all-in"),
            ),
            "bets" => Action::Bet(
                hand.get_player(&line[0].replace(':', "")),
                line[2].replace('$', "").parse::<f64>().unwrap(),
                line.contains(&"all-in"),
            ),
            "raises" => Action::Raise(
                hand.get_player(&line[0].replace(':', "")),
                line[2].replace('$', "").parse::<f64>().unwrap(),
                line[4].replace('$', "").parse::<f64>().unwrap(),
                line.contains(&"all-in"),
            ),
            "checks" => Action::Check(hand.get_player(&line[0].replace(':', ""))),
            "folds" => Action::Fold(hand.get_player(&line[0].replace(':', ""))),
            "leaves" => Action::Leave(hand.get_player(&line[0].replace(':', ""))),
            // first is Uncalled
            "bet" => Action::UncalledBet(
                hand.get_player(line.last().unwrap()),
                line[2].replace(['$', '(', ')'], "").parse::<f64>().unwrap(),
            ),
            _ => panic!("Unknow action : {:#?}", line[1]),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Player {
    pub name: String,
    pub position: u8,
    pub bank: f64,
}

#[derive(Default, Debug, PartialEq)]
pub struct Blind {
    pub player: Player,
    pub amount: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn init() -> Vec<String> {
        let filepath = "test/test_hands.txt";
        let mut filecontent = fs::read_to_string(filepath).expect("Invalid file");
        filecontent = filecontent.replace('\r', "");
        filecontent = filecontent.replace('\u{feff}', "");
        split_hands_content(&filecontent)
    }

    // test function with hand folding at TURN with real money.
    // No player show hand
    #[test]
    fn test_real_fold() {
        let hands_content = init();
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
            small_limit: 0.01,
            big_limit: 0.02,
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
        };

        assert_eq!(actual_hand, expected_hand);
    }

    #[test]
    fn test_fake_showdown() {
        let hands_content = init();
        let actual_hand = parse_hand(&hands_content[1]);

        let players = [
            Some(Player {
                name: "mrdee12".to_string(),
                bank: 9700.,
                position: 1,
            }),
            Some(Player {
                name: "carlitosbomba".to_string(),
                bank: 9178.,
                position: 2,
            }),
            Some(Player {
                name: "PokerZhyte".to_string(),
                bank: 10000.,
                position: 3,
            }),
            Some(Player {
                name: "haroldfried13".to_string(),
                bank: 12004.,
                position: 4,
            }),
            Some(Player {
                name: "gerdi2".to_string(),
                bank: 45153.,
                position: 5,
            }),
            Some(Player {
                name: "ArrAppA-Hi".to_string(),
                position: 6,
                bank: 11063.,
            }),
            None,
            None,
            None,
        ];

        let naive_date =
            NaiveDateTime::parse_from_str("[2024/03/29 12:03:57 ET]", "[%Y/%m/%d %H:%M:%S ET]");
        let offset = FixedOffset::east_opt(5 * 3600).unwrap();
        let date = DateTime::<FixedOffset>::from_naive_utc_and_offset(naive_date.unwrap(), offset);
        let expected_hand = Hand {
            content: hands_content[1].clone(),
            id: 249687478472,
            date,
            small_limit: 50.,
            big_limit: 100.,
            table_name: "NLHE 50/100 6 Max".to_string(),
            table_size: 6,
            button_position: 1,
            players: players.clone(),
            small_blind: Blind {
                player: players[1].clone().unwrap(),
                amount: 50.,
            },
            big_blind: Blind {
                player: players[2].clone().unwrap(),
                amount: 100.,
            },
            end: End {
                pot: 20846.,
                winner: players[1].clone().unwrap(),
            },
            players_card: [
                None,
                Some(["Th".to_string(), "5h".to_string()]),
                Some(["Ah".to_string(), "As".to_string()]),
                None,
                None,
                Some(["Ac".to_string(), "6h".to_string()]),
                None,
                None,
                None,
            ],
            preflop: vec![
                Action::Fold(players[3].clone().unwrap()),
                Action::Call(players[4].clone().unwrap(), 100., false),
                Action::Call(players[5].clone().unwrap(), 100., false),
                Action::Call(players[0].clone().unwrap(), 100., false),
                Action::Call(players[1].clone().unwrap(), 50., false),
                Action::Raise(players[2].clone().unwrap(), 400., 500., false),
                Action::Fold(players[4].clone().unwrap()),
                Action::Call(players[5].clone().unwrap(), 400., false),
                Action::Fold(players[0].clone().unwrap()),
                Action::Call(players[1].clone().unwrap(), 400., false),
            ],
            flop: vec![
                Action::Check(players[1].clone().unwrap()),
                Action::Bet(players[2].clone().unwrap(), 1003., false),
                Action::Call(players[5].clone().unwrap(), 1003., false),
                Action::Call(players[1].clone().unwrap(), 1003., false),
            ],
            turn: vec![
                Action::Bet(players[1].clone().unwrap(), 2000., false),
                Action::Call(players[2].clone().unwrap(), 2000., false),
                Action::Raise(players[5].clone().unwrap(), 7560., 9560., true),
                Action::Call(players[1].clone().unwrap(), 5675., true),
                Action::Fold(players[2].clone().unwrap()),
                Action::UncalledBet(players[5].clone().unwrap(), 1885.),
            ],
            river: vec![],
            flop_card: Some([String::from("8c"), String::from("7c"), String::from("Jc")]),
            turn_card: Some(String::from("9s")),
            river_card: Some(String::from("8s")),
        };

        assert_eq!(actual_hand, expected_hand);
    }
}
