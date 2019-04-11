use crate::domain::model::{Talk, Speaker};
use im::Vector;

pub fn confirmed_talks(talk: &Talk) -> bool {
    status(vector!["confirmed".to_string()])(talk)
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

pub fn status(statuses: Vector<String>) -> impl Fn(&Talk) -> bool {
    move |talk| statuses.contains(&talk.state)
}

pub fn status_and_specific_talks(statuses: Vector<String>, specific_talks: Vector<String>) -> impl Fn(&Talk) -> bool {
    let status_filter= status(statuses);
    move |talk| {
        status_filter(talk)
            || specific_talks.contains(&talk.title)
    }
}

pub fn speaker_in_talks(talks: &Vector<Talk>) -> impl Fn(&Speaker) -> bool {
    let cloned_talks = talks.clone();
    move |speaker| {
        cloned_talks.iter().filter(|talk| talk.speakers.contains(speaker)).count() > 0
    }
}
