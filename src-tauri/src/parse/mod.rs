use crate::models::Hand;
use chrono::{DateTime, FixedOffset, NaiveDateTime};
use std::fmt;
use std::fs;
use std::str::Lines;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

mod re;

pub fn parse_file(filepath: &str) -> Result<Vec<HandDetail>, ParseError> {
  let mut filecontent = fs::read_to_string(filepath).expect("Invalid file");
  filecontent = filecontent.replace('\r', "");
  filecontent = filecontent.replace('\u{feff}', "");

  let start_time: Arc<Mutex<Duration>> = Arc::new(Mutex::new(Duration::new(0, 0)));
  let preflop_time: Arc<Mutex<Duration>> = Arc::new(Mutex::new(Duration::new(0, 0)));
  let flop_time: Arc<Mutex<Duration>> = Arc::new(Mutex::new(Duration::new(0, 0)));
  let turn_time: Arc<Mutex<Duration>> = Arc::new(Mutex::new(Duration::new(0, 0)));
  let river_time: Arc<Mutex<Duration>> = Arc::new(Mutex::new(Duration::new(0, 0)));
  let showdown_time: Arc<Mutex<Duration>> = Arc::new(Mutex::new(Duration::new(0, 0)));

  let hands_content = split_hands_content(&filecontent);
  let mut hands: Vec<HandDetail> = vec![];
  for hand in hands_content {
    let h = HandDetail::parse_hand(
      &hand,
      &start_time,
      &preflop_time,
      &flop_time,
      &turn_time,
      &river_time,
      &showdown_time,
    )?;
    hands.push(h);
  }

  println!("Start time : {}", start_time.lock().unwrap().as_millis());
  println!(
    "preflop time : {}",
    preflop_time.lock().unwrap().as_millis()
  );
  println!("flop time : {}", flop_time.lock().unwrap().as_millis());
  println!("turn time : {}", turn_time.lock().unwrap().as_millis());
  println!("river time : {}", river_time.lock().unwrap().as_millis());
  println!(
    "showdown time : {}",
    showdown_time.lock().unwrap().as_millis()
  );

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
  fn parse_hand(
    hand_txt: &str,
    start_time: &Arc<Mutex<Duration>>,
    preflop_time: &Arc<Mutex<Duration>>,
    flop_time: &Arc<Mutex<Duration>>,
    turn_time: &Arc<Mutex<Duration>>,
    river_time: &Arc<Mutex<Duration>>,
    showdown_time: &Arc<Mutex<Duration>>,
  ) -> Result<Self, ParseError> {
    let mut hand = HandDetail {
      content: hand_txt.to_string(),
      ..Default::default()
    };
    let mut lines = hand_txt.lines();

    let mut instant_start = Instant::now();

    start(&mut hand, &mut lines)?;
    *start_time.lock().unwrap() += instant_start.elapsed();
    instant_start = Instant::now();

    let mut next;
    next = preflop(&mut hand, &mut lines)?;
    *preflop_time.lock().unwrap() += instant_start.elapsed();
    instant_start = Instant::now();

    if next {
      next = flop(&mut hand, &mut lines)?;
      *flop_time.lock().unwrap() += instant_start.elapsed();
      instant_start = Instant::now();
    }
    if next {
      next = turn(&mut hand, &mut lines)?;
      *turn_time.lock().unwrap() += instant_start.elapsed();
      instant_start = Instant::now();
    }
    if next {
      next = river(&mut hand, &mut lines)?;
      *river_time.lock().unwrap() += instant_start.elapsed();
      instant_start = Instant::now();
    }
    if next {
      showdown(&mut hand, &mut lines)?;
      *showdown_time.lock().unwrap() += instant_start.elapsed();
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

  fn get_player(&self, name: &str) -> Result<Player, ParseError> {
    let trimed_name = name.trim_end_matches(':');
    for player in &self.players {
      match player {
        Some(player) => {
          if player.name == trimed_name {
            return Ok(player.clone());
          }
        }
        None => continue,
      }
    }
    Err(err(
      ParseErrorType::Unknown(String::from("get player")),
      name,
    ))
  }
}

#[derive(Debug, Clone)]
enum ParseErrorType {
  Start,
  Preflop,
  Flop,
  Turn,
  River,
  Showdown,
  Unknown(String),
}

#[derive(Debug)]
pub struct ParseError {
  t: ParseErrorType,
  msg: String,
}

// TODO : refactor
impl fmt::Display for ParseError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match &self.t {
      ParseErrorType::Start => write!(f, "Start error: {}", self.msg),
      ParseErrorType::Preflop => write!(f, "Preflop error: {}", self.msg),
      ParseErrorType::Flop => write!(f, "Flop error: {}", self.msg),
      ParseErrorType::Turn => write!(f, "Turn error: {}", self.msg),
      ParseErrorType::River => write!(f, "River error: {}", self.msg),
      ParseErrorType::Showdown => write!(f, "Showdown error: {}", self.msg),
      ParseErrorType::Unknown(m) => write!(f, "Unknown {} error: {}", m, self.msg),
    }
  }
}

