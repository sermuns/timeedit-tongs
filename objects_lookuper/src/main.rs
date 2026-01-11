use color_eyre::Result;
use std::fs::File;
use std::io::{Write, stdout};

use types::{ObjectSearchResponse, ObjectType};

fn main() -> Result<()> {
    let client = reqwest::blocking::Client::new();
    const MAX: u16 = 100;

    let mut all_object_records = Vec::new();
    let mut stdout_lock = stdout().lock();

    println!("doing {:?}", ObjectType::Course);
    {
        let mut start = 0;
        let object_type_string = (ObjectType::Course as i32).to_string();
        loop {
            let raw_response = client
                .get("https://cloud.timeedit.net/liu/web/schema/objects.json")
                .query(&[
                    ("fe", "f"),
                    ("fe", "132.0"),
                    ("fe", "115.20252,20261"),
                    ("sid", "3"),
                    ("types", object_type_string.as_str()),
                    ("max", &MAX.to_string()),
                    ("start", &start.to_string()),
                ])
                .send()?;

            writeln!(stdout_lock, "fetching {}", raw_response.url())?;

            let mut response: ObjectSearchResponse = raw_response.json()?;

            if response.count == 0 {
                break;
            }

            all_object_records.append(&mut response.records);

            writeln!(stdout_lock, "fetched {}-{}", start, start + response.count)?;
            start += MAX;
        }
    }

    {
        let mut start = 0;
        let object_type_string = (ObjectType::StudentGroup as i32).to_string();
        loop {
            let raw_response = client
                .get("https://cloud.timeedit.net/liu/web/schema/objects/o.json")
                .query(&[
                    ("types", object_type_string.as_str()),
                    ("sid", "3"),
                    ("max", &MAX.to_string()),
                    ("start", &start.to_string()),
                ])
                .send()?;

            writeln!(stdout_lock, "fetching {}", raw_response.url())?;

            let mut response: ObjectSearchResponse = raw_response.json()?;

            if response.count == 0 {
                break;
            }

            all_object_records.append(&mut response.records);

            writeln!(stdout_lock, "fetched {}-{}", start, start + response.count)?;
            start += MAX;
        }
    }

    let mut file = File::create("../web/assets/records.bin")?;
    file.write_all(&wincode::serialize(&all_object_records)?)?;

    Ok(())
}
