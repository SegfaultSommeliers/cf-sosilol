-- Migration number: 0002

CREATE TABLE IF NOT EXISTS pastes (
    paste_id TEXT PRIMARY KEY,
    author_id INTEGER,
    r2_key TEXT NOT NULL UNIQUE,

    CONSTRAINT fk_pastes_profiles
        FOREIGN KEY (author_id)
        REFERENCES profiles(profile_id)
        ON DELETE CASCADE
);

CREATE INDEX idx_pastes_author_id ON pastes(author_id);