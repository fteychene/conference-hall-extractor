use crate::domain::ports::{EventSaver};
use crate::domain::model::{Event as DomainEvent, Talk as DomainTalk, Speaker as DomainSpeaker};
use super::model::{Event, Talk, Speaker, Social};

use failure::Error;
use im::Vector;

pub struct HoverboardSaver {}

impl EventSaver for HoverboardSaver {

    fn save_event(event: DomainEvent) -> Result<(), Error> {
        let infra_event: Event = event.into();
        let json = serde_json::to_string(&infra_event)?;
        println!("{}", json);
        Ok(())
    }
}


impl From<DomainEvent> for Event {
    fn from(source: DomainEvent) -> Self {
        Event {
            sessions: source.talks.iter().enumerate().map(|(index, talk)| (index.to_string(), talk.into())).collect(),
            speakers: source.speakers.iter().map(|speaker| (speaker.uid.clone(), speaker.into())).collect()
        }
    }
}

impl From<&DomainTalk> for Talk {
    fn from(source: &DomainTalk) -> Self {
        Talk {
            title: source.title.clone(),
            complexity: source.level.clone(),
            description: source.description.clone(),
            language: Some("French".to_string()),
            tags: source.category.as_ref().map(|category| Vector::singleton(category.name.clone())).unwrap_or_else(|| Vector::new()),
            speakers: source.speakers.iter().map(|speaker| speaker.uid.clone()).collect(),
            presentation: None,
            video: None,
            image: None,
            icon: None
        }
    }
}

fn speakers_socials(speaker: &DomainSpeaker) -> Vector<Social> {
    let mut result = Vector::new();
    speaker.twitter.as_ref().map(|link| result.push_front(Social::twitter(link.clone())));
    speaker.github.as_ref().map(|link| result.push_front(Social::github(link.clone())));
    result
}

impl From<&DomainSpeaker> for Speaker {
    fn from(source: &DomainSpeaker) -> Self {
        Speaker {
            name: source.display_name.clone().unwrap_or_else(|| "???".to_string()),
            photo_url: source.photo_url.clone(),
            socials: speakers_socials(source),
            bio: source.bio.clone().unwrap_or_else(|| "".to_string()),
            company: source.company.clone(),
            country: source.city.clone().unwrap_or_else(|| "France".to_string())
        }
    }
}

