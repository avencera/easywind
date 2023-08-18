use std::path::PathBuf;

use etcetera::{AppStrategy, AppStrategyArgs};
use once_cell::sync::Lazy;

pub static LATEST_TAILWIND_VERSION: &str = "3.3.3";

pub static CONFIG_DIR: Lazy<PathBuf> = Lazy::new(|| {
    etcetera::app_strategy::choose_app_strategy(AppStrategyArgs {
        top_level_domain: "avencera.com".into(),
        author: "praveen".into(),
        app_name: "easywind".into(),
    })
    .expect("unable to get config dir for easywind")
    .config_dir()
});

pub static TAILWIND_BIN_DIR: Lazy<PathBuf> = Lazy::new(|| {
    CONFIG_DIR
        .join("bin")
        .join("tailwind")
        .join(LATEST_TAILWIND_VERSION)
});

pub static TAILWIND_CLI_PATH: Lazy<PathBuf> = Lazy::new(|| TAILWIND_BIN_DIR.join("tailwind"));
