
/****************************************************************************************************/
CREATE TABLE archives (
  id         INTEGER NOT NULL PRIMARY KEY,
  arch_name  TEXT NOT NULL,
  arch_home  TEXT NOT NULL,
  arch_size  BIGINT NOT NULL,
  arch_uuid  TEXT NOT NULL UNIQUE ON CONFLICT IGNORE,
  arch_done  BOOLEAN NOT NULL DEFAULT 0
);
CREATE TABLE books (
  id              INTEGER NOT NULL PRIMARY KEY,
  arch_id         INTEGER NOT NULL REFERENCES archives(id),
  book_file       TEXT NOT NULL,
  book_zip_size   BIGINT NOT NULL,
  book_size       BIGINT NOT NULL,
  book_crc32      BIGINT NOT NULL,
  book_offset     BIGINT NOT NULL,
  CONSTRAINT u_books UNIQUE(arch_id, book_file, book_crc32) ON CONFLICT IGNORE
);
CREATE VIEW books_view AS
SELECT
	arch_id,
	arch_name,
	arch_home,
	arch_size,
	arch_uuid,
  books.id as book_id,
	book_file,
	book_zip_size,
	book_size,
	book_crc32,
	book_offset
FROM books LEFT JOIN archives ON (arch_id = archives.id);

/****************************************************************************************************/
CREATE TABLE authors (
  id          INTEGER NOT NULL PRIMARY KEY,
  first_name  TEXT NOT NULL,
  middle_name TEXT NOT NULL,
  last_name   TEXT NOT NULL,
  nickname    TEXT NOT NULL,
  uuid        TEXT NOT NULL,
  CONSTRAINT u_authors UNIQUE(first_name, middle_name, last_name, nickname, uuid) ON CONFLICT IGNORE
);
CREATE INDEX first_name_idx   ON authors (first_name  COLLATE NOCASE);
CREATE INDEX middle_name_idx  ON authors (middle_name COLLATE NOCASE);
CREATE INDEX last_name_idx    ON authors (last_name   COLLATE NOCASE);
CREATE INDEX nickname_idx     ON authors (nickname    COLLATE NOCASE);

CREATE TABLE  author_links (
  id          INTEGER NOT NULL PRIMARY KEY,
  book_id     INTEGER NOT NULL REFERENCES books(id),
  author_id   INTEGER NOT NULL REFERENCES authors(id),
  CONSTRAINT u_authors UNIQUE(book_id, author_id) ON CONFLICT IGNORE
);
CREATE VIEW authors_view AS
SELECT
	book_id,
	first_name,
	middle_name,
	last_name,
	nickname,
	uuid
FROM author_links LEFT JOIN authors ON (author_id = authors.id);
/****************************************************************************************************/

/*
CREATE VIRTUAL TABLE fts_authors USING fts5(
  id INTEGER NOT NULL PRIMARY KEY,
  first_name,
  middle_name,
  last_name,
  nickname UNINDEXED,
  uuid UNINDEXED
);
--INSERT INTO fts_authors SELECT * FROM authors;
*/

/****************************************************************************************************/
CREATE TABLE titles (
  id          INTEGER NOT NULL PRIMARY KEY,
  book_title  TEXT NOT NULL
);
CREATE TABLE  title_links (
  id          INTEGER NOT NULL PRIMARY KEY,
  book_id     INTEGER NOT NULL REFERENCES books(id),
  title_id    INTEGER NOT NULL REFERENCES titles(id),
  CONSTRAINT u_authors UNIQUE(book_id, title_id) ON CONFLICT IGNORE
);
CREATE VIEW titles_view AS
SELECT
	book_id,
	book_title
FROM title_links LEFT JOIN titles ON (titles.id = title_id);
/****************************************************************************************************/
CREATE TABLE genres (
  id      INTEGER NOT NULL PRIMARY KEY,
  genre_name    TEXT NOT NULL
);
CREATE TABLE  genre_links (
  id          INTEGER NOT NULL PRIMARY KEY,
  book_id     INTEGER NOT NULL REFERENCES books(id),
  genre_id   INTEGER NOT NULL REFERENCES genres(id),
  CONSTRAINT u_authors UNIQUE(book_id, genre_id) ON CONFLICT IGNORE
);
CREATE VIEW genres_view AS
SELECT
	book_id,
	genre_name
FROM genre_links LEFT JOIN genres ON (genres.id = genre_id);
/****************************************************************************************************/

/****************************************************************************************************/
CREATE TABLE genre_groups (
  id    INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  name  TEXT NOT NULL
);

