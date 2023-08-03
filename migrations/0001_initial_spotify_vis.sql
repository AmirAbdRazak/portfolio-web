
create table artists (
    id uuid primary key default gen_random_uuid(),
    spotify_id varchar(50) not null,
    name varchar(100) not null,
    spotify_url varchar(128),

    created_at timestamp not null default now(),
    updated_at timestamp
);

create table albums (
    id uuid primary key default gen_random_uuid(),
    spotify_id varchar(50) not null,
    title varchar(200) not null,
    album_type varchar(10) not null,
    release_date date,
    total_tracks integer not null,
    spotify_url varchar(128),
    image_url varchar(128),
    duration_ms integer,

    created_at timestamp not null default now(),
    updated_at timestamp
);

CREATE TABLE tracks (
    id uuid primary key default gen_random_uuid(),
    spotify_id varchar(50) not null,
    title varchar(200) not null,
    duration integer not null,
    album_id uuid not null,
    preview_url varchar(128) not null,
    track_number integer not null,
    
    created_at timestamp not null default now(),
    updated_at timestamp,
    FOREIGN KEY (album_id) REFERENCES albums (id) ON DELETE CASCADE
);

create table tracks_artists (
    track_id uuid not null,
    artist_id uuid not null,
    primary key (track_id, artist_id),
    foreign key (track_id) references tracks (id) on delete cascade,
    foreign key (artist_id) references artists (id) on delete cascade
);

create table artists_albums (
    artist_id uuid not null,
    album_id uuid not null,
    primary key (artist_id, album_id),
    foreign key (artist_id) references artists (id) on delete cascade,
    foreign key (album_id) references albums (id) on delete cascade
);