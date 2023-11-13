-- Your SQL goes here
CREATE TABLE user (
    "id"   Integer not null primary key AUTOINCREMENT,
    "username" Varchar(255) not null,
    "password" Varchar(255) not null,
    "email" Varchar(100),
    "first_name" Varchar(255),
    "last_name" Varchar(255),
    "is_admin" boolean default false not null
);
