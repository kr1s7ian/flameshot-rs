#[cfg(feature = "image")]
use image::{io::Reader as ImageReader, DynamicImage};
#[cfg(feature = "image")]
use std::io::Cursor;

use crate::errors::FlameshotError;
use std::process::Command;
use std::process::Output;

use std::str::from_utf8;
pub mod errors;
pub mod params;

pub trait CmdParameters {
    fn generate_args(&self) -> Vec<String>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FlameshotOutput {
    pub output: Output,
    raw_enabled: bool,
}

#[cfg(feature = "image")]
impl FlameshotOutput {
    pub fn to_dynamic_image(self) -> Result<DynamicImage, FlameshotError> {
        if self.raw_enabled == false {
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

pub fn execute(params: impl CmdParameters) -> Result<FlameshotOutput, FlameshotError> {
    let args = params.generate_args();
    let raw_enabled = args.contains(&String::from("--raw"));
    let command = Command::new("flameshot").args(args).output();

    let output = match command {
        Ok(output) => output,
        Err(e) => return Err(FlameshotError::Os(e.to_string())),
    };

    match output.stderr.len() > 4 {
        true => {
            let error_message = from_utf8(&output.stderr)
                .unwrap_or("Error message is corrupted!")
                .to_string();

            Err(FlameshotError::Flameshot(error_message))
        }
        false => Ok(FlameshotOutput {
            output,
            raw_enabled,
        }),
    }
}
