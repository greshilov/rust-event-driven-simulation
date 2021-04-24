ALTER TABLE scores 
    ADD COLUMN player_uuid uuid NOT NULL DEFAULT gen_random_uuid();
ALTER TABLE scores
    RENAME COLUMN "name" TO player_name;
CREATE INDEX scores_score_desc ON scores(score DESC);
