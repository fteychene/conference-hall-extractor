use im::Vector;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    pub name: String,
    pub categories: Vector<Category>,
    pub formats: Vector<Format>,
    pub talks: Vector<Talk>,
    pub speakers: Vector<Speaker>
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub struct Category {
    pub name: String,
    pub description: String,
    pub id: Uuid,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub struct Format {
    pub name: String,
    pub description: String,
    pub id: Uuid,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Talk {
    pub title: String,
    pub state: String,
    pub level: Option<String>,
    #[serde(rename = "categories")]
    pub category: Option<Uuid>,
    #[serde(rename = "formats")]
    pub format: Option<Uuid>,
    pub speakers: Vector<String>,
    #[serde(rename = "abstract")]
    pub description: String,
    pub rating: f32,
    pub loves: i8,
    pub hates: i8
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Hash, PartialEq)]
pub struct Speaker {
    pub uid: String,
    #[serde(rename = "displayName")]
    pub display_name : Option<String>,
    pub bio: Option<String>,
    pub company: Option<String>,
    #[serde(rename = "photoURL")]
    pub photo_url: Option<String>,
    pub twitter: Option<String>,
    pub github: Option<String>,
    pub city: Option<String>,
    pub email: Option<String>
}