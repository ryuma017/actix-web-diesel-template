CREATE TABLE users(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    name TEXT NOT NULL,
    created_at timestamp NOT NULL
);