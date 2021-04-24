table! {
    scores (id) {
        id -> Int4,
        player_name -> Varchar,
        score -> Int8,
        created_at -> Timestamptz,
        player_uuid -> Uuid,
    }
}
