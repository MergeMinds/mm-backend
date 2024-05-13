CREATE DATABASE mm_db;
\c mm_db;

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE user_role AS ENUM ('admin', 'teacher', 'student');
CREATE TYPE attempt_state AS ENUM ('in_progress', 'completed');
-- NOTE(nrydanov): Need to think of other types together
CREATE TYPE block_type AS ENUM ('assignment', 'text', 'media', 'quiz');


CREATE TABLE users (
    id uuid NOT NULL PRIMARY KEY,
    -- Auth data
    email varchar(128) UNIQUE NOT NULL,
    password bytea NOT NULL,
    -- Identity data
    name varchar(32) NOT NULL,
    surname varchar(32) NOT NULL,
    patronymic varchar(32),
    date_of_birth date,
    
    -- University data
    -- NOTE(nrydanov): This data should be enough to determine exact academic group
    faculty_id uuid NOT NULL,
    specialty_id uuid NOT NULL,
    admission_date date NOT NULL,
    -- Service info
    -- NOTE(nrydanov): Can be used to make admin a teacher for a while
    role_override user_role,
    created_at timestamp NOT NULL
);


CREATE TABLE course (
    id uuid NOT NULL PRIMARY KEY,
    creator_id uuid NOT NULL,
    name varchar(64) NOT NULL,
    -- Service info 
    created_at timestamp NOT NULL
);


-- NOTE(nrydanov): May be helpful to filter out courses for certain faculty
CREATE TABLE course_faculty (
    course_id uuid NOT NULL,
    faculty_id uuid NOT NULL
);


-- NOTE(nrydanov): May be helpful to auto-assign user for one of existing
-- group within the course
CREATE TABLE group_password (
    password bytea NOT NULL,
    group_id uuid NOT NULL PRIMARY KEY
);


CREATE TABLE user_group (
    id uuid NOT NULL PRIMARY KEY,
    name varchar(64) NOT NULL
);

CREATE TABLE course_user_group (
    user_id uuid NOT NULL,
    course_id uuid NOT NULL,
    group_id uuid NOT NULL
);


CREATE TABLE block (
    id uuid NOT NULL PRIMARY KEY,
    type block_type NOT NULL,
    -- NOTE(nrydanov): May be useful to create chain of blocks
    next_block uuid,
    -- NOTE(nrydanov): Contains different JSON data for each type of block
    data jsonb NOT NULL
);

-- NOTE(nrydanov): group there isn't an academic one, but just multiple tasks
-- on one theme. Maybe there could be a better naming.
CREATE TABLE task_group (
    task_group_id uuid NOT NULL,
    name varchar(64) NOT NULL
);

CREATE TABLE task_group_task (
    task_group_id uuid NOT NULL,
    task_id uuid NOT NULL
);


CREATE TABLE task (
    id uuid NOT NULL PRIMARY KEY,
    available_at timestamp,
    deadline_at timestamp,
    max_grade real NOT NULL,
    max_attempts integer NOT NULL,
    lead_time time NOT NULL,
    -- NOTE(nrydanov): Equal to NULL if task doesn't need separate repo
    repo_id uuid
);

CREATE TABLE task_git_info (
    task_id uuid NOT NULL PRIMARY KEY,
    repo_id uuid NOT NULL,
    file_masks varchar(64)[] NOT NULL
);


CREATE TABLE repository (
    id uuid NOT NULL PRIMARY KEY
);


CREATE TABLE attempt (
    attempt_id uuid NOT NULL PRIMARY KEY,
    user_id uuid NOT NULL,
    task_id uuid NOT NULL 
);

CREATE TABLE attempt_transitions (
    attempt_id uuid NOT NULL,
    state attempt_state NOT NULL,
    transition_at timestamp,
    -- Can be used to save additional transition data (unpredictable for now)
    transition_data jsonb
);

-- NOTE(nrydanov): Here we store git information if user used git repository
-- for submitting attempt
CREATE TABLE attempt_git_info (
    attempt_id uuid NOT NULL,
    repo_id uuid NOT NULL,
    commit_hash bytea NOT NULL
);

CREATE TABLE attempt_classic_info (
    attempt_id uuid NOT NULL,
    uploaded_files uuid[] NOT NULL  
);

CREATE TABLE stored_file (
    id uuid NOT NULL PRIMARY KEY,
    url varchar(64) NOT NULL,
    uploaded_by uuid NOT NULL
);
    

CREATE TABLE task_block (
    task_id uuid NOT NULL,
    block_id uuid NOT NULL,
    index integer NOT NULL,
    hidden boolean NOT NULL
);


