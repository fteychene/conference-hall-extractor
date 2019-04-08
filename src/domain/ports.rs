use super::model::{Event};
use failure::Error;

pub trait ExportLoader {
    fn load_event(&self) -> Result<Event, Error>;
}

pub trait EventSaver {
    fn save_event(event: Event) -> Result<(), Error>;
}