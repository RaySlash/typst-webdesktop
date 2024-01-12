#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_free_icons::{icons::fi_icons, Icon};
use dioxus_router::prelude::*;

#[derive(Clone, Routable)]
enum Route {
    #[route("/")]
    About {},
}

// Shared State Props
struct DarkMode(bool);
struct Title(bool);

pub fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, || DarkMode(false));
    render! {
      style { include_str!("./style.css") }
      Navbar {},
      Workspace {},
    }
}

pub fn Workspace(cx: Scope) -> Element {
    render! {
        div { class: "workspace",
          ToolBar {},
          CodeArea {},
          PreviewArea {},
        }
    }
}

pub fn PreviewArea(cx: Scope) -> Element {
    render! {
      div { class: "preview-container",
        label { class: "previewbox",
          "Preview"
        }
      }
    }
}

pub fn CodeArea(cx: Scope) -> Element {
    render! {
      div { class: "code-container",
        input { class: "codebox",
          "Code"
        }
      }
    }
}

pub fn ToolBar(cx: Scope) -> Element {
    render! {
      div { class: "toolbar",
        div { class: "topicons",
          button {
            Icon { width: 30,
                   height: 30,
                   icon: fi_icons::FiEdit3,
            }
          }
          button {
            Icon { width: 30,
                   height: 30,
                   icon: fi_icons::FiFile,
            }
          }
          button {
            Icon { width: 30,
                   height: 30,
                   icon: fi_icons::FiDroplet,
            }
          }
          button {
            Icon { width: 30,
                   height: 30,
                   icon: fi_icons::FiRefreshCw,
            }
          }
        }
        div { class: "bottomicons",
          button { 
            Icon { width: 30,
                   height: 30,
                   icon: fi_icons::FiTool,
            }
          }
        }
      }
    }
}

pub fn Navbar(cx: Scope) -> Element {
    use_shared_state_provider(cx, || Title(false));

    render! {
          div { class: "navcontainer",
            header { class: "header",
              a { class: "burger",
                       Icon { width: 30,
                          height: 30,
                          icon: fi_icons::FiArrowDown,
                       }
              },
              TitleToggle {},
              div { class: "navtool",
                    DarkModeToggle {},
                    a { class: "git",
                        href: "https://github.com/rayslash/typst-webdesktop",
                        Icon { width: 30,
                               height: 30,
                               icon: fi_icons::FiGithub,
                        }
                    }
              }
            }
          }
    }
}

pub fn TitleToggle(cx: Scope) -> Element {
    let collapsed_state = use_shared_state::<Title>(cx).unwrap();
    let mut collapsed = collapsed_state.read().0;
    let title = if collapsed {
        "Typst Desktop | An application written in Rust (Dioxus)"
    } else {
        "Typst Desktop"
    };

    render! {
      button {
         class: "title",
          onclick: move |_| {
            collapsed = !collapsed;
            collapsed_state.write().0 = collapsed;
          },
        title,
        },
    }
}

pub fn DarkModeToggle(cx: Scope) -> Element {
    let dark_mode_state = use_shared_state::<DarkMode>(cx).unwrap();
    let mut dark_mode = dark_mode_state.read().0;
    let style = if dark_mode {
        "color: var(--bg)"
    } else {
        "color: var(--text)"
    };

    render! {
        button { class: "darkmode",
                 style: "{style}",
                 onclick: move |_| {
                   dark_mode = !dark_mode;
                   dark_mode_state.write().0 = dark_mode;
                 },
                 Icon { width: 30,
                   height: 30,
                   icon: fi_icons::FiMoon,
                 }
        }
    }
}

pub fn About(cx: Scope) -> Element {
    render! {
        p {
          b {"Typyst-Webdesktop"}
          "An open source typst desktop appmade in Rust."
        }
    }
}
