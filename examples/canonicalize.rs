use std::error;
use std::io;

// Example: convert YAML to canonical form.
fn main() -> Result<(), Box<dyn error::Error>> {
    // Parser with default configuration.
    let parser = libyaml::Parser::new(io::stdin())?;

    // Emitter with custom configuration: enable canonical output.
    let mut emitter = libyaml::EmitterBuilder::new(io::stdout())?
        .canonical(true)
        .finish();

    // Parser implements IntoIterator and thus can be used in a for loop.
    for event in parser {
        emitter.emit(event?)?;
    }

    Ok(())
}
