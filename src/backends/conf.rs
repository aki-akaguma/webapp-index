use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config {
    pub base: ConfBase,
    pub apps: Vec<ConfApp>,
    pub dev_apps: Vec<ConfApp>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ConfBase {
    base_path: String,
}

macro_rules! fn_is_xxx_yyy {
    ($fnm:ident, $nm:ident) => {
        #[allow(dead_code)]
        #[inline(always)]
        pub fn $fnm(&self) -> bool {
            if let Some(b) = &self.$nm {
                b.$fnm()
            } else {
                false
            }
        }
    };
}

macro_rules! fn_is_xxx {
    ($fnm:ident, $nm:ident) => {
        #[allow(dead_code)]
        #[inline(always)]
        pub fn $fnm(&self) -> bool {
            if let Some(b) = self.$nm {
                b
            } else {
                false
            }
        }
    };
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ConfApp {
    name: String,
    desc: Option<String>,
    web: Option<bool>,
    desktop: Option<ConfDesktop>,
    android: Option<ConfAndroid>,
}

impl ConfApp {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub fn desc(&self) -> &str {
        if let Some(desc) = &self.desc {
            desc.as_str()
        } else {
            ""
        }
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub fn is_web(&self) -> bool {
        if let Some(b) = self.web {
            b
        } else {
            false
        }
    }
    //
    #[allow(dead_code)]
    #[inline(always)]
    pub fn is_desktop(&self) -> bool {
        self.is_desktop_windows() || self.is_desktop_windows()
    }
    fn_is_xxx_yyy!(is_desktop_linux, desktop);
    fn_is_xxx_yyy!(is_desktop_windows, desktop);
    //
    #[allow(dead_code)]
    #[inline(always)]
    pub fn is_android(&self) -> bool {
        self.is_android_wva() || self.is_android_aarch64() || self.is_android_x86_64()
    }
    fn_is_xxx_yyy!(is_android_wva, android);
    fn_is_xxx_yyy!(is_android_aarch64, android);
    fn_is_xxx_yyy!(is_android_x86_64, android);
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ConfDesktop {
    linux: Option<bool>,
    windows: Option<bool>,
}
impl ConfDesktop {
    fn_is_xxx!(is_desktop_linux, linux);
    fn_is_xxx!(is_desktop_windows, windows);
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ConfAndroid {
    wva: Option<bool>,
    aarch64: Option<bool>,
    x86_64: Option<bool>,
}
impl ConfAndroid {
    fn_is_xxx!(is_android_wva, wva);
    fn_is_xxx!(is_android_aarch64, aarch64);
    fn_is_xxx!(is_android_x86_64, x86_64);
}

#[cfg(test)]
mod test {
    use super::*;
    //
    #[test]
    fn test_seri00() {
        let conf = Config {
            base: ConfBase {
                base_path: "/opt".to_string(),
            },
            apps: vec![],
            dev_apps: vec![],
        };
        let s = toml::to_string(&conf).unwrap();
        assert_eq!(
            s,
            concat!(
                r#"apps = []"#,
                "\n",
                r#"dev_apps = []"#,
                "\n\n",
                r#"[base]"#,
                "\n",
                r#"base_path = "/opt""#,
                "\n"
            )
        );
    }
    #[test]
    fn test_seri01() {
        let conf = Config {
            base: ConfBase {
                base_path: "/opt".to_string(),
            },
            apps: vec![ConfApp {
                name: "".to_string(),
                desc: Some("".to_string()),
                ..Default::default()
            }],
            dev_apps: vec![ConfApp {
                name: "".to_string(),
                desc: Some("".to_string()),
                web: Some(true),
                ..Default::default()
            }],
        };
        let s = toml::to_string(&conf).unwrap();
        assert_eq!(
            s,
            concat!(
                r#"[base]"#,
                "\n",
                r#"base_path = "/opt""#,
                "\n\n",
                r#"[[apps]]"#,
                "\n",
                r#"name = """#,
                "\n",
                r#"desc = """#,
                "\n\n",
                r#"[[dev_apps]]"#,
                "\n",
                r#"name = """#,
                "\n",
                r#"desc = """#,
                "\n",
                r#"web = true"#,
                "\n"
            )
        );
    }
    #[test]
    fn test_seri02() {
        let conf = Config {
            base: ConfBase {
                base_path: "/opt".to_string(),
            },
            apps: vec![ConfApp {
                name: "app1".to_string(),
                desc: None,
                desktop: Some(ConfDesktop {
                    linux: Some(true),
                    ..Default::default()
                }),
                ..Default::default()
            }],
            dev_apps: vec![ConfApp {
                name: "dev-app1".to_string(),
                desc: None,
                web: Some(true),
                desktop: Some(ConfDesktop {
                    linux: Some(true),
                    ..Default::default()
                }),
                ..Default::default()
            }],
        };
        let s = toml::to_string(&conf).unwrap();
        assert_eq!(
            s,
            concat!(
                r#"[base]"#,
                "\n",
                r#"base_path = "/opt""#,
                "\n\n",
                r#"[[apps]]"#,
                "\n",
                r#"name = "app1""#,
                "\n\n",
                r#"[apps.desktop]"#,
                "\n",
                r#"linux = true"#,
                "\n\n",
                r#"[[dev_apps]]"#,
                "\n",
                r#"name = "dev-app1""#,
                "\n",
                r#"web = true"#,
                "\n\n",
                r#"[dev_apps.desktop]"#,
                "\n",
                r#"linux = true"#,
                "\n",
            )
        );
    }
    //
    #[test]
    fn test_deseri00() {
        let conf: Config = toml::from_str(
            r#"
            apps = []
            dev_apps = []
            #
            [base]
            base_path = "/opt"
            "#,
        )
        .unwrap();
        assert_eq!(conf.base.base_path, "/opt");
        assert!(conf.apps.is_empty());
    }
    #[test]
    fn test_deseri01() {
        let conf: Config = toml::from_str(
            r#"
            dev_apps = []
            #
            [base]
            base_path = "/opt"
            #
            [[apps]]
            name = ""
            desc = ""
            "#,
        )
        .unwrap();
        assert_eq!(conf.base.base_path, "/opt");
        assert!(!conf.apps.is_empty());
    }
    #[test]
    fn test_deseri02() {
        let conf: Config = toml::from_str(
            r#"
            dev_apps = []
            #
            [base]
            base_path = "/opt"
            #
            [[apps]]
            name = "app1"
            [apps.desktop]
            linux = true
            "#,
        )
        .unwrap();
        assert_eq!(conf.base.base_path, "/opt");
        assert!(!conf.apps.is_empty());
        //assert_eq!(format!("{conf:?}"), "");
        //
        assert_eq!(conf.apps[0].name, "app1");
        assert!(!conf.apps[0].is_web());
        assert!(conf.apps[0].is_desktop_linux());
        assert!(!conf.apps[0].is_desktop_windows());
        assert!(!conf.apps[0].is_android_wva());
        assert!(!conf.apps[0].is_android_aarch64());
        assert!(!conf.apps[0].is_android_x86_64());
    }
    #[test]
    fn test_deseri03() {
        let conf: Config = toml::from_str(
            r#"
            dev_apps = []
            #
            [base]
            base_path = "/opt"
            #
            [[apps]]
            name = "app1"
            [apps.desktop]
            linux = true
            #
            [[apps]]
            name = "app2"
            [apps.desktop]
            linux = true
            windows = true
            [apps.android]
            wva = true
            "#,
        )
        .unwrap();
        assert_eq!(conf.base.base_path, "/opt");
        assert!(!conf.apps.is_empty());
        //assert_eq!(format!("{conf:?}"), "");
        //
        assert_eq!(conf.apps[0].name, "app1");
        assert!(!conf.apps[0].is_web());
        assert!(conf.apps[0].is_desktop_linux());
        assert!(!conf.apps[0].is_desktop_windows());
        assert!(!conf.apps[0].is_android_wva());
        assert!(!conf.apps[0].is_android_aarch64());
        assert!(!conf.apps[0].is_android_x86_64());
        //
        assert_eq!(conf.apps[1].name, "app2");
        assert!(!conf.apps[1].is_web());
        assert!(conf.apps[1].is_desktop_linux());
        assert!(conf.apps[1].is_desktop_windows());
        assert!(conf.apps[1].is_android_wva());
        assert!(!conf.apps[1].is_android_aarch64());
        assert!(!conf.apps[1].is_android_x86_64());
    }
    #[test]
    fn test_deseri04() {
        let conf: Config = toml::from_str(
            r#"
            [base]
            base_path = "/opt"
            #
            [[apps]]
            name = "app1"
            [apps.desktop]
            linux = true
            #
            [[apps]]
            name = "app2"
            [apps.desktop]
            linux = true
            windows = true
            [apps.android]
            wva = true
            #
            [[dev_apps]]
            name = "dev-app1"
            web = true
            [dev_apps.desktop]
            linux = true
            windows = true
            [dev_apps.android]
            wva = true
            "#,
        )
        .unwrap();
        assert_eq!(conf.base.base_path, "/opt");
        assert!(!conf.apps.is_empty());
        //assert_eq!(format!("{conf:?}"), "");
        //
        assert_eq!(conf.apps[0].name, "app1");
        assert!(!conf.apps[0].is_web());
        assert!(conf.apps[0].is_desktop_linux());
        assert!(!conf.apps[0].is_desktop_windows());
        assert!(!conf.apps[0].is_android_wva());
        assert!(!conf.apps[0].is_android_aarch64());
        assert!(!conf.apps[0].is_android_x86_64());
        //
        assert_eq!(conf.apps[1].name, "app2");
        assert!(!conf.apps[1].is_web());
        assert!(conf.apps[1].is_desktop_linux());
        assert!(conf.apps[1].is_desktop_windows());
        assert!(conf.apps[1].is_android_wva());
        assert!(!conf.apps[1].is_android_aarch64());
        assert!(!conf.apps[1].is_android_x86_64());
        //
        assert_eq!(conf.dev_apps[0].name, "dev-app1");
        assert!(conf.dev_apps[0].is_web());
        assert!(conf.dev_apps[0].is_desktop_linux());
        assert!(conf.dev_apps[0].is_desktop_windows());
        assert!(conf.dev_apps[0].is_android_wva());
        assert!(!conf.dev_apps[0].is_android_aarch64());
        assert!(!conf.dev_apps[0].is_android_x86_64());
    }
    #[test]
    fn test_deseri05() {
        let conf: Config = toml::from_str(
            r#"
            [base]
            base_path = "/opt"
            #
            [[apps]]
            name = "app1"
            [apps.desktop]
            linux = true
            #
            [[apps]]
            name = "app2"
            [apps.desktop]
            linux = true
            windows = true
            [apps.android]
            wva = true
            #
            [[dev_apps]]
            name = "dev-app1"
            web = true
            dev_apps.android.wva = true
            [dev_apps.desktop]
            linux = true
            windows = true
            [dev_apps.android]
            wva = true
            "#,
        )
        .unwrap();
        assert_eq!(conf.base.base_path, "/opt");
        assert!(!conf.apps.is_empty());
        //assert_eq!(format!("{conf:?}"), "");
        //
        assert_eq!(conf.apps[0].name, "app1");
        assert!(!conf.apps[0].is_web());
        assert!(conf.apps[0].is_desktop_linux());
        assert!(!conf.apps[0].is_desktop_windows());
        assert!(!conf.apps[0].is_android_wva());
        assert!(!conf.apps[0].is_android_aarch64());
        assert!(!conf.apps[0].is_android_x86_64());
        //
        assert_eq!(conf.apps[1].name, "app2");
        assert!(!conf.apps[1].is_web());
        assert!(conf.apps[1].is_desktop_linux());
        assert!(conf.apps[1].is_desktop_windows());
        assert!(conf.apps[1].is_android_wva());
        assert!(!conf.apps[1].is_android_aarch64());
        assert!(!conf.apps[1].is_android_x86_64());
        //
        assert_eq!(conf.dev_apps[0].name, "dev-app1");
        assert!(conf.dev_apps[0].is_web());
        assert!(conf.dev_apps[0].is_desktop_linux());
        assert!(conf.dev_apps[0].is_desktop_windows());
        assert!(conf.dev_apps[0].is_android_wva());
        assert!(!conf.dev_apps[0].is_android_aarch64());
        assert!(!conf.dev_apps[0].is_android_x86_64());
    }
}
