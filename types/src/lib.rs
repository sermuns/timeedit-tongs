use chrono::{DateTime, Locale, NaiveDateTime, Utc};
use chrono::{NaiveDate, NaiveTime, TimeZone};
use chrono_tz::Tz;
use serde::Deserialize;
use serde_repr::Deserialize_repr;
use wincode::{SchemaRead, SchemaWrite};

#[derive(Deserialize, Debug)]
pub struct ObjectSearchResponse {
    pub count: u16,
    pub records: Vec<ObjectRecord>,
}

#[derive(Deserialize, Debug, SchemaWrite, SchemaRead, Clone, PartialEq)]
pub struct ObjectRecord {
    pub id: u32,
    pub values: String,
    #[serde(rename = "typeId")]
    pub r#type: ObjectType,
}

#[derive(Deserialize_repr, Debug, SchemaWrite, SchemaRead, Clone, PartialEq)]
#[repr(i32)]
pub enum ObjectType {
    Room = 195,
    StudentGroup = 205,
    Course = 219,
}

#[derive(Debug, Deserialize)]
pub struct CalendarResponse {
    pub reservations: Vec<Reservation>,
}

#[derive(Debug, Deserialize)]
pub struct Reservation {
    pub id: String,
    pub startdate: NaiveDate,
    pub starttime: NaiveTime,
    pub enddate: NaiveDate,
    pub endtime: NaiveTime,
    pub columns: [String; 9],
}

impl Reservation {
    // TODO: https://crates.io/crates/chrono-tz#user-content-limiting-the-timezone-table-to-zones-of-interest

    // NOTE: hardcoded Stockholm timezone because i think TimeEdit API is in that??
    const TIME_ZONE: Tz = chrono_tz::Europe::Stockholm;
    const LOCALE: Locale = chrono::Locale::sv_SE;

    pub fn start_utc(&self) -> DateTime<Utc> {
        let naive = NaiveDateTime::new(self.startdate, self.starttime);

        Self::TIME_ZONE
            .from_local_datetime(&naive)
            .unwrap()
            .with_timezone(&Utc)
    }

    pub fn end_utc(&self) -> DateTime<Utc> {
        let naive = NaiveDateTime::new(self.enddate, self.endtime);

        Self::TIME_ZONE
            .from_local_datetime(&naive)
            .unwrap()
            .with_timezone(&Utc)
    }

    pub fn link(&self) -> String {
        format!(
            "https://cloud.timeedit.net/liu/web/schema/ri.html?sid=3&id={}",
            self.id
        )
    }

    pub fn start_localized_format(&self) -> String {
        format!(
            "{} | {}",
            self.startdate.format_localized("%a %d %b %Y", Self::LOCALE),
            self.starttime.format("kl %H:%M")
        )
    }
}
