-- тестовое
DROP TABLE IF EXISTS common_info_product;
DROP TABLE IF EXISTS items;

CREATE TABLE IF NOT EXISTS common_info_product
(
id integer PRIMARY KEY,
  total integer NOT NULL,
   last_id varchar NOT NULL
   );

CREATE TABLE IF NOT EXISTS items
(
id integer PRIMARY KEY,
 product_id integer NOT NULL,
  offer_id varchar NOT NULL,
   is_fbo_visible bool NOT NULL,
    is_fbs_visible bool NOT NULL,
    archived bool NOT NULL,
    is_discounted bool NOT NULL,
    FOREIGN KEY (product_id) REFERENCES common_info_product (id)
    );
