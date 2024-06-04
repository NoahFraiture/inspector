use app::models::Hand;
use chrono::{DateTime, FixedOffset, NaiveDateTime};
use regex::Regex;
use std::fmt;
use std::fs;
use std::str::Lines;

pub fn parse_file(filepath: &str) -> Result<Vec<HandDetail>, ParseError> {
  let mut filecontent = fs::read_to_string(filepath).expect("Invalid file");
  filecontent = filecontent.replace('\r', "");
  filecontent = filecontent.replace('\u{feff}', "");

  let hands_content = split_hands_content(&filecontent);
  let mut hands: Vec<HandDetail> = vec![];
  for hand in hands_content {
    let h = HandDetail::parse_hand(&hand)?;
    println!("{:#?}", h.id);
    hands.push(h);
  }
  Ok(hands)
}

fn split_hands_content(content: &str) -> Vec<String> {
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

// This structure will be used to compute the stats of the player
#[derive(Default, Debug, PartialEq)]
pub struct HandDetail {
  pub id: i64, // u32 is too small
  pub content: String,
  pub real_money: bool,
  pub date: DateTime<FixedOffset>,
  pub small_limit: f32,
  pub big_limit: f32,
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

impl HandDetail {
  fn parse_hand(hand_txt: &str) -> Result<Self, ParseError> {
    let mut hand = HandDetail {
      content: hand_txt.to_string(),
      ..Default::default()
    };
    let mut lines = hand_txt.lines();

    start(&mut hand, &mut lines)?;
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
    Ok(hand)
  }

  pub fn to_hand(&self) -> Hand {
    Hand {
      id: self.id,
      content: self.content.clone(),
      real_money: self.real_money,
      time: self.date.timestamp(),
      table_name: self.table_name.clone(),
      table_size: self.table_size as i32,
      winner: self.end.winner.name.clone(),
      pot: self.end.pot,
      player1: self.players[0]
        .as_ref()
        .map_or(String::new(), |p| p.name.clone()),
      player2: self.players[1]
        .as_ref()
        .map_or(String::new(), |p| p.name.clone()),
      player3: self.players[2]
        .as_ref()
        .map_or(String::new(), |p| p.name.clone()),
      player4: self.players[3]
        .as_ref()
        .map_or(String::new(), |p| p.name.clone()),
      player5: self.players[4]
        .as_ref()
        .map_or(String::new(), |p| p.name.clone()),
      player6: self.players[5]
        .as_ref()
        .map_or(String::new(), |p| p.name.clone()),
      player7: self.players[6]
        .as_ref()
        .map_or(String::new(), |p| p.name.clone()),
      player8: self.players[7]
        .as_ref()
        .map_or(String::new(), |p| p.name.clone()),
      player9: self.players[8]
        .as_ref()
        .map_or(String::new(), |p| p.name.clone()),
      card1: self
        .flop_card
        .as_ref()
        .map_or(String::new(), |cards| cards[0].clone()),
      card2: self
        .flop_card
        .as_ref()
        .map_or(String::new(), |cards| cards[1].clone()),
      card3: self
        .flop_card
        .as_ref()
        .map_or(String::new(), |cards| cards[2].clone()),
      card4: self.turn_card.as_ref().map_or(String::new(), |c| c.clone()),
      card5: self.turn_card.as_ref().map_or(String::new(), |c| c.clone()),
    }
  }

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

#[derive(Debug, Clone)]
pub enum ParseError {
  Start(String),
  GetAction(String),
}

impl fmt::Display for ParseError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      ParseError::Start(msg) => write!(f, "Start error: {}", msg),
      ParseError::GetAction(msg) => write!(f, "GetAction error: {}", msg),
    }
  }
}

trait FromErr {
  fn err_start(e: impl std::string::ToString) -> Self;
  fn err_start_msg(e: impl std::string::ToString, content: &str) -> Self;
  fn err_action(e: impl std::string::ToString) -> Self;
}

impl FromErr for ParseError {
  fn err_start(e: impl std::string::ToString) -> Self {
    ParseError::Start(format!("Error in parsing start : {}", e.to_string(),))
  }
  fn err_start_msg(e: impl std::string::ToString, content: &str) -> Self {
    ParseError::Start(format!(
      "Error in parsing start : {}. Content : {}",
      e.to_string(),
      content
    ))
  }
  fn err_action(e: impl std::string::ToString) -> Self {
    ParseError::GetAction(format!("Error in getting the action : {}", e.to_string()))
  }
}

