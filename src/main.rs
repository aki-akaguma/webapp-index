use dioxus::prelude::*;
#[cfg(all(not(debug_assertions), feature = "desktop"))]
use dioxus_desktop::{Config, WindowBuilder};

mod backends;
mod components;
mod utils;
mod views;

use components::*;
use views::{Devel, Home};

fn main() {
    // You can set the ports and IP manually with env vars:
    //   server launch:
    //     IP="0.0.0.0" PORT=8080 ./server

    // You can supplement panic on  firefox browser.
    #[cfg(feature = "web")]
    console_error_panic_hook::set_once();

    #[cfg(not(debug_assertions))]
    let level = dioxus_logger::tracing::Level::INFO;
    #[cfg(debug_assertions)]
    let level = dioxus_logger::tracing::Level::DEBUG;
    dioxus_logger::init(level).expect("failed to init logger");

    dioxus::launch(App);
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/css/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/css/tailwind.css");

const APP_IMG: Asset = asset!("/assets/app.png");
const WEBAPP_IMG: Asset = asset!("/assets/img/webapp.png");
const ANDROID_IMG: Asset = asset!("/assets/img/android.png");

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link {
            rel: "preconnect",
            href: "https://fonts.gstatic.com",
            crossorigin: "",
        }
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Bagel+Fat+One:wght@400&display=swap",
        }
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        div { id: "app-main", class: "app-main", Router::<Route> {} }
    }
}

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[route("/")]
    Home,
    #[route("/devel")]
    Devel,
    //
    #[route("/:..segments")]
    PageNotFound { segments: Vec<String> },
}
