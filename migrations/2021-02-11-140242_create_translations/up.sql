-- Your SQL goes here
CREATE TABLE translations (
    translation_id BIGSERIAL UNIQUE,
    translation_keyword VARCHAR PRIMARY KEY NOT NULL,
    -- Please don't kill me but we store JSON since It's a POC
    translation_result JSON
)
