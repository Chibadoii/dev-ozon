--DROP TABLE IF EXISTS items;

CREATE TABLE IF NOT EXISTS public.items
(
id integer PRIMARY KEY,
 product_id integer NOT NULL,
  offer_id varchar NOT NULL,
   is_fbo_visible bool NOT NULL,
    is_fbs_visible bool NOT NULL,
    archived bool NOT NULL,
    is_discounted bool NOT NULL
    );
