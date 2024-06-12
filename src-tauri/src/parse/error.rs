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
      ParseErrorType::Start => write!(f, "Start error:\n{}", self.msg),
      ParseErrorType::Preflop => write!(f, "Preflop error:\n{}", self.msg),
      ParseErrorType::Flop => write!(f, "Flop error:\n{}", self.msg),
      ParseErrorType::Turn => write!(f, "Turn error:\n{}", self.msg),
      ParseErrorType::River => write!(f, "River error:\n{}", self.msg),
      ParseErrorType::Showdown => write!(f, "Showdown error:\n{}", self.msg),
      ParseErrorType::Unknown(m) => write!(f, "Unknown\n{} error:\n{}", m, self.msg),
    }
  }
}

impl ParseError {
  pub fn err(t: ParseErrorType, e: impl std::string::ToString) -> Self {
    ParseError {
      msg: format!("Error :\n{}\n", e.to_string()),
      t,
    }
  }
  pub fn err_msg(t: ParseErrorType, e: impl std::string::ToString, msg: &str) -> Self {
    ParseError {
      t,

      // this way we get after the inner error
      msg: format!("Message:\n{}\nError:\n{}\n", msg, e.to_string()),
    }
  }
}
