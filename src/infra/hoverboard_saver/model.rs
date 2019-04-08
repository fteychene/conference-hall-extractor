use im::{HashMap, Vector};

#[derive(Serialize, Debug)]
pub struct Event {
    pub sessions: HashMap<String, Talk>,
    pub speakers: HashMap<String, Speaker>
}

#[derive(Serialize, Debug, Clone)]
pub struct Talk {
    pub title: String,
    pub complexity: Option<String>,
    pub description: String,
    pub language: Option<String>,
    pub tags: Vector<String>,
    pub speakers: Vector<String>,
    pub presentation: Option<String>,
    #[serde(rename = "videoId")]
    pub video : Option<String>,
    pub image: Option<String>,
    pub icon: Option<String>
}

#[derive(Serialize, Debug, Clone)]
pub struct Speaker {
    pub bio: String,
    pub company: Option<String>,
//    #[serde(rename = "companyLogo")]
//    pub company_logo: Option<String>,
//    #[serde(rename = "companyLogoUrl")]
//    pub company_logo_url: Option<String>,
    pub country: String,
    pub name: String,
//    pub photo: Option<String>,
    #[serde(rename = "photoUrl")]
    pub photo_url: Option<String>,
    pub socials: Vector<Social>,
//    pub title: Option<String>
}

#[derive(Serialize, Debug, Clone)]
pub struct Social {
    link: String,
    icon: &'static str,
    name: &'static str
}

impl Social {
    pub fn twitter(link: String) -> Social {
        Social {
            link,
            icon: "twitter",
            name: "Twitter"
        }
    }

    pub fn github(link: String) -> Social {
        Social {
            link,
            icon: "github",
            name: "GitHub"
        }
    }
}