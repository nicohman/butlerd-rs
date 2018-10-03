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
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub id: i32,
    pub url: String,
    pub title: String,
    pub short_text: String,
    pub cover_url: String,
    #[serde(default)]
    pub still_cover_url: String,
    #[serde(flatten)]
    pub dates: Dates,
    #[serde(default)]
    pub min_price: i32,
    pub can_be_bought: bool,
    #[serde(default)]
    pub has_demo: bool,
    #[serde(default)]
    pub in_press_system: bool,
    pub user: Option<User>,
    pub sale: Option<Sale>,
    pub user_id: Option<i32>,
    pub views_count: Option<i32>,
    pub downloads_count: Option<i32>,
    pub purchases_count: Option<i32>,
    pub published: Option<bool>,
}
/// A Game that the logged-in user's profile owns
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProfileGame {
    pub game: Game,
    pub views_count: i32,
    pub downloads_count: i32,
    pub purchases_count: i32,
    pub published: bool,
}
/// A Profile gives more information about a user through its id, but requires login
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub id: i32,
    pub last_connected: String,
    pub user: User,
}
///A downloadable file. Has a build only if the game is wharf-enabled
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Upload {
    pub id: i32,
    pub storage: String,
    #[serde(default)]
    pub host: String,
    pub filename: String,
    pub display_name: String,
    pub size: i32,
    #[serde(default)]
    pub channel_name: String,
    //    #[serde(default)]
    //    pub build: Build,
    #[serde(default)]
    pub build_id: i32,
    pub preorder: bool,
    pub demo: bool,
    #[serde(flatten)]
    pub dates: Dates,
    pub platforms: Platforms,
}
impl Upload {
    /// Given a str representing an OS, checks if an upload supports it
    /// Possible: windows, linux, osx
    pub fn supports(&self, os: &str) -> bool {
        match os {
            "windows" => self.platforms.windows.is_some(),
            "osx" => self.platforms.osx.is_some(),
            "linux" => self.platforms.linux.is_some(),
            _ => false,
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
    Amd64,
}
/// A struct that holds the platforms an Upload is compatibile with
#[derive(Serialize, Deserialize, Debug)]
pub struct Platforms {
    windows: Option<Archs>,
    osx: Option<Archs>,
    linux: Option<Archs>,
}
///An itch user's basic public info
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i32,
    pub username: String,
    pub display_name: String,
    /// Whether or not the user is a developer.
    pub developer: bool,
    pub press_user: bool,
    pub url: String,
    pub cover_url: String,
    #[serde(default)]
    pub still_cover_url: String,
}
/// A specific build of a Game. Game must be wharf-enabled
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Build {
    pub id: i32,
    pub parent_build_id: i32,
    pub state: String,
    /// The version of the Game
    pub version: i32,
    #[serde(default)]
    pub user_version: String,
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
#[serde(rename_all = "camelCase")]
pub struct Dates {
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
}
/// The base struct for responses with errors
#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseErr {
    #[serde(flatten)]
    pub response: Response,
    pub error: BError,
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
#[serde(rename_all = "camelCase")]
pub struct Sale {
    pub id: i32,
    pub game_id: i32,
    pub rate: i32,
    ///Can be negative due to [reverse sales](https://itch.io/updates/introducing-reverse-sales)
    pub start_date: String,
    pub end_date: String,
}
/// Info on a game install location
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InstallLocationSummary {
    pub id: String,
    pub path: String,
    pub size_info: InstallLocationSizeInfo,
}
/// Info on storage usage for an install location
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InstallLocationSizeInfo {
    /// Number of bytes used by currently installed games
    pub installed_size: i64,
    /// Negative if unknown
    pub free_size: i64,
    /// Negative if unknown
    pub total_size: i64,
}
/// The reason you gave to download a game
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
/// The request to queue up a game installation
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InstallQueueReq {
    pub install_location_id: String,
    pub reason: String,
    pub game: Game,
    pub upload: Upload,
}
/// A download from the download queue
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
    pub started_at: String,
    pub finished_at: Option<String>,
    pub staging_folder: String,
}
/// Butler daemon version info
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VersionInfo {
    pub version: String,
    /// More verbose version
    pub version_string: String,
}
/// What you get back when you check for updates. Each item in updates represents a different game
#[derive(Serialize, Deserialize, Debug)]
pub struct CheckUpdate {
    pub updates: Option<Vec<GameUpdate>>,
    pub warnings: Option<Vec<String>>,
}
/// Information on an avavilable update for a game
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GameUpdate {
    pub cave_id: String,
    pub game: Game,
    /// Whether or not this is a direct update within the same channel. See [here](http://docs.itch.ovh/butlerd/master/#/?id=gameupdate)
    pub direct: bool,
    pub choices: Option<Vec<GameUpdateChoice>>,
}
/// A choice of a possible update for a game. Higher confidence is usually better.
#[derive(Serialize, Deserialize, Debug)]
pub struct GameUpdateChoice {
    pub upload: Upload,
    pub build: Option<Build>,
    /// How confident butler is that this is the `right` update
    pub confidence: f64,
}
/// Butler's response when you login using an username and password
#[derive(Serialize, Deserialize, Debug)]
pub struct PassLogRes {
    pub profile: Profile,
    pub cookie: Map<String, Value>,
}
/// Representation of a download key for a purchased non-free game
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DownloadKey {
    pub id: i64,
    pub game_id: i32,
    pub game: Game,
    pub owner_id: i64,
    #[serde(flatten)]
    pub dates: Dates,
}
/// Returned from fetch_commons. Most of butler's cached info
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Commons {
    pub download_keys: Vec<DownloadKeySummary>,
    pub caves: Vec<CaveSummary>,
    pub install_locations: Vec<InstallLocationSummary>,
}
/// Summary of a DownloadKey, but not an actual DownloadKey
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DownloadKeySummary {
    pub id: i64,
    pub game_id: i32,
    pub created_at: String,
}
/// Summary of a cave's info
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CaveSummary {
    pub id: String,
    pub game_id: i32,
    pub last_touched_at: Option<String>,
    pub seconds_run: i32,
    pub installed_size: i64,
}
/// Info on a game collection
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    pub id: i32,
    pub title: String,
    #[serde(flatten)]
    pub dates: Dates,
    pub games_count: i32,
    /// Presence depends on whether fetched with fetch_collection or fetch_collection_games
    pub collection_games: Option<Vec<CollectionGame>>,
    pub user_id: i32,
    pub user: User,
}
/// Info on a game within a game collection
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CollectionGame {
    pub collection_id: i32,
    pub collection: Collection,
    pub game_id: i32,
    pub game: Game,
    pub position: i32,
    pub blurb: String,
    pub user_id: i32,
    #[serde(flatten)]
    pub dates: Dates,
}
/// Information on a filesystem. Returned by statfs
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FsInfo {
    pub free_size: i64,
    pub total_size: i64,
}
/// Describes a directory that butler could clean up
#[derive(Serialize, Deserialize, Debug)]
pub struct CleanDownloadsEntry {
    pub path: String,
    pub size: i64,
}
/// A generic error struct. If the code is -1 or -2, it was a problem on this crate's side.
/// Otherwise, butler returned this in response to a request
#[derive(Serialize, Deserialize, Debug)]
pub struct BError {
    pub code: i64,
    pub message: String,
}
