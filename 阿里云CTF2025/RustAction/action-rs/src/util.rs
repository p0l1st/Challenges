use std::{
    fs,
    io::{self, Read, Seek},
    path::Path,
};

use crate::{error::AppError, CONFIG};

pub fn extract_zip(reader: impl Read + Seek, target_dir: &Path) -> Result<(), AppError> {
    let mut archive = zip::ZipArchive::new(reader)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => target_dir.join(path),
            None => continue,
        };

        if file.is_symlink() {
            continue;
        }

        if file.is_dir() {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                if !parent.exists() {
                    fs::create_dir_all(&parent)?;
                }
            }

            let mut outfile = fs::File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(())
}

pub fn validate_job(target_dir: &Path) -> Result<(), anyhow::Error> {
    for entry in target_dir.read_dir()? {
        let entry = entry?;
        let file_name = entry.file_name().to_str().unwrap().to_string();

        if file_name != CONFIG.workflow.name && file_name != CONFIG.workflow.work_dir {
            return Err(anyhow::anyhow!("Unexpected workflow file"));
        }
    }

    let workflow_file = target_dir.join(&CONFIG.workflow.name);
    let work_dir = target_dir.join(&CONFIG.workflow.work_dir);

    if !workflow_file.is_file() || !work_dir.is_dir() {
        return Err(anyhow::anyhow!(
            "Neither workflow file nor work dir was found"
        ));
    }

    for entry in work_dir.read_dir()? {
        let entry = entry?;
        let file_type = entry.file_type()?;

        if file_type.is_dir() {
            return Err(anyhow::anyhow!("Sub dir is not allowed in work dir"));
        } else if file_type.is_symlink() {
            return Err(anyhow::anyhow!("Symlink is not allowed in work dir"));
        } else {
            let file_name = entry.file_name().to_str().unwrap().to_string();

            if !CONFIG.workflow.security.files.contains(&file_name) {
                return Err(anyhow::anyhow!("Forbidden file"));
            }
        }
    }

    Ok(())
}
