create table blobs (
  id bigserial primary key,
  encoded bytea not null,
  filenmae varchar not null,
  content_type varchar not null,
  byte_size varchar not null,
  metadata text not null
);
