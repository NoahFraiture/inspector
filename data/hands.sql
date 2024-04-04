DROP TABLE IF EXISTS Hand;
CREATE TABLE Hand(
  id INT PRIMARY KEY,
  time INT NOT NULL, -- timestamp
  table_name TEXT NOT NULL,
  table_size INT NOT NULL,
  winner TEXT REFERENCES Player(name) NOT NULL,
  pot INT NOT NULL,
  player1 TEXT REFERENCES Player(name), -- number are the position at the table starting at UTG
  player2 TEXT REFERENCES Player(name),
  player3 TEXT REFERENCES Player(name),
  player4 TEXT REFERENCES Player(name),
  player5 TEXT REFERENCES Player(name),
  player6 TEXT REFERENCES Player(name),
  player7 TEXT REFERENCES Player(name),
  player8 TEXT REFERENCES Player(name),
  player9 TEXT REFERENCES Player(name),
  card1 TEXT,
  card2 TEXT,
  card3 TEXT,
  card4 TEXT,
  card5 TEXT
);

DROP TABLE IF EXISTS Player;
CREATE TABLE Player(
  name TEXT PRIMARY KEY
  -- stats
);

DROP TABLE IF EXISTS HoleCard;
CREATE TABLE HoleCard(
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  hand INT REFERENCES Hand(id) NOT NULL,
  player TEXT REFERENCES Player(name) NOT NULL,
  card1 TEXT NOT NULL,
  card2 TEXT NOT NULL,
  UNIQUE(hand, player)
);

DROP TABLE IF EXISTS Blind;
CREATE TABLE Blind(
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  player TEXT REFERENCES Player(name) NOT NULL,
  hand INT REFERENCES Hand(id) NOT NULL,
  amount INT NOT NULL,
  kind TEXT NOT NULL, -- big blind, small blind, ante, ...
  UNIQUE(hand, kind)
);

DROP TABLE IF EXISTS Action;
CREATE TABLE Action(
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  player TEXT REFERENCES Player(name) NOT NULL,
  hand INT REFERENCES Hand(id) NOT NULL,
  kind TEXT NOT NULL, -- call, raise, fold,
  moment TEXT NOT NULL, -- pre-flop, flop, ...
  sequence INT NOT NULL, -- first action, second action, ...
  amount1 INT,
  amount2 INT,
  allin BOOLEAN,
  UNIQUE(hand, moment, sequence)
);
