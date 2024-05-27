create table user
(
    id       integer primary key,
    name     text           not null,
    role     text           not null,
    vocadbId integer unique not null
) strict;