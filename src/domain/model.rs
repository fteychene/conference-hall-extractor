use im::vector::Vector;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Event {
    pub name: String,
    pub categories: Vector<Category>,
    pub formats: Vector<Format>,
    pub talks: Vector<Talk>,
    pub speakers: Vector<Speaker>
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Category {
    pub name: String,
    pub description: String,
    pub id: Uuid,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Format {
    pub name: String,
    pub description: String,
    pub id: Uuid,
}

#[derive(Debug, Clone)]
pub struct Talk {
    pub id: Option<String>,
    pub title: String,
    pub state: String,
    pub level: Option<String>,
    pub category: Option<Category>,
    pub format: Option<Format>,
    pub speakers: Vector<Speaker>,
    pub description: String,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Speaker {
    pub uid: String,
    pub display_name : Option<String>,
    pub bio: Option<String>,
    pub company: Option<String>,
    pub photo_url: Option<String>,
    pub twitter: Option<String>,
    pub github: Option<String>,
    pub city: Option<String>
}