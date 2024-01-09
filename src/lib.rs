use dioxus::prelude::*;
use dioxus_free_icons::{icons::fi_icons, Icon};
use dioxus_router::prelude::*;

#[component(no_case_check)]
pub fn GotoGithub(cx: Scope) -> Element {
    render! {
      Link {
      to: "https://github.com/rayslash/typst-webdesktop",
      "GitHub"
      }
    }
}

pub fn app(cx: Scope) -> Element {
    cx.render(rsx! {
      section { class: "app",
        style { include_str!("./style.css") }
        div { class: "navcontainer",
          header { class: "header",
            button { class: "burger",
                     Icon { width: 30,
                        height: 30,
                        icon: fi_icons::FiArrowDown,
                     }
            }
            h1 { class: "title",
                 "Typst Desktop" }
            button { class: "git",
                     onclick: move |_| GotoGithub(cx),
                     Icon { width: 30,
                        height: 30,
                        icon: fi_icons::FiGithub,
                     }
            }
          }
        }
        div { class: "bodycontainer",
          div { class: "workspace",
            div { class: "toolbar",
              button { class: "toolbut",
                       "Hi"
              }
            }

            div { class: "codearea",
              button { class: "codebut",
                       "There"
              }
            }

            div { class: "previewarea",
              button { class: "previewbut",
                       "Stranger"
              }
            }

          }
        }
      }
    })
}