fn err(t: ParseErrorType, e: impl std::string::ToString) -> ParseError {
  ParseError {
    msg: format!("Error : {}", e.to_string()),
    t,
  }
}
fn err_msg(t: ParseErrorType, e: impl std::string::ToString, msg: &str) -> ParseError {
  ParseError {
    t,

    // this way we get after the inner error
    msg: format!("Message: {}\nError: {}", msg, e.to_string()),
  }
}

// ========================================
// === PARSE DIFFERENT PART OF THE FILE ===
// ========================================

fn extract_id(line: &str) -> Result<i64, ParseError> {
  let capture_id = re::TABLE_ID
    .captures(line)
    .ok_or(err(ParseErrorType::Start, "Id regex failed"))?;
  let mut chars = capture_id[0].chars();
  chars.next();
  let content = chars.as_str();
  content
    .parse::<i64>()
    .map_err(|e| err_msg(ParseErrorType::Start, e, content))
}

fn extract_date(line: &str) -> Result<DateTime<FixedOffset>, ParseError> {
  let capture_date = re::BRACKET
    .captures(line)
    .ok_or(err(ParseErrorType::Start, "Date regex failed"))?;
  let date_string = capture_date[0].to_string();

  let date = NaiveDateTime::parse_from_str(&date_string, "[%Y/%m/%d %H:%M:%S ET]")
    .map_err(|e| err(ParseErrorType::Start, e))?;

  let offset =
    FixedOffset::east_opt(5 * 3600).ok_or(err(ParseErrorType::Start, "Offset failed"))?;

  Ok(DateTime::<FixedOffset>::from_naive_utc_and_offset(
    date, offset,
  ))
}

fn extract_limits(line: &str) -> Result<(f32, f32), ParseError> {
  // replace by simple number match and take 2nd and 3rd
  let capture_limites = re::LIMIT
    .captures(line)
    .ok_or(err(ParseErrorType::Start, "Limit not found"))?;
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
      .map_err(|e| err(ParseErrorType::Start, e))?,
    big_limit_str
      .replace('$', "")
      .replace(" USD", "")
      .parse::<f32>()
      .map_err(|e| err(ParseErrorType::Start, e))?,
  ))
}

fn extract_table_name(line: &str) -> Result<String, ParseError> {
  let capture_table_name = re::TABLE_NAME
    .captures(line)
    .ok_or(err(ParseErrorType::Start, "Table name regex failed"))?;
  let mut chars = capture_table_name[0].chars();
  chars.next();
  chars.next_back();
  Ok(chars.as_str().to_string())
}

fn extract_button_position(line: &str) -> Result<u8, ParseError> {
  let capture_button_position = re::TABLE_ID
    .captures(line)
    .ok_or(err(ParseErrorType::Start, "Button position regex failed"))?;
  let mut chars = capture_button_position[0].chars();
  chars.next();
  chars
    .as_str()
    .parse::<u8>()
    .map_err(|e| err(ParseErrorType::Start, e))
}

fn extract_table_size(line: &str) -> Result<u8, ParseError> {
  let capture_table_size = re::TABLE_SIZE.captures(line).unwrap();
  let mut chars = capture_table_size[0].chars();
  chars.next_back();
  chars.next_back();
  chars.next_back();
  chars.next_back();
  chars
    .as_str()
    .parse::<u8>()
    .map_err(|e| err(ParseErrorType::Start, e))
}

