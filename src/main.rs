use dioxus::prelude::*;
#[cfg(feature = "desktop")]
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
    let level = dioxus::logger::tracing::Level::INFO;
    #[cfg(debug_assertions)]
    let level = dioxus::logger::tracing::Level::DEBUG;
    dioxus::logger::init(level).expect("failed to init logger");

    // In the case of release desktop and release mobile,
    // connect backend calls to public api
    #[cfg(not(debug_assertions))]
    #[cfg(any(feature = "desktop", feature = "mobile"))]
    {
        // Specify the URL that previously delpoyed the public webapp.
        // This webapp was created with `dx bundle --web`.
        let backend_url = "https://aki.omusubi.org/akiapp";
        dioxus::fullstack::set_server_url(backend_url);
    }

    // In the case of only release desktop, set a window title
    #[cfg(feature = "desktop")]
    dioxus::LaunchBuilder::new()
        .with_cfg(
            Config::default().with_menu(None).with_window(
                WindowBuilder::new()
                    .with_maximized(false)
                    .with_title("Aki's App List"),
            ),
        )
        .launch(App);

    // In the other case, simple launch app
    #[cfg(not(feature = "desktop"))]
    dioxus::launch(App);
}

// Define default value as empty or temporary value
pub static PUBLIC_URL: GlobalSignal<String> =
    Signal::global(|| "https://aki.omusubi.org".to_string());

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/css/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/css/tailwind.css");

const APP_IMG: Asset = asset!("/assets/app.png");
const EMPTY_IMG: Asset = asset!("/assets/empty.png");
const WEBAPP_IMG: Asset = asset!("/assets/img/webapp.png");
const ANDROID_IMG: Asset = asset!("/assets/img/android.png");
const LINUX_IMG: Asset = asset!("/assets/img/linux.jpg");

#[component]
fn App() -> Element {
    // Get URL from server only once at startup
    use_future(|| async move {
        if let Ok(url) = crate::backends::get_base_config().await {
            *PUBLIC_URL.write() = url;
        }
    });

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
