-- Your SQL goes here
CREATE TABLE products (
  id SERIAL PRIMARY KEY,
  product_name VARCHAR NOT NULL,
  product_price DECIMAL(10,2) NOT NULL
)