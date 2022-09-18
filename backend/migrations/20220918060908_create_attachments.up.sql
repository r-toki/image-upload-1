create table attachments (
  id serial primary key,
  record_type varchar not null,
  record_id varchar not null,
  record_name varchar not null,
  created_at timestamptz not null,
  blob_id int references attachments(id) on delete cascade
);
