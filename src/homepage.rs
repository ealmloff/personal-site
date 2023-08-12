#![allow(non_snake_case)]

use futures_util::stream::StreamExt;
use std::io::Write;

use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;
use reqwest::header::{AUTHORIZATION, USER_AGENT};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct User {
    avatar_url: String,        // https://avatars.githubusercontent.com/u/66571940?v=4
    organizations_url: String, // https://api.github.com/users/Demonthos/orgs
    repos_url: String,         // https://api.github.com/users/Demonthos/repos
    events_url: String,        // https://api.github.com/users/Demonthos/events
    name: String,              // ealmloff
    company: Option<String>,   // null
    blog: Option<String>,      // evanalmloff.me
    location: Option<String>,  // kansas city
    twitter_username: Option<String>, // null
    public_repos: u32,         // 36
    public_gists: u32,         // 0
    followers: u32,            // 23
    following: u32,            // 7
    created_at: DateTime<Utc>, // 2020-06-07T20:12:47Z
    updated_at: DateTime<Utc>, // 2023-01-28T13:29:59Z
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Repo {
    full_name: String,
    name: String,
    description: Option<String>,
    fork: bool,
    archived: bool,
    stargazers_count: u32,
    watchers_count: u32,
    homepage: Option<String>,
    html_url: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    contributors_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct RepoData {
    repo: Repo,
    prs: Option<SearchResult>,
    contributor: Contributor,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Pr {
    html_url: String,
    draft: bool,
    title: String,
    state: String,
    body: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SearchResult {
    total_count: u32,
    incomplete_results: bool,
    items: Vec<Pr>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Organization {
    login: String,
    description: Option<String>,
    repos_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Contributor {
    login: String,
    contributions: u32,
}

pub fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        div { display: "flex", flex_direction: "row", justify_content: "right",
            a { margin: "10px", right: "10px", href: "https://www.linkedin.com/in/evan-almloff-571467213/", img { src: "./In-Blue-34.png", width: "32px", height: "32px" } }
            a { margin: "10px", right: "10px", href: "https://github.com/Demonthos", img { src: "./GitHub-Mark-Light-32px.png", width: "32px", height: "32px" } }
        }
        Body {}
    })
}

fn Body(cx: Scope) -> Element {
    let repos = use_server_future(cx, (), |_| async move {
        let client = reqwest::Client::new();
        let name = "demonthos";
        let user: User = client
            .get(format!("https://api.github.com/users/{name}"))
            .header(USER_AGENT, "personal-website")
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        let orgs: Vec<Organization> = client
            .get(&user.organizations_url)
            .header(USER_AGENT, "personal-website")
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        let new_repos: Vec<Repo> = client
            .get(&user.repos_url)
            .header(USER_AGENT, "personal-website")
            .query(&[("affiliation", "owner,collaborator,organization_member")])
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        let mut new_repos: Vec<_> = new_repos
            .into_iter()
            .filter(|repo| !repo.fork && !repo.archived)
            .collect();
        for org in orgs {
            let orgs_repos: Vec<Repo> = client
                .get(&org.repos_url)
                .header(USER_AGENT, "personal-website")
                .query(&[("affiliation", "owner,collaborator,organization_member")])
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
            let orgs_repos: Vec<_> = orgs_repos
                .into_iter()
                .filter(|repo| !repo.fork && !repo.archived)
                .collect();
            new_repos.extend(orgs_repos);
        }

        let mut built_repos: Vec<RepoData> = Vec::new();

        for repo in new_repos {
            let prs = Default::default();
            let contributors: Vec<Contributor> = client
                .get(&repo.contributors_url)
                .header(USER_AGENT, "personal-website")
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
            if let Some(contributor) = contributors
                .into_iter()
                .find(|contributor| contributor.login.to_lowercase() == name.to_lowercase())
            {
                built_repos.push(RepoData {
                    repo,
                    prs,
                    contributor,
                });
            }
        }

        built_repos.sort_by(|a, b| b.repo.stargazers_count.cmp(&a.repo.stargazers_count));

        built_repos
    })?;
    let repos = use_ref(cx, || repos.value().clone());

    enum HoveredRepo {
        None,
        Hovered(usize),
        Focused(usize),
    }

    impl HoveredRepo {
        fn value(&self) -> Option<usize> {
            match self {
                HoveredRepo::None => None,
                HoveredRepo::Hovered(idx) => Some(*idx),
                HoveredRepo::Focused(idx) => Some(*idx),
            }
        }
    }

    let hovered_repo = use_state(cx, || HoveredRepo::None);
    let repo_request_resolver: &Coroutine<usize> = use_coroutine(cx, {
        to_owned![repos];
        |mut rx| async move {
            let client = reqwest::Client::new();
            let name = "demonthos";
            while let Some(idx) = rx.next().await {
                if let Some(full_name) = {
                    let read = repos.read();
                    read.get(idx).and_then(|repo| {
                        let repo: &RepoData = repo;
                        repo.prs.is_none().then(|| repo.repo.full_name.clone())
                    })
                } {
                    let prs = client
                        .get(format!(
                            "https://api.github.com/search/issues?q=is:pr+repo:{full_name}+author:{name}"
                        ))
                        .header(USER_AGENT, "personal-website")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    let mut write = repos.write();
                    if let Some(repo) = write.get_mut(idx) {
                        let repo: &mut RepoData = repo;
                        repo.prs = Some(prs);
                    }
                }
            }
        }
    });

    let mut cards: [[Element; 3]; 3] = Default::default();
    cards[0][0] = render! {
        Card {
            h1 { "Hi, I'm Evan!" }
            h2 {
                class: "break-words w-11/12",
                "Here are a few projects I have been working on:"
            }
        }
    };

    let current_hovered = hovered_repo.current().value();

    let repos = repos.read();
    let mut prs = current_hovered.and_then({
        |idx| {
            repos
                .get(idx)
                .and_then(|repo| repo.prs.as_ref().map(|prs| prs.items.iter()))
        }
    });
    let mut idx = 0;
    for row in &mut cards {
        for card in row {
            if card.is_none() {
                if prs.is_none() || current_hovered == Some(idx) {
                    if let Some(repo) = repos.get(idx) {
                        let repo = &repo.repo;
                        *card = render! {
                            Card {
                                onhover: move |_| {
                                    repo_request_resolver.send(idx);
                                    if let HoveredRepo::None = &*hovered_repo.current() {
                                        hovered_repo.set(HoveredRepo::Hovered(idx));
                                    }
                                },
                                onfocus: move |_| {
                                    repo_request_resolver.send(idx);
                                    hovered_repo.set(HoveredRepo::Focused(idx));
                                },
                                onfocusout: move |_| {
                                    if let HoveredRepo::Focused(cur_idx) = &*hovered_repo.current() {
                                        if *cur_idx == idx {
                                            hovered_repo.set(HoveredRepo::None);
                                        }
                                    }
                                },
                                onhoverout: move |_| {
                                    if let HoveredRepo::Hovered(cur_idx) = &*hovered_repo.current() {
                                        if *cur_idx == idx {
                                            hovered_repo.set(HoveredRepo::None);
                                        }
                                    }
                                },
                                focusable: true,
                                a {
                                    href: "{repo.html_url}",
                                    class: "text-xl text-blue-600 visited:text-purple-600 capitalize m-4",
                                    "{repo.name}"
                                }
                                repo.description.as_ref().map(|discription| {
                                    rsx! {
                                        p {
                                            class: "break-words w-11/12",
                                            "{discription}"
                                        }
                                    }
                                })
                                p { "Stars: {repo.stargazers_count}" }
                            }
                        };
                    }
                } else if let Some(pr) = prs.as_mut().and_then(|prs| prs.next()) {
                    *card = render! {
                        Card {
                            onfocus: move |_| {
                                repo_request_resolver.send(idx);
                                hovered_repo.set(HoveredRepo::Focused(idx));
                            },
                            focusable: true,
                            h1 {
                                // href: "{pr.html_url}",
                                // class: "text-blue-600 visited:text-purple-600",
                                class: "text-xl capitalize m-4",
                                "{pr.title}"
                            }
                            pr.body.as_ref().map(|discription| {
                                const MAX_LEN: usize = 80;
                                let discription = if discription.len() > MAX_LEN {
                                    discription[..(MAX_LEN-3)].to_string()+"..."
                                } else {
                                    discription.to_string()
                                };
                                rsx! {
                                    p {
                                        class: "break-words w-11/12",
                                        "{discription}"
                                    }
                                }
                            })
                        }
                    };
                }
                idx += 1;
            }
        }
    }

    render! {
        div { class: "flex flex-col justify-center items-center h-full w-full",
            onmousedown: move |_| {
                hovered_repo.set(HoveredRepo::None);
            },
            table {
                tbody{
                    for row in cards {
                        tr {
                            for card in row {
                                td {
                                    if let Some(card) = card {
                                        Some(card)
                                    }
                                    else if repos.is_empty() {
                                        render!{
                                            Card {
                                                div {
                                                    class: "w-1/3 h-6 animate-pulse bg-slate-200 dark:bg-slate-700 rounded-lg m-4",
                                                }
                                                div {
                                                    class: "w-4/5 h-4 animate-pulse bg-slate-200 dark:bg-slate-700 rounded-lg m-1",
                                                }
                                                div {
                                                    class: "w-4/5 h-4 animate-pulse bg-slate-200 dark:bg-slate-700 rounded-lg m-1",
                                                }
                                                div {
                                                    class: "w-4/5 h-4 animate-pulse bg-slate-200 dark:bg-slate-700 rounded-lg m-1",
                                                }
                                                div {
                                                    class: "w-4/5 h-4 animate-pulse bg-slate-200 dark:bg-slate-700 rounded-lg m-1",
                                                }
                                            }
                                        }
                                    }
                                    else {
                                        render! {
                                            Card {}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[inline_props]
fn Card<'a>(
    cx: Scope,
    children: Element<'a>,
    onhover: Option<EventHandler<'a>>,
    onhoverout: Option<EventHandler<'a>>,
    onfocus: Option<EventHandler<'a>>,
    onfocusout: Option<EventHandler<'a>>,
    focusable: Option<bool>,
) -> Element {
    cx.render(rsx! {
        div { class: "flex flex-col justify-center items-center ring-0 hover:ring-4 focus:ring-4 focus:rounded-3xl bg-slate-100 dark:bg-slate-800 hover:bg-blue-100 dark:hover:bg-blue-900 rounded-md hover:rounded-xl shadow-md hover:shadow-lg m-4 transition-all duration-200 text-center w-64 h-64",
            onmouseenter: move |_| {
                if let Some(f) = onhover.as_ref() { f.call(()) }
            },
            onmouseleave: move |_| {
                if let Some(f) = onhoverout.as_ref() { f.call(()) }
            },
            onfocusin: move |_| {
                if let Some(f) = onfocus.as_ref() { f.call(()) }
            },
            onfocusout: move |_| {
                if let Some(f) = onfocusout.as_ref() { f.call(()) }
            },
            tabindex: focusable.filter(|f| *f).map(|_| 0),
            children
        }
    })
}
