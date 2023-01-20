#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_material_icons::{MaterialIconStylesheet, MaterialIcon};

fn main() {
    dioxus_desktop::launch(App);
}

fn App(cx: Scope) -> Element {
    let show_face = use_state(&cx, || false);

    cx.render(rsx!(
        MaterialIconStylesheet {  }
        button {
            style: "padding: 10",
            onclick: move |_| show_face.set(!show_face),
            if *show_face.get() {
                rsx!(MaterialIcon { name: "home", color: "#0000ff".into() })
            } else {
                rsx!(MaterialIcon { name: "home" })
            }
        }
    ))
}