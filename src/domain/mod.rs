pub mod model;
pub mod ports;
pub mod filters;


use model::Event;

impl Event {
    pub fn filter(self, filter: impl FnOnce(Event) -> Event) -> Event {
        filter(self)
    }
}