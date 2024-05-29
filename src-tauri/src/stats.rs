use crate::parse::Action;
use crate::Hand;

#[derive(Default)]
pub struct Player {
  // There's a choice to do : lot of casting, or lack of precision by staying in f64. I choose f64 since it must be
  // faster
  name: String,
  vpip: f64,
  pfr: f64,
  af: f64,
  pre_3bet: f64,
  fold_pre_3bet: f64,
  cbet: f64,
  fold_cbet: f64,
  squeeze: f64,

  // number of hands used to compute stats. Usefull to easily add new hand without recomputing all hands
  nb_hand: f64,
  nb_can_pre_3bet: f64,
  nb_can_fold_pre_3bet: f64,
  nb_can_cbet: f64,
  nb_can_fold_cbet: f64,
  nb_can_squeeze: f64,
  nb_call: f64,
  nb_bet: f64,
  nb_raise: f64,
}

impl Player {
  fn new(name: &str) -> Self {
    let mut s = Self {
      name: String::from(name),
      ..Default::default()
    };
    // update value to -1 when divided is 0
    s.add(vec![]);
    s
  }

  fn add(&mut self, hands: Vec<Hand>) {
    let mut nb_vpip = self.vpip * self.nb_hand;
    let mut nb_pfr = self.pfr * self.nb_hand;

    let mut nb_pre_3bet = self.pre_3bet * self.nb_can_pre_3bet;
    let mut nb_fold_pre_3bet = self.fold_pre_3bet * self.nb_can_fold_pre_3bet;
    let mut nb_cbet = self.cbet * self.nb_can_cbet;
    let mut nb_fold_cbet = self.fold_cbet * self.nb_can_fold_cbet;
    let mut nb_squeeze = self.squeeze * self.nb_can_squeeze;

    for hand in hands {
      let participation = PlayerParticipation::new(&hand, &self.name);
      if participation.vpip {
        nb_vpip += 1.;
      }
      if participation.pfr {
        nb_pfr += 1.;
      }
      self.nb_call += participation.call;
      self.nb_bet += participation.bet;
      self.nb_raise += participation.raise;

      increase(
        &mut self.nb_can_pre_3bet,
        &mut nb_pre_3bet,
        participation.can_pre_3bet,
        participation.pre_3bet,
      );
      increase(
        &mut self.nb_can_fold_pre_3bet,
        &mut nb_fold_pre_3bet,
        participation.can_fold_pre_3bet,
        participation.fold_pre_3bet,
      );
      increase(
        &mut self.nb_can_cbet,
        &mut nb_cbet,
        participation.can_cbet,
        participation.cbet,
      );
      increase(
        &mut self.nb_can_fold_cbet,
        &mut nb_fold_cbet,
        participation.can_fold_cbet,
        participation.fold_cbet,
      );
      increase(
        &mut self.nb_can_squeeze,
        &mut nb_squeeze,
        participation.can_squeeze,
        participation.squeeze,
      );
    }

    self.vpip = divide(nb_vpip, self.nb_hand);
    self.pfr = divide(nb_pfr, self.nb_hand);
    self.af = divide((self.nb_bet + self.nb_raise), self.nb_call);
    self.pre_3bet = divide(nb_pre_3bet, self.nb_can_pre_3bet);
    self.fold_pre_3bet = divide(nb_fold_pre_3bet, self.nb_can_fold_pre_3bet);
    self.cbet = divide(nb_cbet, self.nb_can_cbet);
    self.fold_cbet = divide(nb_fold_cbet, self.nb_can_fold_cbet);
    self.squeeze = divide(nb_squeeze, self.nb_can_squeeze);
  }
}

fn increase(nb_happen: &mut f64, nb_hand: &mut f64, condition: bool, happen: bool) {
  if condition {
    *nb_hand += 1.;
    if happen {
      *nb_happen += 1.;
    }
  }
}

fn divide(num: f64, den: f64) -> f64 {
  if den == 0. {
    -1.
  } else {
    num / den
  }
}

// All these stats can be split by position and moment
// TODO : add fold to squeeze
struct PlayerParticipation {
  name: String,
  vpip: bool, // Volontary put in the pot. Without counting big-blind check
  pfr: bool,  // preflop raise. Count raise (3-bet and more)

  // agression factor, (bet + raise) / call
  call: f64,
  bet: f64,
  raise: f64,
  can_pre_3bet: bool,      // Tells if the next value must be taken in account
  pre_3bet: bool,          // 3bet preflop. Only when possible
  can_fold_pre_3bet: bool, // Tells if the next alue must be taken in account
  fold_pre_3bet: bool,     // fold to 3bet preflop
  can_cbet: bool,          // Tells if the next value must be taken in account
  cbet: bool, // continuation bet flop. The player must have open and be the first to raise
  can_fold_cbet: bool,
  fold_cbet: bool, // fold to cbet flop
  can_squeeze: bool,
  squeeze: bool, // raise after preflop raise and at least a player has call
}

impl PlayerParticipation {
  fn new(hand: &Hand, name: &str) -> Self {
    let pre_3bet = pre_3bet_find(hand, name);
    let fold_pre_3bet = fold_pre_3bet_find(hand, name);
    let cbet = cbet_find(hand, name);
    let fold_cbet = fold_cbet_find(hand, name);
    let squeeze = squeeze_find(hand, name);
    let (call, bet, raise) = af_find(hand, name);

    Self {
      name: String::from(name),
      vpip: vpip_find(hand, name, Moment::Preflop),
      pfr: pfr_find(hand, name),
      call,
      bet,
      raise,
      can_pre_3bet: !matches!(pre_3bet, Bool::Impossible),
      pre_3bet: matches!(pre_3bet, Bool::True),
      can_fold_pre_3bet: !matches!(fold_pre_3bet, Bool::Impossible),
      fold_pre_3bet: matches!(fold_pre_3bet, Bool::True),
      can_cbet: !matches!(cbet, Bool::Impossible),
      cbet: matches!(cbet, Bool::True),
      can_fold_cbet: !matches!(fold_cbet, Bool::Impossible),
      fold_cbet: matches!(fold_cbet, Bool::True),
      can_squeeze: !matches!(squeeze, Bool::Impossible),
      squeeze: matches!(squeeze, Bool::True),
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
fn af_find(hand: &Hand, name: &str) -> (f64, f64, f64) {
  let mut call = 0.;
  let mut bet = 0.;
  let mut raise = 0.;
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
          call += 1.;
        }
      }
      Action::Bet(player, _, _) => {
        if name == player.name {
          bet += 1.;
        }
      }
      Action::Raise(player, _, _, _) => {
        if name == player.name {
          raise += 1.;
        }
      }
      _ => {}
    }
  }
  (call, bet, raise)
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
