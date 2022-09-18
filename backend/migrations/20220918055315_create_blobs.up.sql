create table blobs (
  id serial primary key,
  file_data bytea not null,
  filenmae varchar not null,
  content_type varchar not null,
  byte_size varchar not null,
  metadata text not null
);
