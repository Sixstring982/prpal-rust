use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(from = "RemotePullRequest")]
pub struct PullRequest {
    pub number: u32,
    pub title: String,
    pub author: String,
    pub body: Option<String>,
    pub html_url: String,
    pub branch_name: String,
    pub base_branch_name: String,
    pub diffbase: Vec<u32>,
}

#[derive(Deserialize)]
struct RemotePullRequest {
    number: u32,
    title: String,
    html_url: String,
    body: Option<String>,
    user: User,
    head: Branch,
    base: Branch,
}

#[derive(Deserialize)]
struct Branch {
    label: String,
}

#[derive(Deserialize)]
struct User {
    login: String,
}

impl From<RemotePullRequest> for PullRequest {
    fn from(remote: RemotePullRequest) -> PullRequest {
        PullRequest {
            number: remote.number,
            title: remote.title.clone(),
            author: remote.user.login.clone(),
            body: remote.body.clone(),
            branch_name: remote.head.label.clone(),
            base_branch_name: remote.base.label.clone(),
            html_url: remote.html_url.clone(),
            diffbase: get_diffbase(&remote).unwrap_or(vec![]),
        }
    }
}

fn get_diffbase(remote: &RemotePullRequest) -> Option<Vec<u32>> {
    let body = remote.body.clone()?;
    let diffbase_line = body
        .lines()
        .find(|x| x.to_lowercase().starts_with("* diffbase: "))?;

    Some(
        diffbase_line["* diffbase: ".len()..]
            .split(",")
            .map(|s| {
                s.trim().replace("#", "").parse::<u32>().expect(
                    format!("PR #{}: invalid diffbase number: \"{}\"", remote.number, s).as_str(),
                )
            })
            .collect(),
    )
}
