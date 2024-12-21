use std::path::Path;

use miette::{bail, Result};
use skim::prelude::*;

use crate::{
    git,
    gitlab::{self, Project},
};

pub fn open(
    gitlab_client: &gitlab::Client,
    group: Option<String>,
    project_dir: &str,
) -> Result<()> {
    tracing::debug!("fetching all projects");
    let projects = match group {
        Some(group) => gitlab_client.projects_for_group(&group)?,
        None => gitlab_client.projects()?,
    };

    let project = select_project(&projects)?;
    tracing::debug!(
        "selected project: {} {}",
        project.id,
        project.path_with_namespace
    );

    if Path::new(&format!("{}/{}", project_dir, project.path_with_namespace)).exists() {
        tracing::debug!("project already exists");

        println!("{project_dir}/{}", project.path_with_namespace);

        return Ok(());
    }

    tracing::debug!("cloning");
    git::clone_project(&project, project_dir)?;

    println!("{project_dir}/{}", project.path_with_namespace);

    Ok(())
}

fn select_project(projects: &Vec<Project>) -> Result<Project> {
    let options = SkimOptionsBuilder::default()
        .height("100%".to_owned())
        .multi(false)
        .preview_window("right:30%".to_owned())
        .preview(Some("".to_owned())) // use an empty command as the preview is overriden by the
        // item
        .build()
        .unwrap();

    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();
    for project in projects {
        let _ = tx_item.send(Arc::new(project.clone()));
    }
    drop(tx_item);

    let selected_items = Skim::run_with(&options, Some(rx_item))
        .map(|out| out.selected_items)
        .unwrap_or_default();

    let selected_item = match selected_items.first() {
        Some(item) => item,
        None => bail!("no selection"),
    };

    match projects
        .iter()
        .find(|p| p.path_with_namespace == selected_item.text())
    {
        Some(item) => Ok(item.clone()),
        None => bail!("something went wrong when processing the selection"),
    }
}
