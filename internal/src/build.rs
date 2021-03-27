use anyhow::{ensure, Result};
use std::{ffi::OsStr, path::Path, process::Command};

pub fn build<K, V>(envs: &[(K, V)], path: Option<&Path>) -> Result<()>
where
    K: AsRef<OsStr>,
    V: AsRef<OsStr>,
{
    let mut command = Command::new("cargo");

    if let Some(path) = path {
        command.current_dir(path);
    }

    let envs: Vec<(String, String)> = envs
        .iter()
        .map(|(k, v)| {
            (
                k.as_ref().to_string_lossy().to_string(),
                v.as_ref().to_string_lossy().to_string(),
            )
        })
        .collect();

    log::debug!("{:?}", envs);

    command.envs(envs).args(&["build"]);

    log::debug!("{:?}", command);

    let status = command.status()?;

    ensure!(status.success(), "command failed: {:?}", command);

    Ok(())
}