trait FromNone {
  fn none_start(s: &str) -> Self;
  fn none_action(s: &str) -> Self;
}

impl FromNone for ParseError {
  fn none_start(s: &str) -> Self {
    ParseError::Start(format!("Error in parsing start : {}", s))
  }
  fn none_action(s: &str) -> Self {
    ParseError::GetAction(format!("Error in parsing action: {}", s))
  }
}

// ========================================
// === PARSE DIFFERENT PART OF THE FILE ===
// ========================================

fn extract_id(line: &str) -> Result<i64, ParseError> {
  let re = Regex::new(r"#(\d+)").map_err(ParseError::err_start)?;
  let capture_id = re
    .captures(line)
    .ok_or(ParseError::none_start("Id regex failed"))?;
  let mut chars = capture_id[0].chars();
  chars.next();
  let content = chars.as_str();
  content
    .parse::<i64>()
    .map_err(|e| ParseError::err_start_msg(e, content))
}

fn extract_date(line: &str) -> Result<DateTime<FixedOffset>, ParseError> {
  let re = Regex::new(r"\[(.*?)\]").map_err(ParseError::err_start)?;
  let capture_date = re
    .captures(line)
    .ok_or(ParseError::none_start("Date regex failed"))?;
  let date_string = capture_date[0].to_string();

  let date = NaiveDateTime::parse_from_str(&date_string, "[%Y/%m/%d %H:%M:%S ET]")
    .map_err(ParseError::err_start)?;

  let offset = FixedOffset::east_opt(5 * 3600).ok_or(ParseError::none_start("Offset failed"))?;

  Ok(DateTime::<FixedOffset>::from_naive_utc_and_offset(
    date, offset,
  ))
}

fn extract_limits(line: &str) -> Result<(f32, f32), ParseError> {
  let re =
    Regex::new(r"\(\$?(\d+\.)?\d+\/\$?(\d+\.)?\d+( USD)?\)").map_err(ParseError::err_start)?;
  let capture_limites = re
    .captures(line)
    .ok_or(ParseError::none_start("Limits regex failed"))?;
  let limits_str = capture_limites[0].to_string();
  let mut chars = limits_str.chars();
  chars.next();
  chars.next_back();
  let mut limits = chars.as_str().split('/');
  let small_limit_str = limits.next().unwrap();
  let big_limit_str = limits.next().unwrap();
  Ok((
    small_limit_str
      .replace('$', "")
      .parse::<f32>()
      .map_err(ParseError::err_start)?,
    big_limit_str
      .replace('$', "")
      .replace(" USD", "")
      .parse::<f32>()
      .map_err(ParseError::err_start)?,
  ))
}

fn extract_table_name(line: &str) -> Result<String, ParseError> {
  let re = Regex::new(r"'([^']*)'").map_err(ParseError::err_start)?;
  let capture_table_name = re
    .captures(line)
    .ok_or(ParseError::none_start("Table name regex failed"))?;
  let mut chars = capture_table_name[0].chars();
  chars.next();
  chars.next_back();
  Ok(chars.as_str().to_string())
}

fn extract_button_position(line: &str) -> Result<u8, ParseError> {
  let re = Regex::new(r"#(\d+)").map_err(ParseError::err_start)?;
  let capture_button_position = re
    .captures(line)
    .ok_or(ParseError::none_start("Button position regex failed"))?;
  let mut chars = capture_button_position[0].chars();
  chars.next();
  chars.as_str().parse::<u8>().map_err(ParseError::err_start)
}

fn extract_table_size(line: &str) -> Result<u8, ParseError> {
  let re = Regex::new(r"(\d+)-max").unwrap();
  let capture_table_size = re.captures(line).unwrap();
  let mut chars = capture_table_size[0].chars();
  chars.next_back();
  chars.next_back();
  chars.next_back();
  chars.next_back();
  chars.as_str().parse::<u8>().map_err(ParseError::err_start)
}

fn extract_blind(
  hand: &HandDetail,
  split: &mut std::str::SplitWhitespace,
) -> Result<Blind, ParseError> {
  let player_word = split
    .next()
    .ok_or(ParseError::none_start("Player not found"))?;
  let player = hand.get_player(player_word);
  let amount_word = split
    .last()
    .ok_or(ParseError::none_start("Amount not found"))?;
  let amount = amount_word
    .replace('$', "")
    .parse::<f32>()
    .map_err(|e| ParseError::err_start_msg(e, amount_word))?;
  Ok(Blind { player, amount })
}

