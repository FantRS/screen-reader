use std::io::{Error, ErrorKind};
use std::process::Command;

pub fn make() -> Result<String, Error> {
    let screenshot_path = "/tmp/screenshot_clipboard.png";

    let status = Command::new("maim")
        .arg("-s")
        .arg(screenshot_path)
        .status()?;

    if !status.success() {
        let error = "maim exited with an error";
        return Err(Error::new(ErrorKind::InvalidData, error));
    }

    Ok(screenshot_path.to_string())
}
