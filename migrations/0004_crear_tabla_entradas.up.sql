CREATE TABLE IF NOT EXISTS entradas(
  id UUID PRIMARY KEY,
  nombre varchar(100) UNIQUE NOT NULL,
  fecha DATE NOT NULL,
  tipo UUID NOT NULL REFERENCES tipos(id),
  estado UUID NOT NULL REFERENCES estados(id)
);
