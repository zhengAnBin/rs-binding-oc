use crate::{package_manager::PackageManager, template::Template};
use pico_args::Arguments;
use std::ffi::OsString;

#[derive(Debug)]
pub struct Args {
    pub project_name: Option<String>,
    pub template: Option<Template>,
    pub skip_prompts: bool,
    pub manager: Option<PackageManager>,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            project_name: Some("macos-webview".to_string()),
            template: Some(Template::ReactTs),
            skip_prompts: false,
            manager: Some(PackageManager::Pnpm),
        }
    }
}

pub fn parse(args: Vec<OsString>, bin_name: Option<String>) -> anyhow::Result<Args> {
    let mut args = Arguments::from_vec(args);

    if args.contains(["-h", "--help"]) {
        let help = format!(
            r#"
            Usage: {name} [options] [project_name]
            Options:
                -h, --help      Print this help message
                -t, --template  Template to use"#,
            name = bin_name.unwrap_or_else(|| env!("CARGO_PKG_NAME").to_string()),
        );
        println!("{}", help);
        std::process::exit(0);
    }
    if args.contains(["-v", "--version"]) {
        println!("{}", env!("CARGO_PKG_VERSION"));
        std::process::exit(0);
    }
    let project_name = args.opt_free_from_str()?;
    let template = args.opt_value_from_str(["-t", "--template"])?;
    let skip_prompts = args.contains(["-y", "--yes"]);
    let manager = args.opt_value_from_str(["-m", "--manager"])?;
    Ok(Args {
        project_name,
        template,
        skip_prompts,
        manager,
    })
}
