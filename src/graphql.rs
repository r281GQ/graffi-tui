use reqwest::header::{HeaderMap, ACCEPT, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::error::Error;

const API_URL: &str = "https://rickandmortyapi.com/graphql";
const QUERY: &str = "{\"operationName\":null,\"variables\":{},\"query\":\"{  character(id: 1) { id name status }}\"}";

#[derive(Serialize, Deserialize, Debug)]
pub struct Character {
    id: String,
    name: String,
    status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CharacterDataField {
    character: Character,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GraphQLResponse<T> {
    data: T,
}

pub async fn perform_graphql() -> Result<GraphQLResponse<CharacterDataField>, Box<dyn Error>> {
    let mut headers = HeaderMap::new();

    headers.insert(ACCEPT, "application/json".parse().unwrap());
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

    let client = reqwest::Client::new();

    let response = client
        .post(API_URL)
        .headers(headers)
        .body(QUERY)
        .send()
        .await?
        .text()
        .await?;

    let json_response: GraphQLResponse<CharacterDataField> = serde_json::from_str(&response)?;

    Ok(json_response)
}
