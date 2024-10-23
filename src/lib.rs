//! Dioxus components for [heroicons](https://heroicons.com/)
//!
//! This library provides two components. The [`Icon`] component produces the
//! SVG for a heroicon. The [`IconButton`] component wraps the icon with a
//! HTML `button`.
//!
//! In your own components, you can call them like this:
//!
//! ```rust
//! use dioxus::prelude::*;
//! use dioxus_heroicons::{Icon, IconButton, solid::Shape};
//!
//! #[inline_props]
//! fn DeleteButton(cx: Scope, foo: u8) -> Element {
//!     let onclick = move |evt| {
//!         // Delete a thing
//!     };
//!     let disabled = if *foo < 42 { true } else { false };
//!     cx.render(rsx! {
//!         IconButton {
//!             onclick: onclick,
//!             class: "some-css-class",
//!             title: "Delete it",
//!             disabled: disabled,
//!             size: 30,
//!             icon: Shape::Trash,
//!         }
//!     })
//! }
//!
//! fn PointsRight(cx: Scope) -> Element {
//!     cx.render(rsx! {
//!         Icon {
//!             icon: Shape::ArrowRight,
//!             fill: "blue",
//!         }
//!     })
//! }
//! ```
//!
//! Check out https://jkelleyrtp.github.io/icon-chooser/ for an icon chooser
//! that shows you all the solid icons and lets you copy the relevant
//! component code to the clipboard.

/// This module contains all the mini icon shapes.
pub mod mini;
/// This module contains all the outline icon shapes.
pub mod outline;
/// This module contains all the solid icon shapes.
pub mod solid;

use dioxus::{events::MouseEvent, prelude::*};

/// This trait is used to abstract the icon shape so you can use shapes from
/// the [`outline`] or [`solid`] modules for any property that accepts a
/// shape.
pub trait IconShape: Clone + PartialEq + std::fmt::Debug {
    fn view_box(&self) -> &str;
    fn path(&self) -> Element;
}

/// The properties for the [`IconButton`] component.
#[derive(Clone, PartialEq, Props)]
pub struct IconButtonProps<S: IconShape + 'static> {
    /// An optional onclick handler for the button.
    #[props(default, strip_option)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    #[props(default, strip_option)]
    /// An optional class for the *button itself*.
    pub class: Option<String>,
    /// An optional title for the button element.
    #[props(default, strip_option)]
    pub title: Option<String>,
    /// The size of the icon. This defaults to 20 pixels.
    #[props(default = 20)]
    pub size: u32,
    /// The fill color to use for the icon. This defaults to "currentColor".
    #[props(default = "currentColor".to_string())]
    pub fill: String,
    /// If this is true then the button's `disabled` attribute will be true,
    /// and this will be passed to the `Icon` when it is rendered.
    #[props(default = false)]
    pub disabled: bool,
    /// The fill color to use when `disabled` is true. This is only relevant
    /// for solid icons. This defaults to "#9CA3AF", which is "coolGray 400"
    /// from tailwindcss.
    #[props(default = "#9CA3AF".to_string())]
    pub disabled_fill: String,
    /// The icon shape to use.
    pub icon: S,
    /// An optional class for the `<span>` that is part of this component.
    #[props(default, strip_option)]
    pub span_class: Option<String>,
    /// An optional class that will be passed to the [`Icon`].
    #[props(default, strip_option)]
    pub icon_class: Option<String>,
    pub children: Element,
}

/// Renders a `<button>` containing an SVG icon.
///
/// This component will generate HTML like this:
///
/// ```html
/// <button>
///   <svg ...>
///   <span>
///     Child elements go here
///   </span>
/// </button>
/// ```
///
/// See the [`IconButtonProps`] field documentation for details on the
/// properties it accepts.
///
/// The child elements are optional, and are there so you can add some
/// additional text or other HTML to the button.
#[allow(non_snake_case)]
#[component]
pub fn IconButton<S: IconShape>(props: IconButtonProps<S>) -> Element {
    let disabled = props.disabled;
    let onclick = props.onclick;
    rsx! {
        button {
            onclick: move |evt| if !disabled {
                if let Some(oc) = &onclick {
                    oc.call(evt);
                }
            },
            class: format_args!("{}", props.class.unwrap_or("".to_string())),
            title: format_args!("{}", props.title.unwrap_or("".to_string())),
            disabled: format_args!("{}", if props.disabled { "true" } else { "false" }),
            Icon {
                ..IconProps {
                    class: props.icon_class,
                    size: props.size,
                    fill: props.fill,
                    icon: props.icon.clone(),
                    disabled: props.disabled,
                    disabled_fill: props.disabled_fill,
                },
            },
            span {
                class: format_args!("{}", props.span_class.unwrap_or("".to_string())),
                { &props.children }
            },
        },
    }
}

/// The properties for the [`Icon`] component.
#[derive(Clone, PartialEq, Props)]
pub struct IconProps<S: IconShape + 'static> {
    /// An optional class for the `<svg>` element.
    #[props(default)]
    pub class: Option<String>,
    /// The size of the `<svg>` element. All the heroicons are square, so this
    /// will be turned into the `height` and `width` attributes for the
    /// `<svg>`. Defaults to 20.
    #[props(default = 20)]
    pub size: u32,
    /// The color to use for filling the icon. This is only relevant for solid
    /// icons. Defaults to "currentColor".
    #[props(default = "currentColor".to_string())]
    pub fill: String,
    /// The icon shape to use.
    pub icon: S,
    /// If this is true then the fill color will be the one set in
    /// `disabled_fill` instead of `fill`.
    #[props(default = false)]
    pub disabled: bool,
    /// The fill color to use when `disabled` is true. This is only relevant
    /// for solid icons. This defaults to "#9CA3AF", which is "coolGray 400"
    /// from tailwindcss.
    #[props(default = "#9CA3AF".to_string())]
    pub disabled_fill: String,
}

/// Renders an `<svg>` element for a heroicon.
///
/// See the [`IconProps`] field documentation for details on the properties it
/// accepts.
#[allow(non_snake_case)]
#[component]
pub fn Icon<S: IconShape>(props: IconProps<S>) -> Element {
    let fill = if props.disabled {
        props.disabled_fill
    } else {
        props.fill
    };
    rsx! {
        svg {
            class: format_args!("{}", props.class.unwrap_or("".to_string())),
            height: format_args!("{}", props.size),
            width: format_args!("{}", props.size),
            view_box: format_args!("{}", props.icon.view_box()),
            fill: "{fill}",
            { props.icon.path() }
        }
    }
}
