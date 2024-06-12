use std::fs;
use std::str::Lines;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

mod error;
mod hand;
mod re;
mod start;

use error::{ParseError, ParseErrorType};
pub use hand::{Action, Blind, End, HandDetail, Player};

use crate::db::schema::action::amount1;

trait Dollar {
  fn round_two_digit(self) -> f32;
}

impl Dollar for f32 {
  fn round_two_digit(self) -> f32 {
    (self * 100.).round() / 100.
  }
}

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

    start::start(&mut hand, &mut lines)?;
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
    Err(ParseError::err(
      ParseErrorType::Unknown(String::from("get player")),
      name,
    ))
  }
}

// ========================================
// === PARSE DIFFERENT PART OF THE FILE ===
// ========================================

fn preflop(hand: &mut HandDetail, lines: &mut Lines) -> Result<bool, ParseError> {
  let line = lines.next().unwrap();
  let player_capture = re::DEALT
    .captures(line)
    .ok_or(ParseError::err(ParseErrorType::Preflop, "capture player"))?;
  let mut player_chars = player_capture[0].chars();
  player_chars
    .next()
    .ok_or(ParseError::err(ParseErrorType::Preflop, "player next"))?;
  player_chars
    .next()
    .ok_or(ParseError::err(ParseErrorType::Preflop, "player next"))?;
  player_chars
    .next_back()
    .ok_or(ParseError::err(ParseErrorType::Preflop, "player next"))?;

  let player = hand
    .get_player(player_chars.as_str().trim())
    .map_err(|e| ParseError::err(ParseErrorType::Preflop, e))?;

  // WARN: suppose the user can't have '['
  let cards_capture = re::BRACKET
    .captures(line)
    .ok_or(ParseError::err(ParseErrorType::Preflop, "capture cards"))?;
  let binding = cards_capture[0].replace(['[', ']'], "");
  let mut cards = binding.split_whitespace();
  let card1 = cards
    .next()
    .ok_or(ParseError::err(ParseErrorType::Preflop, "card 1"))?;
  let card2 = cards
    .next()
    .ok_or(ParseError::err(ParseErrorType::Preflop, "card 2"))?;

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
        .ok_or(ParseError::err(ParseErrorType::Preflop, "card 1 board"))?;
      let card2 = cards
        .next()
        .ok_or(ParseError::err(ParseErrorType::Preflop, "card 2 board"))?;
      let card3 = cards
        .next()
        .ok_or(ParseError::err(ParseErrorType::Preflop, "card 3 board"))?;
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
      let mut capture_card = re::BRACKET.captures_iter(line);
      capture_card.next();
      hand.turn_card = Some(
        capture_card.next().ok_or(ParseError::err(
          ParseErrorType::Turn,
          "error in getting turn card",
        ))?[0]
          .to_string()
          .replace(['[', ']'], ""),
      );
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
        capture_card.next().ok_or(ParseError::err(
          ParseErrorType::Turn,
          "error in getting turn card",
        ))?[0]
          .to_string()
          .replace(['[', ']'], ""),
      );
      return Ok(true);
    }
    if Action::is_action(line) {
      hand.turn.push(Action::get_action(hand, line).map_err(|e| {
        ParseError::err_msg(
          ParseErrorType::River,
          e,
          &format!("action in line {}", line),
        )
      })?);
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
      let player_capture = re::BEFORE_COLON.captures(line).ok_or(ParseError::err(
        ParseErrorType::Showdown,
        "player not found",
      ))?;

      // NOTE : suppose the username can't contains ':'
      let player_name = player_capture[0].replace([':'], "");
      let player = hand
        .get_player(&player_name)
        .map_err(|e| ParseError::err(ParseErrorType::Showdown, e))?;

      let capture_card = re::BRACKET
        .captures(line)
        .ok_or(ParseError::err(ParseErrorType::Showdown, "cards not found"))?;
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

impl End {
  fn extract_end(hand: &HandDetail, line: &str) -> Result<Self, ParseError> {
    let player_capture = &re::BEFORE_COLLECTED.captures(line).unwrap();
    let mut words = player_capture[0].split_whitespace();
    words.next_back().unwrap();
    let player_name = words.collect::<Vec<&str>>().join(" ");
    let winner = hand.get_player(&player_name)?;

    let pot_capture = re::AFTER_COLLECTED.captures(line).ok_or(ParseError::err(
      ParseErrorType::Unknown("End".to_string()),
      "pot capture",
    ))?;
    let binding = pot_capture[0].replace(['$'], "");
    let pot_str = binding.split_whitespace().nth(1).ok_or(ParseError::err(
      ParseErrorType::Unknown("End".to_string()),
      "error in pot reading",
    ))?;
    let pot = pot_str
      .parse::<f32>()
      .map(|p| p.round_two_digit())
      .map_err(|e| {
        ParseError::err_msg(
          ParseErrorType::Unknown("End".to_string()),
          e,
          &format!("pot {}", pot_str),
        )
      })?;
    Ok(End { pot, winner })
  }
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
    let capture = re::MONEY_BRACED.captures(line).ok_or(ParseError::err(
      ParseErrorType::Unknown("get uncalled".to_string()),
      "Can't find amount in uncalled",
    ))?;
    let amount = capture[0]
      .replace(['(', ')', '$'], "")
      .parse::<f32>()
      .map(|p| p.round_two_digit())
      .map_err(|e| {
        ParseError::err_msg(
          ParseErrorType::Unknown("get uncalled".to_string()),
          e,
          &format!("amount : '{}'", &capture[0]),
        )
      })?;
    let player = line
      .split_whitespace()
      .skip(5)
      .collect::<Vec<&str>>()
      .join(" ");
    Ok(Action::UncalledBet(hand.get_player(&player)?, amount))
  }

  fn get_action(hand: &HandDetail, line: &str) -> Result<Self, ParseError> {
    if line.starts_with("Uncalled bet") {
      return Action::get_uncalled(hand, line);
    }

    let capture_position = re::BEFORE_COLON.captures(line).ok_or(ParseError::err(
      ParseErrorType::Unknown("get action".to_string()),
      "player not found",
    ))?;
    let player = capture_position[0].replace([':'], "");

    let capture_after = re::AFTER_COLON.captures(line).ok_or(ParseError::err(
      ParseErrorType::Unknown("get action".to_string()),
      "capture after",
    ))?;
    let binding = capture_after[0].replace([':'], "");
    let line = &binding.trim();

    let capture_action = re::WORD.captures(line).ok_or(ParseError::err(
      ParseErrorType::Unknown("get action".to_string()),
      "capture action",
    ))?;
    let binding = capture_action[0].replace([':', '$'], "");
    let action = binding.trim();

    let captures_amount = re::MONEY.captures_iter(line);
    // FIXME shit
    let amounts = {
      let mut vec = Vec::new();
      for cap in captures_amount {
        let x = cap[0]
          .replace(['$'], "")
          .parse::<f32>()
          .map(|p| p.round_two_digit())
          .map_err(|e| ParseError::err(ParseErrorType::Unknown("get action".to_string()), e))?;
        vec.push(x);
      }
      vec
    };

    match action {
      "calls" => Ok(Action::Call(
        hand.get_player(&player)?,
        *amounts.first().unwrap(),
        line.contains("all-in"),
      )),
      "bets" => Ok(Action::Bet(
        hand.get_player(&player)?,
        *amounts.first().unwrap(),
        line.contains("all-in"),
      )),
      "raises" => Ok(Action::Raise(
        hand.get_player(&player)?,
        *amounts.first().unwrap(),
        *amounts.get(1).unwrap(),
        line.contains("all-in"),
      )),
      "checks" => Ok(Action::Check(hand.get_player(&player)?)),
      "folds" => Ok(Action::Fold(hand.get_player(&player)?)),
      "leaves" => Ok(Action::Leave(hand.get_player(&player)?)),
      // first is Uncalled
      // TODO : add Uncalled action
      "bet" => Ok(Action::UncalledBet(
        hand.get_player(&player)?,
        *amounts.first().unwrap(),
      )),
      _ => Err(ParseError::err(
        ParseErrorType::Unknown("get action".to_string()),
        "unknown action",
      )),
    }
  }
}
