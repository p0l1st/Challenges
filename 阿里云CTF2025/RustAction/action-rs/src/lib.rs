use std::{env, fs, path::Path, sync::LazyLock};

use config::Config;
use db::Database;
use model::ActionConfig;

pub mod db;
pub mod error;
pub mod model;
pub mod route;
pub mod util;

pub static CONFIG: LazyLock<ActionConfig> = LazyLock::new(|| {
    let config = load_config().expect("Failed to load config");
    init_config(&config).expect("Failed to init config");
    config
});

pub static DB: LazyLock<Database> = LazyLock::new(|| Database::new());

fn load_config() -> Result<ActionConfig, anyhow::Error> {
    let args = env::args();
    let config_path = args.skip(1).next().expect("Config file is required");

    let config = Config::builder()
        .add_source(config::File::with_name(&config_path))
        .build()?;

    Ok(config.try_deserialize::<ActionConfig>()?)
}

fn init_config(config: &ActionConfig) -> Result<(), anyhow::Error> {
    let jobs_path = Path::new(&config.workflow.jobs.path);
    let artifacts_path = Path::new(&config.workflow.artifacts.path);

    if !jobs_path.exists() {
        fs::create_dir(jobs_path)?;
    }

    if !artifacts_path.exists() {
        fs::create_dir(artifacts_path)?;
    }

    Ok(())
}