CREATE TABLE genre_names (
  id          INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  group_id    INTEGER NOT NULL REFERENCES genre_groups(id),
  code        TEXT NOT NULL UNIQUE,
  name        TEXT NOT NULL
);

CREATE TABLE genre_synonyms (
  id          INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  code        TEXT NOT NULL UNIQUE,
  synonym_id  INTEGER NOT NULL REFERENCES genre_names(id)
);

CREATE VIEW genre_dict_view AS
SELECT
	G.id AS `id`,
	G.name AS `code`,
	ifnull(ifnull(N.name, GN.name), G.name) AS `name`,
	ifnull(GG.name, '') AS `group`
FROM genres G
LEFT JOIN genre_names N ON (G.name = N.code)
LEFT JOIN genre_synonyms S ON (S.code = G.name) LEFT JOIN genre_names GN ON (S.synonym_id = GN.id)
LEFT JOIN genre_groups GG ON (GG.id = N.group_id OR GG.id = GN.group_id);


CREATE VIEW full_view AS
SELECT
	arch_id,
	arch_name,
	arch_home,
	arch_size,
	arch_uuid,
	books_view.book_id,
	book_file,
	book_zip_size,
	book_size,
	book_crc32,
	book_offset,
	book_title,
	first_name,
	middle_name,
	last_name,
	genre_name
FROM books_view
LEFT JOIN titles_view  ON books_view.book_id = titles_view.book_id
LEFT JOIN authors_view ON books_view.book_id = authors_view.book_id
LEFT JOIN genres_view  ON books_view.book_id = genres_view.book_id;


INSERT INTO genre_groups (id, name) VALUES (0,  'не классифицировано');
INSERT INTO genre_groups (id, name) VALUES (1,  'приключения');
INSERT INTO genre_groups (id, name) VALUES (2,  'древняя литература');
INSERT INTO genre_groups (id, name) VALUES (3,  'детская литература');
INSERT INTO genre_groups (id, name) VALUES (4,  'компьютерная литература');
INSERT INTO genre_groups (id, name) VALUES (5,  'детектив');
INSERT INTO genre_groups (id, name) VALUES (6,  'поэзия и драматургия');
INSERT INTO genre_groups (id, name) VALUES (7,  'домоводство');
INSERT INTO genre_groups (id, name) VALUES (8,  'дамские романы');
INSERT INTO genre_groups (id, name) VALUES (9,  'прочее');
INSERT INTO genre_groups (id, name) VALUES (10, 'фантастика и фэнтези');
INSERT INTO genre_groups (id, name) VALUES (11, 'проза');
INSERT INTO genre_groups (id, name) VALUES (12, 'справочная литература');
INSERT INTO genre_groups (id, name) VALUES (13, 'религия');
INSERT INTO genre_groups (id, name) VALUES (14, 'научная и научно-популярная литература');
INSERT INTO genre_groups (id, name) VALUES (15, 'юмористическая литература');

INSERT INTO genre_names (id, group_id, code, name) VALUES (0, 0, 'unknown', 'не классифицировано');

INSERT INTO genre_names (group_id, code, name) VALUES (1, 'adv_animal',   'природа и животные');
INSERT INTO genre_names (group_id, code, name) VALUES (1, 'adv_geo',      'путешествия и география');
INSERT INTO genre_names (group_id, code, name) VALUES (1, 'adv_history',  'исторические приключения');
INSERT INTO genre_names (group_id, code, name) VALUES (1, 'adv_indian',   'приключения про индейцев');
INSERT INTO genre_names (group_id, code, name) VALUES (1, 'adv_maritime', 'морские приключения');
INSERT INTO genre_names (group_id, code, name) VALUES (1, 'adv_western',  'вестерн');
INSERT INTO genre_names (group_id, code, name) VALUES (1, 'adventure',    'прочие приключения');

INSERT INTO genre_names (group_id, code, name) VALUES (2, 'antique_ant', 'античная литература');
INSERT INTO genre_names (group_id, code, name) VALUES (2, 'antique_east', 'древневосточная литература');
INSERT INTO genre_names (group_id, code, name) VALUES (2, 'antique_european', 'европейская старинная литература');
INSERT INTO genre_names (group_id, code, name) VALUES (2, 'antique_myths', 'мифы, легенды, эпос');
INSERT INTO genre_names (group_id, code, name) VALUES (2, 'antique_russian', 'древнерусская литература');
INSERT INTO genre_names (group_id, code, name) VALUES (2, 'antique', 'прочая старинная литература');

