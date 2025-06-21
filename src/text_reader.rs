use rusty_tesseract::{Args, Image, TessError, image_to_string};
use std::collections::HashMap;

pub fn read(path: &str) -> Result<String, TessError> {
    let image = Image::from_path(path)?;

    let default_args = Args {
        lang: "eng+ukr+rus".to_string(),
        config_variables: HashMap::new(),
        dpi: Some(800),
        psm: Some(6),
        oem: Some(1),
    };

    let result = image_to_string(&image, &default_args)?;

    if result.trim().is_empty() {
        let error = String::from("OOCR did not recognize the text!");
        return Err(TessError::ParseError(error));
    }

    Ok(result)
}
