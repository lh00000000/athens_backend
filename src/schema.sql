CREATE TABLE face_event (
  id INTEGER PRIMARY KEY,
  face_id INTEGER KEY,
  time_stamp REAL
);

CREATE TABLE personality (
  id INTEGER PRIMARY KEY,
  personality_type TEXT
);

CREATE TABLE face_personality (
  face_id INTEGER,
  personality_id INTEGER,
  FOREIGN KEY(face_id) REFERENCES face_event(face_id),
  FOREIGN KEY(personality_id) REFERENCES personality(id)
);

INSERT INTO personality (personality_type) VALUES ('open');
INSERT INTO personality (personality_type) VALUES ('conscientious');
INSERT INTO personality (personality_type) VALUES ('extroverted');
INSERT INTO personality (personality_type) VALUES ('agreeable');
INSERT INTO personality (personality_type) VALUES ('neurotic');