INSERT INTO genre_names (group_id, code, name) VALUES (3, 'child_adv', 'детские приключения');
INSERT INTO genre_names (group_id, code, name) VALUES (3, 'child_det', 'детские остросюжетные');
INSERT INTO genre_names (group_id, code, name) VALUES (3, 'child_education', 'детская образовательная литература');
INSERT INTO genre_names (group_id, code, name) VALUES (3, 'child_prose', 'детская проза');
INSERT INTO genre_names (group_id, code, name) VALUES (3, 'child_sf', 'детская фантастика');
INSERT INTO genre_names (group_id, code, name) VALUES (3, 'child_tale', 'сказка');
INSERT INTO genre_names (group_id, code, name) VALUES (3, 'child_verse', 'детские стихи');
INSERT INTO genre_names (group_id, code, name) VALUES (3, 'children', 'прочая детская литература');

INSERT INTO genre_names (group_id, code, name) VALUES (4, 'comp_db', 'базы данных');
INSERT INTO genre_names (group_id, code, name) VALUES (4, 'comp_hard', 'компьютерное железо (аппаратное обеспечение)');
INSERT INTO genre_names (group_id, code, name) VALUES (4, 'comp_osnet', 'ос и сети');
INSERT INTO genre_names (group_id, code, name) VALUES (4, 'comp_programming', 'программирование');
INSERT INTO genre_names (group_id, code, name) VALUES (4, 'comp_soft', 'программы');
INSERT INTO genre_names (group_id, code, name) VALUES (4, 'comp_www', 'интернет');
INSERT INTO genre_names (group_id, code, name) VALUES (4, 'design', 'искусство и дизайн');
INSERT INTO genre_names (group_id, code, name) VALUES (4, 'computers', 'прочая околокомпьтерная литература');

INSERT INTO genre_names (group_id, code, name) VALUES (5, 'det_action', 'боевик');
INSERT INTO genre_names (group_id, code, name) VALUES (5, 'det_classic', 'классический детектив');
INSERT INTO genre_names (group_id, code, name) VALUES (5, 'det_crime', 'криминальный детектив');
INSERT INTO genre_names (group_id, code, name) VALUES (5, 'det_espionage', 'шпионский детектив');
INSERT INTO genre_names (group_id, code, name) VALUES (5, 'det_hard', 'крутой детектив');
INSERT INTO genre_names (group_id, code, name) VALUES (5, 'det_history', 'исторический детектив');
INSERT INTO genre_names (group_id, code, name) VALUES (5, 'det_irony', 'иронический детектив');
INSERT INTO genre_names (group_id, code, name) VALUES (5, 'det_maniac', 'маньяки');
INSERT INTO genre_names (group_id, code, name) VALUES (5, 'det_police', 'полицейский детектив');
INSERT INTO genre_names (group_id, code, name) VALUES (5, 'det_political', 'политический детектив');
INSERT INTO genre_names (group_id, code, name) VALUES (5, 'thriller', 'триллер');
INSERT INTO genre_names (group_id, code, name) VALUES (5, 'detective', 'детектив');

INSERT INTO genre_names (group_id, code, name) VALUES (6, 'dramaturgy', 'драматургия');
INSERT INTO genre_names (group_id, code, name) VALUES (6, 'comedy', 'комедия');
INSERT INTO genre_names (group_id, code, name) VALUES (6, 'poetry', 'поэзия');
INSERT INTO genre_names (group_id, code, name) VALUES (6, 'lyrics', 'лирика');
INSERT INTO genre_names (group_id, code, name) VALUES (6, 'epic_poetry', 'эпическая поэзия');


INSERT INTO genre_names (group_id, code, name) VALUES (7, 'home_cooking', 'кулинария');
INSERT INTO genre_names (group_id, code, name) VALUES (7, 'home_crafts', 'хобби и ремесла');
INSERT INTO genre_names (group_id, code, name) VALUES (7, 'home_diy', 'сделай сам');
INSERT INTO genre_names (group_id, code, name) VALUES (7, 'home_entertain', 'развлечения');
INSERT INTO genre_names (group_id, code, name) VALUES (7, 'home_garden', 'сад и огород');
INSERT INTO genre_names (group_id, code, name) VALUES (7, 'home_health', 'здоровье');
INSERT INTO genre_names (group_id, code, name) VALUES (7, 'home_pets', 'домашние животные');
INSERT INTO genre_names (group_id, code, name) VALUES (7, 'home_sex', 'эротика, секс');
INSERT INTO genre_names (group_id, code, name) VALUES (7, 'home_sport', 'спорт');
INSERT INTO genre_names (group_id, code, name) VALUES (7, 'home_collecting', 'коллекционирование');
INSERT INTO genre_names (group_id, code, name) VALUES (7, 'home', 'прочиее домоводство');

