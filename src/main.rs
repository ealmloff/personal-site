#![allow(non_snake_case)]

use std::cell::Cell;

use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        // Header{}
        div{
            display: "flex",
            flex_direction: "row",
            justify_content: "right",
            a {
                margin: "10px",
                href: "https://www.linkedin.com/in/evan-almloff-571467213/",
                img {
                    src: "./In-Blue-34.png",
                    width: "32px",
                    height: "32px",
                }
            }
            a {
                margin: "10px",
                href: "https://github.com/Demonthos",
                img {
                    src: "./GitHub-Mark-Light-32px.png",
                    width: "32px",
                    height: "32px",
                }
            }
        }
        Body{}
    })
}

fn Body(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            align_items: "center",
            h1 {
                "Hi, I'm Evan!"
            }
            h2 {
                "I'm a computer science student in Overland Park, KS."
            }
            Projects{}
        }
    })
}

fn Header(cx: Scope) -> Element {
    cx.render(rsx! {
        header {
            div {
                display: "flex",
                flex_direction: "row",
                justify_content: "space-between",
                align_items: "center",

                padding: "0px 100px",
                height: "5vh",
                background_color: "#8ac926",
                Link {
                    href: "/aboutme",
                    "About Me"
                }
                Link {
                    href: "/projects",
                    "Projects"
                }
                Link {
                    href: "/technologies",
                    "Technologies"
                }
            }
        }
    })
}

#[derive(Props)]
struct LinkProps<'a> {
    href: &'static str,
    children: Element<'a>,
}

fn Link<'a>(cx: Scope<'a, LinkProps<'a>>) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "link",
            display: "flex",
            flex_direction: "colum",
            align_items: "center",
            justify_content: "center",
            height: "100%",
            width: "100%",
            a {
                font: "sans-serif",
                href: "{cx.props.href}",
                color: "black",
                text_decoration: "none",
                &cx.props.children
            }
        }
    })
}

#[inline_props]
fn Card<'a>(cx: Scope, image: &'static str, children: Element<'a>) -> Element {
    cx.render(rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            align_items: "center",
            justify_content: "center",
            class: "card",
            padding: "20px",
            background: "{image}",
            children
        }
    })
}

