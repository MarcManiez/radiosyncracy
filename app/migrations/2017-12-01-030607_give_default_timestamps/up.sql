ALTER TABLE users
  ALTER COLUMN created_at SET DEFAULT now(),
  ALTER COLUMN updated_at DROP NOT NULL;

ALTER TABLE tracks
  ALTER COLUMN created_at SET DEFAULT now(),
  ALTER COLUMN updated_at DROP NOT NULL;

ALTER TABLE radios
  ALTER COLUMN created_at SET DEFAULT now(),
  ALTER COLUMN updated_at DROP NOT NULL;

ALTER TABLE radio_tracks
  ALTER COLUMN created_at SET DEFAULT now(),
  ALTER COLUMN updated_at DROP NOT NULL;