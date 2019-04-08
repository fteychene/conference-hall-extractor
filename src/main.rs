#[macro_use]
extern crate failure;
extern crate uuid;
extern crate im;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate reqwest;

mod domain;
mod infra;

use failure::{Error};
use infra::export_loader::loader::FileExtractor;
use domain::ports::{ExportLoader, EventSaver};
use domain::filters::{confirmed_and_specific_talks, speaker_in_talks};
use crate::infra::hoverboard_saver::saver::HoverboardSaver;
use im::Vector;
use std::env;



fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let file = args.get(1).ok_or_else(|| format_err!("Missing conference export path argument"))?;


    let event_filtered = FileExtractor { file: file.clone() }.load_event()?
        .filter(|mut event| {
            event.talks = event.talks.into_iter().filter(confirmed_and_specific_talks(Vector::singleton("Back to the Feature".to_string()))).collect();
            event
        })
        .filter(|mut event| {
            event.speakers = event.speakers.into_iter().filter(speaker_in_talks(&event.talks)).collect();
            event
        });
    HoverboardSaver::save_event(event_filtered)?;
    Ok(())
}