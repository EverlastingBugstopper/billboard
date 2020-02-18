mod alignment;
mod border;
mod space;

pub use alignment::Alignment;
pub use border::{BorderColor, BorderComponents, BorderStyle};
use space::Space;

use crate::Boxx;

#[derive(Clone, PartialEq, Debug)]
pub struct Config {
    pub padding: Space,
    pub margin: Space,
    pub border_components: BorderComponents,
    pub border_color: Option<BorderColor>,
    pub box_alignment: Alignment,
    pub text_alignment: Alignment,
}

impl Config {
    pub fn default() -> Config {
        Config {
            padding: Space::uniform(2),
            margin: Space::uniform(0),
            border_color: None,
            border_components: BorderComponents::new(BorderStyle::Single),
            text_alignment: Alignment::Center,
            box_alignment: Alignment::Center,
        }
    }

    pub fn padding<'a>(&'a mut self, padding: usize) -> &'a mut Config {
        self.padding = Space::uniform(padding);
        self
    }

    pub fn margin<'a>(&'a mut self, margin: usize) -> &'a mut Config {
        self.margin = Space::uniform(margin);
        self
    }

    pub fn border_style<'a>(&'a mut self, border_style: BorderStyle) -> &'a mut Config {
        self.border_components = BorderComponents::new(border_style);
        self
    }

    pub fn text_alignment<'a>(&'a mut self, text_alignment: Alignment) -> &'a mut Config {
        self.text_alignment = text_alignment;
        self
    }

    pub fn box_alignment<'a>(&'a mut self, box_alignment: Alignment) -> &'a mut Config {
        self.box_alignment = box_alignment;
        self
    }

    pub fn border_color<'a>(&'a mut self, border_color: BorderColor) -> &'a mut Config {
        self.border_color = Some(border_color);
        self
    }

    pub fn build<'a>(&'a mut self) -> Boxx {
        Boxx {
            config: self.to_owned(),
        }
    }
}
