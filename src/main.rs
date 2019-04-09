use std::io::Write;

use git2::Repository;
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

fn main() -> std::io::Result<()> {
    let bufwtr = BufferWriter::stdout(ColorChoice::Always);
    let mut buffer = bufwtr.buffer();

    buffer.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
    write!(&mut buffer, "green text!")?;

    buffer.set_color(ColorSpec::new().set_fg(None))?;
    bufwtr.print(&buffer)?;
    // note that this cannot be used directly, it must be stored where
    // bash can know how to count width.

    match Repository::open(".") {
        Ok(repo) => println!("got repo"),
        Err(e) => println!("got err: {}", e),
    };

    Ok(())
}
