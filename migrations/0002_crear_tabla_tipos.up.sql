CREATE TABLE IF NOT EXISTS tipos (
  id UUID PRIMARY KEY,
  nombre VARCHAR(100) UNIQUE NOT NULL,
  color hex_color NOT NULL
);
