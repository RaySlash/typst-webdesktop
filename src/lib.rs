use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::fi_icons};

pub fn app(cx: Scope) -> Element {
  cx.render(rsx! {
    section { class: "typstapp",
      style { include_str!("./style.css") }
      div { class: "navcontainer",
        header { class: "header",
          h1 { class: "burger",
               Icon { width: 30,
                      height: 30,
                      icon: fi_icons::FiArrowDown,
               }
          }
          h1 { class: "title",
               "Typst Desktop" }
          h1 { class: "git",
               Icon { width: 30,
                      height: 30,
                      icon: fi_icons::FiGithub,
               }
          }
        }
      }
      div { class: "workspace",
        div { class: "toolbar",
          
        }
        
        div { class: "codearea",
        
        }
        
        div { class: "previewarea",

        }
      }
    }
  })
}
