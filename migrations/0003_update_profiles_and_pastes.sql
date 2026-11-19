-- Migration number: 0003 	 2026-11-19T09:03:01.668Z

-- 1. Create profiles with new auto-generated IDs
CREATE TABLE profiles_new (
    profile_id INTEGER PRIMARY KEY AUTOINCREMENT,
    github_id INTEGER NOT NULL UNIQUE,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 2. Transfer old profile_id values into github_id
-- New profile_id values are generated automatically
INSERT INTO profiles_new (github_id)
SELECT profile_id
FROM profiles
ORDER BY profile_id;

-- 3. Create pastes with the new foreign key
-- and automatic creation timestamp
CREATE TABLE pastes_new (
    paste_id TEXT PRIMARY KEY,
    author_id INTEGER,
    r2_key TEXT NOT NULL UNIQUE,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT fk_pastes_profiles
        FOREIGN KEY (author_id)
        REFERENCES profiles_new(profile_id)
        ON DELETE SET NULL
);

-- 4. Transfer pastes and map old GitHub IDs
-- to the newly generated profile IDs
INSERT INTO pastes_new (paste_id, author_id, r2_key)
SELECT
    p.paste_id,
    pn.profile_id,
    p.r2_key
FROM pastes AS p
LEFT JOIN profiles_new AS pn ON pn.github_id = p.author_id;

-- 5. Remove old tables
DROP TABLE pastes;
DROP TABLE profiles;

-- 6. Rename new tables
ALTER TABLE profiles_new RENAME TO profiles;
ALTER TABLE pastes_new RENAME TO pastes;

-- 7. Restore author index
CREATE INDEX idx_pastes_author_id
    ON pastes(author_id);