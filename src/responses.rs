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
    pub id:String,
    pub url:String,
    pub title:String
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Cave {
    pub id: String,
    pub game:Game
}
