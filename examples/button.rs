#![allow(non_snake_case)]

//! # Button Example
//!
//! This example renders a material icon into a button which can be clicked to toggle the
//! color of the icon.

use dioxus::prelude::*;
use dioxus_material_icons::{IconColor, MaterialIcon, MaterialIconStylesheet, MaterialIconVariant};

fn main() {
    dioxus::launch(App);
}

fn App() -> Element {
    let mut is_blue = use_signal(|| false);

    rsx!(
        MaterialIconStylesheet {
            // Uses the self-hosted approach
            variant: MaterialIconVariant::SelfHosted("examples/assets/MaterialIcons-Regular.woff2")
        }

        button {
            style: "padding: 1rem; font-size: 1rem; display:flex; flex-direction: column; align-items: center;",
            onclick: move |_| is_blue.set(!is_blue()),
            // The size prop was omitted, so both icons inherit their size from the button element above
            if is_blue() {
                // Render material icon "home" in blue
                MaterialIcon { name: "home", size: 48, color: IconColor("blue")  }
            } else {
                // Render material icon "home" in default color
                MaterialIcon { name: "home", size: 48 }
            },
            "Home"
        }
    )
}
