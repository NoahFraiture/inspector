use std::fmt;

#[derive(Debug, Clone)]
pub enum ParseErrorType {
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

impl ParseError {
  pub fn err(t: ParseErrorType, e: impl std::string::ToString) -> Self {
    ParseError {
      msg: format!("Error : {}", e.to_string()),
      t,
    }
  }
  pub fn err_msg(t: ParseErrorType, e: impl std::string::ToString, msg: &str) -> Self {
    ParseError {
      t,

      // this way we get after the inner error
      msg: format!("Message: {}\nError: {}", msg, e.to_string()),
    }
  }
}
