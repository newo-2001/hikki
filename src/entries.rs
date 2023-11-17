use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Connotation {
    Positive,
    Neutral,
    Negative
}

impl Display for Connotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Positive => "positive",
            Self::Negative => "negative",
            Self::Neutral => "neutral"
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WordForm {
    Noun,
    Adjective,
    Verb,
    Adverb,
    Phrase,
    Saying,
    Abbreviation
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EntryId<'a> {
    pub word: &'a str,
    pub form: WordForm
}

impl Display for WordForm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Noun => "noun",
            Self::Adjective => "adj",
            Self::Verb => "verb",
            Self::Adverb => "adv",
            Self::Phrase => "phrase",
            Self::Saying => "say",
            Self::Abbreviation => "abbr"
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Entry<'a> {
    pub word: &'a str,
    pub definition: &'a str,
    pub form: WordForm
}

impl Entry<'_> {
    pub fn id(&self) -> EntryId {
        EntryId { word: self.word, form: self.form }
    }
}

impl Display for EntryId<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [{}]", self.word, self.form)
    }
}

impl<'a> Display for Entry<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.id(), self.definition)
    }
}