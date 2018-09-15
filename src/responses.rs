use serde_json::value::Map;
use serde_json::Value;
#[derive(Serialize, Deserialize, Debug)]
pub struct BStart {
    pub secret: String,
    pub http: Map<String, Value>,
    pub https: Map<String, Value>
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub id:i32,
    pub url:String,
    pub title:String
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Cave {
    pub id: String,
    pub game:Game
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub id:i32,
    pub jsonrpc:String
}
#[derive(Serialize, Deserialize, Debug)]
pub struct FetchCaves  {
    #[serde(flatten)]
    pub response:Response,
    pub result: Map<String, Value>
}