INSERT INTO genre_names (group_id, code, name) VALUES (8, 'love', 'любовные романы');
INSERT INTO genre_names (group_id, code, name) VALUES (8, 'love_contemporary', 'современные любовные романы');
INSERT INTO genre_names (group_id, code, name) VALUES (8, 'love_detective', 'остросюжетные любовные романы');
INSERT INTO genre_names (group_id, code, name) VALUES (8, 'love_erotica', 'любовно-эротические романы');
INSERT INTO genre_names (group_id, code, name) VALUES (8, 'love_history', 'исторические любовные романы');
INSERT INTO genre_names (group_id, code, name) VALUES (8, 'love_sf', 'романтическая фантастика');
INSERT INTO genre_names (group_id, code, name) VALUES (8, 'love_short', 'короткие любовные романы');
INSERT INTO genre_names (group_id, code, name) VALUES (8, 'foreign_love', 'неразделённая любовь');
INSERT INTO genre_names (group_id, code, name) VALUES (8, 'erotica', 'эротика');

INSERT INTO genre_names (group_id, code, name) VALUES (9, 'nonf_biography', 'биографии и Мемуары');
INSERT INTO genre_names (group_id, code, name) VALUES (9, 'nonf_criticism', 'критика');
INSERT INTO genre_names (group_id, code, name) VALUES (9, 'nonf_publicism', 'публицистика');
INSERT INTO genre_names (group_id, code, name) VALUES (9, 'periodic',	'периодика');
INSERT INTO genre_names (group_id, code, name) VALUES (9, 'other', 'прочая литиратура');
INSERT INTO genre_names (group_id, code, name) VALUES (9, 'nonfiction', 'прочая документальная литература');
INSERT INTO genre_names (group_id, code, name) VALUES (9, 'scenarios', 'сценарии');
INSERT INTO genre_names (group_id, code, name) VALUES (9, 'cinema_theatre', 'кино сценарии и театральные программы');

INSERT INTO genre_names (group_id, code, name) VALUES (10, 'sf', 'научная фантастика');
INSERT INTO genre_names (group_id, code, name) VALUES (10, 'sf_action', 'боевая фантастика');
INSERT INTO genre_names (group_id, code, name) VALUES (10, 'sf_cyberpunk', 'киберпанк');
INSERT INTO genre_names (group_id, code, name) VALUES (10, 'sf_detective', 'детективная фантастика');
INSERT INTO genre_names (group_id, code, name) VALUES (10, 'sf_epic', 'эпическая фантастика');
INSERT INTO genre_names (group_id, code, name) VALUES (10, 'sf_fantasy', 'фэнтези');
INSERT INTO genre_names (group_id, code, name) VALUES (10, 'sf_heroic', 'героическая фантастика');
INSERT INTO genre_names (group_id, code, name) VALUES (10, 'sf_history', 'альтернативная история');
INSERT INTO genre_names (group_id, code, name) VALUES (10, 'sf_horror', 'ужасы и мистика');
INSERT INTO genre_names (group_id, code, name) VALUES (10, 'sf_litrpg', 'литературное RPG');
INSERT INTO genre_names (group_id, code, name) VALUES (10, 'sf_social', 'социально-психологическая фантастика');
INSERT INTO genre_names (group_id, code, name) VALUES (10, 'sf_space', 'космическая фантастика');
INSERT INTO genre_names (group_id, code, name) VALUES (10, 'sf_postapocalyptic', 'постапокалиптическая фантастика');
INSERT INTO genre_names (group_id, code, name) VALUES (10, 'popadanec', 'попаданцы');
INSERT INTO genre_names (group_id, code, name) VALUES (10, 'fanfiction', 'фанфики');
INSERT INTO genre_names (group_id, code, name) VALUES (10, 'sf_fantasy_city', 'городское фэнтези');

INSERT INTO genre_names (group_id, code, name) VALUES (11, 'prose_classic', 'классическая проза');
INSERT INTO genre_names (group_id, code, name) VALUES (11, 'prose_contemporary', 'современная проза');
INSERT INTO genre_names (group_id, code, name) VALUES (11, 'prose_counter', 'контркультура');
INSERT INTO genre_names (group_id, code, name) VALUES (11, 'prose_history', 'историческая проза');
INSERT INTO genre_names (group_id, code, name) VALUES (11, 'prose_military', 'военная проза');
INSERT INTO genre_names (group_id, code, name) VALUES (11, 'prose_rus_classic', 'русская классическая проза');
INSERT INTO genre_names (group_id, code, name) VALUES (11, 'prose_su_classics', 'советская классическая проза');
INSERT INTO genre_names (group_id, code, name) VALUES (11, 'russian_contemporary', 'российская современная проза');
INSERT INTO genre_names (group_id, code, name) VALUES (11, 'short_story', 'рассказы');
INSERT INTO genre_names (group_id, code, name) VALUES (11, 'essay', 'эссе');
INSERT INTO genre_names (group_id, code, name) VALUES (11, 'narrative', 'повести');
INSERT INTO genre_names (group_id, code, name) VALUES (11, 'prose', 'прочая проза');