#[inline_props]
fn Projects(cx: Scope) -> Element {
    const CARDS: usize = 6;

    #[derive(Props)]
    struct ProjectsCardProps<'a, 'b> {
        image: &'static str,
        name: &'static str,
        link: &'static str,
        focused: UseState<Option<usize>>,
        altrinates: [Cell<Option<LazyNodes<'a, 'b>>>; CARDS - 1],
        idx: usize,
    }

    fn ProjectsCard<'a, 'b>(cx: Scope<'a, ProjectsCardProps<'a, 'b>>) -> Element<'a> {
        if let Some(focused) = cx.props.focused.get() {
            if *focused != cx.props.idx {
                let content = if *focused > cx.props.idx {
                    cx.props.altrinates[focused - 1].take().unwrap()
                } else {
                    cx.props.altrinates[*focused].take().unwrap()
                };
                return {
                    cx.render(rsx! {
                        div {
                            display: "flex",
                            flex_direction: "column",
                            align_items: "center",
                            justify_content: "center",
                            class: "card",
                            padding: "20px",
                            width: "150px",
                            height: "150px",
                            background: "{cx.props.image}",
                            text_align: "center",
                            content
                        }
                    })
                };
            }
        }
        cx.render(rsx! {
            div {
                display: "flex",
                flex_direction: "column",
                align_items: "center",
                justify_content: "center",
                class: "card hoverablecard",
                padding: "20px",
                width: "150px",
                height: "150px",
                background: "{cx.props.image}",
                onmouseover: |_| cx.props.focused.set(Some(cx.props.idx)),
                onmousemove: |_| cx.props.focused.set(Some(cx.props.idx)),
                onmouseleave: |_| {
                    if *cx.props.focused.get() == Some(cx.props.idx) {
                        cx.props.focused.set(None);
                    }
                },
                a {
                    href: "{cx.props.link}",
                    text_align: "center",
                    onfocusin: |_| cx.props.focused.set(Some(cx.props.idx)),
                    onmousemove: |_| cx.props.focused.set(Some(cx.props.idx)),
                    onfocusout: |_| {
                        if *cx.props.focused.get() == Some(cx.props.idx) {
                            cx.props.focused.set(None);
                        }
                    },
                    "{cx.props.name}"
                }
            }
        })
    }

    let focused = use_state(&cx, || None);

    cx.render(rsx! {
        Card {
            image: "linear-gradient(115deg, #ff595e, #8ac926)",
            h2 {
                "Here are some projects I've worked on:"
            }

            table {
                tr{
                    td{
                        ProjectsCard {
                            image: "linear-gradient(115deg, #ffca3a, #ff595e)",
                            name: "Dioxus",
                            link: "https://github.com/DioxusLabs/dioxus",
                            focused: focused.clone(),
                            altrinates: [
                                Cell::new(Some(rsx!{
                                    p{
                                        "Blitz is a wgpu frontend for dioxus"
                                    }
                                })),
                                Cell::new(Some(rsx!{
                                    p{
                                        "Iron Oxide is a game engine built with the raylib renderer"
                                    }
                                })),
                                Cell::new(Some(rsx!{
                                    p{
                                        "Patina is a cross platform code editor."
                                    }
                                })),
                                Cell::new(Some(rsx!{
                                    p{
                                        "A demonstration of different broad phase collision methods"
                                    }
                                })),
                                Cell::new(Some(rsx!{
                                    p{
                                        "Simulates trafic in a randomly generated city"
                                    }
                                })),
                            ],
                            idx: 0
                        }
                    }
                    td{
                        ProjectsCard {
                            image: "linear-gradient(115deg, #ff595e, #1982c4)",
                            name: "Blitz",
                            link: "https://github.com/DioxusLabs/blitz",
                            focused: focused.clone(),
                            altrinates: [
                                Cell::new(Some(rsx!{
                                    p{
                                        "Dioxus is a react like cross platform web framework"
                                    }
                                })),
                                Cell::new(Some(rsx!{
                                    p{
                                        "I am the solo creator"
                                    }
                                })),
                                Cell::new(Some(rsx!{
                                    p{
                                        "I am the solo creator"
                                    }
                                })),
                                Cell::new(Some(rsx!{
                                    p{
                                        "Contains a implementation of a kd-tree, and quadtree"
                                    }
                                })),
                                Cell::new(Some(rsx!{
                                    p{
                                        "Built with the Tkinter GUI toolkit"
                                    }
                                })),
                            ],
                            idx: 1
                        }
                    }
                    td{
                        ProjectsCard {
                            image: "linear-gradient(115deg, #1982c4, #6a4c93)",
                            name: "Iron Oxide",
                            link: "https://github.com/demonthos/ironoxide",
                            focused: focused.clone(),
                            altrinates: [
                                Cell::new(Some(rsx!{
                                    p{
                                        "I am one of the maintainers"
                                    }
                                })),
                                Cell::new(Some(rsx!{
                                    p{
                                        "It uses incremental computation to quickly update applications"
                                    }
                                })),
                                Cell::new(Some(rsx!{
                                    p{
                                        "It is built with dioxus, and works in the web, terminal, and desktop."
                                    }
                                })),
                                Cell::new(Some(rsx!{
                                    p{
                                        "These data structures help speed up collision in large groups of objects"
                                    }
                                })),
                                Cell::new(Some(rsx!{
                                    p{
                                        "Implements the A* pathfinding algorithm to generate paths between trafic lights"
                                    }
                                })),
                            ],
                            idx: 2
                        }
                    }
                }
                tr{
                    td{
                        ProjectsCard {
                            image: "linear-gradient(-115deg, #ffca3a, #1982c4)",
                            name: "Patina",
                            link: "https://github.com/demonthos/patina",
                            focused: focused.clone(),
                            altrinates: [
                                Cell::new(Some(rsx!{
                                    p{
                                        "One of the top 100 most downloaded #gui rust libraries"
                                    }
                                })),
                                Cell::new(Some(rsx!{
                                    p{
                                        "I am one of the maintainers."
                                    }
                                })),
                                Cell::new(Some(rsx!{
                                    p{
                                        "It uses a entity component system to manage game state"
                                    }
                                })),
                                Cell::new(Some(rsx!{
                                    p{
                                        "It intigrates with pygame to demonstrate the different data structures"
                                    }
                                })),
                                Cell::new(Some(rsx!{
                                    img{
                                        src: "https://user-images.githubusercontent.com/66571940/185226084-94a2d00b-c170-4b2d-93dc-18d9ce256ca5.png",
                                        width: "100%",
                                        height: "100%",
                                    }
                                })),
                            ],
                            idx: 3
                        }
                    }
                    td{
                        ProjectsCard {
                            image: "linear-gradient(-115deg, #ff595e, #ffca3a)",
                            name: "Bounding volumes in pygame",
                            link: "https://github.com/Demonthos/pyGameQuadTree"
                            focused: focused.clone(),
                            altrinates: [
                                Cell::new(Some(rsx!{
                                    p{
                                        "I worked on the terminal renderer, native renderer abstraction, and hot reloading"
                                    }
                                })),
                                Cell::new(Some(rsx!{
                                    p{
                                        "I built the initial implemenatation of the renderer and event system"
                                    }
                                })),
                                Cell::new(Some(rsx!{
                                    p{
                                        "It uses a bounding volume heigharchy and parrellel archetecture to speed up collisions"
                                    }
                                })),
                                Cell::new(Some(rsx!{
                                    p{
                                        "It has multiple cursor support"
                                    }
                                })),
                                Cell::new(Some(rsx!{
                                    p{
                                        "🐍 Built with Python"
                                    }
                                })),
                            ],
                            idx: 4
                        }
                    }
                    td{
                        ProjectsCard {
                            image: "linear-gradient(-115deg, #6a4c93, #ff595e)",
                            name: "Traffic simulation",
                            link: "https://github.com/Demonthos/traficsimpython"
                            focused: focused.clone(),
                            altrinates: [
                                Cell::new(Some(rsx!{
                                    p{
                                        "🦀 Built with Rust"
                                    }
                                })),
                                Cell::new(Some(rsx!{
                                    p{
                                        "🦀 Built with Rust"
                                    }
                                })),
                                Cell::new(Some(rsx!{
                                    p{
                                        "🦀 Built with Rust"
                                    }
                                })),
                                Cell::new(Some(rsx!{
                                    p{
                                        "🦀 Built with Rust"
                                    }
                                })),
                                Cell::new(Some(rsx!{
                                    p{
                                        "🐍 Built with Python"
                                    }
                                })),
                            ],
                            idx: 5
                        }
                    }
                }
            }
        }
    })
}
