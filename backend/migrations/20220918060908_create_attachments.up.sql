create table attachments (
  id bigserial primary key,
  record_type varchar not null,
  record_id varchar not null,
  record_column_name varchar not null,
  blob_id bigint references attachments(id) on delete cascade
);
