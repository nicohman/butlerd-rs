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
#[derive(Serialize, Deserialize, Debug)]
pub struct GamesSearchRes {
    pub games: Option<Vec<Game>>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct DidCancel {
    pub didCancel: bool,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Success {
    pub success: bool,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct FetchProfiles {
    pub profiles: Vec<Profile>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ProfileKeys {
    pub items: Vec<DownloadKey>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct FetchDKey {
    pub downloadKey: DownloadKey
}
#[derive(Serialize, Deserialize, Debug)]
pub struct FetchCollection {
    pub collection: Collection
}
#[derive(Serialize, Deserialize, Debug)]
pub struct FetchCollectionGames {
    pub items : Option<Vec<CollectionGame>>
}
#[derive(Serialize, Deserialize, Debug)]
pub struct FetchPCol {
    pub items : Option<Vec<Collection>>
}
#[derive(Serialize, Deserialize, Debug)]
pub struct SearchUsers {
    pub users : Option<Vec<User>>
}
#[derive(Serialize, Deserialize, Debug)]
pub struct GetRes {
    pub value: Option<String>,
    pub ok: Option<bool>
}
