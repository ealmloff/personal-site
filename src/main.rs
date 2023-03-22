#![allow(non_snake_case)]

use dioxus_storage::server_storage::*;
use dioxus_storage::*;
use futures_util::stream::StreamExt;
use std::io::Write;

use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use reqwest::header::{AUTHORIZATION, USER_AGENT};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
struct RepoData {
    repo: Repo,
    prs: Option<SearchResult>,
    contributor: Contributor,
}

#[derive(Debug, Serialize, Deserialize)]
struct Pr {
    html_url: String,
    draft: bool,
    title: String,
    state: String,
    body: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SearchResult {
    total_count: u32,
    incomplete_results: bool,
    items: Vec<Pr>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Organization {
    login: String,
    description: Option<String>,
    repos_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Contributor {
    login: String,
    contributions: u32,
}

fn main() {
    #[cfg(target_arch = "wasm32")]
    dioxus_web::launch_cfg(app, dioxus_web::Config::new().hydrate(true));
    #[cfg(not(target_arch = "wasm32"))]
    {
        let template_pre_data = r#"<!DOCTYPE html>
<html class="dark:bg-slate-900 dark:text-slate-400">
<head>
  <title>Evan Almloff</title>
  <meta content="text/html;charset=utf-8" http-equiv="Content-Type" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <meta charset="UTF-8" />
  <link rel="stylesheet" href="/output.css" />"#;
        let template_post_data = r#"
</head>
<body>
  <div id="main">"#;
        let template_post = r#"</div>
         <script type="module">
    import init from "/{base_path}/assets/dioxus/{app_name}.js";
    init("/{base_path}/assets/dioxus/{app_name}_bg.wasm").then(wasm => {
      if (wasm.__wbindgen_start == undefined) {
        wasm.main();
      }
    });
  </script>
</body>
</html>"#;
        let mut file = std::fs::File::create("index.html").unwrap();
        let mut vdom = VirtualDom::new(app);
        let _ = vdom.rebuild();
        let renderered = dioxus_ssr::pre_render(&vdom);
        let data = get_data();
        file.write_fmt(format_args!(
            "{template_pre_data}{data}{template_post_data}{renderered}{template_post}"
        ))
        .unwrap();
    }
}

fn app(cx: Scope) -> Element {
    let right = "right";
    cx.render(rsx! {
        div { display: "flex", flex_direction: "row", justify_content: "{right}",
            a { margin: "10px", right: "10px", href: "https://www.linkedin.com/in/evan-almloff-571467213/", img { src: "./In-Blue-34.png", width: "32px", height: "32px" } }
            a { margin: "10px", right: "10px", href: "https://github.com/Demonthos", img { src: "./GitHub-Mark-Light-32px.png", width: "32px", height: "32px" } }
        }
        Body {}
    })
}

fn Body(cx: Scope) -> Element {
    let repos: &UseRef<Vec<RepoData>> = use_ref(cx, || {
        server_state!(|| {
            tokio::runtime::Runtime::new().unwrap().block_on(async move {
                let client = reqwest::Client::new();
                let name = "demonthos";
                let user: User = client
                    .get(format!("https://api.github.com/users/{name}"))
                    .header(
                        AUTHORIZATION,
                        "Bearer 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",
                    ).header(USER_AGENT, "personal-website")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                let orgs: Vec<Organization> = client
                    .get(&user.organizations_url)
                    .header(
                        AUTHORIZATION,
                        "Bearer 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",
                    ).header(USER_AGENT, "personal-website")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                let new_repos: Vec<Repo> = client
                    .get(&user.repos_url)
                    .header(
                        AUTHORIZATION,
                        "Bearer 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",
                    ).header(USER_AGENT, "personal-website")
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
                        .header(
                            AUTHORIZATION,
                            "Bearer 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",
                        ).header(USER_AGENT, "personal-website")
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
                        .header(
                            AUTHORIZATION,
                            "Bearer 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",
                        ).header(USER_AGENT, "personal-website")
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
            })
        })
    });

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
                        .header(
                            AUTHORIZATION,
                            "Bearer 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",
                        ).header(USER_AGENT, "personal-website")
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

    let repos = repos.read();
    let mut cards: [[Option<LazyNodes>; 3]; 3] = Default::default();
    cards[0][0] = Some(rsx! {
        Card {
            h1 { "Hi, I'm Evan!" }
            h2 {
                class: "break-words w-11/12",
                "Here are a few projects I have been working on:"
            }
        }
    });

    let current_hovered = hovered_repo.current().value();
    let mut prs = current_hovered.and_then(|idx| {
        repos
            .get(idx)
            .and_then(|repo| repo.prs.as_ref().map(|prs| prs.items.iter()))
    });
    let mut idx = 0;
    for row in &mut cards {
        for card in row {
            if card.is_none() {
                if prs.is_none() || current_hovered == Some(idx) {
                    if let Some(repo) = repos.get(idx) {
                        let repo = &repo.repo;
                        *card = Some(rsx! {
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
                        });
                    }
                } else if let Some(pr) = prs.as_mut().and_then(|prs| prs.next()) {
                    *card = Some(rsx! {
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
                    });
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
                                        card
                                    }
                                    else if repos.is_empty() {
                                        rsx!{
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
                                        rsx! {
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