CREATE TABLE course_task (
    task_id uuid NOT NULL,
    course_id uuid NOT NULL
);


CREATE TABLE user_course (
    course_id uuid NOT NULL,
    user_id uuid NOT NULL
);


CREATE TABLE user_roles (
    user_id uuid NOT NULL,
    role user_role NOT NULL
);

CREATE TABLE specialty (
    id uuid NOT NULL PRIMARY KEY,
    name varchar(64)
);

CREATE TABLE faculty (
    id uuid NOT NULL PRIMARY KEY,
    name varchar(64)
);


ALTER TABLE users
ADD CONSTRAINT users_specialty_fk1
FOREIGN KEY (specialty_id)
REFERENCES specialty (id);

ALTER TABLE users
ADD CONSTRAINT users_faculty_fk1
FOREIGN KEY (faculty_id)
REFERENCES faculty (id);

ALTER TABLE user_roles
ADD CONSTRAINT user_role_pk
PRIMARY KEY (user_id, role);

ALTER TABLE course
ADD CONSTRAINT course_creator_fk1
FOREIGN KEY (creator_id)
REFERENCES users (id);

ALTER TABLE user_course
ADD CONSTRAINT user_course_pk
PRIMARY KEY (user_id, course_id);

ALTER TABLE user_course
ADD CONSTRAINT user_course_fk1
FOREIGN KEY (user_id)
REFERENCES users (id);

ALTER TABLE user_course
ADD CONSTRAINT user_course_fk2
FOREIGN KEY (course_id)
REFERENCES course (id);


ALTER TABLE course_user_group
ADD CONSTRAINT course_user_group_pk
PRIMARY KEY (user_id, course_id, group_id);


ALTER TABLE course_user_group
ADD CONSTRAINT course_user_group_fk1
FOREIGN KEY (user_id)
REFERENCES users (id);

ALTER TABLE course_user_group
ADD CONSTRAINT course_user_group_fk2
FOREIGN KEY (course_id)
REFERENCES course (id);

ALTER TABLE course_user_group
ADD CONSTRAINT course_user_group_fk3
FOREIGN KEY (group_id)
REFERENCES user_group (id);

ALTER TABLE course_task
ADD CONSTRAINT course_task_pk
PRIMARY KEY (task_id, course_id);

ALTER TABLE course_task
ADD CONSTRAINT course_task_fk1
FOREIGN KEY (course_id)
REFERENCES course (id);

ALTER TABLE course_task
ADD CONSTRAINT course_task_fk2
FOREIGN KEY (task_id)
REFERENCES task (id);

ALTER TABLE task_block
ADD CONSTRAINT task_block_pk
PRIMARY KEY (task_id, block_id);

ALTER TABLE task_block
ADD CONSTRAINT task_block_fk1
FOREIGN KEY (task_id)
REFERENCES task (id);

ALTER TABLE task_block
ADD CONSTRAINT task_block_fk2
FOREIGN KEY (block_id)
REFERENCES block (id);


ALTER TABLE group_password
ADD CONSTRAINT group_password_fk1
FOREIGN KEY (group_id) 
REFERENCES user_group (id);

ALTER TABLE course_faculty
ADD CONSTRAINT course_faculty_pk
PRIMARY KEY (course_id, faculty_id);

ALTER TABLE course_faculty
ADD CONSTRAINT course_faculty_fk1
FOREIGN KEY (course_id)
REFERENCES course (id);

ALTER TABLE course_faculty
ADD CONSTRAINT course_faculty_fk2
FOREIGN KEY (faculty_id)
REFERENCES faculty (id);


ALTER TABLE attempt
ADD CONSTRAINT attempt_user_fk1
FOREIGN KEY (user_id)
REFERENCES users (id);

ALTER TABLE attempt
ADD CONSTRAINT attempt_task_fk2
FOREIGN KEY (task_id)
REFERENCES task (id);

ALTER TABLE task_group_task
ADD CONSTRAINT task_group_task_fk1
FOREIGN KEY (task_id)
REFERENCES task (id);


ALTER TABLE task_git_info
ADD CONSTRAINT task_git_info_fk1
FOREIGN KEY (task_id)
REFERENCES task (id);

ALTER TABLE task_git_info
ADD CONSTRAINT task_git_info_fk2
FOREIGN KEY (repo_id)
REFERENCES repository (id);

ALTER TABLE stored_file
ADD CONSTRAINT stored_file_user_fk
FOREIGN KEY (uploaded_by)
REFERENCES users (id);
