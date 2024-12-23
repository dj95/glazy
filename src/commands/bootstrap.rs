use std::{fs, path::Path};

use crate::{git, gitlab};
use kdl::KdlDocument;
use miette::{miette, IntoDiagnostic, LabeledSpan, Result};

pub fn bootstrap(gitlab_client: &gitlab::Client, file: &str, project_dir: &str) -> Result<()> {
    let layout = load_layout(file)?;
    tracing::debug!("{layout:?}");

    for repo in layout.repos {
        let project = gitlab_client.project(&repo)?;

        if Path::new(&format!("{project_dir}/{}", project.path_with_namespace)).exists() {
            tracing::info!("project {} already exists", project.path_with_namespace);

            continue;
        }

        tracing::info!("cloning project {}", project.path_with_namespace);
        git::clone_project(&project, project_dir)?;
    }

    Ok(())
}

#[derive(Debug)]
struct Layout {
    pub repos: Vec<String>,
}

macro_rules! kdl_error {
    ( $node:expr, $source_code:expr, $message:expr ) => {
        Err(miette!(
            labels = vec![LabeledSpan::at(
                $node.span().offset()..=$node.span().offset() + $node.span().len(),
                $message,
            )],
            help = "Validate the config and consult the documentation",
            "Invalid configuration",
        )
        .with_source_code($source_code.to_owned()))
    };
}

fn load_layout(file: &str) -> Result<Layout> {
    let content = fs::read_to_string(file).into_diagnostic()?;
    let doc = KdlDocument::parse_v2(&content)?;

    let mut repos: Vec<String> = vec![];
    let repos_node = match doc.get("repositories") {
        Some(repos_node) => repos_node,
        None => {
            return Err(miette!(
                help = "Validate the layout and consult the documentation",
                "Layout does not contain 'repsitories' node",
            ))
        }
    };

    let repos_nodes = match repos_node.children() {
        Some(nodes) => nodes,
        None => kdl_error!(repos_node, content, "missing values")?,
    };

    for child in repos_nodes.nodes() {
        tracing::debug!("{child:?}");
        repos.push(child.name().value().to_owned());
    }

    Ok(Layout { repos })
}
