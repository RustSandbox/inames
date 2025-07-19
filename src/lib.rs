use rand::Rng;
use rand::seq::SliceRandom;
use rand::thread_rng;

const ADJECTIVES: &[&str] = &include!(concat!(env!("OUT_DIR"), "/adjectives.rs"));
const NOUNS: &[&str] = &include!(concat!(env!("OUT_DIR"), "/nouns.rs"));

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Name {
    Plain,
    Numbered,
}

pub struct Generator<'a> {
    adjectives: &'a [&'a str],
    nouns: &'a [&'a str],
    rng: rand::rngs::ThreadRng,
    naming: Name,
}

impl<'a> Default for Generator<'a> {
    fn default() -> Self {
        Generator::new(ADJECTIVES, NOUNS, Name::Plain)
    }
}

impl<'a> Generator<'a> {
    pub fn new(adjectives: &'a [&'a str], nouns: &'a [&'a str], naming: Name) -> Self {
        Generator {
            adjectives,
            nouns,
            rng: thread_rng(),
            naming,
        }
    }

    pub fn with_naming(naming: Name) -> Self {
        Generator::new(ADJECTIVES, NOUNS, naming)
    }

    pub fn next(&mut self) -> Option<String> {
        let adj = self.adjectives.choose(&mut self.rng)?;
        let noun = self.nouns.choose(&mut self.rng)?;

        Some(match self.naming {
            Name::Plain => format!("{}-{}", adj, noun),
            Name::Numbered => {
                let num: u16 = self.rng.gen_range(0..10000);
                format!("{}-{}-{:04}", adj, noun, num)
            }
        })
    }
}

impl<'a> Iterator for Generator<'a> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        self.next()
    }
}

pub fn default() -> impl Iterator<Item = String> {
    Generator::default()
}

pub fn with_naming(naming: Name) -> impl Iterator<Item = String> {
    Generator::with_naming(naming)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_generator() {
        let mut generator = Generator::default();
        let name = generator.next().unwrap();
        assert!(name.contains('-'));
        let parts: Vec<&str> = name.split('-').collect();
        assert_eq!(parts.len(), 2);
    }

    #[test]
    fn test_numbered_generator() {
        let mut generator = Generator::with_naming(Name::Numbered);
        let name = generator.next().unwrap();
        let parts: Vec<&str> = name.split('-').collect();
        assert_eq!(parts.len(), 3);
        assert_eq!(parts[2].len(), 4);
        assert!(parts[2].parse::<u16>().is_ok());
    }

    #[test]
    fn test_custom_lists() {
        let adjectives = &["test"];
        let nouns = &["name"];
        let mut generator = Generator::new(adjectives, nouns, Name::Plain);
        let name = generator.next().unwrap();
        assert_eq!(name, "test-name");
    }
}
