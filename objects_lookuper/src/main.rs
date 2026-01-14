use color_eyre::Result;
use std::fs::File;
use std::io::Write;
use types::{ObjectRecord, ObjectSearchResponse, ObjectType};
use wincode::serialize;

const MAX: u16 = 100;

async fn fetch_courses(client: &reqwest::Client) -> Result<Vec<ObjectRecord>> {
    let mut all_objects = Vec::new();
    let mut start = 0;
    let object_type_string = (ObjectType::Course as i32).to_string();

    println!("Fetching courses...");

    loop {
        let resp = client
            .get("https://cloud.timeedit.net/liu/web/schema/objects.json")
            .query(&[
                ("fe", "f"),
                ("fe", "132.0"),
                ("fe", "115.20252,20261"),
                ("sid", "3"),
                ("types", &object_type_string),
                ("max", &MAX.to_string()),
                ("start", &start.to_string()),
            ])
            .send()
            .await?;

        let mut data: ObjectSearchResponse = resp.json().await?;
        if data.count == 0 {
            break;
        }

        all_objects.append(&mut data.records);
        start += MAX;
    }

    Ok(all_objects)
}

async fn fetch_student_groups(client: &reqwest::Client) -> Result<Vec<ObjectRecord>> {
    let mut all_objects = Vec::new();
    let mut start = 0;
    let object_type_string = (ObjectType::StudentGroup as i32).to_string();

    println!("Fetching student groups...");

    loop {
        let resp = client
            .get("https://cloud.timeedit.net/liu/web/schema/objects/o.json")
            .query(&[
                ("types", object_type_string.as_str()),
                ("sid", "3"),
                ("max", &MAX.to_string()),
                ("start", &start.to_string()),
            ])
            .send()
            .await?;

        let mut data: ObjectSearchResponse = resp.json().await?;
        if data.count == 0 {
            break;
        }

        all_objects.append(&mut data.records);
        start += MAX;
    }

    Ok(all_objects)
}

async fn fetch_rooms(client: &reqwest::Client) -> Result<Vec<ObjectRecord>> {
    let mut all_objects = Vec::new();
    let mut start = 0;
    let object_type_string = (ObjectType::Room as i32).to_string();

    println!("Fetching rooms...");

    loop {
        let resp = client
            .get("https://cloud.timeedit.net/liu/web/schema/objects/o.json")
            .query(&[
                ("types", object_type_string.as_str()),
                ("sid", "3"),
                ("max", &MAX.to_string()),
                ("start", &start.to_string()),
            ])
            .send()
            .await?;

        let mut data: ObjectSearchResponse = resp.json().await?;
        if data.count == 0 {
            break;
        }

        all_objects.append(&mut data.records);
        start += MAX;
    }

    Ok(all_objects)
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let client = reqwest::Client::new();

    // Fetch all object types concurrently
    let (courses_res, groups_res, rooms_res) = tokio::join!(
        fetch_courses(&client),
        fetch_student_groups(&client),
        fetch_rooms(&client)
    );

    // Combine results, propagating errors
    let mut all_objects = Vec::new();
    all_objects.extend(courses_res?);
    all_objects.extend(groups_res?);
    all_objects.extend(rooms_res?);

    let mut file = File::create("../web/assets/records.bin")?;
    file.write_all(&serialize(&all_objects)?)?;

    println!(
        "All records fetched and saved. Total: {}",
        all_objects.len()
    );
    Ok(())
}
