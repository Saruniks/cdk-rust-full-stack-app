#[macro_use]
extern crate diesel;
extern crate dotenv;

use anyhow::Result;
use base64::encode;
use bb8_diesel::DieselConnectionManager;
use diesel::sql_types;
use openapi_client::models::{Member, MemberPair, PostPairVoteRequest, VotingResult, VotingResults};
pub mod pagination;
pub mod schema;
use std::collections::HashMap;

use diesel::{pg::PgConnection, prelude::*};
use schema::*;

use crate::pagination::Paginate;

pub async fn get_member_pair(
    conn: bb8::PooledConnection<'_, DieselConnectionManager<PgConnection>>,
    subject: Option<String>,
) -> Result<MemberPair> {
    let subject = subject.unwrap();
    no_arg_sql_function!(RANDOM, (), "Represents the sql RANDOM() function");
    // sql_function!(fn random() -> Text);

    let res = first_group_member::table
        .inner_join(second_group_member::table.on(true.into_sql::<sql_types::Bool>()))
        .left_join(
            pair_vote::table.on(pair_vote::first_member_id
                .eq(first_group_member::id)
                .and(pair_vote::second_member_id.eq(second_group_member::id))
                .and(pair_vote::subject.eq(subject.clone()))),
        )
        .filter(pair_vote::subject.is_null().or(pair_vote::subject.ne(subject)))
        // .filter(pair_vote::subject.ne(subject.clone()))
        // .filter(pair_vote::subject.is_null())
        .select((
            (first_group_member::id, first_group_member::file_blob),
            (second_group_member::id, second_group_member::file_blob),
        ))
        .order(RANDOM)
        .limit(1)
        .get_result::<((i64, Vec<u8>), (i64, Vec<u8>))>(&*conn)
        .unwrap();

    Ok(MemberPair {
        first_member: Member {
            id: res.0 .0,
            file: encode(res.0 .1),
        },
        second_member: Member {
            id: res.1 .0,
            file: encode(res.1 .1),
        },
    })
}

pub async fn post_pair_vote(
    conn: bb8::PooledConnection<'_, DieselConnectionManager<PgConnection>>,
    subject: Option<String>,
    vote: PostPairVoteRequest,
) -> Result<()> {
    let subject = subject.unwrap();

    diesel::insert_into(pair_vote::table)
        .values((
            pair_vote::first_member_id.eq(vote.first_member_id),
            pair_vote::second_member_id.eq(vote.second_member_id),
            pair_vote::diff.eq(vote.diff),
            pair_vote::subject.eq(subject),
        ))
        .on_conflict_do_nothing()
        .execute(&*conn)?;

    Ok(())
}

pub async fn get_voting_results(
    mut conn: bb8::PooledConnection<'_, DieselConnectionManager<PgConnection>>,
    _subject: Option<String>,
    page: Option<i32>,
    limit: Option<i32>,
) -> Result<VotingResults> {
    let (res, total_pages) = voting_result::table
        .inner_join(first_group_member::table)
        .inner_join(second_group_member::table)
        .select((
            (first_group_member::id, first_group_member::file_blob),
            (second_group_member::id, second_group_member::file_blob),
            voting_result::diff,
        ))
        .order(voting_result::diff.desc())
        .paginate(page.unwrap_or(1).into())
        .per_page(limit.unwrap_or(5).into())
        .load_and_count_pages::<((i64, Vec<u8>), (i64, Vec<u8>), i64)>(&mut *conn)
        .unwrap();

    let res = res
        .into_iter()
        .map(|(member_0, member_1, diff_sum)| VotingResult {
            member_pair: MemberPair {
                first_member: Member {
                    id: member_0.0,
                    file: encode(member_0.1),
                },
                second_member: Member {
                    id: member_1.0,
                    file: encode(member_1.1),
                },
            },
            diff_sum,
        })
        .collect::<Vec<VotingResult>>();

    Ok(VotingResults { results: res, total_pages })
}

pub async fn post_voting_results(
    conn: bb8::PooledConnection<'_, DieselConnectionManager<PgConnection>>,
    _subject: Option<String>,
) -> Result<()> {
    // Fix this query, doesn't seem to work correctly
    let res = first_group_member::table
        .inner_join(second_group_member::table.on(true.into_sql::<sql_types::Bool>()))
        .inner_join(
            pair_vote::table.on(pair_vote::first_member_id
                .eq(first_group_member::id)
                .and(pair_vote::second_member_id.eq(second_group_member::id))),
        )
        .select((
            (first_group_member::id, first_group_member::file_blob),
            (second_group_member::id, second_group_member::file_blob),
            pair_vote::diff,
        ))
        .order(pair_vote::diff.desc())
        .get_results::<((i64, Vec<u8>), (i64, Vec<u8>), i64)>(&*conn)
        .unwrap();

    let res = res
        .into_iter()
        .fold(
            Default::default(),
            |mut map: HashMap<((i64, Vec<u8>), (i64, Vec<u8>)), i64>, (member_0, member_1, diff_sum)| {
                // let entry = if member_0.0 < member_1.0 {
                let entry = map.entry(((member_0.0, member_0.1), (member_1.0, member_1.1))).or_default();
                // } else {
                // map.entry(((member_1.0, member_1.1), (member_0.0, member_0.1))).or_default()
                // };

                *entry += diff_sum;
                map
            },
        )
        .into_iter()
        .map(|((member_0, member_1), diff_sum)| VotingResult {
            member_pair: MemberPair {
                first_member: Member {
                    id: member_0.0,
                    file: encode(member_0.1),
                },
                second_member: Member {
                    id: member_1.0,
                    file: encode(member_1.1),
                },
            },
            diff_sum,
        })
        .collect::<Vec<VotingResult>>();

    // Todo batch it
    for res in res {
        let insert_res = diesel::insert_into(voting_result::table)
            .values((
                voting_result::first_member_id.eq(res.member_pair.first_member.id),
                voting_result::second_member_id.eq(res.member_pair.second_member.id),
                voting_result::diff.eq(res.diff_sum),
            ))
            .execute(&*conn)
            .ok();

        if insert_res.is_none() {
            diesel::update(voting_result::table)
                .filter(
                    voting_result::first_member_id
                        .eq(res.member_pair.first_member.id)
                        .and(voting_result::second_member_id.eq(res.member_pair.second_member.id)),
                )
                .set((voting_result::diff.eq(res.diff_sum),))
                .execute(&*conn)?;
        }
    }

    Ok(())
}
