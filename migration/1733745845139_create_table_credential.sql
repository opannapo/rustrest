-- public.credential definition

-- Drop table

-- DROP TABLE public.credential;

CREATE TABLE public.credential
(
    username      varchar(255)              NOT NULL,
    password_hash varchar(255)              NOT NULL,
    user_id       uuid                      NOT NULL,
    status        int4        DEFAULT 0 NULL, -- 0 inactive, 1 active
    created_at    timestamptz DEFAULT now() NOT NULL,
    CONSTRAINT credential_pk PRIMARY KEY (username)
);
CREATE INDEX credential_status_idx ON public.credential USING btree (status);
CREATE INDEX credential_user_id_idx ON public.credential USING btree (user_id);

-- Column comments

COMMENT
ON COLUMN public.credential.status IS '0 inactive, 1 active';