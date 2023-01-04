-- Your SQL goes here
CREATE TABLE first_group_member (
    id bigserial NOT NULL,
    title VARCHAR NOT NULL,
    file_blob bytea NOT NULL,
    CONSTRAINT first_group_member_pkey PRIMARY KEY (id),
    CONSTRAINT first_group_member_title_unique UNIQUE (title)
);

CREATE TABLE second_group_member (
    id bigserial NOT NULL,
    title VARCHAR NOT NULL,
    file_blob bytea NOT NULL,
    CONSTRAINT second_group_member_pkey PRIMARY KEY (id),
    CONSTRAINT second_group_member_title_unique UNIQUE (title)
);

-- CREATE TABLE pair (
--     id bigserial NOT NULL,
--     first_member_id int8 NOT NULL,
--     second_member_id int8 NOT NULL,
--     CONSTRAINT pair_pkey PRIMARY KEY (id),
--     CONSTRAINT pair_first_member_fkey FOREIGN KEY (first_member_id)
--         REFERENCES first_group_member(id),
--     CONSTRAINT pair_second_member_fkey FOREIGN KEY (second_member_id)
--         REFERENCES second_group_member(id)
-- );

CREATE TABLE pair_vote (
    id bigserial NOT NULL,
    first_member_id int8 NOT NULL,
    second_member_id int8 NOT NULL,
    diff int8 NOT NULL,
    subject text NOT NULL,
    CONSTRAINT pair_vote_pkey PRIMARY KEY (id),
    CONSTRAINT pair_first_member_fkey FOREIGN KEY (first_member_id)
        REFERENCES first_group_member(id),
    CONSTRAINT pair_second_member_fkey FOREIGN KEY (second_member_id)
        REFERENCES second_group_member(id),
    CONSTRAINT pair_vote_unique UNIQUE (first_member_id, second_member_id, subject)
);

CREATE TABLE voting_result (
    id bigserial NOT NULL,
    first_member_id int8 NOT NULL,
    second_member_id int8 NOT NULL,
    diff int8 NOT NULL,
    CONSTRAINT voting_result_pkey PRIMARY KEY (id),
    CONSTRAINT voting_result_first_member_fkey FOREIGN KEY (first_member_id)
        REFERENCES first_group_member(id),
    CONSTRAINT voting_result_second_member_fkey FOREIGN KEY (second_member_id)
        REFERENCES second_group_member(id),
    CONSTRAINT voting_result_unique UNIQUE (first_member_id, second_member_id)
);
