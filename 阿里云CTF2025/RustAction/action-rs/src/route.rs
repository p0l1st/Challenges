use std::io::Cursor;

use axum::{
    extract::{Multipart, Path},
    response::Html,
    Json,
};
use chrono::Local;
use tokio::{fs, process::Command};
use uuid::Uuid;

use crate::{
    error::AppError,
    model::{Artifact, Job, Mode, Workflow},
    util, CONFIG, DB,
};

pub async fn index() -> Html<String> {
    let content = include_str!("../public/index.html");
    Html(content.to_string())
}

pub async fn jobs() -> Html<String> {
    let content = include_str!("../public/jobs.html");
    Html(content.to_string())
}

pub async fn list_jobs() -> Json<Vec<Job>> {
    Json(DB.jobs())
}

pub async fn upload_job(mut multipart: Multipart) -> Result<String, AppError> {
    if !&CONFIG.workflow.jobs.enable {
        return Err(AppError(anyhow::anyhow!("Jobs module is disabled")));
    }

    let Some(field) = multipart.next_field().await? else {
        return Err(AppError(anyhow::anyhow!("No file uploaded")));
    };

    let id = Uuid::new_v4();
    let target_dir = std::path::Path::new(&CONFIG.workflow.jobs.path).join(id.to_string());
    fs::create_dir(&target_dir).await?;

    let bytes = field.bytes().await?;
    let cursor = Cursor::new(bytes);

    util::extract_zip(cursor, &target_dir)?;

    if let Err(e) = util::validate_job(&target_dir) {
        fs::remove_dir_all(&target_dir).await?;
        return Err(AppError(e));
    }

    let file = fs::File::open(target_dir.join(&CONFIG.workflow.name)).await?;
    let workflow: Workflow = serde_yaml::from_reader(file.into_std().await)?;

    let job = Job {
        id: id.to_string(),
        date: Local::now(),
        ..workflow.job
    };
    DB.push_job(job);

    Ok(format!("Create Job {} successfully", id))
}

pub async fn run_job(Path(id): Path<String>) -> Result<String, AppError> {
    if !&CONFIG.workflow.jobs.enable {
        return Err(AppError(anyhow::anyhow!("Job module is disabled")));
    }

    let Some(job) = DB.find_job(&id) else {
        return Err(AppError(anyhow::anyhow!("Job not found")));
    };

    let job_dir = std::path::Path::new(&CONFIG.workflow.jobs.path).join(&job.id);
    let temp_dir = tempfile::tempdir()?;
    fs::create_dir(temp_dir.path().join("src")).await?;

    for file in &job.files {
        if !CONFIG.workflow.security.files.contains(file) {
            return Err(AppError(anyhow::anyhow!("Invalid file")));
        }

        let src = job_dir.join(&CONFIG.workflow.work_dir).join(file);
        let dst = temp_dir.path().join("src").join(file);

        if src.is_file() {
            fs::copy(src, dst).await?;
        }
    }

    let cargo_toml = format!(
        include_str!("../templates/Cargo.toml.tpl"),
        name = job.config.name,
        version = job.config.version,
        edition = job.config.edition,
        description = job.config.description,
    );
    fs::write(temp_dir.path().join("Cargo.toml"), cargo_toml).await?;

    if !CONFIG.workflow.security.runs.contains(&job.run) {
        return Err(AppError(anyhow::anyhow!("Invalid command to run")));
    }

    let status = Command::new("/bin/bash")
        .current_dir(temp_dir.path())
        .arg("-c")
        .arg(job.run)
        .status()
        .await?;

    if status.success() {
        let src = match job.mode {
            Mode::Debug => temp_dir.path().join("target/debug").join(&job.config.name),
            Mode::Release => temp_dir
                .path()
                .join("target/release")
                .join(&job.config.name),
        };
        let dst = std::path::Path::new(&CONFIG.workflow.artifacts.path).join(&job.id);
        fs::copy(src, dst).await?;

        let artifact = Artifact {
            id: job.id,
            date: Local::now(),
            name: job.config.name,
        };
        DB.push_artifact(artifact);

        Ok(format!("Run Job {} successfully", id))
    } else {
        Err(AppError(anyhow::anyhow!(
            "Run Job {} failed with exit code: {}",
            id,
            status.code().unwrap()
        )))
    }
}

pub async fn artifacts() -> Html<String> {
    let content = include_str!("../public/artifacts.html");
    Html(content.to_string())
}

pub async fn list_artifacts() -> Json<Vec<Artifact>> {
    Json(DB.artifacts())
}

pub async fn download_artifact(Path(id): Path<String>) -> Result<Vec<u8>, AppError> {
    if !&CONFIG.workflow.artifacts.enable {
        return Err(AppError(anyhow::anyhow!("Artifacts module is disabled")));
    }

    let Some(artifact) = DB.find_artifact(&id) else {
        return Err(AppError(anyhow::anyhow!("Artifact not found")));
    };

    let artifact_path = std::path::Path::new(&CONFIG.workflow.artifacts.path).join(&artifact.id);
    let content = fs::read(artifact_path).await?;
    Ok(content)
}

pub async fn clean() -> Result<String, AppError> {
    fs::remove_dir_all(&CONFIG.workflow.jobs.path).await?;
    fs::remove_dir_all(&CONFIG.workflow.artifacts.path).await?;

    fs::create_dir(&CONFIG.workflow.jobs.path).await?;
    fs::create_dir(&CONFIG.workflow.artifacts.path).await?;

    DB.clear();

    Ok("Clean all the jobs and artifacts successfully".to_string())
}
