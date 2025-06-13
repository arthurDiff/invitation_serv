-- Add migration script here
CREATE TABLE event{
    id uuid NOT NULL PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    -- NEED USER MAPPING
    created_at timestamptz NOT NULL DEFAULT now()
}