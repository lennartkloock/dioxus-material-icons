/*
 * @Date: 2024-07-28 03:54:25
 * @LastEditTime: 2024-07-28 17:39:06
 */

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
            style: "display:flex; flex-direction: column; align-items: center;
             padding: 10; font-size: 24px; color: inherit; background-color: transparent; border: none;",
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
