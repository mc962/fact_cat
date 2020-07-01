use serde::{Deserialize};
use chrono::{DateTime, Utc};

#[derive(Deserialize, Debug)]
pub struct FactsResponse {
    all: Vec<Fact>
}

#[derive(Deserialize, Debug)]
pub struct RandomFact {
    used: bool,
    source: String,
    #[serde(rename(serialize = "fact_type", deserialize = "type"))]
    fact_type: String,
    deleted: bool,
    _id: String,
    user: String,
    pub(crate) text: String,
    __v: i32,
    #[serde(rename(serialize = "updated_at", deserialize = "updatedAt"))]
    updated_at: DateTime<Utc>,
    #[serde(rename(serialize = "created_at", deserialize = "createdAt"))]
    created_at: DateTime<Utc>,
    status: Option<RandomFactStatus>
}

#[derive(Deserialize, Debug)]
struct RandomFactStatus {
    verified: bool,
    #[serde(rename(serialize = "sent_count", deserialize = "sentCount"))]
    sent_count: i32
}

#[derive(Deserialize, Debug)]
struct Fact {
    _id: String,
    text: String,
    #[serde(rename(serialize = "type", deserialize = "fact_type"))]
    fact_type: String,
    user: String,
    upvotes: i32,
    #[serde(rename(serialize = "userUpvoted", deserialize = "user_upvoted"))]
    user_upvoted: bool
}

#[derive(Deserialize, Debug)]
struct User {
    _id: String,
    name: Name
}

#[derive(Deserialize, Debug)]
struct Name {
    first: String,
    last: String
}