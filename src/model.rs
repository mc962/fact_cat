use serde::{Deserialize};
use chrono::{DateTime, Utc};

#[derive(Deserialize, Debug)]
pub struct FactsResponse {
    all: Vec<Fact>
}

#[derive(Deserialize, Debug, Clone)]
pub struct RandomFact {
    used: Option<bool>,
    source: Option<String>,
    #[serde(rename(serialize = "fact_type", deserialize = "type"))]
    fact_type: Option<String>,
    deleted: Option<bool>,
    _id: Option<String>,
    user: Option<String>,
    pub text: String,
    __v: Option<i32>,
    #[serde(rename(serialize = "updated_at", deserialize = "updatedAt"))]
    updated_at: Option<DateTime<Utc>>,
    #[serde(rename(serialize = "created_at", deserialize = "createdAt"))]
    created_at: Option<DateTime<Utc>>,
    status: Option<RandomFactStatus>
}

#[derive(Deserialize, Debug, Clone)]
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