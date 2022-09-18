create table attachments (
  id serial primary key,
  record_type varchar not null,
  record_id varchar not null,
  record_column_name varchar not null,
  blob_id int references attachments(id) on delete cascade
);