fn extract_blind(hand: &HandDetail, line: &str) -> Result<Blind, ParseError> {
  let capture_player = re::BEFORE_COLON
    .captures(line)
    .ok_or(err(ParseErrorType::Start, "extracting blind"))?;
  let player = hand
    .get_player(&capture_player[0].replace([':'], ""))
    .map_err(|e| err(ParseErrorType::Start, e))?;
  let after_line = re::AFTER_COLON.captures(line).ok_or(err(
    ParseErrorType::Start,
    "unable to get the line after colon",
  ))?;
  let line = after_line[0].to_string();
  let capture = re::MONEY
    .captures(&line)
    .ok_or(err(ParseErrorType::Start, "Can't find amount in blind"))?;
  let amount = capture[0]
    .replace(['('], "")
    .parse::<f32>()
    .map_err(|e| err(ParseErrorType::Start, e))?;
  Ok(Blind { player, amount })
}

fn extract_seat(line: &str) -> Result<Player, ParseError> {
  let capture_position = re::NUMBER
    .captures(line)
    .ok_or(err(ParseErrorType::Start, "Position not found"))?;
  let position = capture_position[0]
    .replace(':', "")
    .parse::<u8>()
    .map_err(|e| err(ParseErrorType::Start, e))?;
  let capture_name = re::AFTER_COLON_P
    .captures(line)
    .ok_or(err(ParseErrorType::Start, "Name not found"))?;
  let name = String::from(capture_name[0].replace([':', '('], "").trim());
  let capture_bank = re::MONEY_BRACED
    .captures(line)
    .ok_or(err(ParseErrorType::Start, "Bank not found"))?;
  let bank = capture_bank[0]
    .replace(['$', '('], "")
    .trim()
    .parse::<f32>()
    .map_err(|e| err_msg(ParseErrorType::Start, e, &capture_bank[0]))?;
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
      hand.small_blind = extract_blind(hand, line)?;
    } else if line.contains("posts big blind") {
      hand.big_blind = extract_blind(hand, line)?;
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

fn preflop(hand: &mut HandDetail, lines: &mut Lines) -> Result<bool, ParseError> {
  let line = lines.next().unwrap();
  let player_capture = re::DEALT
    .captures(line)
    .ok_or(err(ParseErrorType::Preflop, "capture player"))?;
  let mut player_chars = player_capture[0].chars();
  player_chars
    .next()
    .ok_or(err(ParseErrorType::Preflop, "player next"))?;
  player_chars
    .next()
    .ok_or(err(ParseErrorType::Preflop, "player next"))?;
  player_chars
    .next_back()
    .ok_or(err(ParseErrorType::Preflop, "player next"))?;

  let player = hand
    .get_player(player_chars.as_str().trim())
    .map_err(|e| err(ParseErrorType::Preflop, e))?;

  // WARN: suppose the user can't have '['
  let cards_capture = re::BRACKET
    .captures(line)
    .ok_or(err(ParseErrorType::Preflop, "capture cards"))?;
  let binding = cards_capture[0].replace(['[', ']'], "");
  let mut cards = binding.split_whitespace();
  let card1 = cards.next().ok_or(err(ParseErrorType::Preflop, "card 1"))?;
  let card2 = cards.next().ok_or(err(ParseErrorType::Preflop, "card 2"))?;

  hand.players_card[player.position as usize - 1] = Some([card1.to_string(), card2.to_string()]);

  for line in lines {
    if line.starts_with("*** SUMMARY ***") {
      return Ok(false);
    }
    if line.starts_with("*** FLOP ***") {
      // 500us
      // TODO : check error of regex, fuck it for now
      let capture_card = &re::BRACKET.captures(line).unwrap();
      let binding = capture_card[0].replace(['[', ']'], "");
      let mut cards = binding.split_whitespace();
      let card1 = cards
        .next()
        .ok_or(err(ParseErrorType::Preflop, "card 1 board"))?;
      let card2 = cards
        .next()
        .ok_or(err(ParseErrorType::Preflop, "card 2 board"))?;
      let card3 = cards
        .next()
        .ok_or(err(ParseErrorType::Preflop, "card 3 board"))?;
      hand.flop_card = Some([card1.to_string(), card2.to_string(), card3.to_string()]);
      return Ok(true);
    }

    if Action::is_action(line) {
      // 4 ms
      hand.preflop.push(
        Action::get_action(hand, line)
          .unwrap_or_else(|e| panic!("Error in preflop action parsing : {}", e)),
      );
    } else if line.contains("collected") {
      // 1ms
      hand.end = End::extract_end(hand, line)?;
    }
  }
  Ok(false)
}

fn flop(hand: &mut HandDetail, lines: &mut Lines) -> Result<bool, ParseError> {
  for line in lines {
    if line.starts_with("*** SUMMARY ***") {
      return Ok(false);
    }
    if line.starts_with("*** TURN ***") {
      let cards_re = &re::BRACKET;
      let cards_capture = cards_re
        .captures(line)
        .ok_or(err(ParseErrorType::Preflop, "capture cards"))?;
      let card = cards_capture[0].replace(['[', ']'], "").to_string();
      hand.turn_card = Some(card);
      return Ok(true);
    }

    if Action::is_action(line) {
      hand.flop.push(
        Action::get_action(hand, line)
          .unwrap_or_else(|e| panic!("Error in flop action parsing : {}", e)),
      );
    } else if line.contains("collected") {
      hand.end = End::extract_end(hand, line)?;
    }
  }
  Ok(false)
}

fn turn(hand: &mut HandDetail, lines: &mut Lines) -> Result<bool, ParseError> {
  for line in lines {
    if line.starts_with("*** SUMMARY ***") {
      return Ok(false);
    }
    if line.starts_with("*** RIVER ***") {
      let mut capture_card = re::BRACKET.captures_iter(line);
      capture_card.next();
      hand.river_card = Some(
        capture_card
          .next()
          .ok_or(err(ParseErrorType::Turn, "error in getting turn card"))?[0]
          .to_string()
          .replace(['[', ']'], ""),
      );
      return Ok(true);
    }
    if Action::is_action(line) {
      hand.turn.push(
        Action::get_action(hand, line)
          .unwrap_or_else(|e| panic!("Error in turn action parsing : {}", e)),
      );
    } else if line.contains("collected") {
      hand.end = End::extract_end(hand, line)?; // FIXME : error
    }
  }
  Ok(false)
}

fn river(hand: &mut HandDetail, lines: &mut Lines) -> Result<bool, ParseError> {
  for line in lines {
    if line.starts_with("*** SUMMARY ***") {
      return Ok(false);
    }
    if line.starts_with("*** SHOW DOWN ***") {
      return Ok(true);
    }
    if Action::is_action(line) {
      hand.river.push(
        Action::get_action(hand, line)
          .unwrap_or_else(|e| panic! {"Error in river action parsing : {}", e}),
      );
    } else if line.contains("collected") {
      hand.end = End::extract_end(hand, line)?;
    }
  }
  Ok(false)
}

fn showdown(hand: &mut HandDetail, lines: &mut Lines) -> Result<(), ParseError> {
  for line in lines {
    if line.starts_with("*** SUMMARY ***") {
      return Ok(());
    }
    // NOTE: ignore muck
    if line.contains("shows") {
      let player_capture = re::BEFORE_COLON
        .captures(line)
        .ok_or(err(ParseErrorType::Showdown, "player not found"))?;

      // NOTE : suppose the username can't contains ':'
      let player_name = player_capture[0].replace([':'], "");
      let player = hand
        .get_player(&player_name)
        .map_err(|e| err(ParseErrorType::Showdown, e))?;

      let capture_card = re::BRACKET
        .captures(line)
        .ok_or(err(ParseErrorType::Showdown, "cards not found"))?;
      let cards_str = capture_card[0].replace(['[', ']'], "");
      let mut cards = cards_str.split_whitespace();
      hand.players_card[player.position as usize - 1] = Some([
        cards.next().unwrap().to_string(),
        cards.next().unwrap().to_string(),
      ]);
    } else if line.contains("collected") {
      hand.end = End::extract_end(hand, line)?;
    }
  }
  Ok(())
}

#[derive(Default, Debug, PartialEq)]
pub struct End {
  pub pot: f32,
  pub winner: Player,
}

impl End {
  fn extract_end(hand: &HandDetail, line: &str) -> Result<Self, ParseError> {
    let player_capture = &re::BEFORE_COLLECTED.captures(line).unwrap();
    let mut words = player_capture[0].split_whitespace();
    words.next_back().unwrap();
    let player_name = words.collect::<Vec<&str>>().join(" ");
    let winner = hand.get_player(&player_name)?;

    let pot_capture = re::AFTER_COLLECTED.captures(line).unwrap();
    let pot_str = pot_capture[0].split_whitespace().nth(1).ok_or(err(
      ParseErrorType::Unknown("End".to_string()),
      "error in pot reading",
    ))?;
    let pot = pot_str
      .parse::<f32>()
      .map_err(|e| err(ParseErrorType::Unknown("End".to_string()), e))?;
    Ok(End { pot, winner })
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

  // need a special treatement
  fn get_uncalled(hand: &HandDetail, line: &str) -> Result<Self, ParseError> {
    let capture = re::MONEY_BRACED.captures(line).ok_or(err(
      ParseErrorType::Unknown("get uncalled".to_string()),
      "Can't find amount in uncalled",
    ))?;
    let amount = capture[0]
      .replace(['(', ')'], "")
      .parse::<f32>()
      .map_err(|e| err(ParseErrorType::Unknown("get uncalled".to_string()), e))?;
    let player = line
      .split_whitespace()
      .skip(5)
      .collect::<Vec<&str>>()
      .join(" ");
    Ok(Action::UncalledBet(hand.get_player(&player)?, amount))
  }

  fn get_action(hand: &HandDetail, line: &str) -> Result<Self, ParseError> {
    let s = Instant::now();
    if line.starts_with("Uncalled bet") {
      return Action::get_uncalled(hand, line);
    }

    let capture_position = re::BEFORE_COLON.captures(line).ok_or(err(
      ParseErrorType::Unknown("get action".to_string()),
      "player not found",
    ))?;
    let player = capture_position[0].replace([':'], "");

    let capture_after = re::AFTER_COLON.captures(line).ok_or(err(
      ParseErrorType::Unknown("get action".to_string()),
      "capture after",
    ))?;
    let binding = capture_after[0].replace([':'], "");
    let line = &binding.trim();

    let capture_action = re::WORD.captures(line).ok_or(err(
      ParseErrorType::Unknown("get action".to_string()),
      "capture action",
    ))?;
    let binding = capture_action[0].replace([':', '$'], "");
    let action = binding.trim();

    let captures_amount = re::MONEY.captures_iter(line);
    let mut amounts = captures_amount
      .map(|a| a[0].replace(['$'], ""))
      .map(|a| a.parse::<f32>());

    match action {
      "calls" => Ok(Action::Call(
        hand.get_player(&player)?,
        amounts
          .next()
          .ok_or(err(
            ParseErrorType::Unknown("get action".to_string()),
            "Amount 1 not found",
          ))?
          .map_err(|e| err(ParseErrorType::Unknown("get action".to_string()), e))?,
        line.contains("all-in"),
      )),
      "bets" => Ok(Action::Bet(
        hand.get_player(&player)?,
        amounts
          .next()
          .ok_or(err(
            ParseErrorType::Unknown("get action".to_string()),
            "Amount 1 not found",
          ))?
          .map_err(|e| err(ParseErrorType::Unknown("get action".to_string()), e))?,
        line.contains("all-in"),
      )),
      "raises" => Ok(Action::Raise(
        hand.get_player(&player)?,
        amounts
          .next()
          .ok_or(err(
            ParseErrorType::Unknown("get action".to_string()),
            "Amount 1 not found",
          ))?
          .map_err(|e| err(ParseErrorType::Unknown("get action".to_string()), e))?,
        amounts
          .next()
          .ok_or(err(
            ParseErrorType::Unknown("get action".to_string()),
            "Amount 2 not found",
          ))?
          .map_err(|e| err(ParseErrorType::Unknown("get action".to_string()), e))?,
        line.contains("all-in"),
      )),
      "checks" => Ok(Action::Check(hand.get_player(&player)?)),
      "folds" => Ok(Action::Fold(hand.get_player(&player)?)),
      "leaves" => Ok(Action::Leave(hand.get_player(&player)?)),
      // first is Uncalled
      // TODO : add Uncalled action
      "bet" => Ok(Action::UncalledBet(
        hand.get_player(&player)?,
        amounts
          .next()
          .ok_or(err(
            ParseErrorType::Unknown("get action".to_string()),
            "Amount 1 not found",
          ))?
          .map_err(|e| err(ParseErrorType::Unknown("get action".to_string()), e))?,
      )),
      _ => Err(err(
        ParseErrorType::Unknown("get action".to_string()),
        "unknown action",
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
