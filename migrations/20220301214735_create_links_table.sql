create table links
(
    id          TEXT    not null
        constraint links_pk
            primary key,
    redirect_to TEXT    not null,
    max_uses    integer not null,
    invocations integer not null,
    created_at  integer not null,
    valid_for   integer not null
);