#![allow(non_snake_case)]

//! # Button Example
//!
//! This example renders a material icon into a button which can be clicked to toggle the
//! color of the icon.

use dioxus::prelude::*;

use dioxus_material_icons::{
    MaterialIcon, MaterialIconColor, MaterialIconStylesheet, MaterialIconVariant,
};

fn main() {
    launch(App);
}

fn App() -> Element {
    let mut is_blue = use_signal(|| false);

    rsx!(
        link {

        }
        MaterialIconStylesheet {
            // Uses the self-hosted approach
            variant: MaterialIconVariant::Sharp
        }
        button {
            style: "padding: 10; font-size: 48px;",
            onclick: move |_| is_blue.toggle(),
            // The size prop was omitted, so both icons inherit their size from the button element above
            if is_blue() {
                // Render material icon "home" in blue
                MaterialIcon { name: "home", color: MaterialIconColor::Custom("blue".to_string()) }
            } else {
                // Render material icon "home" in default color
                MaterialIcon { name: "home" }
            }
        }
    )
}
