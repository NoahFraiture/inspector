-- Your SQL goes here
CREATE TABLE hand(
  id INT PRIMARY KEY,
  time INT NOT NULL, -- timestamp
  table_name TEXT NOT NULL,
  table_size INT NOT NULL,
  winner TEXT NOT NULL REFERENCES player(name),
  pot INT NOT NULL,
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
  name TEXT PRIMARY KEY,
  vpip                 FLOAT,
  pfr                  FLOAT,
  af                   FLOAT,
  pre_3bet             FLOAT,
  fold_pre_3bet        FLOAT,
  cbet                 FLOAT,
  fold_cbet            FLOAT,
  squeeze              FLOAT,

  -- number of hands used to compute stats. Usefull to easily add new hand without recomputing all hands
  nb_hand              FLOAT,
  nb_can_pre_3bet      FLOAT,
  nb_can_fold_pre_3bet FLOAT,
  nb_can_cbet          FLOAT,
  nb_can_fold_cbet     FLOAT,
  nb_can_squeeze       FLOAT,
  nb_call              FLOAT,
  nb_bet               FLOAT,
  nb_raise             FLOAT
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
