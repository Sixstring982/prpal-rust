use log::debug;
use std::error::Error;

use super::pull_request::PullRequest;

pub async fn list_pull_requests(request: Request) -> Result<Response, Box<dyn Error>> {
    let client = reqwest::Client::new();

    let response = client
        .get(format!(
            "https://api.github.com/repos/{}/pulls",
            request.repo
        ))
        .header(reqwest::header::ACCEPT, "application/vnd.github+json")
        .header(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", request.auth_token),
        )
        .header(reqwest::header::USER_AGENT, "prpal")
        .send()
        .await?
        .text()
        .await?;

    debug!("list_pull_requests response: {}", response);

    let pull_requests: Vec<PullRequest> = serde_json::from_str(&response)?;
    let pull_requests = pull_requests.into_iter()
        .filter(|x| match &request.author {
            None => true,
            Some(a) => a.to_lowercase() == x.author.to_lowercase()
        })
        .collect();

    Ok (Response { pull_requests })
}

pub struct Request {
    pub auth_token: String,
    pub author: Option<String>,
    pub repo: String,
}

#[derive(Debug)]
pub struct Response {
    pub pull_requests: Vec<PullRequest>,
}
