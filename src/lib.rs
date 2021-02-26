#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![doc(html_root_url = "https://docs.rs/billboard/0.1.0")]

//! billboard
//!
//! The `billboard` crate provides a convenient, high-level API
//! for creating billboards in the terminal.

mod config;

pub use config::{Alignment, BorderColor, BorderComponents, BorderStyle, Config};

use console::{measure_text_width, pad_str, Style};
use std::{cmp, fmt::Display};

const SPACE: &str = " ";
const NEWLINE: &str = "\n";

/// `Billboard` is a customizable data structure that will pretty print content
/// separated by newlines (`\n`).
#[derive(PartialEq, Clone, Debug)]
pub struct Billboard {
    /// Display configuration of the `Billboard`. This field dictates how a `Billboard` will
    /// be displayed in the terminal.
    pub config: Config,
}

impl Billboard {
    /// Create a `Billboard` with default configuration.
    ///
    /// # Example
    ///
    /// ```
    /// use billboard::Billboard;
    ///
    /// let billboard = Billboard::default();
    /// ```
    pub fn default() -> Billboard {
        Config::default().build()
    }

    /// Creates a new `Billboard` from a pre-made `Config`.
    ///
    /// # Example
    ///
    /// ```
    /// use billboard::{Billboard, Config};
    ///
    /// let config = Config::default();
    /// let billboard = Billboard::new(config);
    /// ```
    pub fn new(config: Config) -> Billboard {
        Billboard { config }
    }

    /// Get a configuration builder to customize the appearance of a `Billboard`.
    ///
    /// # Example
    ///
    /// ```
    /// use billboard::Billboard;
    ///
    /// let billboard = Billboard::builder().padding(3).margin(1).build();
    /// ```
    pub fn builder() -> Config {
        Config::default()
    }

    /// Prints your `Billboard`ed content to `stdout`
    /// If your content is long, separate it with line breaks (`\n`).
    ///
    /// If the user's terminal is too small, or something goes wrong with displaying
    /// your content in a box, this function will print the content passed to it
    /// with no modifications.
    ///
    /// # Example
    ///
    /// ```
    /// use billboard::Billboard;
    ///
    /// Billboard::default().print("Hello, World!\nNew lines can be created with the newline separator :).");
    /// ```
    pub fn print(&self, content: impl Display) {
        println!("{}", self.enclose(content));
    }

    /// Prints your `Billboard`ed content to `stderr`
    /// If your content is long, separate it with line breaks (`\n`).
    ///
    /// If the user's terminal is too small, or something goes wrong with displaying
    /// your content in a box, this function will print the content passed to it
    /// with no modifications.
    ///
    /// # Example
    ///
    /// ```
    /// use billboard::Billboard;
    ///
    /// Billboard::default().eprint("Hello, World!\nNew lines can be created with the newline separator :).");
    /// ```
    pub fn eprint(&self, content: impl Display) {
        println!("{}", self.enclose(content));
    }

    /// Get your content in a `Billboard` as a `String`.
    ///
    /// # Example
    ///
    /// ```
    /// use billboard::Billboard;
    ///
    /// let result = Billboard::default().enclose("Hello, World!");
    /// println!("{}", result);
    /// ```
    pub fn enclose(&self, content: impl Display) -> String {
        let content = content.to_string();
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
            widest_length = cmp::max(widest_length, measure_text_width(line.as_str()));
        }
        for (i, line) in lines.clone().iter().enumerate() {
            lines[i] = pad_str(&line, widest_length, self.config.text_alignment, None).to_string();
        }
        let content_width = widest_length + self.config.padding.left + self.config.padding.right;
        let output = if let Some((max_width, _)) = term_size::dimensions() {
            if content_width > max_width {
                return content.to_string();
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
            let mut horizontal_str_width = measure_text_width(horizontal_str.as_str());
            if horizontal_str_width == 0 {
                horizontal_str.push_str(" ");
                horizontal_str_width = 1;
            }
            let mut horizontal = horizontal_str.repeat(
                (content_width % horizontal_str_width) + (content_width / horizontal_str_width),
            );
            while measure_text_width(horizontal.as_str()) > content_width {
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
            if measure_text_width(vertical.as_str()) == 0 {
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
                    content_width - measure_text_width(line.as_str()) - &self.config.padding.left,
                ));
                middle.push_str(&vertical);
                middle.push_str(NEWLINE);
            }
            format!("{}{}{}", top, middle, bottom)
        } else {
            content.to_string()
        };
        output
    }
}
