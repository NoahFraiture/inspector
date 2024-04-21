use super::extractor::parse::{Action, Hand};

// All these stats can be split by position and moment
// TODO : remove these
#[derive(Default, Debug)]
struct PlayerParticipation {
  name: String,
  vpip: bool,              // Volontary put in the pot. Without counting big-blind check
  pfr: bool,               // preflop raise. Count raise (3-bet and more)
  af: [u32; 3],            // agression factor, (bet + raise) / call
  can_pre_3bet: bool,      // Tells if the next value must be taken in account
  pre_3bet: bool,          // 3bet preflop. Only when possible
  can_fold_pre_3bet: bool, // Tells if the next alue must be taken in account
  fold_pre_3bet: bool,     // fold to 3bet preflop
  can_cbet: bool,          // Tells if the next value must be taken in account
  cbet: bool, // continuation bet flop. The player must have open and be the first to raise
  fold_cbet: bool, // fold to cbet flop
  squeeze: bool, // raise after preflop raise and at least a player has call
}

impl PlayerParticipation {
  // TODO : change the way we find 'can_...', functions should return true, false, can't
  fn new(hand: &Hand, name: &str) -> Self {
    // compute vpip
    let _vpip = vpip_find(hand, name, Moment::Preflop);
    let _pfr = pfr_find(hand, name);
    let _af = af_find(hand, name);

    let pre_3bet = pre_3bet_find(hand, name);
    let _can_pre_3bet = !matches!(pre_3bet, Bool::Impossible);
    let _pre_3bet = !matches!(pre_3bet, Bool::True);

    let fold_pre_3bet = fold_pre_3bet_find(hand, name);
    let _can_fold_pre_3bet = !matches!(fold_pre_3bet, Bool::Impossible);
    let _fold_pre_3bet = matches!(fold_pre_3bet, Bool::True);

    let cbet = cbet_find(hand, name);
    let _can_cbet = !matches!(cbet, Bool::Impossible);
    let _cbet = matches!(cbet, Bool::True);

    let fold_cbet = fold_cbet_find(hand, name);
    let _can_fold_cbet = !matches!(fold_cbet, Bool::Impossible);
    let _fold_cbet = matches!(fold_cbet, Bool::True);

    let squeeze = squeeze_find(hand, name);
    let _can_squeeze = !matches!(squeeze, Bool::Impossible);
    let _squeeze = matches!(squeeze, Bool::True);

    Self {
      ..Default::default()
    }
  }
}

fn vpip_find(hand: &Hand, name: &str, moment: Moment) -> bool {
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
      Moment::Preflop => vpip_find(hand, name, Moment::Flop),
      Moment::Flop => vpip_find(hand, name, Moment::Turn),
      Moment::Turn => vpip_find(hand, name, Moment::River),
      Moment::River => false,
    };
  }
  false
}

fn pfr_find(hand: &Hand, name: &str) -> bool {
  for action in &hand.preflop {
    if let Action::Raise(player, _, _, _) = action {
      if name == player.name {
        return true;
      }
    }
  }
  false
}

// return number of action [call, bet, raise]
fn af_find(hand: &Hand, name: &str) -> [u32; 3] {
  let mut call = 0;
  let mut bet = 0;
  let mut raise = 0;
  // TODO : add other than preflop
  for action in []
    .iter()
    .chain(hand.preflop.iter())
    .chain(hand.flop.iter())
    .chain(hand.turn.iter())
    .chain(hand.river.iter())
  {
    match action {
      Action::Call(player, _, _) => {
        if name == player.name {
          call += 1;
        }
      }
      Action::Bet(player, _, _) => {
        if name == player.name {
          bet += 1;
        }
      }
      Action::Raise(player, _, _, _) => {
        if name == player.name {
          raise += 1;
        }
      }
      _ => {}
    }
  }
  [call, bet, raise]
}

fn pre_3bet_find(hand: &Hand, name: &str) -> Bool {
  let mut raise_before = 0;
  for action in &hand.preflop {
    match action {
      Action::Raise(player, _, _, _) => {
        if name == player.name {
          if raise_before == 1 {
            return Bool::True;
          } else {
            return Bool::Impossible;
          }
        } else {
          raise_before += 1
        }
      }
      Action::Call(player, _, _) | Action::Fold(player) => {
        if player.name == name && raise_before == 1 {
          return Bool::False;
        }
      }
      _ => {}
    }
  }
  Bool::Impossible
}

