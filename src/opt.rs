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
    Info{url: String},
    Search{terms: String},
}