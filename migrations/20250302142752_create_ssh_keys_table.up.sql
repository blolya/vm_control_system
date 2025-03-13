create table ssh_keys(
    id uuid primary key default gen_random_uuid(),
    name varchar(255) not null unique,
    key varchar(1024) not null,
    user_id uuid not null,
    created_at timestamp not null default current_timestamp,
    foreign key (user_id) references users(id)
)
