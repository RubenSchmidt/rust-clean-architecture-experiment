create table todos (
  id uuid primary key,
  title text not null,
  completed boolean not null default false
);