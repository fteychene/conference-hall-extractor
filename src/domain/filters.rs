use crate::domain::model::{Talk, Speaker};
use im::Vector;

pub fn confirmed_talks(talk: &Talk) -> bool {
    talk.state == "confirmed"
}

pub fn not<T, F>(f: F) -> impl Fn(&T) -> bool
    where F: Fn(&T) -> bool {
    move | v | { !f(v) }
}

pub fn confirmed_and_specific_talks(specific_talks: Vector<String>) -> impl Fn(&Talk) -> bool {
    move |talk| {
        confirmed_talks(talk)
            || specific_talks.contains(&talk.title)
    }
}

pub fn speaker_in_talks(talks: &Vector<Talk>) -> impl Fn(&Speaker) -> bool {
    let cloned_talks = talks.clone();
    move |speaker| {
        cloned_talks.iter().filter(|talk| talk.speakers.contains(speaker)).count() > 0
    }
}
