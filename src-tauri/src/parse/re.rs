use regex::Regex;

lazy_static! {
  pub static ref BEFORE_COLON: Regex = Regex::new(r".*:").unwrap();
  pub static ref AFTER_COLON: Regex = Regex::new(r":.*").unwrap();
  pub static ref BRACKET: Regex = Regex::new(r"\[.+?\]").unwrap();
  pub static ref TABLE_NAME: Regex = Regex::new(r"'([^']*)'").unwrap();
  pub static ref MONEY_BRACED: Regex = Regex::new(r"\(\$?(\d+\.)?\d+").unwrap();
  pub static ref MONEY: Regex = Regex::new(r"\$?(\d+\.)?\d+").unwrap();
  pub static ref LIMIT: Regex = Regex::new(r"\(\$?(\d+\.)?\d+\/\$?(\d+\.)?\d+( USD)?\)").unwrap();
  pub static ref TABLE_ID: Regex = Regex::new(r"#(\d+)").unwrap();
  pub static ref TABLE_SIZE: Regex = Regex::new(r"(\d+)-max").unwrap();
  pub static ref DEALT: Regex = Regex::new(r"to .*\[").unwrap();
  pub static ref WORD: Regex = Regex::new(r"\w+").unwrap();
  pub static ref BEFORE_COLLECTED: Regex = Regex::new(r".* collected").unwrap();
  pub static ref AFTER_COLLECTED: Regex = Regex::new(r"collected \$?(\d+\.)?\d+").unwrap();
  pub static ref NUMBER: Regex = Regex::new(r"[0-9]").unwrap();
  pub static ref AFTER_COLON_P: Regex = Regex::new(r": .* \(").unwrap();
}
