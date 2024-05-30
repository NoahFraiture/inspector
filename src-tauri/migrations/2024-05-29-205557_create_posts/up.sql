-- Your SQL goes here
CREATE TABLE hand(
  id BIGINT NOT NULL PRIMARY KEY,
  content TEXT NOT NULL,
  real_money BOOLEAN NOT NULL,
  time BIGINT NOT NULL, -- timestamp
  table_name TEXT NOT NULL,
  table_size INTEGER NOT NULL,
  winner TEXT NOT NULL REFERENCES player(name),
  pot FLOAT NOT NULL,
  player1 TEXT NOT NULL REFERENCES player(name), -- number are the position at the table starting at UTG
  player2 TEXT NOT NULL REFERENCES player(name),
  player3 TEXT NOT NULL REFERENCES player(name),
  player4 TEXT NOT NULL REFERENCES player(name),
  player5 TEXT NOT NULL REFERENCES player(name),
  player6 TEXT NOT NULL REFERENCES player(name),
  player7 TEXT NOT NULL REFERENCES player(name),
  player8 TEXT NOT NULL REFERENCES player(name),
  player9 TEXT NOT NULL REFERENCES player(name),
  card1 TEXT NOT NULL,
  card2 TEXT NOT NULL,
  card3 TEXT NOT NULL,
  card4 TEXT NOT NULL,
  card5 TEXT NOT NULL
);

CREATE TABLE player(
  name TEXT NOT NULL,
  real_money BOOLEAN NOT NULL,
  vpip                 FLOAT NOT NULL,
  pfr                  FLOAT NOT NULL,
  af                   FLOAT NOT NULL,
  pre_3bet             FLOAT NOT NULL,
  fold_pre_3bet        FLOAT NOT NULL,
  cbet                 FLOAT NOT NULL,
  fold_cbet            FLOAT NOT NULL,
  squeeze              FLOAT NOT NULL,

  -- number of hands used to compute stats. Usefull to easily add new hand without recomputing all hands
  nb_hand              FLOAT NOT NULL,
  nb_can_pre_3bet      FLOAT NOT NULL,
  nb_can_fold_pre_3bet FLOAT NOT NULL,
  nb_can_cbet          FLOAT NOT NULL,
  nb_can_fold_cbet     FLOAT NOT NULL,
  nb_can_squeeze       FLOAT NOT NULL,
  nb_call              FLOAT NOT NULL,
  nb_bet               FLOAT NOT NULL,
  nb_raise             FLOAT NOT NULL,
  PRIMARY KEY (name, real_money)
);

CREATE TABLE holeCard(
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  hand BIGINT NOT NULL REFERENCES hand(id),
  player TEXT NOT NULL REFERENCES player(name),
  card1 TEXT NOT NULL,
  card2 TEXT NOT NULL,
  UNIQUE(hand, player)
);

CREATE TABLE blind(
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  player TEXT NOT NULL REFERENCES player(name),
  hand BIGINT NOT NULL REFERENCES hand(id),
  amount FLOAT NOT NULL,
  kind TEXT NOT NULL, -- big blind, small blind, ante, ...
  UNIQUE(hand, kind)
);

CREATE TABLE action(
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  player TEXT NOT NULL REFERENCES player(name),
  hand BIGINT NOT NULL REFERENCES hand(id),
  kind TEXT NOT NULL, -- call, raise, fold,
  moment TEXT NOT NULL, -- pre-flop, flop, ...
  sequence INTEGER NOT NULL, -- first action, second action, ...
  amount1 FLOAT NOT NULL,
  amount2 FLOAT NOT NULL,
  allin BOOLEAN NOT NULL,
  UNIQUE(hand, moment, sequence)
);
