use crate::domain::model::{Event};
use crate::domain::ports::{EventSaver};
use failure::Error;
use im::vector::Vector;

pub struct MailSaver {}

impl EventSaver for MailSaver {
    fn save_event(event: Event) -> Result<(), Error> {
        let mails: Vector<String> = event.speakers.iter()
            .map(|speaker| speaker.email.clone())
            .filter(|option| option.is_some())
            .map(|option| option.unwrap())
            .collect();
        let json = serde_json::to_string(&mails)?;
        println!("{}", json);
        Ok(())
    }
}