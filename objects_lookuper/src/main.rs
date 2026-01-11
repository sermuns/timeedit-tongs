use color_eyre::Result;
use serde::Deserialize;
use std::fs::File;
use std::io::Write;
use wincode::{SchemaRead, SchemaWrite};

#[derive(Deserialize, Debug, SchemaWrite, SchemaRead)]
struct ObjectRecord {
    id: u32,
    values: String,
}

#[derive(Deserialize, Debug)]
struct ObjectSearchResponse {
    count: u16,
    records: Vec<ObjectRecord>,
}

fn main() -> Result<()> {
    let client = reqwest::blocking::Client::new();

    const MAX: u16 = 100;
    let mut start = 0;

    let mut all_object_records = Vec::new();

    loop {
        let mut response: ObjectSearchResponse = client
            .get("https://cloud.timeedit.net/liu/web/schema/objects/o.json")
            .query(&[
                ("types", "205"),
                ("sid", "3"),
                ("max", &MAX.to_string()),
                ("start", &start.to_string()),
            ])
            .send()?
            .json()?;

        all_object_records.append(&mut response.records);

        println!("fetched {}-{}", start, start + response.count);
        if response.count < MAX {
            break;
        }
        start += MAX;
    }

    let mut file = File::create("records.bin")?;
    file.write_all(&wincode::serialize(&all_object_records)?)?;

    Ok(())
}
