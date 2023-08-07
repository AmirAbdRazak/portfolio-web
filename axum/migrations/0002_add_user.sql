create table users (
    id uuid primary key default gen_random_uuid(),
    spotify_id varchar(50),
    email varchar(100),
    image_url varchar(100),
    registered_unixtime integer,
    playcount integer,
    artist_count integer,
    album_count integer, 
    track_count integer,
    created_at timestamp not null default now(),
    updated_at timestamp
);

create table users_top_artists (
    user_id uuid not null,
    artist_id uuid not null,
    latest_in_leaderboard timestamp,
    playcount integer,
    foreign key (user_id) references users (id) on delete cascade,
    foreign key (artist_id) references artists (id) on delete cascade
);

create table weekly_chart (
    id uuid primary key default gen_random_uuid(),
    chart_from integer,
    chart_to integer
);

alter table artists
add column user_id uuid unique;