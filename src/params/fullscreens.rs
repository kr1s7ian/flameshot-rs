use serde::{Deserialize, Serialize};
use std::process::Command;
use std::{fmt::format, process::Output};

use crate::CmdParameters;

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Hash, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FullscreensArgs {
    path: Option<String>,
    clipboard: bool,
    delay: Option<usize>,
    region: Option<(usize, usize, usize, usize)>,
    raw: bool,
    upload: bool,
    pub args: Vec<String>,
}

impl FullscreensArgs {
    pub fn builder() -> FullscreensArgsBuilder {
        FullscreensArgsBuilder::default()
    }
}

impl CmdParameters for FullscreensArgs {
    fn generate_args(&self) -> Vec<String> {
        let mut args = vec![String::from("full")];
        if let Some(path) = self.path.to_owned() {
            args.push(format!("--path={path}"));
        };

        if self.clipboard {
            args.push(String::from("--clipboard"));
        };

        if let Some(delay) = self.delay {
            args.push(format!("--delay={delay}"));
        };

        if let Some(region) = self.region {
            args.push(format!(
                "--region={},{},{},{}",
                region.0, region.1, region.2, region.3,
            ));
        };

        if self.raw {
            args.push(String::from("--raw"));
        };
        if self.upload {
            args.push(String::from("--upload"));
        };

        args
    }
}
pub struct FullscreensArgsBuilder {
    path: Option<String>,
    clipboard: bool,
    delay: Option<usize>,
    region: Option<(usize, usize, usize, usize)>,
    raw: bool,
    upload: bool,
    args: Vec<String>,
}

impl Default for FullscreensArgsBuilder {
    fn default() -> Self {
        Self {
            path: None,
            clipboard: false,
            delay: None,
            region: None,
            raw: false,
            upload: false,
            args: vec![],
        }
    }
}

impl FullscreensArgsBuilder {
    pub fn path(mut self, path: &str) -> Self {
        self.path = Some(path.to_string());
        self
    }

    pub fn clipboard(mut self) -> Self {
        self.clipboard = true;
        self
    }

    pub fn delay(mut self, delay: usize) -> Self {
        self.delay = Some(delay);
        self
    }

    pub fn region(mut self, region: (usize, usize, usize, usize)) -> Self {
        self.region = Some(region);
        self
    }

    pub fn raw(mut self) -> Self {
        self.raw = true;
        self
    }

    pub fn upload(mut self) -> Self {
        self.upload = true;
        self
    }

    pub fn build(self) -> FullscreensArgs {
        let mut cmd = FullscreensArgs {
            path: self.path,
            clipboard: self.clipboard,
            delay: self.delay,
            region: self.region,
            raw: self.raw,
            upload: self.upload,
            args: self.args,
        };
        cmd.args = cmd.generate_args();

        cmd
    }
}
