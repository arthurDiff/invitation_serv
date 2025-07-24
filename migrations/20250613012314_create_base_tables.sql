CREATE OR REPLACE FUNCTION auto_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ language 'plpgsql'; 

-- ENUMS
CREATE TYPE member_role AS ENUM ('owner', 'facilitator', 'attendee');

-- EVENT TABLE
CREATE TABLE event (
    id uuid NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    description TEXT,
    budget real DEFAULT 0,
    starts_at timestamptz,
    ends_at timestamptz,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz
);

CREATE TRIGGER update_event_updated_at BEFORE UPDATE ON event FOR EACH ROW EXECUTE PROCEDURE auto_updated_at();
-- EVENT TABLE

-- MEMBER TABLE
CREATE TABLE member(
    id uuid NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id varchar(32) NOT NULL,
    event_id uuid NOT NULL,
    role member_role NOT NULL DEFAULT 'attendee'::member_role,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz,
    CONSTRAINT fk_event FOREIGN KEY(event_id) REFERENCES event(id) ON DELETE CASCADE
);

CREATE TRIGGER update_member_updated_at BEFORE UPDATE ON member FOR EACH ROW EXECUTE PROCEDURE auto_updated_at();
-- MEMBER TABLE

-- INVITE TABLE
CREATE TABLE invite(
    id uuid NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id varchar(32),
    email text,
    accepted BOOLEAN NOT NULL DEFAULT FALSE,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz,
    event_id uuid NOT NULL,
    CONSTRAINT fk_event FOREIGN KEY(event_id) REFERENCES event(id) ON DELETE CASCADE
);
-- INVITE TABLE

-- DESTINATION TABLE
CREATE TABLE destination (
    id uuid NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    loc_id uuid NOT NULL UNIQUE,
    name TEXT NOT NULL,
    selected BOOLEAN DEFAULT FALSE,
    starts_at timestamptz,
    ends_at timestamptz,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz,
    event_id uuid NOT NULL,
    member_id uuid NOT NULL,
    CONSTRAINT fk_event FOREIGN KEY(event_id) REFERENCES event(id) ON DELETE CASCADE,
    CONSTRAINT fk_member FOREIGN KEY(member_id) REFERENCES member(id)
);

CREATE TRIGGER update_destination_updated_at BEFORE UPDATE ON destination FOR EACH ROW EXECUTE PROCEDURE auto_updated_at();
-- DESTINATION TABLE

-- LOCATION TABLE
CREATE TABLE location(
    id uuid NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    street_1 TEXT NOT NULL,
    street_2 TEXT,
    province TEXT NOT NULL,
    postal_code TEXT NOT NULL,
    country TEXT NOT NULL,
    dest_id uuid UNIQUE,
    dest_loc_id uuid,
    member_id uuid UNIQUE,
    CONSTRAINT fk_destination FOREIGN KEY(dest_id) REFERENCES destination(id) ON DELETE CASCADE,
    CONSTRAINT fk_destination_loc FOREIGN KEY(dest_loc_id) REFERENCES destination(loc_id) ON DELETE CASCADE,
    CONSTRAINT fk_member FOREIGN KEY(member_id) REFERENCES member(id) ON DELETE CASCADE
);
-- LOCATION TABLE