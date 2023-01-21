#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(Props, PartialEq)]
pub struct MaterialIconStylesheetProps {
    variation: MaterialIconVariation,
}

#[derive(PartialEq)]
pub enum MaterialIconVariation {
    Regular,
    Outlined,
    Round,
    Sharp,
    TwoTone,
}

pub fn MaterialIconStylesheet(cx: Scope<MaterialIconStylesheetProps>) -> Element {
    let href = match &cx.props.variation {
        MaterialIconVariation::Regular => "https://fonts.googleapis.com/icon?family=Material+Icons",
        MaterialIconVariation::Outlined => "https://fonts.googleapis.com/icon?family=Material+Icons+Outlined",
        MaterialIconVariation::Round => "https://fonts.googleapis.com/icon?family=Material+Icons+Round",
        MaterialIconVariation::Sharp => "https://fonts.googleapis.com/icon?family=Material+Icons+Sharp",
        MaterialIconVariation::TwoTone => "https://fonts.googleapis.com/icon?family=Material+Icons+Two+Tone",
    };
    cx.render(rsx!(
        link { href: "{href}", rel: "stylesheet" }
    ))
}

#[derive(Props, PartialEq)]
pub struct MaterialIconProps<'a> {
    name: &'a str,
    #[props(default = 24)]
    size: u32,
    #[props(default = MaterialIconColor::Dark)]
    color: MaterialIconColor<'a>,
}

#[derive(PartialEq)]
pub enum MaterialIconColor<'a> {
    Dark,
    DarkInactive,
    Light,
    LightInactive,
    Custom(&'a str),
}

impl<'a> From<&'a str> for MaterialIconColor<'a> {
    fn from(value: &'a str) -> Self {
        Self::Custom(value)
    }
}

pub fn MaterialIcon<'a>(cx: Scope<'a, MaterialIconProps<'a>>) -> Element<'a> {
    let color = match cx.props.color {
        MaterialIconColor::Dark => "rgba(0, 0, 0, 0.54)",
        MaterialIconColor::DarkInactive => "rgba(0, 0, 0, 0.26)",
        MaterialIconColor::Light => "rgba(255, 255, 255, 1)",
        MaterialIconColor::LightInactive => "rgba(255, 255, 255, 0.3)",
        MaterialIconColor::Custom(c) => c,
    };
    cx.render(rsx!(
        span {
            class: "material-icons material-icons-outlined material-icons-round material-icons-sharp material-icons-two-tone md-48",
            style: "font-size: {cx.props.size}px; color: {color}",
            cx.props.name
        }
    ))
}
