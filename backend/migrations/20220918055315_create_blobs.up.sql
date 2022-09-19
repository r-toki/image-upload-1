create table blobs (
  id varchar primary key,
  data bytea not null,
  name varchar not null,
  content_type varchar not null,
  byte_size varchar not null,
  metadata text not null,
  created_at timestamptz not null
);
