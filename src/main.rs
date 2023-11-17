#![feature(iterator_try_collect)]

use std::io::Write;
use std::fs::{self, OpenOptions};
use ahash::HashMap;
use anyhow::Result;
use console::Term;
use entries::{Entry, Connotation, EntryId};
use parsing::{parse_lines, connoted_entry};
use tupletools::Snd;

mod parsing;
mod entries;


fn main() -> Result<()> {
    let content = fs::read_to_string("vocabulary.txt")?;
    let entries = parse_lines(Entry::parse, &content)?;

    connote(&entries)?;

    Ok(())
}

fn connote<'a>(entries: &'a [Entry]) -> Result<HashMap<EntryId<'a>, Connotation>> {
    let connoted_content = fs::read_to_string("connoted.txt")
        .unwrap_or_default();

    let mut connoted_entries: HashMap<EntryId, Connotation> = parse_lines(connoted_entry, &connoted_content)?
        .into_iter()
        .collect();

    entries.iter()
        .map(|entry| {
            let id = entry.id();
            let connotation = connoted_entries.remove_entry(&id)
                .map_or_else(|| connote_entry(entry), |entry| Ok(entry.snd()))?;
            Ok((entry.id(), connotation))
        }).try_collect()
}

fn connote_entry(entry: &Entry) -> Result<Connotation> {
    let terminal = Term::stdout();

    let connotation = loop {
        terminal.clear_screen()?;
        terminal.write_line(&format!("What is the connotation of: {entry}?"))?;
        terminal.write_line("'+' for positive, '-' for negative or '.' for neutral")?;

        match terminal.read_char()? {
            '+' => break Connotation::Positive,
            '-' => break Connotation::Negative,
            '.' => break Connotation::Neutral,
            _ => {}
        };
    };

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("connoted.txt")?;

    writeln!(file, "{} - {}", entry.id(), connotation)?;
    Ok(connotation)
}