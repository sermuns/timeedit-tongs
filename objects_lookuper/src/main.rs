use color_eyre::Result;
use std::fs::File;
use std::io::{Write, stdout};

use types::{ObjectSearchResponse, ObjectType};

fn main() -> Result<()> {
    let client = reqwest::blocking::Client::new();
    const MAX: u16 = 100;

    let mut all_object_records = Vec::new();
    let mut stdout_lock = stdout().lock();
    for object_type in [ObjectType::StudentGroup, ObjectType::Course] {
        println!("doing {:?}", &object_type);
        let object_type_string = (object_type as i32).to_string();
        let mut start = 0;
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
