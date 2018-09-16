use serde_json::value::Map;
use serde_json::Value;
#[derive(Serialize, Deserialize, Debug)]
pub struct BStart {
    pub secret: String,
    pub http: Map<String, Value>,
    pub https: Map<String, Value>,
}

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
    pub userId: Option<i32>,
    pub viewsCount: Option<i32>,
    pub downloadsCount: Option<i32>,
    pub purchasesCount: Option<i32>,
    pub published: Option<bool>,
}
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
}
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub displayName: String,
    pub developer: bool,
    pub pressUser: bool,
    pub url: String,
    pub coverUrl: String,
    #[serde(default)]
    pub stillCoverUrl: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Build {
    pub id: i32,
    pub parentBuildId: i32,
    pub state: String,
    pub version: i32,
    #[serde(default)]
    pub userVersion: String,
    //  Todo
    //  pub files: BuildFiles[]
    pub user: User,
    #[serde(flatten)]
    pub dates: Dates,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Cave {
    pub id: String,
    pub game: Game,
    pub upload: Upload,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub id: i32,
    pub jsonrpc: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Dates {
    #[serde(default)]
    pub createdAt: String,
    #[serde(default)]
    pub updatedAt: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseRes {
    #[serde(flatten)]
    pub response: Response,
    pub result: Map<String, Value>,
}
