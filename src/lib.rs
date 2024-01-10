#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_free_icons::{icons::fi_icons, Icon};
use dioxus_router::prelude::*;

#[derive(Clone, Routable)]
enum Route {
    #[route("/")]
    About {},
}

pub fn App(cx: Scope) -> Element {
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
    render!(rsx! {
          div { class: "navcontainer",
            header { class: "header",
              a { class: "burger",
                       Icon { width: 30,
                          height: 30,
                          icon: fi_icons::FiArrowDown,
                       }
              }
              h1 { class: "title",
                   "Typst Desktop" }
              a { class: "git",
                  href: "https://github.com/rayslash/typst-webdesktop",
                  Icon { width: 30,
                         height: 30,
                         icon: fi_icons::FiGithub,
                  }
              }
            }
          }
    })
}

pub fn About(cx: Scope) -> Element {
    render!(rsx! {
        p {
          b {"Typyst-Webdesktop"}
          "An open source typst desktop appmade in Rust."
        }
    })
}
