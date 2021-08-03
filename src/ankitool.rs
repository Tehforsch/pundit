pub mod anki;
pub mod logger;
pub mod named;

use crate::anki::anki_args::{AnkiOpts, AnkiSubCommand};
use crate::anki::anki_collection::AnkiCollection;
use crate::anki::{close_connection, get_model_by_name, read_collection};
use anyhow::Result;
use clap::Clap;
use logger::init_logger;
use rusqlite::Connection;
use std::error::Error;
use log::info;

pub fn list_models(collection: &AnkiCollection) -> Result<()> {
    for model in collection.models.iter() {
        info!("{}", &model.name);
    }
    Ok(())
}

pub fn list_decks(collection: &AnkiCollection) -> Result<()> {
    for deck in collection.decks.iter() {
        info!("{}", &deck.name)
    }
    Ok(())
}

pub fn list_fields(collection: &AnkiCollection, model_name: &str) -> Result<()> {
    let model = get_model_by_name(collection, model_name)?;
    for field in model.flds.iter() {
        info!("{}", field.name);
    }
    Ok(())
}

pub fn list_templates(collection: &AnkiCollection, model_name: &str) -> Result<()> {
    let model = get_model_by_name(collection, model_name)?;
    for template in model.tmpls.iter() {
        info!("{}", template.name);
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = get_args();
    init_logger(false).unwrap();
    let connection = Connection::open(&args.database_path).unwrap();
    let collection = read_collection(&connection)?;
    run(&connection, &collection, args)?;
    close_connection(connection)?;
    Ok(())
}

fn get_args() -> AnkiOpts {
    AnkiOpts::parse()
}

fn run(_connection: &Connection, collection: &AnkiCollection, args: AnkiOpts) -> Result<()> {
    match args.subcmd {
        AnkiSubCommand::ListModels(_) => list_models(&collection)?,
        AnkiSubCommand::ListDecks(_) => list_decks(&collection)?,
        AnkiSubCommand::ListFields(l) => list_fields(&collection, &l.model)?,
        AnkiSubCommand::ListTemplates(l) => list_templates(&collection, &l.model)?,
    }
    Ok(())
}
