use std::borrow::Cow;

use ansi_term::Style;
use gitlab::{
    api::{self, groups, projects, Query},
    Gitlab,
};
use miette::{IntoDiagnostic, Result};
use serde::Deserialize;
use skim::SkimItem;

#[derive(Debug)]
pub struct Client {
    client: Gitlab,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Project {
    pub id: usize,
    pub name: String,
    pub path_with_namespace: String,
    pub description: Option<String>,
    pub ssh_url_to_repo: String,
}

impl SkimItem for Project {
    fn text(&self) -> Cow<str> {
        Cow::Owned(self.path_with_namespace.to_owned())
    }

    fn display<'a>(&'a self, _context: skim::DisplayContext<'a>) -> skim::AnsiString<'a> {
        skim::AnsiString::from(format!("{} {}", self.id, self.path_with_namespace,))
    }

    fn preview(&self, _context: skim::PreviewContext) -> skim::ItemPreview {
        let description = match &self.description {
            Some(desc) => Style::new().paint(desc),
            None => Style::new()
                .italic()
                .dimmed()
                .paint("no description available"),
        };

        skim::ItemPreview::AnsiText(format!(
            "{} {}\n{} {}\n\n{}",
            Style::new().bold().fg(ansi_term::Color::Blue).paint("id"),
            self.id,
            Style::new()
                .bold()
                .fg(ansi_term::Color::Blue)
                .paint("full path"),
            self.path_with_namespace,
            description,
        ))
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Group {
    pub id: usize,
    pub name: String,
    pub full_path: String,
}

impl Client {
    pub fn new(url: String, token: String) -> Result<Self> {
        let client = Gitlab::new(url, token).into_diagnostic()?;

        Ok(Self { client })
    }

    pub fn project(&self, path: &str) -> Result<Project> {
        let endpoint = projects::Project::builder()
            .project(path)
            .build()
            .into_diagnostic()?;

        endpoint.query(&self.client).into_diagnostic()
    }

    pub fn projects(&self) -> Result<Vec<Project>> {
        let pageable_endpoint = projects::Projects::builder().build().into_diagnostic()?;

        api::paged(pageable_endpoint, api::Pagination::All)
            .query(&self.client)
            .into_diagnostic()
    }

    #[tracing::instrument]
    pub fn projects_for_group(&self, group: &str) -> Result<Vec<Project>> {
        let mut groups = self.get_all_subgroups(group)?;
        groups.push(self.group(group)?);

        tracing::debug!("{groups:?}");

        Ok(groups
            .iter()
            .flat_map(|g| self.get_projects_for_group(g))
            .flatten()
            .collect::<Vec<Project>>())
    }

    pub fn group(&self, group: &str) -> Result<Group> {
        let endpoint = groups::Group::builder()
            .group(group)
            .build()
            .into_diagnostic()?;

        endpoint.query(&self.client).into_diagnostic()
    }

    fn get_projects_for_group(&self, group: &Group) -> Result<Vec<Project>> {
        let pageable_endpoint = groups::projects::GroupProjects::builder()
            .group(&group.full_path)
            .build()
            .into_diagnostic()?;

        api::paged(pageable_endpoint, api::Pagination::All)
            .query(&self.client)
            .into_diagnostic()
    }

    fn get_all_subgroups(&self, group: &str) -> Result<Vec<Group>> {
        let pageable_endpoint = groups::subgroups::GroupSubgroups::builder()
            .group(group)
            .build()
            .into_diagnostic()?;

        let subgroups: Vec<Group> = api::paged(pageable_endpoint, api::Pagination::All)
            .query(&self.client)
            .into_diagnostic()?;

        let mut output = subgroups.clone();

        for sg in subgroups {
            let mut groups = self.get_all_subgroups(&sg.full_path)?;
            output.append(&mut groups);
        }

        Ok(output)
    }
}
