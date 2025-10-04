use std::sync::Mutex;

use crate::model::{Artifact, Job};

pub struct Database {
    jobs: Mutex<Vec<Job>>,
    artifacts: Mutex<Vec<Artifact>>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            jobs: Mutex::new(Vec::new()),
            artifacts: Mutex::new(Vec::new()),
        }
    }

    pub fn jobs(&self) -> Vec<Job> {
        self.jobs.lock().unwrap().clone()
    }

    pub fn artifacts(&self) -> Vec<Artifact> {
        self.artifacts.lock().unwrap().clone()
    }

    pub fn push_job(&self, job: Job) {
        self.jobs.lock().unwrap().push(job);
    }

    pub fn push_artifact(&self, artifact: Artifact) {
        self.artifacts.lock().unwrap().push(artifact);
    }

    pub fn find_job(&self, id: &str) -> Option<Job> {
        self.jobs
            .lock()
            .unwrap()
            .iter()
            .find(|job| job.id == id)
            .cloned()
    }

    pub fn find_artifact(&self, id: &str) -> Option<Artifact> {
        self.artifacts
            .lock()
            .unwrap()
            .iter()
            .find(|artifact| artifact.id == id)
            .cloned()
    }

    pub fn clear(&self) {
        self.jobs.lock().unwrap().clear();
        self.artifacts.lock().unwrap().clear();
    }
}
