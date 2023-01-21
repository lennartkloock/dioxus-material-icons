# 🧬 Dioxus Material Icons

<a href="https://crates.io/crates/dioxus-material-icons">
    <img src="https://img.shields.io/crates/v/dioxus-material-icons.svg" alt="Crates.io version" />
</a>

This project provides a simple but configurable component to render Google's Material Icons in Dioxus.

## 🚀 How to get started

`cargo add dioxus-material-icons`

This project introduces two components:

1. `MaterialIconStylesheet`
2. `MaterialIcon`

To be able to use the `MaterialIcon` component anywhere in your code, you first have to include
a `MaterialIconStylesheet` component. When you want to use the default settings, just add it to your app's root
component like this:

```
MaterialIconStylesheet { }
```

Have a look at the docs for more options like self-hosting the icon font file.

After that you can use the `MaterialIcon` component like you would expect it:

```
MaterialIcon { name: "settings" }
```

You can additionally specify the color and size.

```
MaterialIcon {
    name: "settings",
    size: 24,
    color: MaterialIconColor::Light,
}
```

## 💡 Examples

- [Button Example](https://github.com/lennartkloock/dioxus-material-icons/blob/main/examples/button.rs)

## 🔗 Useful links

- [Overview of all icons](https://fonts.google.com/icons?selected=Material+Icons) (including names)

### Alternatives

- [dioxus-free-icons](https://crates.io/crates/dioxus-free-icons) (Support for other icon packs)

## 📜 License

This software is licensed under the terms of the MIT License.

Note: All Material Icons are licensed under the Apache License 2.0.

&copy; 2023 Lennart Kloock
