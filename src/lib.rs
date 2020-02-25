#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![doc(html_root_url = "https://docs.rs/boxx/0.0.1-alpha")]

//! boxx
//!
//! The `boxx` crate provides a convenient, high-level API
//! for creating boxes in the terminal.

mod config;
mod visual;

pub use config::{Alignment, BorderColor, BorderComponents, BorderStyle, Config};
use visual::{visual_width, VisualWidthError};

use console::Style;
use std::cmp;

const SPACE: &str = " ";
const NEWLINE: &str = "\n";

/// `Boxx` is a customizable data structure that will pretty print content
/// separated by newlines (`\n`).
#[derive(PartialEq, Clone, Debug)]
pub struct Boxx {
    /// Display configuration of the `Boxx`. This field dictates how a `Boxx` will
    /// be displayed in the terminal.
    pub config: Config,
}

impl Boxx {
    /// Create `Boxx` with deafult configuration.
    ///
    /// # Example
    ///
    /// ```
    /// use boxx::Boxx;
    /// let boxx = Boxx::default();
    /// ```
    pub fn default() -> Boxx {
        Config::default().build()
    }

    /// Creates a new `Boxx` from a pre-made `Config`.
    pub fn new(config: Config) -> Boxx {
        Boxx { config }
    }

    /// Creates a `Config` to configure a `Boxx`
    pub fn builder() -> Config {
        Config::default()
    }

    /// Prints your `Boxx`ed content to `stdout`
    /// If your content is long, separate it with line breaks (`\n`)
    pub fn display(&self, content: &str) {
        println!("{}", self.as_str(content).unwrap_or(content.to_string()));
    }

    /// Get your content in a `Boxx` as a `String`
    pub fn as_str(&self, content: &str) -> Result<String, VisualWidthError> {
        let border_color = match self.config.border_color {
            Some(color) => Style::from_dotted_str(&format!("{:?}", color).to_lowercase()),
            None => Style::default(),
        };
        let mut lines: Vec<String> = Vec::new();
        lines.push(SPACE.repeat(self.config.padding.top));
        for line in content.split_terminator(NEWLINE) {
            lines.push(line.to_string());
        }
        lines.push(SPACE.repeat(self.config.padding.bottom));
        let mut widest_length: usize = 0;
        for line in lines.clone() {
            widest_length = cmp::max(widest_length, visual_width(line.as_str())?);
        }
        for (i, line) in lines.clone().iter().enumerate() {
            let padding = widest_length - visual_width(line.as_str())?;
            lines[i] = match &self.config.text_alignment {
                Alignment::Left => line.clone(),
                Alignment::Right => format!("{}{}", SPACE.repeat(padding), line),
                Alignment::Center => format!("{}{}", SPACE.repeat(padding / 2), line),
            };
        }
        let content_width = widest_length + self.config.padding.left + self.config.padding.right;
        let output = if let Some((max_width, _)) = term_size::dimensions() {
            if content_width > max_width {
                return Ok(content.to_string());
            }
            let padding_left = SPACE.repeat(self.config.padding.left);
            let margin_left = match &self.config.box_alignment {
                Alignment::Left => SPACE.repeat(self.config.padding.left),
                Alignment::Center => {
                    let space_width = cmp::max((max_width - content_width) / 2, 0);
                    SPACE.repeat(space_width)
                }
                Alignment::Right => {
                    let space_width =
                        cmp::max(max_width - content_width - self.config.margin.right - 2, 0);
                    SPACE.repeat(space_width)
                }
            };
            let mut horizontal_str = self.config.border_components.horizontal.to_string();
            let mut horizontal_str_width = visual_width(horizontal_str.as_str())?;
            if horizontal_str_width == 0 {
                horizontal_str.push_str(" ");
                horizontal_str_width = 1;
            }
            let mut horizontal = horizontal_str.repeat(
                (content_width % horizontal_str_width) + (content_width / horizontal_str_width),
            );
            while visual_width(horizontal.as_str())? > content_width {
                horizontal.pop();
            }
            let top = format!(
                "{}{}{}{}{}",
                NEWLINE.repeat(self.config.margin.top),
                margin_left,
                border_color.apply_to(&self.config.border_components.top_left),
                border_color.apply_to(&horizontal),
                border_color.apply_to(&self.config.border_components.top_right)
            );
            let bottom = format!(
                "{}{}{}{}{}",
                margin_left,
                border_color.apply_to(&self.config.border_components.bottom_left),
                border_color.apply_to(&horizontal),
                border_color.apply_to(&self.config.border_components.bottom_right),
                NEWLINE.repeat(self.config.margin.bottom)
            );
            let mut middle = String::from("\n");
            let mut vertical = self.config.border_components.vertical.to_string();
            if visual_width(vertical.as_str())? == 0 {
                vertical.push_str(" ");
            }
            let vertical = vertical.repeat(lines.len());
            let mut vertical_chars = vertical.chars();
            for line in lines {
                let vertical = border_color
                    .apply_to(&vertical_chars.next().unwrap())
                    .to_string();
                middle.push_str(&margin_left);
                middle.push_str(&vertical);
                middle.push_str(&padding_left);
                middle.push_str(&line);
                middle.push_str(&SPACE.repeat(
                    content_width - visual_width(line.as_str())? - &self.config.padding.left,
                ));
                middle.push_str(&vertical);
                middle.push_str(NEWLINE);
            }
            format!("{}{}{}", top, middle, bottom)
        } else {
            content.to_string()
        };
        Ok(output)
    }
}