INSERT INTO genre_names (group_id, code, name) VALUES (12, 'ref_dict', 'словари');
INSERT INTO genre_names (group_id, code, name) VALUES (12, 'ref_encyc', 'энциклопедии');
INSERT INTO genre_names (group_id, code, name) VALUES (12, 'ref_guide', 'руководства');
INSERT INTO genre_names (group_id, code, name) VALUES (12, 'ref_ref', 'справочники');
INSERT INTO genre_names (group_id, code, name) VALUES (12, 'reference', 'прочая справочная литература');

INSERT INTO genre_names (group_id, code, name) VALUES (13, 'religion_budda', 'буддизм');
INSERT INTO genre_names (group_id, code, name) VALUES (13, 'religion_christianity', 'христианство');
INSERT INTO genre_names (group_id, code, name) VALUES (13, 'religion_catholicism', 'католичество');
INSERT INTO genre_names (group_id, code, name) VALUES (13, 'religion_esoterics', 'эзотерика');
INSERT INTO genre_names (group_id, code, name) VALUES (13, 'religion_islam', 'ислам');
INSERT INTO genre_names (group_id, code, name) VALUES (13, 'religion_judaism', 'иудаизм');
INSERT INTO genre_names (group_id, code, name) VALUES (13, 'religion_orthodoxy', 'православие');
INSERT INTO genre_names (group_id, code, name) VALUES (13, 'religion_paganism', 'язычество');
INSERT INTO genre_names (group_id, code, name) VALUES (13, 'psy_personal', 'самопознание');
INSERT INTO genre_names (group_id, code, name) VALUES (13, 'religion_self', 'самосовершенствование');
INSERT INTO genre_names (group_id, code, name) VALUES (13, 'religion', 'прочая религионая литература');

INSERT INTO genre_names (group_id, code, name) VALUES (14, 'sci_biology', 'биология');
INSERT INTO genre_names (group_id, code, name) VALUES (14, 'sci_business', 'деловая литература');
INSERT INTO genre_names (group_id, code, name) VALUES (14, 'sci_chem', 'химия');
INSERT INTO genre_names (group_id, code, name) VALUES (14, 'sci_cosmos', 'космология');
INSERT INTO genre_names (group_id, code, name) VALUES (14, 'sci_culture', 'культурология');
INSERT INTO genre_names (group_id, code, name) VALUES (14, 'sci_economic', 'экономика');
INSERT INTO genre_names (group_id, code, name) VALUES (14, 'sci_history',  'история');
INSERT INTO genre_names (group_id, code, name) VALUES (14, 'sci_juris', 'юриспруденция');
INSERT INTO genre_names (group_id, code, name) VALUES (14, 'sci_linguistic', 'языкознание');
INSERT INTO genre_names (group_id, code, name) VALUES (14, 'sci_math', 'математика');
INSERT INTO genre_names (group_id, code, name) VALUES (14, 'sci_medicine', 'медицина');
INSERT INTO genre_names (group_id, code, name) VALUES (14, 'sci_pedagogy', 'педагогика');
INSERT INTO genre_names (group_id, code, name) VALUES (14, 'sci_philology', 'филология');
INSERT INTO genre_names (group_id, code, name) VALUES (14, 'sci_philosophy', 'философия');
INSERT INTO genre_names (group_id, code, name) VALUES (14, 'management', 'менеджмент');
INSERT INTO genre_names (group_id, code, name) VALUES (14, 'marketing', 'маркетинг');
INSERT INTO genre_names (group_id, code, name) VALUES (14, 'accounting', 'бухгалтерия и учет');
INSERT INTO genre_names (group_id, code, name) VALUES (14, 'economics', 'экономика');
INSERT INTO genre_names (group_id, code, name) VALUES (14, 'sci_phys', 'физика');
INSERT INTO genre_names (group_id, code, name) VALUES (14, 'nonf_military', 'военное дело');
INSERT INTO genre_names (group_id, code, name) VALUES (14, 'sci_politics', 'политика');
INSERT INTO genre_names (group_id, code, name) VALUES (14, 'sci_popular', 'научно-популярная литература');
INSERT INTO genre_names (group_id, code, name) VALUES (14, 'sci_psychology', 'психология');
INSERT INTO genre_names (group_id, code, name) VALUES (14, 'sci_religion', 'религиоведение');
INSERT INTO genre_names (group_id, code, name) VALUES (14, 'sci_sociology', 'социология');
INSERT INTO genre_names (group_id, code, name) VALUES (14, 'sci_tech', 'технические науки');
INSERT INTO genre_names (group_id, code, name) VALUES (14, 'sci_transport', 'транспортные науки');
INSERT INTO genre_names (group_id, code, name) VALUES (14, 'architecture', 'архитектура');
INSERT INTO genre_names (group_id, code, name) VALUES (14, 'science', 'прочая научная литература');

