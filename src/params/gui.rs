#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::CmdParameters;

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Hash, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
/// Allows to build parameters for flameshot gui capture
pub struct GuiArgs {
    path: Option<String>,
    clipboard: bool,
    delay: Option<usize>,
    region: Option<(usize, usize, usize, usize)>,
    last_region: bool,
    raw: bool,
    print_geometry: bool,
    upload: bool,
    pin: bool,
    accept_on_select: bool,
    pub args: Vec<String>,
}

impl GuiArgs {
    pub fn builder() -> GuiArgsBuilder {
        GuiArgsBuilder::default()
    }
}

impl CmdParameters for GuiArgs {
    fn generate_args(&self) -> Vec<String> {
        let mut args = vec![String::from("gui")];
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

        if self.last_region {
            args.push(String::from("--last-region"));
        }

        if self.raw {
            args.push(String::from("--raw"));
        };

        if self.print_geometry {
            args.push(String::from("--print-geometry"));
        }

        if self.upload {
            args.push(String::from("--upload"));
        };

        if self.pin {
            args.push(String::from("--pin"));
        }

        if self.accept_on_select {
            args.push(String::from("--accept-on-select"))
        }

        args
    }
}
pub struct GuiArgsBuilder {
    path: Option<String>,
    clipboard: bool,
    delay: Option<usize>,
    region: Option<(usize, usize, usize, usize)>,
    last_region: bool,
    raw: bool,
    print_geometry: bool,
    upload: bool,
    pin: bool,
    accept_on_select: bool,
    pub args: Vec<String>,
}

impl Default for GuiArgsBuilder {
    fn default() -> Self {
        Self {
            path: None,
            clipboard: false,
            delay: None,
            region: None,
            last_region: false,
            raw: false,
            print_geometry: false,
            upload: false,
            pin: false,
            accept_on_select: false,
            args: vec![],
        }
    }
}

impl GuiArgsBuilder {
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

    pub fn last_region(mut self) -> Self {
        self.last_region = true;
        self
    }

    pub fn raw(mut self) -> Self {
        self.raw = true;
        self
    }

    pub fn print_geometry(mut self) -> Self {
        self.print_geometry = true;
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

    pub fn accept_on_select(mut self) -> Self {
        self.accept_on_select = true;
        self
    }

    pub fn build(self) -> GuiArgs {
        let mut cmd = GuiArgs {
            path: self.path,
            clipboard: self.clipboard,
            delay: self.delay,
            region: self.region,
            last_region: self.last_region,
            raw: self.raw,
            print_geometry: self.print_geometry,
            upload: self.upload,
            pin: self.pin,
            accept_on_select: self.accept_on_select,
            args: self.args,
        };
        cmd.args = cmd.generate_args();

        cmd
    }
}
