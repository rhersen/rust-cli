use reqwest::header::{HeaderMap, CONTENT_TYPE};
use reqwest::Error;
use train_announcement::TrainAnnouncement;
use types::Root;

mod train_announcement;
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
                    <GT name='AdvertisedTimeAtLocation' value='2024-02-09T07:00:04.137Z' />
                    <LT name='AdvertisedTimeAtLocation' value='2024-02-09T07:59:04.137Z' />
                </AND>
            </FILTER>
            <INCLUDE>ActivityType</INCLUDE>
            <INCLUDE>AdvertisedTimeAtLocation</INCLUDE>
            <INCLUDE>AdvertisedTrainIdent</INCLUDE>
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
                    "{}\t{}\t{}\t{} {}",
                    announcement.train_ident(),
                    announcement.to_location(),
                    announcement.activity_type(),
                    announcement.advertised_time(),
                    announcement.time_at_location()
                )
            }
        }
        Err(e) => {
            eprintln!("error: {}", e)
        }
    }
}
