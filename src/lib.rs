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
    cx.render(rsx! {
      style { include_str!("./style.css") }
      Navbar {},
      Workspace {},
    })
}

pub fn Workspace(cx: Scope) -> Element {
    render!(rsx! {
        div { class: "workspace",
          ToolBar {},
          CodeArea {},
          PreviewArea {},
        }
    })
}

pub fn PreviewArea(cx: Scope) -> Element {
    render!(rsx! {
      div { class: "previewarea",
        button { class: "previewbut",
          "Stranger"
        }
      }
    })
}

pub fn CodeArea(cx: Scope) -> Element {
    render!(rsx! {
      div { class: "codearea",
        button { class: "codebut",
          "There"
        }
      }
    })
}

pub fn ToolBar(cx: Scope) -> Element {
    render!(rsx! {
      div { class: "toolbar",
        button { class: "toolbut",
          "Hi"
        }
      }
    })
}

pub fn Navbar(cx: Scope) -> Element {
    use_shared_state_provider(cx, || Title(false));

    render!(rsx! {
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
    })
}

// The title doesn't switch but currently doesnt break layout
pub fn TitleToggle(cx: Scope) -> Element {
    let collapsed_state = use_shared_state::<Title>(cx).unwrap();
    let mut collapsed = collapsed_state.read().0;
    let title = if collapsed {
        "Typst Desktop"
    } else {
        "
        Typst Desktop
        An application written in Rust (Dioxius)
    "
    };

    render! {rsx!(
      label {
        label { class: "title",
          onclick: move |_| {
            collapsed = !collapsed;
            collapsed_state.write().0 = collapsed;
          },
        },
        title,
      }
    )}
}

pub fn DarkModeToggle(cx: Scope) -> Element {
    let dark_mode_state = use_shared_state::<DarkMode>(cx).unwrap();
    let mut dark_mode = dark_mode_state.read().0;
    let style = if dark_mode {
        "color: var(--bg)"
    } else {
        "color: var(--text)"
    };

    render! { rsx!(
      label {
        style: "{style}",
        label { class: "darkmode",
              onclick: move |_| {
                dark_mode = !dark_mode;
                dark_mode_state.write().0 = dark_mode;
              },
              Icon { width: 30,
                 height: 30,
                 icon: fi_icons::FiMoon,
              }
        },
      })
    }
}

pub fn About(cx: Scope) -> Element {
    render!(rsx! {
        p {
          b {"Typyst-Webdesktop"}
          "An open source typst desktop appmade in Rust."
        }
    })
}
