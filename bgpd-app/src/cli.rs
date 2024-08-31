use anyhow::{anyhow, Error};
use clap::Parser;
use duration_string::DurationString;
use std::path::Path;
use std::time::Duration;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    #[arg(long,short)]
    config_file: Option<String>,

    #[arg(long,short, default_value = "5m")]
    poll_interval: DurationString,
}

impl Cli {
    pub(crate) fn poll_interval(&self) -> Duration {
        self.poll_interval.into()
    }

    pub(crate) fn config_file(&self) -> Option<Result<&Path, Error>> {
        match &self.config_file {
            Some(path_name) => {
                let path = Path::new(path_name);

                Some(match path.exists() && path.is_file(){
                    true => Ok(path),
                    false => Err(anyhow!("Config file {} does not exists", path_name)),
                })
            },
            None => None
        }
    }
}

#[cfg(test)]
mod test {
    use crate::cli::Cli;
    use clap::Parser;
    use std::fs::File;
    use std::io::Write;
    use std::time::Duration;
    use tempdir::TempDir;

    #[test]
    fn should_parse_default_arguments() {
        let args = Vec::<String>::new();

        match Cli::try_parse_from(args.iter()) {
            Err(err) => panic!("Cannot parse command line args {err}"),
            Ok(cli) => {
                assert_eq!(cli.config_file().is_none(), true);
                assert_eq!(cli.poll_interval(), Duration::from_secs(300));
            }
        }
    }
    #[test]
    fn should_parse_with_unkonwn_file() {
        let mut args = Vec::<String>::new();

        args.push("bgpd".to_string());
        args.push("--config-file".to_string());
        args.push("/foo/bar".to_string());

        match Cli::try_parse_from(args.iter()) {
            Err(err) => panic!("Cannot parse command line args {err}"),
            Ok(cli) => {
                assert_eq!(cli.config_file.is_some(), true);
                assert_eq!(cli.config_file.clone().unwrap(), "/foo/bar".to_string());
                assert_eq!(cli.config_file().unwrap().is_err(), true);
                assert_eq!(cli.poll_interval(), Duration::from_secs(300));
            }
        }
    }

    #[test]
    fn should_parse_with_known_file() {
        let tmp_dir = TempDir::new("bgpd_test").unwrap();
        let file_path = tmp_dir.path().join("config_file");
        let mut tmp_file = File::create(&file_path).unwrap();

        writeln!(tmp_file, "foo bar").expect("Cannot write temp file");

        let mut args = Vec::<String>::new();

        let tmp_file_path = file_path.to_str().unwrap().to_string();

        args.push("bgpd".to_string());
        args.push("--config-file".to_string());
        args.push(tmp_file_path.clone());

        match Cli::try_parse_from(args.iter()) {
            Err(err) => panic!("Cannot parse command line args {err}"),
            Ok(cli) => {
                assert_eq!(cli.config_file.is_some(), true);
                assert_eq!(cli.config_file.clone().unwrap(), tmp_file_path);
                assert_eq!(cli.config_file().unwrap().is_ok(), true);
                assert_eq!(cli.poll_interval(), Duration::from_secs(300));
            }
        }

        drop(tmp_file);
        tmp_dir.close().unwrap();
    }

    #[test]
    fn should_parse_with_poll_interval() {
        let mut args = Vec::<String>::new();

        args.push("bgpd".to_string());
        args.push("--poll-interval".to_string());
        args.push("10m".to_string());

        match Cli::try_parse_from(args.iter()) {
            Err(err) => panic!("Cannot parse command line args {err}"),
            Ok(cli) => {
                assert_eq!(cli.config_file().is_none(), true);
                assert_eq!(cli.poll_interval(), Duration::from_secs(600));
            }
        }
    }

}