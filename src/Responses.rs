//! Structs that define how the butlerd api responds to specific queries. These will often
//! be returned from functions or requests for you to use.
#![allow(non_snake_case)]
use serde_json::value::Map;
use serde_json::Value;
///What butlerd prints at startup
#[derive(Serialize, Deserialize, Debug)]
pub struct BStart {
    pub secret: String,
    pub http: Map<String, Value>,
    pub https: Map<String, Value>,
}
///Game Information
#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub id: i32,
    pub url: String,
    pub title: String,
    pub shortText: String,
    pub coverUrl: String,
    #[serde(default)]
    pub stillCoverUrl: String,
    #[serde(flatten)]
    pub dates: Dates,
    #[serde(default)]
    pub minPrice: i32,
    pub canBeBought: bool,
    #[serde(default)]
    pub hasDemo: bool,
    #[serde(default)]
    pub inPressSystem: bool,
    pub user: Option<User>,
    pub sale: Option<Sale>,
    pub userId: Option<i32>,
    pub viewsCount: Option<i32>,
    pub downloadsCount: Option<i32>,
    pub purchasesCount: Option<i32>,
    pub published: Option<bool>,
}
/// A Game that the logged-in user's profile owns
#[derive(Serialize, Deserialize, Debug)]
pub struct ProfileGame {
    pub game: Game,
    pub viewsCount: i32,
    pub downloadsCount: i32,
    pub purchasesCount: i32,
    pub published: bool,
}
/// A Profile gives more information about a user through its id, but requires login
#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    pub id: i32,
    pub lastConnected: String,
    pub user: User,
}
///A downloadable file. Has a build only if the game is wharf-enabled
#[derive(Serialize, Deserialize, Debug)]
pub struct Upload {
    pub id: i32,
    pub storage: String,
    #[serde(default)]
    pub host: String,
    pub filename: String,
    pub displayName: String,
    pub size: i32,
    #[serde(default)]
    pub channelName: String,
    //    #[serde(default)]
    //    pub build: Build,
    #[serde(default)]
    pub buildId: i32,
    pub preorder: bool,
    pub demo: bool,
    #[serde(flatten)]
    pub dates: Dates,
    pub platforms: Platforms
}
impl Upload {
    /// Given an OS string, checks if an upload supports it
    pub fn supports(&self, os: &str) -> bool {
        match os {
            "windows" => self.platforms.windows.is_some(),
            "osx" => self.platforms.osx.is_some(),
            "linux" => self.platforms.linux.is_some(),
            _ => false
        }
    }
}
/// The architectures that an Upload suppports
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Archs {
    All,
    #[serde(rename = "386")]
    i386,
    Amd64
}
/// A struct that holds the platforms an Upload is compatibile with
#[derive(Serialize, Deserialize, Debug)]
pub struct Platforms {
   windows:Option<Archs>,
   osx:Option<Archs>,
   linux:Option<Archs>
}
///An itch user's basic public info
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub displayName: String,
    /// Whether or not the user is a developer.
    pub developer: bool,
    pub pressUser: bool,
    pub url: String,
    pub coverUrl: String,
    #[serde(default)]
    pub stillCoverUrl: String,
}
/// A specific build of a Game. Game must be wharf-enabled
#[derive(Serialize, Deserialize, Debug)]
pub struct Build {
    pub id: i32,
    pub parentBuildId: i32,
    pub state: String,
    /// The version of the Game
    pub version: i32,
    #[serde(default)]
    pub userVersion: String,
    //  Todo
    //  pub files: BuildFiles[]
    /// The user that published the Build
    pub user: Option<User>,
    #[serde(flatten)]
    pub dates: Dates,
}
/// A Cave holds a Game and associated info
#[derive(Serialize, Deserialize, Debug)]
pub struct Cave {
    pub id: String,
    /// The game this cave is associated with
    pub game: Game,
    pub upload: Upload,
}
/// The base Response struct
#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub id: i32,
    /// The version of JSONRPC that butlerd is using
    pub jsonrpc: String,
}
/// The base struct for publish/update dates
#[derive(Serialize, Deserialize, Debug)]
pub struct Dates {
    #[serde(default)]
    pub createdAt: String,
    #[serde(default)]
    pub updatedAt: String,
}
/// The base struct for responses with results
#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseRes {
    #[serde(flatten)]
    pub response: Response,
    pub result: Map<String, Value>,
}
/// A sale on a given Game
#[derive(Serialize, Deserialize, Debug)]
pub struct Sale {
    pub id: i32,
    pub gameId: i32,
    pub rate: i32,
    ///Can be negative due to [reverse sales](https://itch.io/updates/introducing-reverse-sales)
    pub startDate: String,
    pub endDate: String,
}
/// Info on a game install location
#[derive(Serialize, Deserialize, Debug)]
pub struct InstallLocationSummary {
    pub id: String,
    pub path: String,
    pub sizeInfo: InstallLocationSizeInfo,
}
/// Info on storage usage for an install location
#[derive(Serialize, Deserialize, Debug)]
pub struct InstallLocationSizeInfo {
    /// Number of bytes used by currently installed games
    pub installedSize: i64,
    /// Negative if unknown
    pub freeSize: i64,
    /// Negative if unknown
    pub totalSize: i64,
}
/// Reason to download a game
#[derive(Serialize, Deserialize, Debug)]
pub enum DownloadReason {
    #[serde(rename = "install")]
    Install,
    #[serde(rename = "reinstall")]
    Reinstall,
    #[serde(rename = "update")]
    Update,
    #[serde(rename = "version-switch")]
    VersionSwitch,
}
/// The response from queueing a game to be downloaded
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QueueResponse {
    pub id: String,
    pub reason: DownloadReason,
    pub cave_id: String,
    pub game: Game,
    pub upload: Upload,
    pub build: Build,
    pub install_folder: String,
    pub staging_folder: String,
    pub install_location_id: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InstallQueueReq {
    pub install_location_id: String,
    pub reason: String,
    pub game: Game,
    pub upload: Upload,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Download {
    pub id: String,
    pub error: Option<String>,
    pub error_message: Option<String>,
    pub error_code: Option<String>,
    pub reason: DownloadReason,
    pub position: i32,
    pub cave_id: String,
    pub game: Game,
    pub upload: Upload,
    pub build: Option<Build>,
    pub startedAt: String,
    pub finishedAt: Option<String>,
    pub stagingFolder: String,
}