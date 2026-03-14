-- Add up migration script here
CREATE TABLE events (
    id uuid primary key,
    "type" varchar not null,
    data jsonb not null
);