fn extract_seat(line: &str) -> Result<Player, ParseError> {
  let position_re = Regex::new(r"[0-9]").map_err(ParseError::err_start)?;
  let capture_position = position_re
    .captures(line)
    .ok_or(ParseError::none_start("Position not found"))?;
  let position = capture_position[0]
    .replace(':', "")
    .parse::<u8>()
    .map_err(ParseError::err_start)?;
  let name_re = Regex::new(r": .* \(").map_err(ParseError::err_start)?;
  let capture_name = name_re
    .captures(line)
    .ok_or(ParseError::none_start("Name not found"))?;
  let name = String::from(capture_name[0].replace([':', '('], "").trim());
  let bank_re = Regex::new(r"\(\$?(\d+\.)?\d+ ").map_err(ParseError::err_start)?;
  let capture_bank = bank_re
    .captures(line)
    .ok_or(ParseError::none_start("Bank not found"))?;
  let bank = capture_bank[0]
    .replace(['$', '('], "")
    .trim()
    .parse::<f32>()
    .map_err(|e| ParseError::err_start_msg(e, &capture_bank[0]))?;
  Ok(Player {
    name,
    position,
    bank,
  })
}

fn start(hand: &mut HandDetail, lines: &mut Lines) -> Result<(), ParseError> {
  let first_line = lines.next().unwrap();
  hand.id = extract_id(first_line)?;
  hand.date = extract_date(first_line)?;

  // NOTE: may be useless since we create blind object later
  (hand.small_limit, hand.big_limit) = extract_limits(first_line)?;

  let second_line = lines.next().unwrap();
  hand.table_name = extract_table_name(second_line)?;

  // extract button position to latter shift and get actual position of the players
  hand.button_position = extract_button_position(second_line)?;
  hand.table_size = extract_table_size(second_line)?;

  // not very optimized but very easy
  for line in lines {
    let mut split = line.split_whitespace();
    if line.contains("posts small blind") {
      hand.small_blind = extract_blind(hand, &mut split)?;
    } else if line.contains("posts big blind") {
      hand.big_blind = extract_blind(hand, &mut split)?;
    } else if line.starts_with("Seat ") {
      split.next(); // "Seat"
      let player = extract_seat(line)?;
      let position = player.position as usize - 1;
      hand.players[position] = Some(player);
    } else if line == "*** HOLE CARDS ***" {
      return Ok(());
    }
  }
  Ok(())
}

fn preflop(hand: &mut HandDetail, lines: &mut Lines) -> bool {
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
      hand.preflop.push(
        Action::get_action(hand, line)
          .unwrap_or_else(|e| panic!("Error in preflop action parsing : {}", e)),
      );
    } else if line.contains("collected") {
      hand.end = End::extract_end(hand, line)
    }
  }
  false
}

fn flop(hand: &mut HandDetail, lines: &mut Lines) -> bool {
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
      hand.flop.push(
        Action::get_action(hand, line)
          .unwrap_or_else(|e| panic!("Error in flop action parsing : {}", e)),
      );
    } else if line.contains("collected") {
      hand.end = End::extract_end(hand, line)
    }
  }
  false
}

fn turn(hand: &mut HandDetail, lines: &mut Lines) -> bool {
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
      hand.turn.push(
        Action::get_action(hand, line)
          .unwrap_or_else(|e| panic!("Error in turn action parsing : {}", e)),
      );
    } else if line.contains("collected") {
      hand.end = End::extract_end(hand, line)
    }
  }
  false
}

fn river(hand: &mut HandDetail, lines: &mut Lines) -> bool {
  for line in lines {
    if line.starts_with("*** SUMMARY ***") {
      return false;
    }
    if line.starts_with("*** SHOW DOWN ***") {
      return true;
    }
    if Action::is_action(line) {
      hand.river.push(
        Action::get_action(hand, line)
          .unwrap_or_else(|e| panic! {"Error in river action parsing : {}", e}),
      );
    } else if line.contains("collected") {
      hand.end = End::extract_end(hand, line)
    }
  }
  false
}

