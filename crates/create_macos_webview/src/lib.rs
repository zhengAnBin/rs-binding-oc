use crate::{colors::*, package_manager::PackageManager};
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use std::{ffi::OsString, fs, process::exit};
mod cli;
mod colors;
mod package_manager;
mod template;
pub fn run<I, A>(args: I, bin_name: Option<String>)
where
    I: IntoIterator<Item = A>,
    A: Into<OsString> + Clone,
{
    if let Err(e) = try_run(args, bin_name) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn try_run<I, A>(args: I, bin_name: Option<String>) -> anyhow::Result<()>
where
    I: IntoIterator<Item = A>,
    A: Into<OsString> + Clone,
{
    let args = cli::parse(args.into_iter().map(Into::into).collect(), bin_name)?;
    let defaults = cli::Args::default();
    let skip = args.skip_prompts;
    let cwd = std::env::current_dir()?;

    let project_name = args.project_name.unwrap_or_else(|| {
        if skip {
            defaults.project_name.unwrap()
        } else {
            Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Project name")
                .default("macos-webview".to_string())
                .interact_text()
                .unwrap()
        }
    });

    let target_dir = cwd.join(&project_name);

    let package_name = if is_valid_pkg_name(&project_name) {
        project_name.clone()
    } else {
        let valid_name = to_valid_pkg_name(&project_name);
        if skip {
            valid_name
        } else {
            // 等待终端输入项目名称
            Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Package name")
                .default(valid_name.clone())
                .with_initial_text(valid_name)
                .validate_with(|input: &String| {
                    if is_valid_pkg_name(input) {
                        Ok(())
                    } else {
                        Err("Invalid package name".to_string())
                    }
                })
                .interact_text()
                .unwrap()
        }
    };

    if target_dir.exists() && target_dir.read_dir()?.next().is_some() {
        let overrwite = if skip {
            false
        } else {
            Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt(format!(
                    "{} directory is not empty, do you want to overwrite?",
                    if target_dir == cwd {
                        "Current directory".to_string()
                    } else {
                        target_dir
                            .file_name()
                            .unwrap()
                            .to_string_lossy()
                            .to_string()
                    }
                ))
                .default(false)
                .interact()
                .unwrap()
        };
        if !overrwite {
            eprintln!("Operation Cancelled");
            exit(1);
        }
    }

    let pkg_manager = args.manager.unwrap_or_else(|| {
        if skip {
            defaults.manager.unwrap()
        } else {
            let managers = PackageManager::ALL.to_vec();
            let managers = args
                .template
                .map(|t| {
                    managers
                        .iter()
                        .copied()
                        .filter(|p| p.templates().contains(&t))
                        .collect::<Vec<_>>()
                })
                .unwrap_or(managers);

            if managers.len() == 1 {
                managers[0]
            } else {
                let index = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Select package manager")
                    .items(&managers)
                    .default(0)
                    .interact()
                    .unwrap();
                managers[index]
            }
        }
    });

    let templates = pkg_manager.templates();
    let template = args.template.unwrap_or_else(|| {
        if skip {
            defaults.template.unwrap()
        } else {
            let index = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select template")
                .items(&templates)
                .default(0)
                .interact()
                .unwrap();
            templates[index]
        }
    });

    if !templates.contains(&template) {
        eprintln!("error: the template is not suppported");
        exit(1);
    }

    if target_dir.exists() {
        // safe to remove, because upon reaching this line, the user accepted to overwrite
        fs::remove_dir_all(&target_dir)?
    };
    fs::create_dir_all(&target_dir)?;

    template.render(&target_dir, pkg_manager, &package_name)?;

    Ok(())
}

// 验证包名是否合法
fn is_valid_pkg_name(project_name: &str) -> bool {
    !project_name
        .chars()
        .next()
        .map(|c| c.is_ascii_digit())
        .unwrap_or_default()
        && !project_name
            .chars()
            .any(|ch| !(ch.is_alphanumeric() || ch == '-' || ch == '_'))
}

// 将包名转换为合法的包名
fn to_valid_pkg_name(project_name: &str) -> String {
    #[allow(clippy::collapsible_str_replace)]
    let ret = project_name
        .trim()
        .to_lowercase()
        .replace(':', "-")
        .replace(';', "-")
        .replace(' ', "-")
        .replace('~', "-")
        .replace('.', "")
        .replace('\\', "")
        .replace('/', "");

    let ret = ret
        .chars()
        .skip_while(|ch| ch.is_ascii_digit() || *ch == '-')
        .collect::<String>();

    ret
}
