//! Dioxus components for [heroicons](https://heroicons.com/)
//!
//! This library provides two components. The [`Icon`] component produces the SVG for a heroicon. The
//! [`IconButton`] component wraps the icon with an HTML `button`.
//!
//! In your own components, you can call them like this:
//!
//! ```rust
//! use dioxus::prelude::*;
//! use dioxus_heroicons::{Icon, IconButton, solid::Shape};
//!
//! #[derive(Props, PartialEq, Clone)]
//! struct DeleteButtonProps {
//!     foo: u8,
//! }
//!
//! fn DeleteButton(props: DeleteButtonProps) -> Element {
//!     let onclick = move |evt| {
//!         // Delete a thing
//!     };
//!     let disabled = if props.foo < 42 { true } else { false };
//!     rsx! {
//!         IconButton {
//!             onclick: onclick,
//!             class: "some-css-class",
//!             title: "Delete it",
//!             disabled: disabled,
//!             size: 30,
//!             icon: Shape::Trash,
//!         }
//!     }
//! }
//!
//! fn PointsRight() -> Element {
//!     rsx! {
//!         Icon {
//!             icon: Shape::ArrowRight,
//!             fill: "blue",
//!         }
//!     }
//! }
//! ```
//!
//! Check out <https://jkelleyrtp.github.io/icon-chooser/> for an icon chooser that shows you all the
//! solid icons and lets you copy the relevant component code to the clipboard.

/// This module contains all the mini icon shapes.
pub mod mini;
/// This module contains all the outline icon shapes.
pub mod outline;
/// This module contains all the solid icon shapes.
pub mod solid;

use dioxus::{events::MouseEvent, prelude::*};

const DISABLED_FILL_COLOR: &str = "#9CA3AF";

/// This trait is used to abstract the icon shape so you can use shapes from the [`outline`] or
/// [`solid`] modules for any property that accepts a shape.
pub trait IconShape: Clone + PartialEq + std::fmt::Debug {
    fn view_box(&self) -> &str;
    #[allow(clippy::missing_errors_doc)]
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
    /// If this is true then the button's `disabled` attribute will be true, and this will be passed
    /// to the `Icon` when it is rendered.
    #[props(default = false)]
    /// If this is true then the button's `disabled` attribute will be true, and this will be passed
    /// to the `Icon` when it is rendered.
    #[props(default = false)]
    pub disabled: bool,
    /// The fill color to use when `disabled` is true. This is only relevant for solid icons. This
    /// defaults to "#9CA3AF", which is "coolGray 400" from tailwindcss.
    #[props(default = DISABLED_FILL_COLOR.to_string())]
    pub disabled_fill: String,
    /// The icon shape to use.
    pub icon: S,
    /// An optional class for the `<span>` that is part of this component.
    #[props(default, strip_option)]
    pub span_class: Option<String>,
    /// An optional class that will be passed to the [`Icon`].
    #[props(default, strip_option)]
    pub icon_class: Option<String>,
    /// These are the child elements of the `IconButton` component.
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
/// See the [`IconButtonProps`] field documentation for details on the properties it accepts.
///
/// Passing children is optional. This is there so you can add some additional text or other HTML
/// to the button.
#[allow(clippy::missing_errors_doc, non_snake_case)]
#[component]
pub fn IconButton<S: IconShape>(props: IconButtonProps<S>) -> Element {
    let disabled = props.disabled;
    let onclick = props.onclick;
    rsx! {
        button {
            onclick: move |evt| if !disabled {
                if let Some(oc) = onclick {
                    oc.call(evt);
                }
            },
            class: if let Some(class) = props.class { class },
            title: if let Some(title) = props.title { title },
            disabled: disabled,
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
            if props.children != VNode::empty() {
                span {
                    class: if let Some(span_class) = props.span_class { span_class },
                    { props.children }
                },
            }
        },
    }
}

/// The properties for the [`Icon`] component.
#[derive(Clone, PartialEq, Props)]
pub struct IconProps<S: IconShape + 'static> {
    /// An optional class for the `<svg>` element.
    #[props(default)]
    pub class: Option<String>,
    /// The size of the `<svg>` element. All the heroicons are square, so this will be turned into
    /// the `height` and `width` attributes for the `<svg>`. Defaults to 20.
    #[props(default = 20)]
    pub size: u32,
    /// The color to use for filling the icon. This is only relevant for solid icons. Defaults to
    /// "currentColor".
    #[props(default = "currentColor".to_string())]
    pub fill: String,
    /// The icon shape to use.
    pub icon: S,
    /// If this is true then the fill color will be the one set in
    /// `disabled_fill` instead of `fill`.
    #[props(default = false)]
    pub disabled: bool,
    /// The fill color to use when `disabled` is true. This is only relevant for solid icons. This
    /// defaults to "#9CA3AF", which is "coolGray 400" from tailwindcss.
    #[props(default = DISABLED_FILL_COLOR.to_string())]
    pub disabled_fill: String,
}

