-- This file should undo anything in `up.sql`
DELETE FROM animals where name in ('Emil', 'Siegbert', 'Kurt');
DELETE FROM zoos where name in ('ZOOM Erlebniswelt', 'Zoo Dortmund', 'Allwetterzoo Münster');
DELETE FROM species where name in ('Esel', 'Kamel', 'Schwein', 'Kuh');
