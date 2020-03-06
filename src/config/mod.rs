mod border;
mod space;

pub use border::{BorderColor, BorderComponents, BorderStyle};
pub use console::Alignment;
use space::Space;

use crate::Boxx;

/// `Config` holds the configuration of a `Boxx`
/// and determines how it should be displayed
#[derive(Clone, PartialEq, Debug)]
pub struct Config {
    pub(crate) padding: Space,
    pub(crate) margin: Space,
    pub(crate) border_components: BorderComponents,
    pub(crate) border_color: Option<BorderColor>,
    pub(crate) box_alignment: Alignment,
    pub(crate) text_alignment: Alignment,
}

impl Config {
    /// The default `Config` is a centered box with centered text
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

    /// Set the padding of your `Boxx` to a specific size
    /// This is the space around the text inside of your `Boxx`
    pub fn padding<'a>(&'a mut self, padding: usize) -> &'a mut Config {
        self.padding = Space::uniform(padding);
        self
    }

    /// Set the margin of your `Boxx` to a specific size
    /// This is the space around the outside of your `Boxx`
    pub fn margin<'a>(&'a mut self, margin: usize) -> &'a mut Config {
        self.margin = Space::uniform(margin);
        self
    }

    /// Set the `BorderStyle` of your `Boxx`
    pub fn border_style<'a>(&'a mut self, border_style: BorderStyle) -> &'a mut Config {
        self.border_components = BorderComponents::new(border_style);
        self
    }

    /// Set the `Alignment` of the text in your `Boxx`
    pub fn text_alignment<'a>(&'a mut self, text_alignment: Alignment) -> &'a mut Config {
        self.text_alignment = text_alignment;
        self
    }

    /// Set the `Alignment` of your `Boxx` in the terminal
    pub fn box_alignment<'a>(&'a mut self, box_alignment: Alignment) -> &'a mut Config {
        self.box_alignment = box_alignment;
        self
    }

    /// Set the border color of your `Boxx`
    pub fn border_color<'a>(&'a mut self, border_color: BorderColor) -> &'a mut Config {
        self.border_color = Some(border_color);
        self
    }

    /// Get a `Boxx` from a `Config`
    /// Use this after you have applied the styling you desire
    pub fn build<'a>(&'a mut self) -> Boxx {
        Boxx {
            config: self.to_owned(),
        }
    }
}
