use crate::types::TrainAnnouncement;
use reqwest::header::{HeaderMap, CONTENT_TYPE};
use reqwest::Error;
use serde::Deserialize;
use types::Root;

mod types;

async fn post_xml_data() -> Result<Vec<TrainAnnouncement>, Error> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/xml".parse().unwrap());

    let api_key = std::env::var("TRAFIKVERKET_API_KEY").unwrap_or_default();

    let xml_data = format!(
        r#"
    <REQUEST>
        <LOGIN authenticationkey='{}' />
        <QUERY objecttype='TrainAnnouncement' schemaversion='1.6'>
            <FILTER>
                <AND>
                    <EQ name='LocationSignature' value='Tul' />
                    <GT name='AdvertisedTimeAtLocation' value='2024-01-12T11:00:04.137Z' />
                    <LT name='AdvertisedTimeAtLocation' value='2024-01-12T11:59:04.137Z' />
                </AND>
            </FILTER>
            <INCLUDE>ActivityType</INCLUDE>
            <INCLUDE>AdvertisedTimeAtLocation</INCLUDE>
            <INCLUDE>FromLocation</INCLUDE>
            <INCLUDE>TimeAtLocationWithSeconds</INCLUDE>
            <INCLUDE>ToLocation</INCLUDE>
        </QUERY>
    </REQUEST>
"#,
        api_key
    );
    let res = client
        .post("https://api.trafikinfo.trafikverket.se/v2/data.json")
        .headers(headers)
        .body(xml_data)
        .send()
        .await?;

    println!("Status: {}", res.status());

    let data: Root = res.json().await?;

    Ok(data.RESPONSE.RESULT[0].TrainAnnouncement.clone())
}

#[tokio::main]
async fn main() {
    match post_xml_data().await {
        Ok(announcements) => {
            for announcement in announcements {
                println!(
                    "{}\t{}\t{} {}",
                    announcement.ToLocation[0].LocationName,
                    announcement.ActivityType,
                    &announcement.AdvertisedTimeAtLocation[11..16],
                    match &announcement.TimeAtLocationWithSeconds {
                        None => "-",
                        Some(s) => &s[11..19],
                    }
                )
            }
        }
        Err(e) => {
            eprintln!("error: {}", e)
        }
    }
}
