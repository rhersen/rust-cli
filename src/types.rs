use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Location {
    pub LocationName: String,
    pub Priority: i32,
    pub Order: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TrainAnnouncement {
    pub ActivityType: String,
    pub AdvertisedTimeAtLocation: String,
    pub TimeAtLocationWithSeconds: Option<String>,
    pub FromLocation: Vec<Location>,
    pub ToLocation: Vec<Location>,
}

#[derive(Deserialize, Debug)]
pub struct Result {
    pub TrainAnnouncement: Vec<TrainAnnouncement>,
}

#[derive(Deserialize, Debug)]
pub struct Response {
    pub RESULT: Vec<Result>,
}

#[derive(Deserialize, Debug)]
pub struct Root {
    pub RESPONSE: Response,
}
