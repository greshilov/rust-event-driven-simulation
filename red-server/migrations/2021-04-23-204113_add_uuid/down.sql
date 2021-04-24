ALTER TABLE scores DROP COLUMN player_uuid;
ALTER TABLE scores RENAME COLUMN player_name TO "name";
DROP INDEX scores_score_desc;
