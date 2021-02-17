-- Your SQL goes here
INSERT INTO species(name)
VALUES ('Esel');
INSERT INTO species(name)
VALUES ('Kamel');
INSERT INTO species(name)
VALUES ('Schwein');
INSERT INTO species(name)
VALUES ('Kuh');

INSERT INTO zoos(id, street, name)
VALUES (1, 'Bleckstraße 64, 45889 Gelsenkirchen', 'ZOOM Erlebniswelt');
INSERT INTO zoos(id, street, name)
VALUES (2, 'Mergelteichstraße 80, 44225 Dortmund', 'Zoo Dortmund');
INSERT INTO zoos(id, street, name)
VALUES (3, 'Sentruper Str. 315, 48161 Münster', 'Allwetterzoo Münster');

INSERT INTO animals(name, species, zoo_id, age)
VALUES ('Emil', 'Esel', 1, 3);
INSERT INTO animals(name, species, zoo_id, age)
VALUES ('Siegbert', 'Schwein', 2, 4);
INSERT INTO animals(name, species, zoo_id, age)
VALUES ('Kurt', 'Kuh', 1, 7);