INSERT INTO genre_names (group_id, code, name) VALUES (15, 'humor', 'прочий юмор');
INSERT INTO genre_names (group_id, code, name) VALUES (15, 'humor_anecdote', 'анекдоты');
INSERT INTO genre_names (group_id, code, name) VALUES (15, 'humor_fantasy', 'юмористическая фэнтези');
INSERT INTO genre_names (group_id, code, name) VALUES (15, 'sf_humor', 'юмористическая фантастика');
INSERT INTO genre_names (group_id, code, name) VALUES (15, 'humor_prose', 'юмористическая проза');
INSERT INTO genre_names (group_id, code, name) VALUES (15, 'humor_verse', 'юмористические стихи');
INSERT INTO genre_names (group_id, code, name) VALUES (15, 'foreign_humor', 'юмор в зарубежной литературе');
INSERT INTO genre_names (group_id, code, name) VALUES (15, 'humor_satire', 'юмор и сатира');


INSERT INTO genre_names (group_id, code, name) VALUES (0, 'foreign_adventure', 'foreign_adventure');
INSERT INTO genre_names (group_id, code, name) VALUES (0, 'foreign_antique', 'foreign_antique');
INSERT INTO genre_names (group_id, code, name) VALUES (0, 'foreign_business', 'foreign_business');
INSERT INTO genre_names (group_id, code, name) VALUES (0, 'foreign_children', 'foreign_children');
INSERT INTO genre_names (group_id, code, name) VALUES (0, 'foreign_contemporary', 'foreign_contemporary');
INSERT INTO genre_names (group_id, code, name) VALUES (0, 'foreign_detective', 'foreign_detective');
INSERT INTO genre_names (group_id, code, name) VALUES (0, 'foreign_dramaturgy', 'foreign_dramaturgy');
INSERT INTO genre_names (group_id, code, name) VALUES (0, 'foreign_edu', 'foreign_edu');
INSERT INTO genre_names (group_id, code, name) VALUES (0, 'foreign_fantasy', 'foreign_fantasy');
INSERT INTO genre_names (group_id, code, name) VALUES (0, 'foreign_home', 'foreign_home');
INSERT INTO genre_names (group_id, code, name) VALUES (0, 'foreign_language', 'foreign_language');
INSERT INTO genre_names (group_id, code, name) VALUES (0, 'foreign_prose', 'foreign_prose');
INSERT INTO genre_names (group_id, code, name) VALUES (0, 'foreign_poetry', 'foreign_poetry');
INSERT INTO genre_names (group_id, code, name) VALUES (0, 'foreign_psychology', 'foreign_psychology');
INSERT INTO genre_names (group_id, code, name) VALUES (0, 'foreign_publicism', 'foreign_publicism');
INSERT INTO genre_names (group_id, code, name) VALUES (0, 'foreign_religion', 'foreign_religion');
INSERT INTO genre_names (group_id, code, name) VALUES (0, 'foreign_sf', 'foreign_sf');
INSERT INTO genre_names (group_id, code, name) VALUES (0, 'geo_guides', 'geo_guides');
INSERT INTO genre_names (group_id, code, name) VALUES (0, 'great_story', 'great_story');
INSERT INTO genre_names (group_id, code, name) VALUES (0, 'love_fantasy', 'love_fantasy');
INSERT INTO genre_names (group_id, code, name) VALUES (0, 'aphorism_quote', 'aphorism_quote');
INSERT INTO genre_names (group_id, code, name) VALUES (0, 'job_hunting', 'job_hunting');
INSERT INTO genre_names (group_id, code, name) VALUES (0, 'magician_book', 'magician_book');
INSERT INTO genre_names (group_id, code, name) VALUES (0, 'music_dancing', 'music_dancing');
INSERT INTO genre_names (group_id, code, name) VALUES (0, 'network_literature', 'network_literature');
INSERT INTO genre_names (group_id, code, name) VALUES (0, 'org_behavior', 'org_behavior');
INSERT INTO genre_names (group_id, code, name) VALUES (0, 'personal_finance', 'personal_finance');
INSERT INTO genre_names (group_id, code, name) VALUES (0, 'proce', 'proce');
INSERT INTO genre_names (group_id, code, name) VALUES (0, 'psy_childs', 'psy_childs');
INSERT INTO genre_names (group_id, code, name) VALUES (0, 'psy_generic', 'psy_generic');
INSERT INTO genre_names (group_id, code, name) VALUES (0, 'psy_sex_and_family', 'psy_sex_and_family');
INSERT INTO genre_names (group_id, code, name) VALUES (0, 'psy_social', 'psy_social');
INSERT INTO genre_names (group_id, code, name) VALUES (0, 'psy_theraphy', 'psy_theraphy');
INSERT INTO genre_names (group_id, code, name) VALUES (0, 'romance_fantasy', 'romance_fantasy');
INSERT INTO genre_names (group_id, code, name) VALUES (0, 'romance_sf', 'romance_sf');
INSERT INTO genre_names (group_id, code, name) VALUES (0, 'screenplays', 'screenplays');
INSERT INTO genre_names (group_id, code, name) VALUES (0, 'sketch', 'sketch');
INSERT INTO genre_names (group_id, code, name) VALUES (0, 'stock', 'stock');
INSERT INTO genre_names (group_id, code, name) VALUES (0, 'story', 'story');
INSERT INTO genre_names (group_id, code, name) VALUES (0, 'upbringing_book', 'upbringing_book');
INSERT INTO genre_names (group_id, code, name) VALUES (0, 'visual_arts', 'visual_arts');
INSERT INTO genre_names (group_id, code, name) VALUES (0, 'ya', 'ya');

