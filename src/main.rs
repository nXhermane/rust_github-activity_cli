use serde::Deserialize;
use std::str::FromStr;

#[derive(Deserialize, Debug)]
struct GithubActivityActor {
    id: i32,
    login: String,
    display_login: String,
    url: String,
}

#[derive(Deserialize, Debug)]
struct GithubActivityRepo {
    id: i32,
    name: String,
    url: String,
}
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum GithubEventType {
    PushEvent,
    CreateEvent,
    DeleteEvent,
    IssuesEvent,
    WatchEvent,
    PullRequestEvent,
}

impl FromStr for GithubEventType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PushEvent" => Ok(GithubEventType::PushEvent),
            "CreateEvent" => Ok(GithubEventType::CreateEvent),
            "DeleteEvent" => Ok(GithubEventType::DeleteEvent),
            "IssuesEvent" => Ok(GithubEventType::IssuesEvent),
            "WatchEvent" => Ok(GithubEventType::WatchEvent),
            "PullRequestEvent" => Ok(GithubEventType::PullRequestEvent),
            _ => Err(format!("Unknown GitHub event type: {}", s)),
        }
    }
}

#[derive(Deserialize, Debug)]
struct GithubActivityEventsResponse {
    id: String,
    r#type: GithubEventType,
    actor: GithubActivityActor,
    repo: GithubActivityRepo,
}

fn main() {
    let github_username = "nXhermane";
    let url = format!("https://api.github.com/users/{}/events", github_username);
    let body: String = ureq::get(url)
        .call()
        .unwrap()
        .body_mut()
        .read_to_string()
        .unwrap();
    let github_activity_event: Vec<GithubActivityEventsResponse> =
        serde_json::from_str(&body).unwrap();
    println!("Request Response : {:?}", github_activity_event);
    println!("Done!");
}
