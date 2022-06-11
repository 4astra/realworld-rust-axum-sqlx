create table if not exists users
(
    id         bigint generated by default as identity,
    username   varchar   not null default '',
    email      varchar   not null default '',
    password   varchar   not null default '',
    bio        varchar   not null default '',
    image      varchar   not null default '',
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp
)