-- -- Add migration script here

-- CREATE TABLE usr (
--     id SERIAL PRIMARY KEY,
--     -- might be better to use varchar
--     user_id TEXT NOT NULL UNIQUE
-- );

-- -- bridge table
-- CREATE TABLE usr_event (
--     user_id TEXT NOT NULL,
--     event_id uuid NOT NULL,
--     PRIMARY KEY(user_id, event_id),
--     CONSTRAINT fk_usr FOREIGN KEY(user_id) REFERENCES usr(user_id),
--     CONSTRAINT fk_event FOREIGN KEY(event_id) REFERENCES event(id),
-- );

CREATE TABLE event (
    id uuid NOT NULL PRIMARY KEY,
    user_id varchar(32)[] NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    -- NEED USER MAPPING
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz
);