use chrono::{DateTime, FixedOffset, NaiveDateTime};
use std::str::Lines;

use crate::parse::re;
use crate::parse::{Blind, HandDetail, ParseError, ParseErrorType, Player};

fn extract_id(line: &str) -> Result<i64, ParseError> {
  let capture_id = re::TABLE_ID
    .captures(line)
    .ok_or(ParseError::err(ParseErrorType::Start, "Id regex failed"))?;
  let mut chars = capture_id[0].chars();
  chars.next();
  let content = chars.as_str();
  content
    .parse::<i64>()
    .map_err(|e| ParseError::err_msg(ParseErrorType::Start, e, content))
}

fn extract_date(line: &str) -> Result<DateTime<FixedOffset>, ParseError> {
  let capture_date = re::BRACKET
    .captures(line)
    .ok_or(ParseError::err(ParseErrorType::Start, "Date regex failed"))?;
  let date_string = capture_date[0].to_string();

  let date = NaiveDateTime::parse_from_str(&date_string, "[%Y/%m/%d %H:%M:%S ET]")
    .map_err(|e| ParseError::err(ParseErrorType::Start, e))?;

  let offset = FixedOffset::east_opt(5 * 3600)
    .ok_or(ParseError::err(ParseErrorType::Start, "Offset failed"))?;

  Ok(DateTime::<FixedOffset>::from_naive_utc_and_offset(
    date, offset,
  ))
}

fn extract_limits(line: &str) -> Result<(f32, f32), ParseError> {
  // replace by simple number match and take 2nd and 3rd
  let capture_limites = re::LIMIT
    .captures(line)
    .ok_or(ParseError::err(ParseErrorType::Start, "Limit not found"))?;
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
      .map_err(|e| ParseError::err(ParseErrorType::Start, e))?,
    big_limit_str
      .replace('$', "")
      .replace(" USD", "")
      .parse::<f32>()
      .map_err(|e| ParseError::err(ParseErrorType::Start, e))?,
  ))
}

fn extract_table_name(line: &str) -> Result<String, ParseError> {
  let capture_table_name = re::TABLE_NAME.captures(line).ok_or(ParseError::err(
    ParseErrorType::Start,
    "Table name regex failed",
  ))?;
  let mut chars = capture_table_name[0].chars();
  chars.next();
  chars.next_back();
  Ok(chars.as_str().to_string())
}

fn extract_button_position(line: &str) -> Result<u8, ParseError> {
  let capture_button_position = re::TABLE_ID.captures(line).ok_or(ParseError::err(
    ParseErrorType::Start,
    "Button position regex failed",
  ))?;
  let mut chars = capture_button_position[0].chars();
  chars.next();
  chars
    .as_str()
    .parse::<u8>()
    .map_err(|e| ParseError::err(ParseErrorType::Start, e))
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
    .map_err(|e| ParseError::err(ParseErrorType::Start, e))
}

fn extract_blind(hand: &HandDetail, line: &str) -> Result<Blind, ParseError> {
  let capture_player = re::BEFORE_COLON
    .captures(line)
    .ok_or(ParseError::err(ParseErrorType::Start, "extracting blind"))?;
  let player = hand
    .get_player(&capture_player[0].replace([':'], ""))
    .map_err(|e| ParseError::err(ParseErrorType::Start, e))?;
  let after_line = re::AFTER_COLON.captures(line).ok_or(ParseError::err(
    ParseErrorType::Start,
    "unable to get the line after colon",
  ))?;
  let line = after_line[0].to_string();
  let capture = re::MONEY.captures(&line).ok_or(ParseError::err(
    ParseErrorType::Start,
    "Can't find amount in blind",
  ))?;
  let amount = capture[0]
    .replace(['('], "")
    .parse::<f32>()
    .map_err(|e| ParseError::err(ParseErrorType::Start, e))?;
  Ok(Blind { player, amount })
}

fn extract_seat(line: &str) -> Result<Player, ParseError> {
  let capture_position = re::NUMBER
    .captures(line)
    .ok_or(ParseError::err(ParseErrorType::Start, "Position not found"))?;
  let position = capture_position[0]
    .replace(':', "")
    .parse::<u8>()
    .map_err(|e| ParseError::err(ParseErrorType::Start, e))?;
  let capture_name = re::AFTER_COLON_P
    .captures(line)
    .ok_or(ParseError::err(ParseErrorType::Start, "Name not found"))?;
  let name = String::from(capture_name[0].replace([':', '('], "").trim());
  let capture_bank = re::MONEY_BRACED
    .captures(line)
    .ok_or(ParseError::err(ParseErrorType::Start, "Bank not found"))?;
  let bank = capture_bank[0]
    .replace(['$', '('], "")
    .trim()
    .parse::<f32>()
    .map_err(|e| ParseError::err_msg(ParseErrorType::Start, e, &capture_bank[0]))?;
  Ok(Player {
    name,
    position,
    bank,
  })
}

pub fn start(hand: &mut HandDetail, lines: &mut Lines) -> Result<(), ParseError> {
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