fn fold_pre_3bet_find(hand: &Hand, name: &str) -> Bool {
  let mut raised = false; // opponent 3 bet
  for action in &hand.preflop {
    match action {
      Action::Bet(player, _, _) => {
        if player.name != name {
          return Bool::Impossible;
        }
      }
      // At this point we know that 'name' has opened
      // If the player to play is 'name', 'raised' must be true
      Action::Raise(player, _, _, _) => {
        if name == player.name {
          return Bool::False;
        }
        if !raised {
          raised = true;
        } else {
          return Bool::Impossible;
        }
      }
      Action::Fold(player) => {
        if name == player.name {
          if raised {
            return Bool::True;
          } else {
            return Bool::Impossible;
          }
        }
      }
      Action::Call(player, _, _) => {
        if player.name == name && raised {
          return Bool::False;
        }
      }
      _ => {}
    }
  }
  // every one checked
  Bool::Impossible
}

fn cbet_find(hand: &Hand, name: &str) -> Bool {
  let mut open = false;
  for action in &hand.preflop {
    match action {
      // Raise is when player is big blind, bet on any other position
      Action::Raise(player, _, _, _) | Action::Bet(player, _, _) => {
        open = name == player.name;
      }
      _ => {}
    }
  }
  if !open {
    return Bool::Impossible;
  }

  for action in &hand.flop {
    match action {
      Action::Check(player) => {
        if name == player.name {
          return Bool::False;
        }
      }
      Action::Bet(player, _, _) => {
        if name != player.name {
          return Bool::Impossible;
        }
        return Bool::True;
      }
      _ => {}
    }
  }
  panic!("we should not be here")
}

fn fold_cbet_find(hand: &Hand, name: &str) -> Bool {
  let mut adversary: String = String::new();
  for action in &hand.preflop {
    match action {
      Action::Raise(player, _, _, _) | Action::Bet(player, _, _) => {
        // if someone raised preflop after open, we must update the cbetter
        adversary = String::from(&player.name);
      }
      _ => {}
    }
  }

  // nobody bet or the last who raise is the player himself
  if adversary == name || adversary.is_empty() {
    return Bool::Impossible;
  }

  // player who bet is the last raiser
  // if someone bet before your turn and the bet, this doesn't count anymore
  // NOTE: in this case we only consider the case where opener can bet and nobody has bet before
  for action in &hand.flop {
    match action {
      Action::Bet(player, _, _) => {
        if player.name != adversary {
          return Bool::Impossible;
        }
      }

      // if any other player raise, this doesn't count anymore
      // If we reach here, the opener has cbet already
      Action::Raise(player, _, _, _) | Action::Call(player, _, _) => {
        if player.name == name {
          return Bool::False;
        }
      }

      Action::Fold(player) => {
        if player.name == name {
          return Bool::True;
        }
      }
      _ => {}
    }
  }
  // every one has checked
  Bool::Impossible
}

fn squeeze_find(hand: &Hand, name: &str) -> Bool {
  let mut caller = false;
  let mut open = false;
  for action in &hand.preflop {
    match action {
      Action::Raise(player, _, _, _) => {
        if player.name != name {
          if open {
            return Bool::Impossible;
          }
          open = true;
        } else if open && caller {
          return Bool::True;
        } else {
          return Bool::Impossible;
        }
      }
      Action::Call(player, _, _) => {
        if player.name == name && open {
          if caller {
            return Bool::False;
          } else {
            return Bool::Impossible;
          }
        }
        if player.name != name && open {
          caller = true;
        }
      }
      Action::Check(player) | Action::Fold(player) => {
        if player.name == name && open {
          if caller {
            return Bool::False;
          } else {
            return Bool::Impossible;
          }
        }
      }
      _ => {}
    }
  }
  panic!("Should not reach here");
}

enum Bool {
  True,
  False,
  Impossible,
}

enum Moment {
  Preflop,
  Flop,
  Turn,
  River,
}
