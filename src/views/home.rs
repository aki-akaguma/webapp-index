use crate::components::List;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let page_header = "Aki App";
    let page_desc = "These are apps created by Aki. I aimed for simplicity and clarity.";
    let webapp_desc = "Web application. Tap to use immediately.";
    let android_desc = "Android APK. Tap to download and install.";
    //
    rsx! {
        div { class: "app-header",
            h1 { class: "app-header-h font-bagel",
                img { class: "app-img", alt: "App", src: crate::APP_IMG }
                p { "{page_header}" }
            }
            p { class: "app-header-p", "{page_desc}" }
        }
        div { class: "app-section",
            div { class: "app-section-webapp",
                img {
                    class: "app-section-img",
                    alt: "Web",
                    src: crate::WEBAPP_IMG,
                }
                p { "{webapp_desc}" }
            }
            div { class: "app-section-android",
                img {
                    class: "app-section-img",
                    alt: "Android",
                    src: crate::ANDROID_IMG,
                }
                p { "{android_desc}" }
            }
        }
        List { is_devel: false }
    }
}
