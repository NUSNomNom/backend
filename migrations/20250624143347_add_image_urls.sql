-- Add migration script here

-- Add image_url columns to canteen, store, and item tables
ALTER TABLE canteen ADD COLUMN image_url VARCHAR(255);
ALTER TABLE store ADD COLUMN image_url VARCHAR(255);
ALTER TABLE item ADD COLUMN image_url VARCHAR(255);

-- Update image URLs for canteens
UPDATE canteen SET image_url = 'https://nomnom-image.sgp1.cdn.digitaloceanspaces.com/canteen_fine_food.jpeg'
WHERE canteen_name = 'Fine Food';
UPDATE canteen SET image_url = 'https://nomnom-image.sgp1.cdn.digitaloceanspaces.com/canteen_flavours.jpeg'
WHERE canteen_name = 'Flavours@UTown';
UPDATE canteen SET image_url = 'https://nomnom-image.sgp1.cdn.digitaloceanspaces.com/canteen_techno_edge.jpeg'
WHERE canteen_name = 'Techno Edge';
UPDATE canteen SET image_url = 'https://nomnom-image.sgp1.cdn.digitaloceanspaces.com/canteen_the_deck.jpeg'
WHERE canteen_name = 'The Deck';
UPDATE canteen SET image_url = 'https://nomnom-image.sgp1.cdn.digitaloceanspaces.com/canteen_terrace.jpeg'
WHERE canteen_name = 'Terrace';
UPDATE canteen SET image_url = 'https://nomnom-image.sgp1.cdn.digitaloceanspaces.com/canteen_pgp.jpeg'
WHERE canteen_name = 'PGP';
UPDATE canteen SET image_url = 'https://nomnom-image.sgp1.cdn.digitaloceanspaces.com/canteen_frontier.jpeg'
WHERE canteen_name = 'Frontier';

-- Update image URLs for stores based on cuisine
UPDATE store SET image_url = 'https://nomnom-image.sgp1.cdn.digitaloceanspaces.com/store_chinese.jpeg'
WHERE cuisine = 'Chinese';
UPDATE store SET image_url = 'https://nomnom-image.sgp1.cdn.digitaloceanspaces.com/store_muslim.jpeg'
WHERE cuisine = 'Muslim';
UPDATE store SET image_url = 'https://nomnom-image.sgp1.cdn.digitaloceanspaces.com/store_indian.jpeg'
WHERE cuisine = 'Indian';
UPDATE store SET image_url = 'https://nomnom-image.sgp1.cdn.digitaloceanspaces.com/store_peranakan.jpeg'
WHERE cuisine = 'Peranakan';
UPDATE store SET image_url = 'https://nomnom-image.sgp1.cdn.digitaloceanspaces.com/store_western.jpeg'
WHERE cuisine = 'Western';
UPDATE store SET image_url = 'https://nomnom-image.sgp1.cdn.digitaloceanspaces.com/store_japanese.jpeg'
WHERE cuisine = 'Japanese';

-- Update image URLs for items based on cuisine and id
UPDATE item
SET image_url = 'https://nomnom-image.sgp1.cdn.digitaloceanspaces.com/item_chinese_1.jpeg'
WHERE store_id IN (
    SELECT store_id FROM store WHERE cuisine = 'Chinese'
) AND item_id % 2 = 1;
UPDATE item
SET image_url = 'https://nomnom-image.sgp1.cdn.digitaloceanspaces.com/item_chinese_2.jpeg'
WHERE store_id IN (
    SELECT store_id FROM store WHERE cuisine = 'Chinese'
) AND item_id % 2 = 0;

UPDATE item
SET image_url = 'https://nomnom-image.sgp1.cdn.digitaloceanspaces.com/item_muslim_1.jpeg'
WHERE store_id IN (
    SELECT store_id FROM store WHERE cuisine = 'Muslim'
) AND item_id % 2 = 1;
UPDATE item
SET image_url = 'https://nomnom-image.sgp1.cdn.digitaloceanspaces.com/item_muslim_2.jpeg'
WHERE store_id IN (
    SELECT store_id FROM store WHERE cuisine = 'Muslim'
) AND item_id % 2 = 0;

UPDATE item
SET image_url = 'https://nomnom-image.sgp1.cdn.digitaloceanspaces.com/item_peranakan_1.jpeg'
WHERE store_id IN (
    SELECT store_id FROM store WHERE cuisine = 'Peranakan'
) AND item_id % 2 = 1;
UPDATE item
SET image_url = 'https://nomnom-image.sgp1.cdn.digitaloceanspaces.com/item_peranakan_2.jpeg'
WHERE store_id IN (
    SELECT store_id FROM store WHERE cuisine = 'Peranakan'
) AND item_id % 2 = 0;

UPDATE item
SET image_url = 'https://nomnom-image.sgp1.cdn.digitaloceanspaces.com/item_indian_1.jpeg'
WHERE store_id IN (
    SELECT store_id FROM store WHERE cuisine = 'Indian'
) AND item_id % 2 = 1;
UPDATE item
SET image_url = 'https://nomnom-image.sgp1.cdn.digitaloceanspaces.com/item_indian_2.jpeg'
WHERE store_id IN (
    SELECT store_id FROM store WHERE cuisine = 'Indian'
) AND item_id % 2 = 0;

UPDATE item
SET image_url = 'https://nomnom-image.sgp1.cdn.digitaloceanspaces.com/item_western_1.jpeg'
WHERE store_id IN (
    SELECT store_id FROM store WHERE cuisine = 'Western'
) AND item_id % 2 = 1;
UPDATE item
SET image_url = 'https://nomnom-image.sgp1.cdn.digitaloceanspaces.com/item_western_2.jpeg'
WHERE store_id IN (
    SELECT store_id FROM store WHERE cuisine = 'Western'
) AND item_id % 2 = 0;

UPDATE item
SET image_url = 'https://nomnom-image.sgp1.cdn.digitaloceanspaces.com/item_japanese_1.jpeg'
WHERE store_id IN (
    SELECT store_id FROM store WHERE cuisine = 'Japanese'
) AND item_id % 2 = 1;
UPDATE item
SET image_url = 'https://nomnom-image.sgp1.cdn.digitaloceanspaces.com/item_japanese_2.jpeg'
WHERE store_id IN (
    SELECT store_id FROM store WHERE cuisine = 'Japanese'
) AND item_id % 2 = 0;

-- Add non-null constraint to image_url columns
ALTER TABLE canteen MODIFY image_url VARCHAR(255) NOT NULL;
ALTER TABLE store MODIFY image_url VARCHAR(255) NOT NULL;
ALTER TABLE item MODIFY image_url VARCHAR(255) NOT NULL;