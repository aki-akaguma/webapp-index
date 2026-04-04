use dioxus::prelude::*;

#[component]
pub fn List() -> Element {
    let apps_r = use_resource(move || async move { crate::backends::list_apps().await });
    rsx! {
        div { class: "app-list",
            if let Some(apps_r) = &*apps_r.read() {
                if let Ok(apps) = apps_r {
                    for (app_nm , desc , apk_fnms) in apps.iter() {
                        AppListRowCm {
                            app_nm,
                            desc,
                            apk_fnms: apk_fnms.to_vec(),
                        }
                    }
                } else if let Err(e) = apps_r {
                    "Error:{e}"
                } else {
                    "Not reached"
                }
            } else {
                "Loading..."
            }
        }
    }
}

#[derive(Props, Debug, Clone, PartialEq)]
struct AppListRowProps {
    app_nm: String,
    desc: String,
    apk_fnms: Vec<String>,
}

#[component]
pub fn AppListRowCm(props: AppListRowProps) -> Element {
    let app_nm = props.app_nm;
    let desc = props.desc;
    let apk_fnms = props.apk_fnms;
    rsx! {
        div { class: "app-list-row",
            h3 { class: "app-list-row-h", "{app_nm}" }
            p { class: "app-list-row-p", "{desc}" }
            div { class: "app-list-row-links",
                a { class: "app-list-row-links-a", href: "/{app_nm}/",
                    img {
                        class: "app-list-row-links-a-img",
                        alt: "Web",
                        src: crate::WEBAPP_IMG,
                    }
                }
                for apk_fnm in apk_fnms.iter() {
                    a {
                        class: "app-list-row-links-a",
                        href: "android/{app_nm}/{apk_fnm}",
                        img {
                            class: "app-list-row-links-a-img",
                            alt: "Android",
                            src: crate::ANDROID_IMG,
                        }
                    }
                }
            }
        }
    }
}
