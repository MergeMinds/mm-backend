CREATE DATABASE mm_db;
\c mm_db;

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE user_role AS ENUM ('admin', 'teacher', 'student');

CREATE TABLE users (
    id uuid NOT NULL PRIMARY KEY,
    email varchar(128) UNIQUE NOT NULL,
    name varchar(32) NOT NULL,
    surname varchar(32) NOT NULL,
    patronymic varchar(32),
    role user_role NOT NULL,
    password bytea NOT NULL
);
