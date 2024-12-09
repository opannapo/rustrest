-- public."user" definition

-- Drop table

-- DROP TABLE public."user";

CREATE TABLE public."user"
(
    id        uuid DEFAULT gen_random_uuid() NOT NULL,
    "name"    varchar NULL,
    birthdate timestamptz NULL,
    gender    varchar NULL,
    CONSTRAINT user_pk PRIMARY KEY (id)
);