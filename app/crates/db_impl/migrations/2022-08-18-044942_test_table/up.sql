-- Your SQL goes here
CREATE TABLE test_table (
    id bigserial NOT NULL,
    title VARCHAR NOT NULL,
    file_blob bytea NOT NULL,
    CONSTRAINT test_table_pkey PRIMARY KEY (id),
    CONSTRAINT test_table_title_unique UNIQUE (title)
);
