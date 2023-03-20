#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::CmdParameters;

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Hash, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
/// Allows to build parameters for flameshot screen capture
pub struct ScreenArgs {
    number: usize,
    clipboard: bool,
    path: Option<String>,
    delay: Option<usize>,
    region: Option<(usize, usize, usize, usize)>,
    raw: bool,
    upload: bool,
    pin: bool,
    pub args: Vec<String>,
}

impl ScreenArgs {
    pub fn builder() -> ScreenArgsBuilder {
        ScreenArgsBuilder::default()
    }
}

impl CmdParameters for ScreenArgs {
    fn generate_args(&self) -> Vec<String> {
        let mut args = vec![String::from("screen")];

        if self.number != 0 {
            args.push(format!("--number={}", self.number));
        }

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

#[derive(Default)]
pub struct ScreenArgsBuilder {
    number: usize,
    path: Option<String>,
    clipboard: bool,
    delay: Option<usize>,
    region: Option<(usize, usize, usize, usize)>,
    raw: bool,
    upload: bool,
    pin: bool,
    args: Vec<String>,
}

impl ScreenArgsBuilder {
    pub fn number(mut self, number: usize) -> Self {
        self.number = number;
        self
    }

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

    pub fn pin(mut self) -> Self {
        self.pin = true;
        self
    }

    pub fn build(self) -> ScreenArgs {
        let mut cmd = ScreenArgs {
            number: self.number,
            path: self.path,
            clipboard: self.clipboard,
            delay: self.delay,
            region: self.region,
            raw: self.raw,
            upload: self.upload,
            pin: self.pin,
            args: self.args,
        };
        cmd.args = cmd.generate_args();

        cmd
    }
}
