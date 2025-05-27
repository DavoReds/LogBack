CREATE DOMAIN hex_color AS CHAR(7) CHECK (VALUE ~ '^#[0-9A-Fa-f]{6}$');
