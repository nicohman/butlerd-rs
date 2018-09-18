#![allow(non_snake_case)]
use Responses::*;
#[derive(Serialize, Deserialize, Debug)]
pub struct FetchUploads {
    pub uploads: Vec<Upload>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct FetchCaves {
    pub items: Vec<Cave>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct FetchCave {
    pub cave: Cave,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct FetchGame {
    pub game: Game,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct FetchProfile {
    pub profile: Profile,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct FetchGames {
    pub items: Vec<Game>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct FetchPGames {
    pub items: Vec<ProfileGame>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct FetchSale {
    pub sale: Option<Sale>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct FetchIDirs {
    pub installLocations: Vec<InstallLocationSummary>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct DownList {
    pub downloads: Option<Vec<Download>>,
}
