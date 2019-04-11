use crate::domain::ports::ExportLoader;
use crate::domain::model::{Event as DomainEvent, Category as DomainCategory, Format as DomainFormat, Speaker as DomainSpeaker, Talk as DomainTalk};
use super::model::{Event, Category, Format, Speaker, Talk};
use failure::Error;
use std::fs::File;
use im::Vector;
use uuid::Uuid;
use std::io::Read;


pub struct FileExtractor {
    pub file: String
}

impl ExportLoader for FileExtractor {
    fn load_event(&self) -> Result<DomainEvent, Error> {
        // Read the input file to string.
        let mut file = File::open(self.file.clone())?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let data: Event = serde_json::from_str(&contents)?;
        Ok(data.into())
    }
}

impl Into<DomainEvent> for Event {
    fn into(self) -> DomainEvent {
        let speakers = &self.speakers;
        let categories = &self.categories;
        let formats = &self.formats;
        DomainEvent {
            name: self.name,
            speakers: self.speakers.iter().map(|speaker| speaker.into()).collect(),
            formats: self.formats.iter().map(|format| format.into()).collect(),
            categories: self.categories.iter().map(|category| category.into()).collect(),
            talks: self.talks.iter().map(|talk| TalkConverter { talk, speakers, categories, formats }.into()).collect(),
        }
    }
}

impl Into<DomainFormat> for &Format {
    fn into(self) -> DomainFormat {
        DomainFormat {
            id: self.id.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
        }
    }
}

impl Into<DomainCategory> for &Category {
    fn into(self) -> DomainCategory {
        DomainCategory {
            id: self.id.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
        }
    }
}

impl Into<DomainSpeaker> for &Speaker {
    fn into(self) -> DomainSpeaker {
        DomainSpeaker {
            uid: self.uid.clone(),
            display_name: self.display_name.clone(),
            bio: self.bio.clone(),
            company: self.company.clone(),
            github: self.github.clone(),
            photo_url: self.photo_url.clone(),
            twitter: self.twitter.clone(),
            city: self.city.clone(),
            email: self.email.clone()
        }
    }
}

struct TalkConverter<'a> {
    talk: &'a Talk,
    speakers: &'a Vector<Speaker>,
    categories: &'a Vector<Category>,
    formats: &'a Vector<Format>,
}

fn find_format(id: Option<Uuid>, formats: &Vector<Format>) -> Option<DomainFormat> {
    id.and_then(|id| formats.iter().find(|format| format.id == id)).map(|value| value.into())
}

fn find_category(id: Option<Uuid>, categories: &Vector<Category>) -> Option<DomainCategory> {
    id.and_then(|id| categories.iter().find(|category| category.id == id)).map(|value| value.into())
}

fn find_speakers(ids: &Vector<String>, speakers: &Vector<Speaker>) -> Vector<DomainSpeaker> {
    speakers.iter().filter(|speaker| ids.contains(&speaker.uid)).map(|speaker| speaker.into()).collect()
}

impl<'a> Into<DomainTalk> for TalkConverter<'a> {
    fn into(self) -> DomainTalk {
        DomainTalk {
            id: None,
            description: self.talk.description.clone(),
            title: self.talk.title.clone(),
            state: self.talk.state.clone(),
            level: self.talk.level.clone(),
            format: find_format(self.talk.format, self.formats),
            category: find_category(self.talk.category, self.categories),
            speakers: find_speakers(&self.talk.speakers, self.speakers),
        }
    }
}