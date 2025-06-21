mod text_reader;
mod screen_maker;

use arboard::Clipboard;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = screen_maker::make()?;
    let read_result = text_reader::read(path.as_str())?;

    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(read_result)?;

    Ok(())
}
