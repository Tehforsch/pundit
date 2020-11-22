pub mod anki;

use crate::anki::anki_args::{AnkiOpts, AnkiSubCommand};
use crate::anki::anki_collection::AnkiCollection;
use crate::anki::{close_connection, read_collection};
use anyhow::Result;
use clap::Clap;
use rusqlite::Connection;
use std::error::Error;

pub fn list_models(collection: &AnkiCollection) -> Result<()> {
    for model in collection.models.iter() {
        println!("{}", &model.name);
    }
    Ok(())
}

pub fn list_decks(collection: &AnkiCollection) -> Result<()> {
    for deck in collection.decks.iter() {
        println!("{}", &deck.name)
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = get_args();
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
    }
    Ok(())
}
