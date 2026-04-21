use dioxus::prelude::*;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescMsg {
    pub webapp: String,
    pub android: String,
    pub linux: String,
}

#[derive(Store, Default, Debug)]
struct AppDialog {
    is_open: bool,
    app_nm: String,
    desc: String,
    a_href: String,
    a_file_name: String,
    img_src: String,
    msg: String,
}

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

    // Inside the List component
    let is_open = dialog.is_open();

    // Define “side effects” that monitor state and call JS methods
    use_effect(move || {
        if is_open() {
            spawn(async move {
                let js = r#"document.getElementById("app-list-dialog").showModal();"#;
                let _ = document::eval(js).await;
            });
        } else {
            spawn(async move {
                let js = r#"document.getElementById("app-list-dialog").close();"#;
                let _ = document::eval(js).await;
            });
        }
    });
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
                                dialog.is_open().set(false);
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
    let app_info_sig = use_signal(|| props.app_info.clone());
    let desc_msg_sig = use_signal(|| props.desc.clone());
    let dialog = props.dialog;
    //
    let app_nm = app_info_sig.read().name().to_string();
    let desc = app_info_sig.read().desc().to_string();
    //
    rsx! {
        div { class: "app-list-row",
            h3 { class: "app-list-row-h", "{app_nm}" }
            p { class: "app-list-row-p", "{desc}" }
            div { class: "app-list-row-links",
                a {
                    class: "app-list-row-links-a",
                    onclick: move |_evt| async move {
                        //dioxus::logger::tracing::info!("{_evt:#?}");
                        let info = app_info_sig.read();
                        let name = info.name().to_string();
                        let base = crate::PUBLIC_URL();
                        let url = format!("{base}/{}/", name);
                        //
                        dialog.app_nm().set(name);
                        dialog.desc().set(info.desc().to_string());
                        dialog.a_href().set(url);
                        dialog.a_file_name().set("".to_string());
                        dialog.img_src().set(crate::WEBAPP_IMG.to_string());
                        dialog.msg().set(desc_msg_sig().webapp.clone());
                        dialog.is_open().set(true);
                    },
                    img {
                        class: "app-list-row-links-a-img",
                        alt: "Web",
                        src: crate::WEBAPP_IMG,
                    }
                }
                // APK link
                for apk_fnm in app_info_sig.read().apk_fnms().to_vec() {
                    a {
                        class: "app-list-row-links-a",
                        onclick: move |_evt| {
                            let info = app_info_sig.read();
                            let name = info.name().to_string();
                            let desc = info.desc().to_string();
                            let msg = desc_msg_sig.read().android.clone();
                            let apk = apk_fnm.clone();
                            let base = crate::PUBLIC_URL();
                            //
                            spawn(async move {
                                let url = format!("{base}/akiapp/android/{name}/{apk}");
                                dialog.app_nm().set(name);
                                dialog.desc().set(desc);
                                dialog.a_href().set(url);
                                dialog.a_file_name().set(apk);
                                dialog.img_src().set(crate::ANDROID_IMG.to_string());
                                dialog.msg().set(msg);
                                dialog.is_open().set(true);
                            });
                        },
                        img {
                            class: "app-list-row-links-a-img",
                            alt: "Android",
                            src: crate::ANDROID_IMG,
                        }
                    }
                }
                // AppImage Link
                for appimage_fnm in app_info_sig.read().appimage_fnms().to_vec() {
                    a {
                        class: "app-list-row-links-a",
                        onclick: move |_evt| {
                            let info = app_info_sig.read();
                            let name = info.name().to_string();
                            let desc = info.desc().to_string();
                            let msg = desc_msg_sig.read().linux.clone();
                            let img_fnm = appimage_fnm.clone();
                            let base = crate::PUBLIC_URL();
                            //
                            spawn(async move {
                                let url = format!("{base}/akiapp/desktop/{name}/{img_fnm}");
                                dialog.app_nm().set(name);
                                dialog.desc().set(desc);
                                dialog.a_href().set(url);
                                dialog.a_file_name().set(img_fnm);
                                dialog.img_src().set(crate::LINUX_IMG.to_string());
                                dialog.msg().set(msg);
                                dialog.is_open().set(true);
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
