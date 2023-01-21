#![allow(non_snake_case)]

//! # Button Example
//!
//! This example renders a material icon into a button which can be clicked to toggle the
//! color of the icon.

use dioxus::prelude::*;

use dioxus_material_icons::{MaterialIcon, MaterialIconStylesheet, MaterialIconVariant};

fn main() {
    dioxus_desktop::launch(App);
}

fn App(cx: Scope) -> Element {
    let show_face = use_state(&cx, || false);

    cx.render(rsx!(
        MaterialIconStylesheet {
            // Uses the self-hosted approach
            variant: MaterialIconVariant::SelfHosted("examples/assets/MaterialIcons-Regular.ttf")
        }
        button {
            style: "padding: 10",
            onclick: move |_| show_face.set(!show_face),
            if *show_face.get() {
                // Render material icon "home" in blue
                rsx!(MaterialIcon { name: "home", color: "blue".into() })
            } else {
                // Render material icon "home" in default color
                rsx!(MaterialIcon { name: "home" })
            }
        }
    ))
}
