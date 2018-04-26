CREATE TABLE users (
  id SERIAL NOT NULL PRIMARY KEY,
  email TEXT NOT NULL,
  username TEXT NOT NULL,
  password TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  UNIQUE (email, username)
);

 INSERT INTO users (id, email, username, password, created_at) VALUES
  (1, 'admin@163.com', 'admin', '$2y$12$yXTjrGePVLBPUH6YVs2f5OsUEGSotZxdL5Uu/70r63I5GtynVVjkK', '2017-09-08 13:00:26.353041'),
  (2, 'aaaa@163.com', 'aaaa', '$2y$12$3lOwd/qun2g.KBQpYz7DQu4HgreLODO4aJgYwFAQNj2AqgS14DAMK', '2017-09-08 13:00:28.353041'),
  (3, 'zzzz@163.com', 'zzzz', '$2y$12$6ofSZ3hpsGtDt6bM0WU0geDgZLLETFUVB6FpMXI61SbAvuQD5RiWK', '2017-09-08 13:00:38.353041');
 SELECT setval('users_id_seq', 3, true);

CREATE TABLE  theme (
  id SERIAL NOT NULL PRIMARY KEY,
  user_id INTEGER NOT NULL,
  category TEXT NOT NULL,
  status INTEGER NOT NULL DEFAULT '0',
  title TEXT NOT NULL,
  content TEXT NOT NULL,
  view_count INTEGER NOT NULL DEFAULT '0',
  comment_count INTEGER NOT NULL DEFAULT '0',
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

 INSERT INTO theme (id, user_id, category, status, title, content, view_count, comment_count, created_at) VALUES
 (1, 1, 'Topic',  0, 'Rust Article', 'Rust 2017 Survey Results', 0, 3, '2017-07-24 23:41:45.672805609'),
 (2, 2, 'Blog', 0, 'The Rust Libz Blitz','This post covers the library team’s major initiative: raising a solid core of the Rust crate ecosystem to a consistent level of completeness and quality. ', 0, 3, '2017-07-23 23:41:45.672805609'),
 (3, 3, 'Help', 0, 'Rust 2017 roadmap', 'This year, the overarching theme is productivity, especially for early-stage Rust users. ', 0, 1, '2017-07-23 23:41:45.672805609'),
 (4, 1, 'Share', 0, 'Incremental Compilation', 'One of the projects that is building on these foundations, and that should help improve compile times a lot for typical workflows, is incremental compilation. ', 0, 1, '2017-07-24 23:41:45.672805609'),
 (5, 2, 'Job', 0, 'Rust jobs','Today we are announcing an alpha version of incremental compilation', 0, 1, '2017-07-23 23:41:45.672805609'),
 (6, 3, 'Office', 0, 'Introducing MIR','MIR is the key to ticking off a number of our highest priorities for Rust', 0, 0, '2017-07-23 23:41:45.672805609'),
 (7, 1, 'Announce',  0, 'MIR Compilation', 'overarching 2017 Survey Results', 0, 0, '2017-07-24 23:41:45.672805609'),
 (8, 2, 'Topic',  0, 'Results productivity', 'announcing 2017 Survey Results', 0, 0, '2017-07-24 23:41:45.672805609'),
 (9, 1, 'Announce',  0, 'One Survey', 'overarching 2017 Survey Results', 0, 0, '2017-07-24 23:41:45.672805609'),
 (10, 2, 'Share',  0, 'Blitz productivity', 'announcing 2017 Survey Results', 0, 0, '2017-07-24 23:41:45.672805609'),
 (11, 1, 'Announce',  0, 'Survey ticking', 'overarching 2017 Survey Results', 0, 0, '2017-07-24 23:41:45.672805609');
 SELECT setval('theme_id_seq', 11, true);

 CREATE TABLE  comment (
  id SERIAL NOT NULL PRIMARY KEY,
  theme_id integer NOT NULL,
  user_id integer NOT NULL,
  content text NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

 INSERT INTO comment (id, theme_id, user_id, content, created_at) VALUES
 (1, 1, 1, 'Faster execution time', '2017-07-23 23:41:45.672805609'),
 (2, 1, 1, 'Faster compilation time', '2017-07-23 23:41:45.672805609'),
 (3, 3, 2, 'More precise type checking.', '2017-07-23 23:41:45.672805609'),
 (4, 2, 2, 'Eliminating redundancy！', '2017-07-23 23:41:45.672805609'),
 (5, 4, 2, 'Raising ambitions！', '2017-07-23 23:41:45.672805609'),
 (6, 5, 2, 'MIR construction is type-driven', '2017-07-23 23:41:45.672805609'),
 (7, 2, 2, 'Some MIR primitives are more powerful than the structured construct they replace', '2017-07-23 23:41:45.672805609'),
 (8, 2, 2, 'MIR makes all types explicit', '2017-07-23 23:41:45.672805609'),
 (9, 1, 1, 'Faster execution time', '2017-07-23 23:41:45.672805609');
  SELECT setval('comment_id_seq', 9, true);

  CREATE TABLE message (
  id SERIAL NOT NULL PRIMARY KEY,
  theme_id integer NOT NULL,
  comment_id integer NOT NULL,
  from_user_id integer NOT NULL,
  to_user_id integer NOT NULL,
  content text NOT NULL,
  type integer NOT NULL DEFAULT '0',
  status integer NOT NULL DEFAULT '0',
  created_at timestamp with time zone NOT NULL
);