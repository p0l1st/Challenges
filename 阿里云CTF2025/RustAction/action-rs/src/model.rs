use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ActionConfig {
    pub app: AppConfig,
    pub workflow: WorkflowConfig,
}

#[derive(Deserialize)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct WorkflowConfig {
    pub name: String,
    pub work_dir: String,
    pub jobs: JobsConfig,
    pub artifacts: ArtifactsConfig,
    pub security: SecurityConfig,
}

#[derive(Deserialize)]
pub struct JobsConfig {
    pub enable: bool,
    pub path: String,
}

#[derive(Deserialize)]
pub struct ArtifactsConfig {
    pub enable: bool,
    pub path: String,
}

#[derive(Deserialize)]
pub struct SecurityConfig {
    pub files: Vec<String>,
    pub runs: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Workflow {
    pub job: Job,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Job {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub date: DateTime<Local>,
    pub name: String,
    pub mode: Mode,
    pub config: Config,
    pub files: Vec<String>,
    pub run: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Mode {
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "release")]
    Release,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    pub name: String,
    pub version: String,
    pub edition: String,
    pub description: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Artifact {
    pub id: String,
    pub date: DateTime<Local>,
    pub name: String,
}
