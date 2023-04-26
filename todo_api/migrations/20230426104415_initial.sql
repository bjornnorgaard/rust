create table todos
(
    id       uuid primary key ,
    title    varchar(100)             not null,
    complete boolean                  not null default false,
    created  timestamp with time zone not null default now()
);
