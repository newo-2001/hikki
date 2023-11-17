use std::fmt::Display;

use nom::{IResult, branch::alt, combinator::{value, all_consuming, opt}, bytes::complete::{tag, take_until}, Parser, sequence::{delimited, pair, separated_pair, terminated}, character::complete::{char, line_ending, not_line_ending}, error::ParseError, multi::separated_list0};
use tupletools::snd;
use anyhow::{anyhow, Result};

use crate::entries::{WordForm, Connotation, Entry, EntryId};

type ParseResult<'a, T, I = &'a str, E = nom::error::Error<I>> = IResult<I, T, E>;

impl WordForm {
    fn parse(input: &str) -> ParseResult<Self> {
        alt((
            value(Self::Noun, tag("noun")),
            value(Self::Adjective, tag("adj")),
            value(Self::Verb, tag("verb")),
            value(Self::Phrase, tag("phrase")),
            value(Self::Saying, tag("say")),
            value(Self::Adverb, tag("adv")),
            value(Self::Abbreviation, tag("abbr"))
        )).parse(input)
    }
}

impl Connotation {
    fn parse(input: &str) -> ParseResult<Self> {
        alt((
            value(Self::Positive, tag("positive")),
            value(Self::Negative, tag("negative")),
            value(Self::Neutral, tag("neutral"))
        )).parse(input)
    }
}

impl<'a> EntryId<'a> {
    fn parse(input: &'a str) -> ParseResult<Self> {
        pair(
            take_until(" ["),
            delimited(tag(" ["), WordForm::parse, char(']')),
        ).map(|(word, form)| EntryId { word, form })
            .parse(input)
    }
}

impl Entry<'_> {
    pub fn parse(input: &str) -> ParseResult<Entry> {
        separated_pair(EntryId::parse, tag(" - "), not_line_ending)
            .map(|(EntryId { word, form }, definition)| Entry { word, definition, form })
            .parse(input)
    }
}

pub fn connoted_entry(input: &str) -> ParseResult<(EntryId, Connotation)> {
    separated_pair(EntryId::parse, tag(" - "), Connotation::parse)
        .parse(input)
}

pub fn parse_lines<'a, O, E, F>(parser: F, input: &'a str) -> Result<Vec<O>>
    where F: Parser<&'a str, O, E>,
          E: ParseError<&'a str>,
          nom::Err<E>: Display
{
    all_consuming(terminated(separated_list0(line_ending, parser), opt(line_ending)))
        .parse(input)
        .map_err(|err| anyhow!(err.to_string()))
        .map(snd)
}