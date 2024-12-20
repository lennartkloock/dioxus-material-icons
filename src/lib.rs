#![allow(non_snake_case)]
#![warn(missing_docs)]

//! # 🧬 Dioxus Material Icons
//!
//! This project provides a simple but configurable component to render Google's Material Icons in Dioxus.
//!
//! ## 🚀 How to get started
//!
//! `cargo add dioxus-material-icons`
//!
//! This project introduces two components:
//!
//! 1. [`MaterialIconStylesheet`](MaterialIconStylesheet)
//! 2. [`MaterialIcon`](MaterialIcon)
//!
//! To be able to use the [`MaterialIcon`](MaterialIcon) component anywhere in your code, you first have to include
//! a [`MaterialIconStylesheet`](MaterialIconStylesheet) component. When you want to use the default settings,
//! just add it to your app's root component like this:
//!
//! ```no_rust
//! MaterialIconStylesheet { }
//! ```
//!
//! Have a look at the [`MaterialIconStylesheet`](MaterialIconStylesheet) docs for more options like self-hosting the icon font file.
//!
//! After that you can use the `MaterialIcon` component like you would expect it:
//!
//! ```no_rust
//! MaterialIcon { name: "settings" }
//! ```
//!
//! You can additionally specify the color and size.
//!
//! ```no_rust
//! MaterialIcon {
//!     name: "settings",
//!     size: 24,
//!     color: MaterialIconColor::Light,
//! }
//! ```
//!
//! ## 💡 Examples
//!
//! - [Button Example](https://github.com/lennartkloock/dioxus-material-icons/blob/main/examples/button.rs)
//!
//! ## 🔗 Useful links
//!
//! - [Overview of all icons](https://fonts.google.com/icons?selected=Material+Icons) (including names)
//!
//! ### Alternatives
//!
//! - [dioxus-free-icons](https://crates.io/crates/dioxus-free-icons) (Support for other icon packs)
//!
//! ## 📜 License
//!
//! This software is licensed under the terms of the MIT License.
//!
//! Note: All Material Icons are licensed under the Apache License 2.0.
//!
//! &copy; 2023 Lennart Kloock

use dioxus::prelude::*;

/// Props for the [`MaterialIconStylesheet`](MaterialIconStylesheet) component
#[derive(Props, PartialEq, Clone)]
pub struct MaterialIconStylesheetProps {
    /// Variant prop for the [`MaterialIconStylesheet`](MaterialIconStylesheet) component
    ///
    /// See [`MaterialIconVariant`](MaterialIconVariant) for more information.
    #[props(default = MaterialIconVariant::Regular)]
    pub variant: MaterialIconVariant,
}

/// Variants (also called categories) of the Material Icon font
///
/// See all variants [here](https://fonts.google.com/icons?selected=Material+Icons).
#[derive(PartialEq, Clone)]
pub enum MaterialIconVariant {
    /// Regular
    ///
    /// Also called Filled.
    Regular,
    /// Outlined
    Outlined,
    /// Round
    Round,
    /// Sharp
    Sharp,
    /// Two tone
    TwoTone,
    /// Self hosted font file
    ///
    /// Provide an url to a ttf or otf file.
    /// You can download the files [here](https://github.com/google/material-design-icons/tree/master/font).
    SelfHosted(String),
}

/// Stylesheet component
///
/// This component includes the Material Icon stylesheet.
/// This is required to render all Material Icons correctly.
///
/// You can provide a variant as a prop (e.g. Round).
/// When you want to provide your own self-hosted font file,
/// please use [`MaterialIconVariant::SelfHosted`](MaterialIconVariant::SelfHosted) and pass the
/// file path or url to your .ttf or .otf file to it.
/// See the [button example](https://github.com/lennartkloock/dioxus-material-icons/blob/main/examples/button.rs).
pub fn MaterialIconStylesheet(props: MaterialIconStylesheetProps) -> Element {
    let href = match &props.variant {
        MaterialIconVariant::SelfHosted(file) => {
            return rsx!(
                style { {format!(include_str!("./self-hosted-styles.css"), file)} }
            );
        }
        MaterialIconVariant::Regular => "https://fonts.googleapis.com/icon?family=Material+Icons",
        MaterialIconVariant::Outlined => {
            "https://fonts.googleapis.com/icon?family=Material+Icons+Outlined"
        }
        MaterialIconVariant::Round => {
            "https://fonts.googleapis.com/icon?family=Material+Icons+Round"
        }
        MaterialIconVariant::Sharp => {
            "https://fonts.googleapis.com/icon?family=Material+Icons+Sharp"
        }
        MaterialIconVariant::TwoTone => {
            "https://fonts.googleapis.com/icon?family=Material+Icons+Two+Tone"
        }
    };
    rsx!(
        document::Link { href: "{href}", rel: "stylesheet" }
    )
}

/// Props for the [`MaterialIcon`](MaterialIcon) component
#[derive(Props, PartialEq, Clone)]
pub struct MaterialIconProps {
    /// Name (e.g. `home`)
    ///
    /// Browse all icons [here](https://fonts.google.com/icons?selected=Material+Icons).
    #[props(into)]
    pub name: String,
    /// Size in pixels
    ///
    /// Optional
    pub size: Option<u32>,
    /// Color
    ///
    /// Optional
    #[props(into)]
    pub color: Option<MaterialIconColor>,
}

/// Colors of Material Icons
///
/// As described [here](https://developers.google.com/fonts/docs/material_icons#styling_icons_in_material_design).
#[derive(PartialEq, Clone)]
pub enum MaterialIconColor {
    /// For using icons as black on a light background.
    Dark,
    /// For using icons as black on a light background.
    DarkInactive,
    /// For using icons as white on a dark background.
    Light,
    /// For using icons as white on a dark background.
    LightInactive,
    /// Custom color, any valid CSS color
    ///
    /// E.g.: `#0000ff` or `red`
    Custom(String),
}

impl<T: Into<String>> From<T> for MaterialIconColor {
    fn from(value: T) -> Self {
        Self::Custom(value.into())
    }
}

impl MaterialIconColor {
    /// Converts the color to its corresponding CSS color
    pub fn to_css_color(&self) -> &str {
        match self {
            MaterialIconColor::Dark => "rgba(0, 0, 0, 0.54)",
            MaterialIconColor::DarkInactive => "rgba(0, 0, 0, 0.26)",
            MaterialIconColor::Light => "rgba(255, 255, 255, 1)",
            MaterialIconColor::LightInactive => "rgba(255, 255, 255, 0.3)",
            MaterialIconColor::Custom(c) => c,
        }
    }
}

/// Create a new `MaterialIconColor::Custom` variant
pub fn IconColor<T: Into<String>>(color: T) -> MaterialIconColor {
    MaterialIconColor::Custom(color.into())
}

/// Material Icon component
///
/// This component can be used to render a Material Icon.
pub fn MaterialIcon(props: MaterialIconProps) -> Element {
    // The `font-size` attribute has to be explicitly declared as `inherit` because the stylesheet sets a default of 24px
    let css_size = props
        .size
        .map(|s| format!("{s}px"))
        .unwrap_or_else(|| "inherit".to_string());
    let css_color = props
        .color
        .as_ref()
        .map(|c| format!("color: {};", c.to_css_color()))
        .unwrap_or_default();
    rsx!(
        span {
            class: "material-icons material-icons-outlined material-icons-round material-icons-sharp material-icons-two-tone md-48",
            style: "font-size: {css_size}; {css_color} user-select: none;",
            {props.name}
        }
    )
}
