// @generated automatically by Diesel CLI.

diesel::table! {
    first_group_member (id) {
        id -> Int8,
        title -> Varchar,
        file_blob -> Bytea,
    }
}

diesel::table! {
    pair_vote (id) {
        id -> Int8,
        first_member_id -> Int8,
        second_member_id -> Int8,
        diff -> Int8,
        subject -> Text,
    }
}

diesel::table! {
    second_group_member (id) {
        id -> Int8,
        title -> Varchar,
        file_blob -> Bytea,
    }
}

diesel::table! {
    test_table (id) {
        id -> Int8,
        title -> Varchar,
        file_blob -> Bytea,
    }
}

diesel::table! {
    voting_result (id) {
        id -> Int8,
        first_member_id -> Int8,
        second_member_id -> Int8,
        diff -> Int8,
    }
}

diesel::joinable!(pair_vote -> first_group_member (first_member_id));
diesel::joinable!(pair_vote -> second_group_member (second_member_id));
diesel::joinable!(voting_result -> first_group_member (first_member_id));
diesel::joinable!(voting_result -> second_group_member (second_member_id));

diesel::allow_tables_to_appear_in_same_query!(first_group_member, pair_vote, second_group_member, test_table, voting_result,);
