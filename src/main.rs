extern crate failure;
extern crate reqwest;
extern crate uuid;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate im;

use im::vector::Vector;
use std::hash::Hash;
use failure::Error;
use uuid::Uuid;
use std::env;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Event {
    name: String,
    categories: Vector<Category>,
    formats: Vector<Format>,
    talks: Vector<Talk>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
struct Category {
    name: String,
    description: String,
    id: Uuid,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
struct Format {
    name: String,
    description: String,
    id: Uuid,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Talk {
    title: String,
    state: String,
    level: Option<String>,
    #[serde(rename = "categories")]
    category: Option<Uuid>,
    #[serde(rename = "formats")]
    format: Option<Uuid>,
    speakers: Vector<String>,
    #[serde(rename = "abstract")]
    description: String,
}

fn group_by<A, R, I>(origin: &Vector<A>, id: I) -> HashMap<R, Vector<&A>>
    where R: Hash + Eq + Clone, A: Clone, I: Fn(&A) -> R {
    origin.iter()
        .fold(HashMap::new(), |mut acc, value| {
            let values = acc.entry(id(value)).or_insert(Vector::new());
            values.push_back(value);
            acc
        })
}

fn map_keys<'a, A, B, R, F>(map: HashMap<A, B>, operation: F) -> HashMap<R, B>
    where F: Fn(&A) -> R, A : Hash + Eq, R : Hash + Eq {
    map.into_iter()
        .map(|(key, value)| (operation(&key), value))
        .collect()
}

fn talks_by_categories<'a>(categories: &Vector<Category>, talks: &'a Vector<Talk>) -> HashMap<Option<Category>, Vector<&'a Talk>> {
    let values = group_by(talks, |talk| talk.category);
    map_keys(values, |uuid| uuid.and_then(|id| categories.into_iter().find(|v| v.id == id).map(|v| v.clone())))
}

fn talks_by_format<'a>(talks: &'a Vector<Talk>, formats: &Vector<Format>) -> HashMap<Option<Format>, Vector<&'a Talk>> {
    let group_by = group_by(talks, |talk| talk.format);
    map_keys(group_by, |uuid| uuid.and_then(|id| formats.into_iter().find(|v| v.id == id).map(|v| v.clone())))
}

fn main() -> Result<(), Error> {
    let project_id = env::var("PROJECT_ID").expect("PROJECT_ID is a mandatory environment variable");
    let api_key = env::var("API_KEY").expect("API_KEY is a mandatory environment variable");

    let test: Event = reqwest::get(&format!("https://conference-hall.io/api/v1/event/{}?key={}", project_id, api_key))?.json()?;
    for (category, talks_by_categories) in talks_by_categories(&test.categories, &test.talks) {
        let rp = talks_by_categories.into_iter().map(|v| v.clone()).collect();
        println!("Categorie : {}", category.map(|v| v.name).get_or_insert("None".to_string()));
        for (format, size) in talks_by_format(&rp, &test.formats) {
            println!("\t{} => {}", format.map(|f| f.name).get_or_insert("Undefined".to_string()), size.into_iter().count())
        }
    };
    Ok(())
}
