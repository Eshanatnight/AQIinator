#![allow(non_snake_case)]
mod token;
mod opt;
mod response;

use token::AQI_TOKEN;   // AQI_TOKEN variable stores the API token which is defined in the token module.
use color_eyre::{eyre, Result};
use structopt::StructOpt;
use opt::{Aqi, Opt};
use response::{SearchResponse, InfoResponse};
use inflector::Inflector;


fn print_search_response(response: &SearchResponse)
{
    for r in &response.data
    {
        println!("Place: {:2}\nAQI: {}\nURL: {}\n\n", r.station.name, r.aqi, r.station.url);
    }
}

#[tokio::main]
async fn main() -> Result<()>
{
    color_eyre::install()?;
    let args = Aqi::from_args();
    let client = reqwest::Client::new();

    match args.command
    {
        Opt::Info {url}=>
        {
            // the response will be in a JSON format automaticall
            let response = client
                        .get(format!("https://api.waqi.info/feed/{}/",
                            url
                            .trim_end_matches("/")
                            .trim_start_matches("/")
                            .to_kebab_case())
                        )
                        .query(&[("token", AQI_TOKEN)])
                        .send()
                        .await?
                        .json::<InfoResponse>()
                        .await?;

            let data = response.data;
            println!("Name: {}\nAQI: {}\n", data.city.name, data.aqi);
        },

        Opt::Search{terms} =>
        {
            // the response will be in a JSON format automaticall
            let response = client.get("https://api.waqi.info/search/")
            .query(&[("token", AQI_TOKEN), ("keyword", terms.as_str())])
            .send()
            .await?
            .json::<SearchResponse>()
            .await?;

            print_search_response(&response);
        }
    };

    Ok(())
}