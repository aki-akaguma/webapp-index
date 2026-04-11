use crate::components::List;
use dioxus::prelude::*;

#[component]
pub fn Devel() -> Element {
    let page_header = "Aki App Development";
    let page_desc = "These are apps created by Aki. I aimed for simplicity and clarity.";
    //
    rsx! {
        div { class: "app-header",
            h1 { class: "app-header-h font-bagel",
                a { href: ".",
                    img { class: "app-img", alt: "App", src: crate::APP_IMG }
                }
                p { "{page_header}" }
            }
            p { class: "app-header-p", "{page_desc}" }
        }
        List { is_devel: true }
    }
}
