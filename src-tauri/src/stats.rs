use super::extractor::parse::{Action, Hand, Player};

// All these stats can be split by position and moment
// TODO : remove these
#[derive(Default, Debug)]
struct PlayerParticipation {
  name: String,
  vpip: bool,          // Volontary put in the pot. Without counting big-blind check
  pfr: bool,           // preflop raise. Count raise (3-bet and more)
  af: bool,            // agression factor, (bet + raise) / call
  pre_3bet: bool,      // 3bet preflop. Only when possible
  fold_pre_3bet: bool, // fold to 3bet preflop
  cbet: bool,          // continuation bet flop
  fold_cbet: bool,     // fold to cbet flop
  squeeze: bool,       // raise after preflop raise and at least a player has call
}

impl PlayerParticipation {
  fn new(hand: &Hand, name: &str) -> Self {
    // compute vpip
    let vpip = did_pay(hand, name, Moment::Preflop);

    // compute pfr
    // reuse same function but different

    Self {
      ..Default::default()
    }
  }
}

fn did_pay(hand: &Hand, name: &str, moment: Moment) -> bool {
  let is_big_blind = hand.big_blind.player.name == name;
  let mut next = false;
  let actions = match moment {
    Moment::Preflop => &hand.preflop,
    Moment::Flop => &hand.flop,
    Moment::Turn => &hand.turn,
    Moment::River => &hand.river,
  };
  for action in actions {
    match action {
      Action::Call(player, _, _) | Action::Bet(player, _, _) | Action::Raise(player, _, _, _) => {
        if name == player.name {
          return true;
        }
      }
      Action::Check(player) => {
        if is_big_blind && name == player.name {
          next = true;
        }
      }
      _ => {}
    };
  }
  if next {
    return match moment {
      Moment::Preflop => did_pay(hand, name, Moment::Flop),
      Moment::Flop => did_pay(hand, name, Moment::Turn),
      Moment::Turn => did_pay(hand, name, Moment::River),
      Moment::River => false,
    };
  }
  false
}

enum Moment {
  Preflop,
  Flop,
  Turn,
  River,
}
