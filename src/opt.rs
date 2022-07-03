use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Aqi
{
    #[structopt(subcommand)]
    pub command: Opt
}

#[derive(StructOpt)]
pub enum Opt
{
    /// Get the Info by suppling the <url>
    Info{url: String},
    /// Search for a place(Station) by name
    Search{terms: String},
}