INSERT INTO genre_synonyms (code, synonym_id) SELECT 'litrpg', id FROM genre_names WHERE code = 'sf_litrpg';
INSERT INTO genre_synonyms (code, synonym_id) SELECT 'literature_short', id FROM genre_names WHERE code = 'short_story';
INSERT INTO genre_synonyms (code, synonym_id) SELECT 'sociology_book', id FROM genre_names WHERE code = 'sci_sociology';
INSERT INTO genre_synonyms (code, synonym_id) SELECT 'sci_social_studies', id FROM genre_names WHERE code = 'sci_sociology';
INSERT INTO genre_synonyms (code, synonym_id) SELECT 'popular_business', id FROM genre_names WHERE code = 'sci_popular';
INSERT INTO genre_synonyms (code, synonym_id) SELECT 'sf_etc', id FROM genre_names WHERE code = 'sf';
INSERT INTO genre_synonyms (code, synonym_id) SELECT 'фантастика', id FROM genre_names WHERE code = 'sf';
INSERT INTO genre_synonyms (code, synonym_id) SELECT 'fantastic', id FROM genre_names WHERE code = 'sf';
INSERT INTO genre_synonyms (code, synonym_id) SELECT 'sf_mystic', id FROM genre_names WHERE code = 'sf_horror';
INSERT INTO genre_synonyms (code, synonym_id) SELECT 'vampire_book', id FROM genre_names WHERE code = 'sf_horror';
INSERT INTO genre_synonyms (code, synonym_id) SELECT 'sf_irony', id FROM genre_names WHERE code = 'sf_humor';
INSERT INTO genre_synonyms (code, synonym_id) SELECT 'story', id FROM genre_names WHERE code = 'short_story';
INSERT INTO genre_synonyms (code, synonym_id) SELECT 'russian_fantasy', id FROM genre_names WHERE code = 'sf_fantasy';
INSERT INTO genre_synonyms (code, synonym_id) SELECT 'фэнтези', id FROM genre_names WHERE code = 'sf_fantasy';
INSERT INTO genre_synonyms (code, synonym_id) SELECT 'fantasy', id FROM genre_names WHERE code = 'sf_fantasy';
INSERT INTO genre_synonyms (code, synonym_id) SELECT 'fantasy_fight', id FROM genre_names WHERE code = 'sf_fantasy';
INSERT INTO genre_synonyms (code, synonym_id) SELECT 'historical_fantasy', id FROM genre_names WHERE code = 'sf_fantasy';
INSERT INTO genre_synonyms (code, synonym_id) SELECT 'pedagogy_book', id FROM genre_names WHERE code = 'sci_pedagogy';
INSERT INTO genre_synonyms (code, synonym_id) SELECT 'military_history', id FROM genre_names WHERE code = 'nonf_military';
INSERT INTO genre_synonyms (code, synonym_id) SELECT 'military_special', id FROM genre_names WHERE code = 'nonf_military';
INSERT INTO genre_synonyms (code, synonym_id) SELECT 'military_weapon', id FROM genre_names WHERE code = 'nonf_military';
INSERT INTO genre_synonyms (code, synonym_id) SELECT 'roman', id FROM genre_names WHERE code = 'prose';
INSERT INTO genre_synonyms (code, synonym_id) SELECT 'drama', id FROM genre_names WHERE code = 'dramaturgy';
INSERT INTO genre_synonyms (code, synonym_id) SELECT 'folk_tale', id FROM genre_names WHERE code = 'child_tale';
INSERT INTO genre_synonyms (code, synonym_id) SELECT 'сказка', id FROM genre_names WHERE code = 'child_tale';
INSERT INTO genre_synonyms (code, synonym_id) SELECT 'fantasy_alt_hist', id FROM genre_names WHERE code = 'sf_history';
INSERT INTO genre_synonyms (code, synonym_id) SELECT 'essays', id FROM genre_names WHERE code = 'essay';
INSERT INTO genre_synonyms (code, synonym_id) SELECT 'иронический детектив', id FROM genre_names WHERE code = 'det_irony';
INSERT INTO genre_synonyms (code, synonym_id) SELECT 'sf_cyber_punk', id FROM genre_names WHERE code = 'sf_cyberpunk';
INSERT INTO genre_synonyms (code, synonym_id) SELECT 'city_fantasy', id FROM genre_names WHERE code = 'sf_fantasy_city';
INSERT INTO genre_synonyms (code, synonym_id) SELECT 'religion_rel', id FROM genre_names WHERE code = 'religion';
INSERT INTO genre_synonyms (code, synonym_id) SELECT 'попаданцы', id FROM genre_names WHERE code = 'popadanec';
INSERT INTO genre_synonyms (code, synonym_id) SELECT 'Фэнтези', id FROM genre_names WHERE code = 'sf_fantasy';
INSERT INTO genre_synonyms (code, synonym_id) SELECT '', id FROM genre_names WHERE code = 'unknown';
INSERT INTO genre_synonyms (code, synonym_id) SELECT 'unrecognised', id FROM genre_names WHERE code = 'unknown';
INSERT INTO genre_synonyms (code, synonym_id) SELECT 'Приключенческое фэнтези', id FROM genre_names WHERE code = 'sf_fantasy';
INSERT INTO genre_synonyms (code, synonym_id) SELECT 'theatre', id FROM genre_names WHERE code = 'cinema_theatre';
INSERT INTO genre_synonyms (code, synonym_id) SELECT 'geography_book', id FROM genre_names WHERE code = 'adv_geo';
INSERT INTO genre_synonyms (code, synonym_id) SELECT 'prose_magic', id FROM genre_names WHERE code = 'sf_fantasy';
INSERT INTO genre_synonyms (code, synonym_id) SELECT 'ЛитРПГ', id FROM genre_names WHERE code = 'sf_litrpg';
INSERT INTO genre_synonyms (code, synonym_id) SELECT 'sf_writing', id FROM genre_names WHERE code = 'sf';
INSERT INTO genre_synonyms (code, synonym_id) SELECT 'literature_19', id FROM genre_names WHERE code = 'prose';
INSERT INTO genre_synonyms (code, synonym_id) SELECT 'literature_20', id FROM genre_names WHERE code = 'prose';

