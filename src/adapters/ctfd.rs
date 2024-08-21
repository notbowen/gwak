use crate::types::adapter::{Adapter, Challenge};

use async_trait::async_trait;
use reqwest;
use serde::Deserialize;

const API_KEY: &str = "Token ctfd_afb2c45e6da75e7d6707ce279077bde5fd29d7372fd7044dcc97d0af9b3d565e";

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
        CtfdAdapter {
            client: reqwest::Client::new(),
            url,
            username,
            password,
            token,
        }
    }
}

#[async_trait]
impl Adapter for CtfdAdapter {
    async fn get_challenges(&self) -> Vec<Challenge> {
        // Get the entire list of challenges to extract their IDs
        let challenges: IdQueryResponse = match self
            .client
            .get(format!("{}/api/v1/challenges", self.url))
            .header(reqwest::header::AUTHORIZATION, API_KEY)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .send()
            .await
        {
            Ok(res) => res.json().await.unwrap(),
            Err(e) => panic!(),
        };

        let challenge_ids: Vec<u32> = challenges.data.into_iter().map(|c| c.id).collect();
        let mut challenge_data: Vec<ChallengeQueryResponse> = vec![];

        // Fetch each challenge into a challenge struct
        for id in challenge_ids {
            let req = self
                .client
                .get(format!("{}/api/v1/challenges/{}", self.url, id))
                .header(reqwest::header::AUTHORIZATION, API_KEY)
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .send()
                .await
                .unwrap();

            challenge_data.push(req.json().await.unwrap());
        }

        challenge_data.iter().map(|c| c.data.clone()).collect()
    }
}
