use dioxus::prelude::*;

#[component]
pub fn Projects() -> Element {
    rsx! {
        div { class: "bg-white py-24 sm:py-32",
            div { class: "mx-auto max-w-7xl px-6 lg:px-8",
                div { class: "mx-auto grid max-w-2xl grid-cols-1 gap-x-8 gap-y-16 sm:gap-y-20 lg:mx-0 lg:max-w-none lg:grid-cols-3",
                    h2 { class: "text-pretty text-3xl font-bold tracking-tight text-gray-900 sm:text-4xl",
                        "Hey there 👋"
                        br {}
                        "I'm Evan Almloff,"
                        br {}
                        "here's a few projects I've been working on:"
                    }
                    dl { class: "col-span-2 grid grid-cols-1 gap-x-8 gap-y-16 sm:grid-cols-2",
                        div {
                            dt { class: "text-base font-semibold leading-7 text-gray-900",
                                div { class: "mb-6 flex h-10 w-10 items-center justify-center rounded-lg bg-indigo-600",
                                    svg {
                                        stroke: "currentColor",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        fill: "none",
                                        "aria-hidden": "true",
                                        "data-slot": "icon",
                                        class: "h-6 w-6 text-white",
                                        "viewBox": "0 0 24 24",
                                        "stroke-width": "1.5",
                                        path {
                                            d: "M12 21a9.004 9.004 0 0 0 8.716-6.747M12 21a9.004 9.004 0 0 1-8.716-6.747M12 21c2.485 0 4.5-4.03 4.5-9S14.485 3 12 3m0 18c-2.485 0-4.5-4.03-4.5-9S9.515 3 12 3m0 0a8.997 8.997 0 0 1 7.843 4.582M12 3a8.997 8.997 0 0 0-7.843 4.582m15.686 0A11.953 11.953 0 0 1 12 10.5c-2.998 0-5.74-1.1-7.843-2.918m15.686 0A8.959 8.959 0 0 1 21 12c0 .778-.099 1.533-.284 2.253m0 0A17.919 17.919 0 0 1 12 16.5c-3.162 0-6.133-.815-8.716-2.247m0 0A9.015 9.015 0 0 1 3 12c0-1.605.42-3.113 1.157-4.418",
                                            "stroke-linejoin": "round",
                                            "stroke-linecap": "round",
                                        }
                                    }
                                }
                                a { href: "https://github.com/dioxuslabs/dioxus", class: "text-blue-900 visited:text-purple-900 capitalize",
                                    "dioxus"
                                }
                            }
                            dd { class: "mt-1 text-base leading-7 text-gray-600",
                                "I'm one of the maintainers of Dioxus, a cross platform fullstack UI library for Rust. I work on everything from the core virtual dom library and reactivity system, to the backend integrations and front end router."
                            }
                        }
                        div {
                            dt { class: "text-base font-semibold leading-7 text-gray-900",
                                div { class: "mb-6 flex h-10 w-10 items-center justify-center rounded-lg bg-indigo-600",
                                    svg {
                                        fill: "none",
                                        "stroke-width": "1.5",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        stroke: "currentColor",
                                        "viewBox": "0 0 24 24",
                                        "data-slot": "icon",
                                        "aria-hidden": "true",
                                        class: "h-6 w-6 text-white",
                                        path {
                                            "stroke-linejoin": "round",
                                            "stroke-linecap": "round",
                                            d: "m3.75 13.5 10.5-11.25L12 10.5h8.25L9.75 21.75 12 13.5H3.75Z",
                                        }
                                    }
                                }
                                a { href: "https://github.com/ealmloff/sledgehammer_bindgen", class: "text-blue-900 visited:text-purple-900 capitalize",
                                    "sledgehammer bindgen"
                                }
                            }
                            dd { class: "mt-1 text-base leading-7 text-gray-600",
                                "I am the creator of Sledgehammer Bindgen, the fastest way to get data from Rust into JavaScript. It leverages batched string decoding and an list of opcodes generated for each framework."
                            }
                        }
                        div {
                            dt { class: "text-base font-semibold leading-7 text-gray-900",
                                div { class: "mb-6 flex h-10 w-10 items-center justify-center rounded-lg bg-indigo-600",
                                    svg {
                                        "stroke-width": "1.5",
                                        stroke: "currentColor",
                                        "viewBox": "0 0 24 24",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        fill: "none",
                                        "aria-hidden": "true",
                                        "data-slot": "icon",
                                        class: "h-6 w-6 text-white",
                                        path {
                                            "stroke-linecap": "round",
                                            "stroke-linejoin": "round",
                                            d: "M9.813 15.904 9 18.75l-.813-2.846a4.5 4.5 0 0 0-3.09-3.09L2.25 12l2.846-.813a4.5 4.5 0 0 0 3.09-3.09L9 5.25l.813 2.846a4.5 4.5 0 0 0 3.09 3.09L15.75 12l-2.846.813a4.5 4.5 0 0 0-3.09 3.09ZM18.259 8.715 18 9.75l-.259-1.035a3.375 3.375 0 0 0-2.455-2.456L14.25 6l1.036-.259a3.375 3.375 0 0 0 2.455-2.456L18 2.25l.259 1.035a3.375 3.375 0 0 0 2.456 2.456L21.75 6l-1.035.259a3.375 3.375 0 0 0-2.456 2.456ZM16.894 20.567 16.5 21.75l-.394-1.183a2.25 2.25 0 0 0-1.423-1.423L13.5 18.75l1.183-.394a2.25 2.25 0 0 0 1.423-1.423l.394-1.183.394 1.183a2.25 2.25 0 0 0 1.423 1.423l1.183.394-1.183.394a2.25 2.25 0 0 0-1.423 1.423Z",
                                        }
                                    }
                                }
                                a { href: "https://github.com/floneum/floneum", class: "text-blue-900 visited:text-purple-900 capitalize",
                                    "kalosm"
                                }
                            }
                            dd { class: "mt-1 text-base leading-7 text-gray-600",
                                "Kalosm is a local first framework for controllable AI. It lets you run anything from a simple chatbot to a live audio RAG system in a few lines of code. Kalosm gives you fine grained control over the output of models which means you can return almost any rust type from your LLMs instead of just text."
                            }
                        }
                        div {
                            dt { class: "text-base font-semibold leading-7 text-gray-900",
                                div { class: "mb-6 flex h-10 w-10 items-center justify-center rounded-lg bg-indigo-600",
                                    svg {
                                        "viewBox": "0 0 24 24",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        "stroke-width": "1.5",
                                        fill: "none",
                                        stroke: "currentColor",
                                        "data-slot": "icon",
                                        "aria-hidden": "true",
                                        class: "h-6 w-6 text-white",
                                        path {
                                            "stroke-linecap": "round",
                                            d: "M12 6.042A8.967 8.967 0 0 0 6 3.75c-1.052 0-2.062.18-3 .512v14.25A8.987 8.987 0 0 1 6 18c2.305 0 4.408.867 6 2.292m0-14.25a8.966 8.966 0 0 1 6-2.292c1.052 0 2.062.18 3 .512v14.25A8.987 8.987 0 0 0 18 18a8.967 8.967 0 0 0-6 2.292m0-14.25v14.25",
                                            "stroke-linejoin": "round",
                                        }
                                    }
                                }
                                "The "
                                a { href: "https://floneum.com/kalosm/docs", class: "text-blue-900 visited:text-purple-900 capitalize",
                                    "Kalosm"
                                }
                                " and "
                                a { href: "https://dioxuslabs.com/learn/0.5/guide", class: "text-blue-900 visited:text-purple-900 capitalize",
                                    "Dioxus"
                                }
                                " books"
                            }
                            dd { class: "mt-1 text-base leading-7 text-gray-600",
                                "Libraries are useless without docs. I enjoy writing documentation for both Dioxus and Kalosm. I am a core contributor to the official Dioxus book and creator of the Kalosm book. I have also worked extensively on inline documentation for both libraries." 
                            }
                        }
                    }
                }
            }
        }
    }
}
