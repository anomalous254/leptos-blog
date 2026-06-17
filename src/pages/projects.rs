use crate::components::ui::Card;
use crate::components::ui::FastA;
use leptos::prelude::*;

struct Project {
    title: &'static str,
    path: &'static str,
    desc: &'static str,
}

type Projects = Vec<Project>;

#[component]
pub fn ProjectsPage() -> impl IntoView {
    let projects: Projects = vec![
        Project {
            title: "Authbox",
            path: "https://authbox-docs.vercel.app/",
            desc: "A lightweight, modular, async-first authentication framework for Rust.",
        },

         
        Project {
        title: "paystack-client",
        path: "https://github.com/anomalous254/paystack-client",
        desc: "A lightweight, async Rust client for interacting with the Paystack API. Provides helpers for initializing and verifying payments using reqwest and tokio.",
    },
    Project {
        title: "scrapurl",
        path: "https://github.com/anomalous254/scrapurl",
        desc: "A colorful Rust CLI tool to scrape and list all links from a given webpage.",
    },
    Project {
        title: "online radio",
        path: "https://github.com/anomalous254/online-radio",
        desc: "A simple online radio station that streams live radio stations from around the world.",
    },
    Project {
        title: "live_radio",
        path: "https://pypi.org/project/live-radio/0.1.2/",
        desc: "Python library that gets live radio stations stream info by country using pyradios.",
    },
    Project {
        title: "rust-calculator-actix-web",
        path: "https://github.com/anomalous254/rust-calculator-actix-web",
        desc: "Simple Rust calculator using Actix Web.",
    },
    Project {
        title: "drf-jwt",
        path: "https://github.com/anomalous254/drf-jwt",
        desc: "Django REST Framework starter project with JWT authentication using Djoser.",
    },
    Project {
        title: "daraja-client",
        path: "https://pypi.org/project/daraja-client/",
        desc: "Python module for integrating Safaricom MPESA Daraja 2.0 API.",
    },
    Project {
        title: "githubapiclient",
        path: "https://pypi.org/project/githubapiclient/",
        desc: "Python library for interacting with the GitHub API (repos, commits, PRs, README).",
    },
    Project {
        title: "Language Africa",
        path: "https://www.languageafrica.com/",
        desc: "A language learning app for African languages built with React, Django, Firebase, and REST APIs.",
    },
    ];

    view! {
        <Card title="Public Projects">

            <div class="back-link">
                <FastA href="/">"← Go Back"</FastA>
            </div>

            // About-style intro
            <div class="about-content">
                <h2>"Overview"</h2>

                <p>
                    "A collection of open-source tools and production-ready systems "
                    "built with Rust and modern web technologies."
                </p>

                <p>"Focused on performance, modular architecture, and developer-friendly APIs."</p>
            </div>

            // simple stacked list (NOT cards)
            <div class="project-list">

                {projects
                    .into_iter()
                    .map(|project| {
                        view! {
                            <div class="project-item">

                                <FastA href=project.path>
                                    <div class="project-title">{project.title}</div>
                                </FastA>

                                <div class="project-desc">{project.desc}</div>

                            </div>
                        }
                    })
                    .collect_view()}

            </div>

        </Card>
    }
}
