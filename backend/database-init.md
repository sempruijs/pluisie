CREATE TABLE users (
    user_id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password TEXT NOT NULL,
    is_super BOOLEAN NOT NULL,
    iva TEXT NOT NULL,
    phone_number TEXT NOT NULL,
    date_of_birth TEXT NOT NULL
);

CREATE TABLE organisations (
    org_id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    picture TEXT,
    description TEXT
);

CREATE TABLE role_history (
    role_id SERIAL PRIMARY KEY,
    date DATE NOT NULL,
    user_id UUID REFERENCES users(user_id),
    org_id UUID REFERENCES organisations(org_id),
    is_admin BOOLEAN
);

CREATE TABLE timeslots (
    timeslot_id UUID PRIMARY KEY,
    created TIMESTAMPTZ NOT NULL DEFAULT now(),
    org_id UUID REFERENCES organisations(org_id),
    user_id UUID REFERENCES users(user_id),
    date DATE NOT NULL,
    hour SMALLINT NOT NULL CHECK (hour >= 0 AND hour <= 23),
    is_enrolled BOOLEAN NOT NULL
);

CREATE TABLE access_notifications (
    notification_id SERIAL PRIMARY KEY,
    org_id UUID REFERENCES organisations(org_id),
    user_id UUID REFERENCES users(user_id),
    date DATE NOT NULL,
    is_accepted BOOLEAN,
    description TEXT NOT NULL
);

CREATE TABLE day_detail (
    detail_id SERIAL PRIMARY KEY,
    created TIMESTAMPTZ NOT NULL DEFAULT now(),
    org_id UUID REFERENCES organisations(org_id),
    date DATE NOT NULL,
    description TEXT NOT NULL
);
