use dioxus::prelude::*;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescMsg {
    pub webapp: String,
    pub android: String,
    pub linux: String,
}

#[derive(Store, Default, Debug)]
struct AppDialog {
    app_nm: String,
    desc: String,
    a_href: String,
    a_file_name: String,
    img_src: String,
    msg: String,
}

const BASE_URL: &str = "https://aki.omusubi.org";

#[component]
pub fn List(is_devel: bool, desc: DescMsg) -> Element {
    let apps_r = use_resource(move || async move { crate::backends::list_apps(is_devel).await });
    //
    let dialog = use_store(AppDialog::default);
    let dialog_app_nm = dialog.app_nm();
    let dialog_desc = dialog.desc();
    let dialog_a_href = dialog.a_href();
    let dialog_a_file_name = dialog.a_file_name();
    let dialog_img_src = dialog.img_src();
    let dialog_msg = dialog.msg();
    //
    rsx! {
        div { class: "app-list",
            if let Some(apps_r) = &*apps_r.read() {
                if let Ok(apps) = apps_r {
                    dialog { id: "app-list-dialog", class: "app-list-dialog",
                        h3 { class: "app-list-row-h", "{dialog_app_nm}" }
                        p { class: "app-list-row-p", "{dialog_desc}" }
                        a {
                            id: "download_link1",
                            class: "app-list-row-links-a",
                            target: "_blank",
                            href: "{dialog_a_href}",
                            download: "{dialog_a_file_name}",
                            onclick: move |_evt| async move {
                                download_file(dialog.a_href().to_string()).await;
                            },
                            img {
                                class: "app-list-row-links-a-img",
                                alt: "Web",
                                src: "{dialog_img_src}",
                            }
                            p { "{dialog_msg}" }
                        }
                        button {
                            class: "app-list-dialog-btn",
                            onclick: move |_evt| async move {
                                //dioxus::logger::tracing::info!("{_evt:#?}");
                                let js = r#"document.getElementById("app-list-dialog").close();"#;
                                let _ = document::eval(js).await;
                            },
                            "Close"
                        }
                    }
                    for app_info in apps.iter() {
                        AppListRowCm {
                            app_info: app_info.clone(),
                            dialog,
                            desc: desc.clone(),
                        }
                    }
                } else if let Err(e) = apps_r {
                    "Error:{e}"
                }
            } else {
                "Loading..."
            }
        }
    }
}

#[derive(Props, Debug, Clone, PartialEq)]
struct AppListRowProps {
    app_info: crate::backends::AppInfo,
    dialog: Store<AppDialog>,
    desc: DescMsg,
}

