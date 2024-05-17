CREATE DATABASE mm_db;
\c mm_db;

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE user_role AS ENUM ('admin', 'teacher', 'student');
CREATE TYPE attempt_state AS ENUM ('submitted', 'graded');
-- NOTE(nrydanov): Need to think of other types together
CREATE TYPE block_type AS ENUM ('assignment', 'text', 'media', 'quiz');


-- NOTE(nrydanov): Hierarchical organization data
CREATE TABLE organization (
    id uuid PRIMARY KEY,
    name varchar(256) NOT NULL
);

CREATE TABLE faculty (
    id uuid PRIMARY KEY,
    name varchar(256) NOT NULL,
    organization_id uuid NOT NULL REFERENCES organization(id)
);

CREATE TABLE specialty (
    id uuid PRIMARY KEY,
    -- NOTE(nrydanov): Like 03.02.01
    code varchar(8),
    name varchar(256) NOT NULL,
    faculty_id uuid NOT NULL REFERENCES specialty(id)
);

-- NOTE(nrydanov): General user data
CREATE TABLE users (
    id uuid PRIMARY KEY,
    -- Auth data
    email varchar(128) UNIQUE NOT NULL,
    password bytea NOT NULL,
    -- Identity data
    name varchar(32) NOT NULL,
    surname varchar(32) NOT NULL,
    patronymic varchar(32),
    date_of_birth date, 
    -- University data
    -- NOTE(nrydanov): This data should be enough to determine exact academic groups
    -- Service info
    -- NOTE(nrydanov): Can be used to make admin a teacher for a while
    role_override user_role,
    created_at timestamp NOT NULL
);

-- NOTE(nrydanov): Specific student data
CREATE TABLE student (
    user_id uuid NOT NULL REFERENCES users(id),
    specialty_id uuid NOT NULL REFERENCES specialty(id),
    admission_date date NOT NULL,
    expelled boolean NOT NULL
);

-- NOTE(nrydanov): Specific teacher data
CREATE TABLE teacher (
    user_id uuid NOT NULL REFERENCES users(id),
    faculty_id uuid NOT NULL REFERENCES faculty(id),
    promoted_by uuid NOT NULL REFERENCES users(id),
    promoted_at timestamp NOT NULL
    -- TODO(nrydanov): Add additional required teacher data if required
);

-- NOTE(nrydanov): Specific admin data
CREATE table admin (
    user_id uuid NOT NULL REFERENCES users(id),
    promoted_by uuid NOT NULL REFERENCES users(id),
    promoted_at timestamp NOT NULL
);


-- NOTE(nrydanov): General course data
CREATE TABLE course (
    id uuid NOT NULL PRIMARY KEY,
    creator_id uuid NOT NULL REFERENCES users(id),
    short_name varchar(64) NOT NULL,
    -- Service info 
    created_at timestamp NOT NULL
);


-- NOTE(nrydanov): May be helpful to filter out courses for certain faculty
CREATE TABLE course_faculty (
    course_id uuid NOT NULL REFERENCES course(id),
    faculty_id uuid NOT NULL REFERENCES faculty(id)
);

CREATE TABLE groups (
    id uuid NOT NULL PRIMARY KEY,
    name varchar(64) NOT NULL
);

-- NOTE(nrydanov): May be helpful to auto-assign users for one of existing
-- groups within the course
CREATE TABLE groups_password (
    password bytea NOT NULL,
    group_id uuid NOT NULL REFERENCES groups(id)
);



CREATE TABLE course_users_groups (
    user_id uuid NOT NULL REFERENCES users(id),
    course_id uuid NOT NULL REFERENCES course(id),
    group_id uuid NOT NULL REFERENCES groups(id)
);


CREATE TABLE block (
    id uuid NOT NULL PRIMARY KEY,
    type block_type NOT NULL,
    -- NOTE(nrydanov): May be useful to create chain of blocks
    next_block uuid,
    -- NOTE(nrydanov): Contains different JSON data for each type of block
    data jsonb NOT NULL
);


CREATE TABLE task_topic (
    id uuid NOT NULL PRIMARY KEY,
    title varchar(128) NOT NULL
);


CREATE TABLE task_topic_task (
    task_topic_id uuid NOT NULL REFERENCES task_topic(id),
    task_id uuid NOT NULL
);

CREATE TABLE repository (
    id uuid NOT NULL PRIMARY KEY
);

CREATE TABLE task (
    id uuid NOT NULL PRIMARY KEY,
    available_at timestamp,
    deadline_at timestamp,
    max_grade real NOT NULL,
    max_attempts integer NOT NULL,
    lead_time time,
    -- NOTE(nrydanov): Equal to NULL if task doesn't need separate repo
    repo_id uuid REFERENCES repository(id)
);

CREATE TABLE task_git_info (
    task_id uuid NOT NULL REFERENCES task(id),
    repo_id uuid NOT NULL REFERENCES repository(id),
    file_masks varchar(64)[] NOT NULL
);


CREATE TABLE attempt (
    id uuid NOT NULL PRIMARY KEY,
    user_id uuid NOT NULL REFERENCES users(id),
    task_id uuid NOT NULL REFERENCES task(id) 
);

CREATE TABLE attempt_transitions (
    attempt_id uuid NOT NULL REFERENCES attempt(id),
    state attempt_state NOT NULL,
    transition_at timestamp NOT NULL,
    -- Can be used to save additional transition data (unpredictable for now)
    transition_data jsonb
);

-- NOTE(nrydanov): Here we store git information if users used git repository
-- for submitting attempt
CREATE TABLE attempt_git_info (
    attempt_id uuid NOT NULL REFERENCES attempt(id),
    repo_id uuid NOT NULL REFERENCES repository(id),
    commit_hash bytea NOT NULL
);

CREATE TABLE attempt_classic_info (
    attempt_id uuid NOT NULL REFERENCES attempt(id),
    uploaded_files uuid[] NOT NULL
);

CREATE TABLE stored_file (
    id uuid NOT NULL PRIMARY KEY,
    url varchar(64) NOT NULL,
    uploaded_by uuid NOT NULL REFERENCES users(id)
);
    

CREATE TABLE task_block (
    task_id uuid NOT NULL REFERENCES task(id),
    block_id uuid NOT NULL REFERENCES block(id),
    index integer NOT NULL,
    hidden boolean NOT NULL
);


CREATE TABLE course_task (
    task_id uuid NOT NULL REFERENCES task(id),
    course_id uuid NOT NULL REFERENCES course(id)
);


CREATE TABLE users_course (
    course_id uuid NOT NULL REFERENCES course(id),
    user_id uuid NOT NULL REFERENCES users(id)
);


CREATE TABLE users_role (
    user_id uuid NOT NULL REFERENCES users(id),
    role user_role NOT NULL
);
