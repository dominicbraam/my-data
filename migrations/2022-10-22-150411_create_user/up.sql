CREATE TABLE `user` (
  `id` int unsigned NOT NULL AUTO_INCREMENT PRIMARY KEY,
  `username` varchar(20) NOT NULL,
  `first_name` varchar(20) NOT NULL,
  `last_name` varchar(20) NOT NULL,
  `dob` DATE NOT NULL,
  CONSTRAINT UC_user UNIQUE (username)
);
