-- Add up migration script here

-- Nomer table
CREATE TABLE IF NOT EXISTS nomer (
    PRIMARY KEY (nomer_id),
    nomer_id           INTEGER         NOT NULL UNIQUE AUTO_INCREMENT,
    display_name       VARCHAR(255)    NOT NULL UNIQUE,
    email              VARCHAR(255)    NOT NULL UNIQUE,
    password_hash      VARCHAR(255)    NOT NULL UNIQUE
);

-- Location table
CREATE TABLE IF NOT EXISTS canteen (
    PRIMARY KEY (canteen_id),
    canteen_id         INTEGER         NOT NULL UNIQUE AUTO_INCREMENT,
    canteen_name       VARCHAR(255)    NOT NULL UNIQUE,
    latitude           DECIMAL(10, 6)  NOT NULL,
    longitude          DECIMAL(10, 6)  NOT NULL
);

-- Store table
CREATE TABLE IF NOT EXISTS store (
    PRIMARY KEY (store_id),
    store_id           INTEGER         NOT NULL UNIQUE AUTO_INCREMENT,
    store_name         VARCHAR(255)    NOT NULL,
    is_open            BOOLEAN         NOT NULL,
    cuisine            VARCHAR(255)    NOT NULL,
    information        VARCHAR(255)    NOT NULL,
    canteen_id         INTEGER         NOT NULL,
    FOREIGN KEY (canteen_id) REFERENCES canteen(canteen_id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

-- Item table
CREATE TABLE IF NOT EXISTS item (
    PRIMARY KEY (item_id),
    item_id            INTEGER         NOT NULL UNIQUE AUTO_INCREMENT,
    item_name          VARCHAR(255)    NOT NULL,
    price              DECIMAL(10, 2)  NOT NULL,
    is_available       BOOLEAN         NOT NULL,
    information        VARCHAR(255)    NOT NULL,
    store_id           INTEGER         NOT NULL,
    FOREIGN KEY (store_id) REFERENCES store(store_id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

-- Review table
CREATE TABLE IF NOT EXISTS review (
    PRIMARY KEY (review_id),
    review_id          INTEGER         NOT NULL UNIQUE AUTO_INCREMENT,
    score              INTEGER         NOT NULL CHECK (score >= 1 AND score <= 5),
    comment            VARCHAR(255)    NOT NULL,
    nomer_id           INTEGER         NOT NULL,
    store_id           INTEGER         NOT NULL,
    FOREIGN KEY (nomer_id) REFERENCES nomer(nomer_id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    FOREIGN KEY (store_id) REFERENCES store(store_id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

-- Real canteens
INSERT INTO canteen (canteen_name, latitude, longitude) VALUES
('Fine Food'        , 1.304100, 103.773678),
('Flavours@UTown'   , 1.304836, 103.772735),
('The Deck'         , 1.294718, 103.772573),
('Techno Edge'      , 1.298000, 103.771746),
('Terrace'          , 1.296425, 103.780364),
('PGP'              , 1.290874, 103.780790),
('Frontier'         , 1.296423, 103.780365);

-- Mock stores (Generated by ChatGPT)
INSERT INTO store (store_name, is_open, cuisine, information, canteen_id) VALUES
-- Location 1
('Golden Wok Chinese', TRUE, 'Chinese', 'Authentic Chinese dishes with wok hei flavors.', 1),
('Nasi Lemak House', TRUE, 'Muslim', 'Serving fragrant nasi lemak with sambal.', 1),
('Spice Route Indian', FALSE, 'Indian', 'Rich and flavorful North and South Indian dishes.', 1),
('Peranakan Heritage Kitchen', TRUE, 'Peranakan', 'Classic Nyonya dishes with rich heritage.', 1),
('Western Grill & Pasta', TRUE, 'Western', 'Grilled meats and classic pasta dishes.', 1),
('Tokyo Ramen House', FALSE, 'Japanese', 'Authentic ramen with rich broths.', 1),
-- Location 2
('Dim Sum Paradise', TRUE, 'Chinese', 'Steamed and fried dim sum specialties.', 2),
('Satay Corner', FALSE, 'Muslim', 'Grilled satay skewers with peanut sauce.', 2),
('Roti Prata Hub', TRUE, 'Indian', 'Freshly made roti prata and curries.', 2),
('Baba Nyonya Flavours', TRUE, 'Peranakan', 'Fusion of Chinese and Muslim culinary traditions.', 2),
('The Burger Joint', FALSE, 'Western', 'Gourmet burgers with local twists.', 2),
('Sakura Sushi Bar', TRUE, 'Japanese', 'Fresh sushi and sashimi prepared daily.', 2),
-- Location 3
('Spicy Szechuan Delights', TRUE, 'Chinese', 'Bold and spicy Szechuan cuisine.', 3),
('Kampong Muslim Kitchen', TRUE, 'Muslim', 'Traditional Muslim recipes and vibrant spices.', 3),
('Banana Leaf Biryani', FALSE, 'Indian', 'Authentic biryani served on banana leaves.', 3),
('Little Nyonya Cafe', TRUE, 'Peranakan', 'Home-style Peranakan cooking and desserts.', 3),
('Steakhouse 81', TRUE, 'Western', 'Premium cuts and hearty Western meals.', 3),
('Okonomiyaki Street', FALSE, 'Japanese', 'Savory Japanese pancakes and street food.', 3),
-- Location 4
('Wok Master', TRUE, 'Chinese', 'Homestyle stir-fried dishes with wok hei.', 4),
('Seri Melayu Delights', FALSE, 'Muslim', 'Delicious kampong-style Muslim food.', 4),
('Curry Express', TRUE, 'Indian', 'Quick and flavorful Indian curries.', 4),
('Nyonya Street Eats', TRUE, 'Peranakan', 'Peranakan street food classics.', 4),
('Urban Western Bites', FALSE, 'Western', 'Trendy Western meals and quick bites.', 4),
('Sushi Zen', TRUE, 'Japanese', 'Elegant sushi platters and bento sets.', 4),
-- Location 5
('Canton Garden', TRUE, 'Chinese', 'Traditional Cantonese dishes and soups.', 5),
('Ayam Penyet King', TRUE, 'Muslim', 'Crispy smashed chicken with sambal.', 5),
('Tandoori Flame', FALSE, 'Indian', 'Authentic tandoori grilled specialties.', 5),
('Baba’s Kitchen', TRUE, 'Peranakan', 'Family-style Nyonya cooking.', 5),
('Grillhouse 21', TRUE, 'Western', 'Casual grilled favorites and sandwiches.', 5),
('Ramen Ichiban', FALSE, 'Japanese', 'Classic ramen with a modern twist.', 5),
-- Location 6
('Dragon’s Gate', TRUE, 'Chinese', 'Specialty dim sum and roast meats.', 6),
('Nasi Padang Express', TRUE, 'Muslim', 'Variety of nasi padang dishes.', 6),
('Masala Haven', FALSE, 'Indian', 'A haven for lovers of Indian spices.', 6),
('Peranakan Family Table', TRUE, 'Peranakan', 'Comforting Nyonya family recipes.', 6),
('Western Diner Co.', TRUE, 'Western', 'All-day brunch and Western classics.', 6),
('Yokohama Sushi House', FALSE, 'Japanese', 'Freshly prepared sushi and donburi.', 6),
-- Location 7
('Chopsticks Chinese', TRUE, 'Chinese', 'Authentic Chinese dishes with a modern twist.', 7),
('Muslim Spice Kitchen', TRUE, 'Muslim', 'Spicy Muslim dishes with traditional flavors.', 7),
('Indian Curry House', FALSE, 'Indian', 'Rich and aromatic Indian curries.', 7),
('Nyonya Delight', TRUE, 'Peranakan', 'Delicious Nyonya dishes with a twist.', 7),
('Western Bistro', TRUE, 'Western', 'Casual dining with Western favorites.', 7),
('Sushi & Sashimi Bar', FALSE, 'Japanese', 'Fresh sushi and sashimi daily.', 7);

-- Mock items (Generated by ChatGPT)
INSERT INTO item (item_name, price, is_available, information, store_id) VALUES
-- Store 1 (Chinese)
('Sweet and Sour Pork', 12.50, TRUE, 'Classic Cantonese stir‑fry with pineapple.', 1),
('Beef Chow Fun', 10.00, TRUE, 'Flat rice noodles with beef and soy sauce.', 1),
-- Store 2 (Muslim)
('Nasi Lemak', 8.00, TRUE, 'Rice cooked in coconut milk with sambal.', 2),
('Mee Rebus', 7.50, FALSE, 'Yellow noodles in spicy sweet potato gravy.', 2),
-- Store 3 (Indian)
('Chicken Tikka Masala', 11.00, TRUE, 'Grilled tikka chicken in creamy tomato sauce.', 3),
('Mutton Biryani', 13.50, FALSE, 'Aromatic biryani with spiced mutton.', 3),
-- Store 4 (Peranakan)
('Ayam Buah Keluak', 14.00, TRUE, 'Peranakan chicken with black nuts.', 4),
('Nyonya Laksa', 9.50, TRUE, 'Spicy coconut noodle soup with prawns.', 4),
-- Store 5 (Western)
('Ribeye Steak', 22.00, TRUE, 'Grilled ribeye with garlic butter.', 5),
('Spaghetti Carbonara', 15.00, FALSE, 'Pasta with pancetta and creamy sauce.', 5),
-- Store 6 (Japanese)
('Salmon Sashimi', 18.00, TRUE, 'Fresh sliced salmon sashimi.', 6),
('Tonkotsu Ramen', 13.00, TRUE, 'Rich pork bone broth ramen.', 6),
-- Store 7 (Chinese)
('Mapo Tofu', 9.00, TRUE, 'Spicy Szechuan tofu with minced pork.', 7),
('Peking Duck Wrap', 16.00, FALSE, 'Roast duck slices in pancake.', 7),
-- Store 8 (Muslim)
('Rendang Daging', 12.00, TRUE, 'Slow‑cooked beef in coconut spices.', 8),
('Kuih Seri Muka', 4.50, TRUE, 'Layered glutinous rice cake dessert.', 8),
-- Store 9 (Indian)
('Fish Curry', 11.50, FALSE, 'Tangy fish curry with tamarind.', 9),
('Garlic Naan', 3.50, TRUE, 'Oven‑baked garlic flatbread.', 9),
-- Store 10 (Peranakan)
('Ulak Laksa', 10.50, TRUE, 'Spicy noodle soup with fishballs.', 10),
('Kueh Pie Tee', 6.00, TRUE, 'Crispy tart shells with shrimp.', 10),
-- Store 11 (Western)
('BBQ Ribs', 19.00, TRUE, 'Slow‑cooked ribs with BBQ sauce.', 11),
('Caesar Salad', 9.00, TRUE, 'Romaine lettuce with creamy dressing.', 11),
-- Store 12 (Japanese)
('Unagi Don', 16.50, TRUE, 'Grilled eel over rice bowl.', 12),
('Chicken Katsu Curry', 12.00, FALSE, 'Breaded chicken with Japanese curry.', 12),
-- Store 13 (Chinese)
('Char Siew Rice', 8.50, TRUE, 'Roast pork over rice.', 13),
('Wonton Soup', 7.00, TRUE, 'Shrimp wontons in clear broth.', 13),
-- Store 14 (Muslim)
('Laksa Lemak', 9.50, TRUE, 'Coconut laksa with prawns.', 14),
('Roti Jala', 5.00, FALSE, 'Lace pancakes with curry dip.', 14),
-- Store 15 (Indian)
('Butter Chicken', 12.50, TRUE, 'Creamy tomato chicken curry.', 15),
('Dosa Masala', 7.00, TRUE, 'Crispy rice pancake with potato masala.', 15),
-- Store 16 (Peranakan)
('Putu Mayam', 5.50, TRUE, 'String hoppers with coconut sugar.', 16),
('Ikan Assam Pedas', 13.00, FALSE, 'Spicy sour fish stew.', 16),
-- Store 17 (Western)
('Grilled Salmon', 18.50, TRUE, 'Salmon fillet with lemon butter.', 17),
('Mac & Cheese', 8.00, TRUE, 'Creamy baked macaroni and cheese.', 17),
-- Store 18 (Japanese)
('Tempura Udon', 11.00, TRUE, 'Udon noodles with shrimp tempura.', 18),
('Matcha Ice Cream', 4.50, TRUE, 'Green tea flavored ice cream.', 18),
-- Store 19 (Chinese)
('Steamed Fish', 15.00, TRUE, 'Whole fish with ginger and soy.', 19),
('Egg Fried Rice', 6.00, TRUE, 'Classic fried rice with egg.', 19),
-- Store 20 (Muslim)
('Soto Ayam', 7.50, FALSE, 'Chicken soup with turmeric and noodles.', 20),
('Apam Balik', 4.00, TRUE, 'Folded pancake with peanuts.', 20),
-- Store 21 (Indian)
('Paneer Tikka', 10.00, TRUE, 'Grilled marinated cottage cheese.', 21),
('Masala Chai', 3.00, TRUE, 'Spiced Indian milk tea.', 21),
-- Store 22 (Peranakan)
('Kueh Lapis', 6.00, TRUE, 'Layered spice cake.', 22),
('Prawn Assam Chili', 14.00, FALSE, 'Prawns in spicy tamarind sauce.', 22),
-- Store 23 (Western)
('Club Sandwich', 8.50, TRUE, 'Triple‑decker sandwich with bacon.', 23),
('Tomato Soup', 5.00, TRUE, 'Creamy tomato basil soup.', 23),
-- Store 24 (Japanese)
('Yakitori Skewers', 9.00, TRUE, 'Grilled chicken skewers with sauce.', 24),
('Chawanmushi', 5.50, TRUE, 'Savory steamed egg custard.', 24),
-- Store 25 (Chinese)
('Spring Rolls', 5.00, TRUE, 'Crispy vegetable rolls.', 25),
('Kung Pao Chicken', 11.00, FALSE, 'Spicy chicken stir‑fry with peanuts.', 25),
-- Store 26 (Muslim)
('Cendol', 4.50, TRUE, 'Icy dessert with coconut and palm sugar.', 26),
('Beef Rendang', 12.00, TRUE, 'Rich slow‑cooked beef curry.', 26),
-- Store 27 (Indian)
('Aloo Gobi', 9.00, TRUE, 'Potato and cauliflower dry curry.', 27),
('Idli Sambar', 6.50, TRUE, 'Steamed rice cakes with lentil soup.', 27),
-- Store 28 (Peranakan)
('Kueh Salat', 5.50, TRUE, 'Pandan custard on rice cake.', 28),
('Laksa Kari', 10.00, FALSE, 'Spicy coconut noodle soup.', 28),
-- Store 29 (Western)
('Fish & Chips', 10.50, TRUE, 'Battered fish with fries.', 29),
('Quiche Lorraine', 7.00, TRUE, 'Savory tart with bacon & cheese.', 29),
-- Store 30 (Japanese)
('Soba Noodles', 9.50, TRUE, 'Cold buckwheat soba with dipping sauce.', 30),
('Gyoza', 6.00, TRUE, 'Pan‑fried dumplings.', 30),
-- Store 31 (Chinese)
('Hot and Sour Soup', 6.00, TRUE, 'Spicy and tangy broth.', 31),
('Char Kway Teow', 8.00, TRUE, 'Fried flat noodles with char.' , 31),
-- Store 32 (Muslim)
('Ikan Bakar', 11.00, FALSE, 'Grilled fish with chili paste.', 32),
('Pulut Hitam', 4.00, TRUE, 'Black glutinous rice dessert.', 32),
-- Store 33 (Indian)
('Chicken 65', 9.50, TRUE, 'Fried spicy chicken bite.', 33),
('Rasam', 3.50, TRUE, 'Tangy South Indian soup.', 33),
-- Store 34 (Peranakan)
('Laksa Sarawak', 11.00, TRUE, 'Spicy Sarawak laksa.', 34),
('Otak‑Otak', 6.50, FALSE, 'Grilled fish cake in banana leaf.', 34),
-- Store 35 (Western)
('Chicken Alfredo', 12.50, TRUE, 'Fettuccine with creamy alfredo sauce.', 35),
('Greek Salad', 8.00, TRUE, 'Salad with feta, olives, cucumber.', 35),
-- Store 36 (Japanese)
('Onigiri', 3.00, TRUE, 'Rice ball with filling.', 36),
('Miso Soup', 2.50, TRUE, 'Traditional soybean miso soup.', 36),
-- Store 37 (Chinese)
('Beef and Broccoli', 10.50, TRUE, 'Stir-fried beef with broccoli in oyster sauce.', 37),
('Egg Tarts', 4.00, TRUE, 'Crispy pastry with creamy egg filling.', 37),
-- Store 38 (Muslim)
('Mee Goreng Mamak', 7.50, TRUE, 'Spicy fried noodles with vegetables.', 38),
('Kuih Lapis', 5.00, TRUE, 'Layered steamed cake with pandan flavor.', 38),
-- Store 39 (Indian)
('Chana Masala', 8.00, TRUE, 'Spicy chickpea curry.', 39),
('Pani Puri', 4.00, TRUE, 'Crispy puris filled with spiced water.', 39),
-- Store 40 (Peranakan)
('Ayam Penyet', 10.00, TRUE, 'Smashed fried chicken with sambal.', 40),
('Kueh Talam', 5.50, TRUE, 'Steamed pandan and coconut cake.', 40),
-- Store 41 (Western)
('Pulled Pork Burger', 11.50, TRUE, 'Slow-cooked pulled pork with coleslaw.', 41),
('Cauliflower Steak', 9.00, TRUE, 'Grilled cauliflower with chimichurri.', 41),
-- Store 42 (Japanese)
('Sashimi Platter', 20.00, TRUE, 'Assorted fresh sashimi.', 42),
('Takoyaki', 7.00, TRUE, 'Octopus balls with bonito flakes.', 42);