fn showdown(hand: &mut HandDetail, lines: &mut Lines) {
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
pub struct End {
  pub pot: f32,
  pub winner: Player,
}

impl End {
  fn extract_end(hand: &HandDetail, line: &str) -> Self {
    let mut split = line.split_whitespace();
    let winner = hand.get_player(split.next().unwrap());
    split.next(); // "collected"
    let pot = split
      .next()
      .unwrap()
      .replace('$', "")
      .parse::<f32>()
      .unwrap();
    End { pot, winner }
  }
}

#[derive(Debug, PartialEq)]
pub enum Action {
  Call(Player, f32, bool),
  /// is all-in
  Bet(Player, f32, bool),
  Raise(Player, f32, f32, bool), // raise from .. to ..
  Check(Player),
  Fold(Player),
  Leave(Player),
  UncalledBet(Player, f32),
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

  fn get_action(hand: &HandDetail, line: &str) -> Result<Self, ParseError> {
    println!("{}", line);
    let player_re = Regex::new(r".*:").map_err(ParseError::err_action)?;
    let capture_position = player_re
      .captures(line)
      .ok_or(ParseError::none_action("player not found"))?;
    let player = capture_position[0].replace([':'], "");

    let line_after_player_re = Regex::new(r":.*").map_err(ParseError::err_action)?;
    let capture_after = line_after_player_re
      .captures(line)
      .ok_or(ParseError::none_action("':' not found"))?;
    let binding = capture_after[0].replace([':'], "");
    let line = &binding.trim();

    let action_re = Regex::new(r"\w+").map_err(ParseError::err_action)?;
    let capture_action = action_re
      .captures(line)
      .ok_or(ParseError::none_action("action not found"))?;
    let binding = capture_action[0].replace([':', '$'], "");
    let action = binding.trim();

    let amount_re = Regex::new(r"\$?(\d+\.)?\d+").map_err(ParseError::err_action)?;
    let captures_amount = amount_re.captures_iter(line);
    let mut amounts = captures_amount
      .map(|a| a[0].replace(['$'], ""))
      .map(|a| a.parse::<f32>());

    match action {
      "calls" => Ok(Action::Call(
        hand.get_player(&player),
        amounts
          .next()
          .ok_or(ParseError::none_action("Amount 1 not found"))?
          .map_err(ParseError::err_action)?,
        line.contains("all-in"),
      )),
      "bets" => Ok(Action::Bet(
        hand.get_player(&player),
        amounts
          .next()
          .ok_or(ParseError::none_action("Amount 1 not found"))?
          .map_err(ParseError::err_action)?,
        line.contains("all-in"),
      )),
      "raises" => Ok(Action::Raise(
        hand.get_player(&player),
        amounts
          .next()
          .ok_or(ParseError::none_action("Amount 1 not found"))?
          .map_err(ParseError::err_action)?,
        amounts
          .next()
          .ok_or(ParseError::none_action("Amount 2 not found"))?
          .map_err(ParseError::err_action)?,
        line.contains("all-in"),
      )),
      "checks" => Ok(Action::Check(hand.get_player(&player))),
      "folds" => Ok(Action::Fold(hand.get_player(&player))),
      "leaves" => Ok(Action::Leave(hand.get_player(&player))),
      // first is Uncalled
      // TODO : add Uncalled action
      "bet" => Ok(Action::UncalledBet(
        hand.get_player(&player),
        amounts
          .next()
          .ok_or(ParseError::none_action("Amount 1 not found"))?
          .map_err(ParseError::err_action)?,
      )),
      _ => Err(ParseError::GetAction(
        String::from("Unknow action : ") + action,
      )),
    }
  }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Player {
  pub name: String,
  pub position: u8,
  pub bank: f32,
}

#[derive(Default, Debug, PartialEq)]
pub struct Blind {
  pub player: Player,
  pub amount: f32,
}

#[cfg(test)]
mod tests {
  // TODO: modify player to match new db player
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
    let actual_hand = HandDetail::parse_hand(&hands_content[0]);

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
    let expected_hand = HandDetail {
      id: 249638850870,
      content: hands_content[0].clone(),
      real_money: true,
      date,
      small_limit: 0.01,
      big_limit: 0.02,
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
      flop_card: Some(["Qh".to_string(), "9s".to_string(), "3d".to_string()]),
      turn_card: Some("6s".to_string()),
      river_card: None,
    };

    // assert_eq!(actual_hand, expected_hand);
  }

  #[test]
  fn test_fake_showdown() {
    let hands_content = init();
    let actual_hand = HandDetail::parse_hand(&hands_content[1]);

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
    let expected_hand = HandDetail {
      content: hands_content[1].clone(),
      real_money: false,
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

    // assert_eq!(actual_hand, expected_hand);
  }
}
