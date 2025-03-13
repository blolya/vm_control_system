create table os_images(
    id uuid primary key default gen_random_uuid(),
    name varchar(255) not null unique,
    path varchar(255) not null unique,
    created_at timestamp not null default current_timestamp
)
