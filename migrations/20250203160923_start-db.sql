-- Add migration script here
-- habilitar la extensión uuid
create extension if not exists "uuid-ossp";


create table space(
    id uuid primary key default uuid_generate_v4(),
    name varchar(255) not null,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp
);
-- crear las tablas con uuids desde el inicio
create table medical_society (
    id uuid primary key default uuid_generate_v4(),
    name varchar(255) not null,
    emergency_phone varchar(255) not null,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp
);

create table members (
    id uuid primary key default uuid_generate_v4(),
    name varchar(255) not null,
    lastname varchar(255) not null,
    ci varchar(255) not null,
    birth_date date not null,
    phone varchar(255) not null,
    tutor_name varchar(255),
    tutor_lastname varchar(255),
    tutor_phone varchar(255),
    observation text,
    medical_society_id uuid not null references medical_society(id),
    address varchar(255) not null,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp
);

create table dues (
    id uuid primary key default uuid_generate_v4(),
    member_id uuid not null references members(id),
    amount decimal(10, 2) not null,
    payment_date date not null,
    month int not null,
    year int not null,
    is_payed boolean not null default false,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp
);

create table activities (
    id uuid primary key default uuid_generate_v4(),
    name varchar(255) not null,
    category varchar(255),
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp
);

create table members_activities (
    id uuid primary key default uuid_generate_v4(),
    member_id uuid not null references members(id),
    activity_id uuid not null references activities(id),
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp
);

create table activities_schedule (
    id uuid primary key default uuid_generate_v4(),
    activity_id uuid not null references activities(id),
    day varchar(255) not null,
    start_time time not null,
    end_time time not null,
    space_id uuid not null references space(id),
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp
);


create table rents (
    id uuid primary key default uuid_generate_v4(),
    full_name varchar(255) not null,
    phone varchar(255) not null,
    start_time time not null,
    end_time time not null,
    space_id uuid not null references space(id),
    cost decimal(10, 2) not null,
    payment_date date not null,
    is_payed boolean not null default false,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp
);

create table users (
    id uuid primary key default uuid_generate_v4(),
    name varchar(255) not null,
    rolename varchar(255) not null,
    email varchar(255) not null unique,
    password varchar(255) not null,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp
);

create table employees (
    id uuid primary key default uuid_generate_v4(),
    name varchar(255) not null,
    lastname varchar(255) not null,
    ci varchar(255) not null,
    phone varchar(255) not null,
    address varchar(255) not null,
    medical_society_id uuid not null references medical_society(id),
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp
);

create table employees_payments (
    id uuid primary key default uuid_generate_v4(),
    employee_id uuid not null references employees(id),
    amount decimal(10, 2) not null,
    payment_date date not null,
    month int not null,
    year int not null,
    is_payed boolean not null default false,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp
);