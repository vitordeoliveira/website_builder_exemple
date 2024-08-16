use std::fmt::Display;

use super::I18N;

pub trait Translatable {
    fn title(&self) -> Translation;
    fn stringvalue(&self) -> Translation;
}

impl Translatable for I18N {
    fn title(&self) -> Translation {
        let text = match self {
            I18N::En => "title",
            I18N::Fr => "titre",
            I18N::Es => "título",
            I18N::De => "titel",
        };

        Translation(text.to_string())
    }

    fn stringvalue(&self) -> Translation {
        let text = match self {
            I18N::En => "Hello from myownvalue",
            I18N::Fr => "Bonjour de ma propre valeur",
            I18N::Es => "Hola de mi propio valor",
            I18N::De => "Hallo von meinem eigenen Wert",
        };

        Translation(text.to_string())
    }
}

#[derive(Debug, PartialEq)]
pub struct Translation(pub String);

impl Display for Translation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
