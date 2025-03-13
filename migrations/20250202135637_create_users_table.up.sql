create table users (
    id uuid primary key default gen_random_uuid(),
    username varchar(255) not null unique,
    password_hash varchar(255) not null,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp
);