use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct SearchResponse
{
    pub data: Vec<StationAQI>
}

#[derive(Deserialize, Debug)]
pub struct InfoResponse
{
    pub data: StationInfo
}

#[derive(Deserialize, Debug)]
pub struct StationInfo
{
    pub aqi: u32,
    pub city: City,
    pub attributions: serde_json::Value,
    pub forecast: serde_json::Value,
    pub iaqi: serde_json::Value
}

#[derive(Deserialize, Debug)]
pub struct City {
    pub name: String
}

#[derive(Deserialize, Debug)]
pub struct StationAQI
{
    pub aqi: String, // the returned AQI is a String not an integer, so a new string will be constructed and owned?
    pub station: Station
}

#[derive(Deserialize, Debug)]
pub struct Station
{
    pub name: String,
    pub url: String,
}