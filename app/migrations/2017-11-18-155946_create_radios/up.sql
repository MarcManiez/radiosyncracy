CREATE TABLE radios (
  id SERIAL PRIMARY KEY,
  user_id INTEGER REFERENCES users ON DELETE CASCADE,
  last_played_radio_track_number INTEGER,
  name VARCHAR NOT NULL,
  seconds_played_on_last_radio_track INTEGER DEFAULT 0 CHECK (seconds_played_on_last_radio_track >= 0),
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL
)