/*
SELECT
	count(gv.book_id) CNT,
	gv.genre_name AS orig

FROM genres_view gv
LEFT JOIN genre_synonyms gs ON (gs.code = gv.genre_name)
LEFT JOIN genre_names n1 ON (n1.code = gv.genre_name)
LEFT JOIN genre_groups g1  ON (g1.id = n1.group_id)
LEFT JOIN genre_names n2 ON (n2.id = gs.synonym_id)
LEFT JOIN genre_groups g2  ON (g2.id = n2.group_id)

WHERE orig IS NOT NULL AND orig != '' AND ifnull(g1.name, g2.name) IS NULL

GROUP BY orig
ORDER BY 1 DESC

*/

/*
SELECT
	gv.book_id,
	ifnull(g1.name, g2.name) AS genre_group,
	ifnull(n1.name, n2.name) AS genre_name,
	gv.genre_name AS orig

FROM genres_view gv
LEFT JOIN genre_synonyms gs ON (gs.code = gv.genre_name)
LEFT JOIN genre_names n1 ON (n1.code = gv.genre_name)
LEFT JOIN genre_groups g1  ON (g1.id = n1.group_id)
LEFT JOIN genre_names n2 ON (n2.id = gs.synonym_id)
LEFT JOIN genre_groups g2  ON (g2.id = n2.group_id)

WHERE orig IS NOT NULL AND orig != '' AND ifnull(g1.name, g2.name) IS NULL
ORDER BY 4

*/



