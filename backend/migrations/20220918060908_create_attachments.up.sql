create table attachments (
  id varchar primary key,
  record_type varchar,
  record_id varchar,
  record_name varchar,
  blob_id varchar,
  created_at timestamptz not null
);
