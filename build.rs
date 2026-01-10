use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let objects = reqwest::blocking::get(
        "https://cloud.timeedit.net/liu/web/schema/objects//o.json?fr=f&types=205&sid=3",
    )?;
    Ok(())
}
