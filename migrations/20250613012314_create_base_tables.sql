-- -- Add migration script here

CREATE OR REPLACE FUNCTION auto_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ language 'plpgsql'; 

CREATE TABLE event (
    id uuid NOT NULL PRIMARY KEY,
    user_id varchar(32)[] NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    budget real,
    daterange daterange,
    -- NEED USER MAPPING
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz
);

CREATE TRIGGER update_event_updated_at BEFORE UPDATE ON event FOR EACH ROW EXECUTE PROCEDURE auto_updated_at();