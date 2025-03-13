create table domains(
    id uuid primary key default gen_random_uuid(),
    user_id uuid not null,
    os_image_id uuid not null,
    name varchar(255) not null,
    vcpu_count smallint not null,
    memory_size smallint not null,
    disk_size smallint not null,
    created_at timestamp not null default current_timestamp,
    foreign key (user_id) references users(id),
    foreign key (os_image_id) references os_images(id)
)