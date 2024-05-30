-- Your SQL goes here
CREATE TABLE hand(
  id INT PRIMARY KEY,
  content TEXT NOT NULL,
  real_money BOOLEAN NOT NULL,
  time INT NOT NULL, -- timestamp
  table_name TEXT NOT NULL,
  table_size INT NOT NULL,
  winner TEXT NOT NULL REFERENCES player(name),
  pot FLOAT NOT NULL,
  player1 TEXT REFERENCES player(name), -- number are the position at the table starting at UTG
  player2 TEXT REFERENCES player(name),
  player3 TEXT REFERENCES player(name),
  player4 TEXT REFERENCES player(name),
  player5 TEXT REFERENCES player(name),
  player6 TEXT REFERENCES player(name),
  player7 TEXT REFERENCES player(name),
  player8 TEXT REFERENCES player(name),
  player9 TEXT REFERENCES player(name),
  card1 TEXT,
  card2 TEXT,
  card3 TEXT,
  card4 TEXT,
  card5 TEXT
);

CREATE TABLE player(
  name TEXT,
  real_money BOOLEAN,
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
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  hand INT REFERENCES hand(id) NOT NULL,
  player TEXT REFERENCES player(name) NOT NULL,
  card1 TEXT NOT NULL,
  card2 TEXT NOT NULL,
  UNIQUE(hand, player)
);

CREATE TABLE blind(
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  player TEXT REFERENCES player(name) NOT NULL,
  hand INT REFERENCES hand(id) NOT NULL,
  amount INT NOT NULL,
  kind TEXT NOT NULL, -- big blind, small blind, ante, ...
  UNIQUE(hand, kind)
);

CREATE TABLE action(
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  player TEXT REFERENCES player(name) NOT NULL,
  hand INT REFERENCES hand(id) NOT NULL,
  kind TEXT NOT NULL, -- call, raise, fold,
  moment TEXT NOT NULL, -- pre-flop, flop, ...
  sequence INT NOT NULL, -- first action, second action, ...
  amount1 INT,
  amount2 INT,
  allin BOOLEAN,
  UNIQUE(hand, moment, sequence)
);
