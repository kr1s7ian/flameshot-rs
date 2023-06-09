#[cfg(feature = "image")]
use image::{io::Reader as ImageReader, DynamicImage};
#[cfg(feature = "image")]
use std::io::Cursor;

use std::process::Command;
use std::process::Output;
use std::str::from_utf8;

pub use crate::flameshot_error::FlameshotError;
pub use crate::params::FullArgs;
pub use crate::params::GuiArgs;
pub use crate::params::ScreenArgs;
pub mod flameshot_error;
pub mod params;

/// Implements Cli parameters for flameshot.
pub trait CmdParameters {
    fn generate_args(&self) -> Vec<String>;
}

/// Gets returned from flameshot::execute(params)
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FlameshotOutput {
    pub output: Output,
    raw_enabled: bool,
}

/// Checks if flameshot stderr has produced an error
fn has_error(stderr: &str) -> bool {
    stderr.contains("error")
}

#[cfg(feature = "image")]
impl FlameshotOutput {
    /// Consumes self and returns a DynamicImage from the flameshot stdout, requires .raw() in params
    pub fn to_dynamic_image(self) -> Result<DynamicImage, FlameshotError> {
        if !self.raw_enabled {
            return Err(FlameshotError::Image(
                "You forgot to add .raw() to the param builder! which is a requirement for converting to a dynamic_image.".to_string(),
            ));
        }

        let buffer = Cursor::new(self.output.stdout);
        let reader = ImageReader::new(buffer).with_guessed_format();

        let img = match reader {
            Err(e) => return Err(FlameshotError::Image(e.to_string())),
            Ok(img) => img,
        };

        match img.decode() {
            Err(e) => Err(FlameshotError::Image(e.to_string())),
            Ok(img) => Ok(img),
        }
    }
}

/// Executes a flameshot cli command with the specified params
pub fn execute(params: impl CmdParameters) -> Result<FlameshotOutput, FlameshotError> {
    let args = params.generate_args();
    let raw_enabled = args.contains(&String::from("--raw"));
    let output = Command::new("flameshot")
        .args(args)
        .output()
        .map_err(|e| FlameshotError::Os(e.to_string()))?;

    let stderr = from_utf8(&output.stderr).unwrap_or("").to_string();

    match has_error(&stderr) {
        true => Err(FlameshotError::Os(stderr)),
        false => Ok(FlameshotOutput {
            output,
            raw_enabled,
        }),
    }
}
