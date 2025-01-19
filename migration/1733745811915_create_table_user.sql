-- public."user" definition

-- Drop table

-- DROP TABLE public."user";

CREATE TABLE public."user"
(
    id         uuid        DEFAULT gen_random_uuid() NOT NULL,
    "name"     varchar NULL,
    birthdate  timestamptz NULL,
    gender     varchar(2) NULL,
    latitude   float8                                NOT NULL,
    longitude  float8                                NOT NULL,
    created_at timestamptz DEFAULT now() NULL,
    CONSTRAINT user_pk PRIMARY KEY (id)
);