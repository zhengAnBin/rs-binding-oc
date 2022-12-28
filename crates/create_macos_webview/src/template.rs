use std::{
    fmt::{Display, Formatter},
    path,
    str::FromStr,
};

use crate::package_manager::PackageManager;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Template {
    Vue,
    VueTs,
    React,
    ReactTs,
}

impl Default for Template {
    fn default() -> Self {
        Self::ReactTs
    }
}

impl Display for Template {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Vue => write!(f, "vue"),
            Self::VueTs => write!(f, "vue-ts"),
            Self::React => write!(f, "react"),
            Self::ReactTs => write!(f, "react-ts"),
        }
    }
}

impl FromStr for Template {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "vue" => Ok(Self::Vue),
            "vue-ts" => Ok(Self::VueTs),
            "react" => Ok(Self::React),
            "react-ts" => Ok(Self::ReactTs),
            _ => Err(format!("Invalid template: {}", s)),
        }
    }
}

impl<'a> Template {
    pub const ALL: &'a [Template] = &[
        Template::Vue,
        Template::VueTs,
        Template::React,
        Template::ReactTs,
    ];

    // 生成项目模版
    pub fn render(
        &self,
        target_dir: &path::Path,
        pkg_manager: PackageManager,
        package_name: &str,
    ) -> anyhow::Result<()> {
        todo!()
    }
}
