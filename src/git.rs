use crate::gitlab::Project;
use miette::{IntoDiagnostic, Result};
use std::process::Command;

pub fn clone_project(project: &Project, project_dir: &str) -> Result<()> {
    let path = format!("{project_dir}/{}", project.path_with_namespace);

    Command::new("git")
        .args(["clone", &project.ssh_url_to_repo, &path])
        .output()
        .into_diagnostic()?;

    Ok(())
}
