CREATE TABLE radio_tracks (
  id SERIAL PRIMARY KEY,
  track_id INTEGER REFERENCES tracks ON DELETE CASCADE,
  radio_id INTEGER REFERENCES radios ON DELETE CASCADE,
  track_order INTEGER CHECK (track_order >= 0),
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL
)
