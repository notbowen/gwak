use crate::types::adapter::{Adapter, Challenge};

use anyhow::Result;
use async_trait::async_trait;
use reqwest;
use serde::Deserialize;

pub struct CtfdAdapter {
    pub client: reqwest::Client,
    pub url: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub token: Option<String>,
}

#[derive(Deserialize, Debug)]
struct IdQueryResponse {
    data: Vec<Ids>,
}

#[derive(Deserialize, Debug)]
struct Ids {
    id: u32,
}

#[derive(Deserialize, Debug)]
struct ChallengeQueryResponse {
    data: Challenge,
}

impl CtfdAdapter {
    pub fn new(
        url: String,
        username: Option<String>,
        password: Option<String>,
        token: Option<String>,
    ) -> CtfdAdapter {
        let client = match (&username, &password, &token) {
            (Some(user), Some(pass), None) => todo!(),
            (None, None, Some(tok)) => setup_token_client(tok),
            _ => panic!(),
        };

        CtfdAdapter {
            client,
            url,
            username,
            password,
            token,
        }
    }
}

fn setup_token_client(token: &str) -> reqwest::Client {
    let mut headers = reqwest::header::HeaderMap::new();

    headers.insert(
        reqwest::header::AUTHORIZATION,
        reqwest::header::HeaderValue::from_str(format!("Token {}", token).as_str()).unwrap(),
    );
    headers.insert(
        reqwest::header::CONTENT_TYPE,
        reqwest::header::HeaderValue::from_static("application/json"),
    );

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();

    client
}

#[async_trait]
impl Adapter for CtfdAdapter {
    async fn get_challenges(&self) -> Result<Vec<Challenge>> {
        // Get the entire list of challenges to extract their IDs
        let res = self
            .client
            .get(format!("{}/api/v1/challenges", self.url))
            .send()
            .await?;

        let challenges: IdQueryResponse = res.json().await?;

        let challenge_ids: Vec<u32> = challenges.data.into_iter().map(|c| c.id).collect();
        let mut challenge_data: Vec<ChallengeQueryResponse> = vec![];

        // Fetch each challenge into a challenge struct
        for id in challenge_ids {
            let req = self
                .client
                .get(format!("{}/api/v1/challenges/{}", self.url, id))
                .send()
                .await?;

            challenge_data.push(req.json().await?);
        }

        Ok(challenge_data.iter().map(|c| c.data.clone()).collect())
    }
}
