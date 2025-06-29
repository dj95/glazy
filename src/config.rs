use std::fs;

use kdl::{KdlDocument, KdlNode};
use miette::{bail, miette, IntoDiagnostic, LabeledSpan, Result};
use xdg::BaseDirectories;

macro_rules! kdl_first_entry_as_string {
    ( $node:expr ) => {
        $node
            .entries()
            .iter()
            .next()
            .and_then(|s| s.value().as_string())
    };
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

#[derive(Debug)]
pub struct Config {
    pub gitlab: GitLabConfig,
    pub local: LocalConfig,
}

#[derive(Debug)]
pub struct GitLabConfig {
    pub host: String,
    pub token: String,
}

#[derive(Debug)]
pub struct LocalConfig {
    pub project_dir: String,
}

#[tracing::instrument]
pub fn config_file_path(file_name_candidate: Option<String>) -> Result<String> {
    match file_name_candidate {
        Some(file_name) => Ok(file_name),
        None => {
            let config_file = match BaseDirectories::with_prefix("glazy").get_config_file("config.kdl") {
                Some(config_file) => config_file,
                None => bail!("cannot get config file path"),
            };

            Ok(config_file
                .into_os_string()
                .into_string()
                .unwrap())
        }
    }
}

pub fn read_config(file_name: Option<String>) -> Result<Config> {
    let file_name = config_file_path(file_name)?;
    tracing::debug!(target: "detected config file", config_file = file_name);

    let contents = fs::read_to_string(&file_name).map_err(|err| {
        miette!(
            code = "config::read_config",
            help = "Validate the config path and check that the file exists.",
            "Config file: {file_name}\n{err}"
        )
    })?;

    let nodes = contents.parse::<KdlDocument>().into_diagnostic()?;

    Ok(Config {
        gitlab: parse_gitlab_config(&file_name, nodes.get("gitlab"))?,
        local: parse_local_config(&file_name, nodes.get("local"))?,
    })
}

#[tracing::instrument]
fn parse_local_config(file_name: &str, node: Option<&KdlNode>) -> Result<LocalConfig> {
    let node = match node {
        Some(node) => node,
        None => {
            return Err(miette!(
                help = "Validate the config and consult the documentation",
                "Configuration does not contain 'local' node",
            ))?
        }
    };

    let source_code = node.to_string();

    let node = match node.children() {
        Some(node) => node,
        None => kdl_error!(node, source_code, "local node is empty")?,
    };

    Ok(LocalConfig {
        project_dir: get_value_or_error(node, source_code.to_owned(), "project_dir")?,
    })
}

#[tracing::instrument]
fn parse_gitlab_config(file_name: &str, node: Option<&KdlNode>) -> Result<GitLabConfig> {
    let node = match node {
        Some(node) => node,
        None => {
            return Err(miette!(
                help = "Validate the config and consult the documentation",
                "Configuration does not contain 'gitlab' node",
            ))?
        }
    };

    let source_code = node.to_string();

    let node = match node.children() {
        Some(node) => node,
        None => kdl_error!(node, source_code, "gitlab node is empty")?,
    };

    Ok(GitLabConfig {
        host: get_value_or_error(node, source_code.to_owned(), "host")?,
        token: get_value_or_error(node, source_code, "token")?,
    })
}

fn get_value_or_error(node: &KdlDocument, source_code: String, name: &str) -> Result<String> {
    match node.get(name) {
        Some(node) => match kdl_first_entry_as_string!(node) {
            Some(value) => Ok(value.to_owned()),
            None => kdl_error!(node, source_code, "missing value")?,
        },
        None => kdl_error!(node, source_code, "missing 'token'")?,
    }
}

#[cfg(test)]
mod test {
    use crate::config::*;
    use rstest::*;

    const EXAMPLE_CONFIG: &str = "gitlab {
    host \"host_val\"
    token \"token_val\"
}";

    #[rstest]
    #[case(EXAMPLE_CONFIG, false)]
    #[case(
        "gitlab {
    host \"host_val\"
}",
        true
    )]
    #[case(
        "gitlab {
    token \"token_val\"
}",
        true
    )]
    #[case("", true)]
    #[test_log::test]
    fn test_parse_gitlab(#[case] input: &str, #[case] expected_err: bool) {
        let doc = input.parse::<KdlDocument>().unwrap();

        let result = parse_gitlab_config("file_name", doc.get("gitlab"));

        tracing::debug!("{result:?}");
        assert_eq!(result.is_err(), expected_err);
    }

    #[rstest]
    #[case(Some("foo.yaml".to_owned()), "foo.yaml")]
    #[case(None, "$HOME/.config/glazy/config.kdl")]
    #[test_log::test]
    fn test_config_file_path(#[case] input: Option<String>, #[case] expected: String) {
        let result = config_file_path(input);

        let expected = match expected.contains("$HOME") {
            true => expected.replace("$HOME", env!("HOME")),
            false => expected,
        };

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }
}
