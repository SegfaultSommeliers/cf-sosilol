-- Migration number: 0004

CREATE TABLE pastes_new (
    paste_id TEXT PRIMARY KEY,
    author_id INTEGER,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT fk_pastes_profiles
        FOREIGN KEY (author_id)
        REFERENCES profiles(profile_id)
        ON DELETE SET NULL
);

INSERT INTO pastes_new (paste_id, author_id, created_at)
SELECT paste_id, author_id, created_at FROM pastes;

DROP TABLE pastes;
ALTER TABLE pastes_new RENAME TO pastes;

CREATE INDEX idx_pastes_author_id ON pastes(author_id);