#[component]
pub fn AppListRowCm(props: AppListRowProps) -> Element {
    let app_info = props.app_info;
    let app_nm = app_info.name();
    let desc = app_info.desc();
    //
    let descmsg_s = use_signal(|| props.desc.clone());
    let app_nm_s = use_signal(|| app_nm.to_string());
    let desc_s = use_signal(|| desc.to_string());
    let app_info_s = use_store(|| app_info.clone());
    let apk_fnms_s = use_store(|| app_info_s().apk_fnms().to_vec());
    let appimage_fnms_s = use_store(|| app_info_s().appimage_fnms().to_vec());
    rsx! {
        div { class: "app-list-row",
            h3 { class: "app-list-row-h", "{app_nm}" }
            p { class: "app-list-row-p", "{desc}" }
            div { class: "app-list-row-links",
                a {
                    class: "app-list-row-links-a",
                    onclick: move |_evt| async move {
                        //dioxus::logger::tracing::info!("{_evt:#?}");
                        let url = format!("{BASE_URL}/{}/", app_nm_s());
                        props.dialog.app_nm().set(app_nm_s());
                        props.dialog.desc().set(desc_s());
                        props.dialog.a_href().set(url);
                        props.dialog.a_file_name().set("".to_string());
                        props.dialog.img_src().set(crate::WEBAPP_IMG.to_string());
                        props.dialog.msg().set(descmsg_s().webapp.clone());
                        let js = r#"document.getElementById("app-list-dialog").showModal();"#;
                        let _ = document::eval(js).await;
                    },
                    img {
                        class: "app-list-row-links-a-img",
                        alt: "Web",
                        src: crate::WEBAPP_IMG,
                    }
                }
                for apk_fnm in apk_fnms_s() {
                    a {
                        class: "app-list-row-links-a",
                        onclick: move |_evt| {
                            let app_nm = app_nm_s().clone();
                            let apk_fnm = apk_fnm.to_string();
                            spawn(async move {
                                //dioxus::logger::tracing::info!("{_evt:#?}");
                                let url = format!("{BASE_URL}/akiapp/android/{app_nm}/{apk_fnm}");
                                props.dialog.app_nm().set(app_nm_s());
                                props.dialog.desc().set(desc_s());
                                props.dialog.a_href().set(url);
                                props.dialog.a_file_name().set(apk_fnm.to_string());
                                props.dialog.img_src().set(crate::ANDROID_IMG.to_string());
                                props.dialog.msg().set(descmsg_s().android.clone());
                                let js = r#"document.getElementById("app-list-dialog").showModal();"#;
                                let _ = document::eval(js).await;
                            });
                        },
                        img {
                            class: "app-list-row-links-a-img",
                            alt: "Android",
                            src: crate::ANDROID_IMG,
                        }
                    }
                }
                for appimage_fnm in appimage_fnms_s() {
                    a {
                        class: "app-list-row-links-a",
                        onclick: move |_evt| {
                            let app_nm = app_nm_s().clone();
                            let appimage_fnm = appimage_fnm.to_string();
                            spawn(async move {
                                //dioxus::logger::tracing::info!("{_evt:#?}");
                                let url = format!("{BASE_URL}/akiapp/desktop/{app_nm}/{appimage_fnm}");
                                props.dialog.app_nm().set(app_nm_s());
                                props.dialog.desc().set(desc_s());
                                props.dialog.a_href().set(url);
                                props.dialog.a_file_name().set(appimage_fnm.to_string());
                                props.dialog.img_src().set(crate::LINUX_IMG.to_string());
                                props.dialog.msg().set(descmsg_s().linux.clone());
                                let js = r#"document.getElementById("app-list-dialog").showModal();"#;
                                let _ = document::eval(js).await;
                            });
                        },
                        img {
                            class: "app-list-row-links-a-img",
                            alt: "Linux",
                            src: crate::LINUX_IMG,
                        }
                    }
                }
            }
        }
    }
}

#[cfg(not(feature = "desktop"))]
async fn download_file(_url: String) {}

#[cfg(feature = "desktop")]
async fn download_file(_url: String) {
    /*
    //dioxus_logger::tracing::debug!("data: {:?}", evt.data());
    let js = format!(r#"{{return getAnchorsDownloadHref('{}');}}"#, id);
    let v = document::eval(&js).await.unwrap();
    let s = v.to_string();
    let anchorinfo = AnchorInfo::from_json_str(&s).unwrap();
    let filename = anchorinfo.download.unwrap();
    dioxus_logger::tracing::debug!("filename: {filename}");
    if let Some(path) = rfd::FileDialog::new().set_file_name(filename).save_file() {
        let content = anchorinfo.href.unwrap();
        let is_data = content.starts_with("data:");
        let is_blob = content.starts_with("blob:");
        if is_data || is_blob {
            let data_url = if is_blob {
                let js = format!(r#"{{parseBlobData_dxsend('{}');}}"#, content);
                let mut eval = document::eval(&js);
                let data_url = eval.recv::<String>().await.unwrap();
                data_url
            } else {
                content
            };
            save_data_uri0(&data_url, &path).unwrap();
        }
    }
    */
}
