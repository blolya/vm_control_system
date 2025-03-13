create table refresh_tokens(
    token uuid primary key default gen_random_uuid(),
    user_id uuid not null unique,
    created_at timestamp not null default current_timestamp,
    expires_at timestamp not null default current_timestamp,
    foreign key (user_id) references users(id)
)