
CREATE TABLE IF NOT EXISTS hcm_data (
  id SERIAL NOT NULL,
  content TEXT NOT NULL,
  data TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT now(),
  updated_at TIMESTAMP NOT NULL DEFAULT now()
);