/// Renders an `<svg>` element for a heroicon.
///
/// See the [`IconProps`] field documentation for details on the properties it accepts.
#[allow(clippy::missing_errors_doc, non_snake_case)]
#[component]
pub fn Icon<S: IconShape>(props: IconProps<S>) -> Element {
    let fill = if props.disabled {
        props.disabled_fill
    } else {
        props.fill
    };
    rsx! {
        svg {
            class: if let Some(class) = props.class { class },
            height: format_args!("{}", props.size),
            width: format_args!("{}", props.size),
            view_box: format_args!("{}", props.icon.view_box()),
            fill: "{fill}",
            { props.icon.path() }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use dioxus_ssr;
    use html_compare_rs::assert_html_eq;

    #[test]
    fn icon_default() {
        assert_rsx_eq(
            rsx! {
                Icon {
                    icon: outline::Shape::ArrowLeft,
                },
            },
            rsx! {
                svg {
                    height: 20,
                    width: 20,
                    view_box: outline::VIEW_BOX,
                    fill: "currentColor",
                    { outline::Shape::ArrowLeft.path() },
                },
            },
        );
    }

    #[test]
    fn icon_class() {
        assert_rsx_eq(
            rsx! {
                Icon {
                    icon: outline::Shape::ArrowLeft,
                    class: "foo",
                },
            },
            rsx! {
                svg {
                    class: "foo",
                    height: 20,
                    width: 20,
                    view_box: outline::VIEW_BOX,
                    fill: "currentColor",
                    { outline::Shape::ArrowLeft.path() },
                },
            },
        );
    }

    #[test]
    fn icon_disabled() {
        assert_rsx_eq(
            rsx! {
                Icon {
                    icon: outline::Shape::ArrowLeft,
                    disabled: true,
                },
            },
            rsx! {
                svg {
                    height: 20,
                    width: 20,
                    view_box: outline::VIEW_BOX,
                    fill: DISABLED_FILL_COLOR,
                    { outline::Shape::ArrowLeft.path() },
                },
            },
        );
    }

    #[test]
    fn icon_button_default() {
        assert_rsx_eq(
            rsx! {
                IconButton {
                    icon: outline::Shape::ArrowLeft,
                },
            },
            rsx! {
                button {
                    svg {
                        height: 20,
                        width: 20,
                        view_box: outline::VIEW_BOX,
                        fill: "currentColor",
                        {
                            outline::Shape::ArrowLeft.path()
                        },
                    },
                },
            },
        );
    }

    #[test]
    fn icon_button_with_span_children() {
        assert_rsx_eq(
            rsx! {
                IconButton {
                    icon: outline::Shape::ArrowLeft,
                    b {
                        "button text"
                    },
                },
            },
            rsx! {
                button {
                    svg {
                        height: 20,
                        width: 20,
                        view_box: outline::VIEW_BOX,
                        fill: "currentColor",
                        {
                            outline::Shape::ArrowLeft.path()
                        },
                    },
                    span {
                        b {
                            "button text"
                        }
                    },
                },
            },
        );
    }

    #[test]
    fn icon_button_with_props() {
        assert_rsx_eq(
            rsx! {
                IconButton {
                    class: "some-button",
                    icon: outline::Shape::ArrowLeft,
                    title: "Foo",
                },
            },
            rsx! {
                button {
                    class: "some-button",
                    title: "Foo",
                    svg {
                        height: 20,
                        width: 20,
                        view_box: outline::VIEW_BOX,
                        fill: "currentColor",
                        {
                            outline::Shape::ArrowLeft.path()
                        },
                    },
                },
            },
        );
    }

    #[test]
    fn icon_button_disabled() {
        assert_rsx_eq(
            rsx! {
                IconButton {
                    icon: outline::Shape::ArrowLeft,
                    disabled: true,
                },
            },
            rsx! {
                button {
                    disabled: true,
                    svg {
                        height: 20,
                        width: 20,
                        view_box: outline::VIEW_BOX,
                        fill: DISABLED_FILL_COLOR,
                        {
                            outline::Shape::ArrowLeft.path()
                        },
                    },
                },
            },
        );
    }

    fn assert_rsx_eq(first: Element, second: Element) {
        let first = dioxus_ssr::render_element(first);
        let second = dioxus_ssr::render_element(second);
        assert_html_eq!(first, second);
    }
}
