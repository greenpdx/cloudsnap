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

CREATE TABLE  themes (
  id SERIAL NOT NULL PRIMARY KEY,
  user_id INTEGER NOT NULL,
  community_id INTEGER NOT NULL,
  theme_status INTEGER NOT NULL DEFAULT '0',
  title TEXT NOT NULL,
  content TEXT NOT NULL,
  view_count INTEGER NOT NULL DEFAULT '0',
  comment_count INTEGER NOT NULL DEFAULT '0',
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

 INSERT INTO themes (id, user_id, community_id, theme_status, title, content, view_count, comment_count, created_at) VALUES
 (1, 1, 1,  0, 'Rust Article', 'Rust 2017 Survey Results', 0, 3, '2017-07-24 23:41:45.353041'),
 (2, 2, 2, 0, 'The Rust Libz Blitz','This post covers the library team’s major initiative: raising a solid core of the Rust crate ecosystem to a consistent level of completeness and quality. ', 0, 3, '2017-07-23 23:41:45.353041'),
 (3, 3, 3, 0, 'Rust 2017 roadmap', 'This year, the overarching theme is productivity, especially for early-stage Rust users. ', 0, 1, '2017-07-23 23:41:45.353041'),
 (4, 1, 4, 0, 'Incremental Compilation', 'One of the projects that is building on these foundations, and that should help improve compile times a lot for typical workflows, is incremental compilation. ', 0, 1, '2017-07-24 23:41:45.353041'),
 (5, 2, 5, 0, 'Rust jobs','Today we are announcing an alpha version of incremental compilation', 0, 1, '2017-07-23 23:41:45.353041'),
 (6, 3, 6, 0, 'Introducing MIR','MIR is the key to ticking off a number of our highest priorities for Rust', 0, 0, '2017-07-23 23:41:45.353041'),
 (7, 1, 7,  0, 'MIR Compilation', 'overarching 2017 Survey Results', 0, 0, '2017-07-24 23:41:45.353041'),
 (8, 2, 8,  0, 'Results productivity', 'announcing 2017 Survey Results', 0, 0, '2017-07-24 23:41:45.353041'),
 (9, 1, 9,  0, 'One Survey', 'overarching 2017 Survey Results', 0, 0, '2017-07-24 23:41:45.353041'),
 (10, 2, 1,  0, 'Blitz productivity', 'announcing 2017 Survey Results', 0, 0, '2017-07-24 23:41:45.353041'),
 (11, 1, 2,  0, 'Survey ticking', 'overarching 2017 Survey Results', 0, 0, '2017-07-24 23:41:45.353041');
 SELECT setval('themes_id_seq', 11, true);

CREATE TABLE  communitys (
  id SERIAL NOT NULL PRIMARY KEY,
  create_user_id INTEGER NOT NULL,
  community_name TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  UNIQUE (community_name)
);

 INSERT INTO communitys (id, create_user_id, community_name, created_at) VALUES
  (1, 1, 'Rust', '2017-09-08 13:00:26.353041'),
  (2, 2, 'Dart', '2017-09-08 13:00:28.353041'),
  (3, 3, 'C++', '2017-09-08 13:00:38.353041'),
  (4, 1, 'Go', '2017-09-08 13:00:26.353041'),
  (5, 2, 'Swift', '2017-09-08 13:00:28.353041'),
  (6, 3, 'JS', '2017-09-08 13:00:38.353041'),
  (7, 1, 'Python', '2017-09-08 13:00:26.353041'),
  (8, 2, 'Java', '2017-09-08 13:00:28.353041'),
  (9, 3, 'Ruby', '2017-09-08 13:00:38.353041');
 SELECT setval('communitys_id_seq', 9, true);


 CREATE TABLE  comments (
  id SERIAL NOT NULL PRIMARY KEY,
  theme_id INTEGER NOT NULL,
  user_id INTEGER NOT NULL,
  content TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

 INSERT INTO comments (id, theme_id, user_id, content, created_at) VALUES
 (1, 1, 1, 'Faster execution time', '2017-07-23 23:41:45.353041'),
 (2, 1, 1, 'Faster compilation time', '2017-07-23 23:41:45.353041'),
 (3, 3, 2, 'More precise type checking.', '2017-07-23 23:41:45.353041'),
 (4, 2, 2, 'Eliminating redundancy！', '2017-07-23 23:41:45.353041'),
 (5, 4, 2, 'Raising ambitions！', '2017-07-23 23:41:45.353041'),
 (6, 5, 2, 'MIR construction is type-driven', '2017-07-23 23:41:45.353041'),
 (7, 2, 2, 'Some MIR primitives are more powerful than the structured construct they replace', '2017-07-23 23:41:45.353041'),
 (8, 2, 2, 'MIR makes all types explicit', '2017-07-23 23:41:45.353041'),
 (9, 1, 1, 'Faster execution time', '2017-07-23 23:41:45.353041');
  SELECT setval('comments_id_seq', 9, true);

  CREATE TABLE messages (
  id SERIAL NOT NULL PRIMARY KEY,
  theme_id INTEGER NOT NULL,
  comment_id INTEGER NOT NULL,
  from_user_id INTEGER NOT NULL,
  to_user_id INTEGER NOT NULL,
  content TEXT NOT NULL,
  types INTEGER NOT NULL DEFAULT '0',
  message_status INTEGER NOT NULL DEFAULT '0',
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

  CREATE TABLE joins (
  id SERIAL NOT NULL PRIMARY KEY,
  user_id INTEGER NOT NULL,
  user_role TEXT NOT NULL DEFAULT 'common',
  community_id INTEGER NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

 INSERT INTO joins (id, user_id, user_role, community_id, created_at) VALUES
  (1, 1, 'admin', 1, '2017-09-08 13:00:26.353041'),
  (2, 2, 'admin', 2, '2017-09-08 13:00:28.353041'),
  (3, 3, 'admin', 3, '2017-09-08 13:00:38.353041'),
  (4, 1, 'common', 4, '2017-09-08 13:00:26.353041'),
  (5, 2, 'common', 5, '2017-09-08 13:00:28.353041'),
  (6, 3, 'common', 6, '2017-09-08 13:00:38.353041'),
  (7, 1, 'common', 7, '2017-09-08 13:00:26.353041'),
  (8, 2, 'common', 8, '2017-09-08 13:00:28.353041'),
  (9, 3, 'common', 9, '2017-09-08 13:00:38.353041');
 SELECT setval('joins_id_seq', 9, true);


