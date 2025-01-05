use clap::Parser;
use heck::ToUpperCamelCase;
use itertools::Itertools;
use scraper::{Html, Selector};
use std::{fs, path::PathBuf, process::Command};
use walkdir::WalkDir;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the heroicons repo
    #[clap(long)]
    heroicons: PathBuf,
    /// Path to the directory where files will be written
    #[clap(short, long)]
    to: PathBuf,
}

#[derive(Debug)]
struct Icon {
    name: String,
    viewbox: String,
    path: String,
    clip_rule: Option<String>,
    fill_rule: Option<String>,
}

fn style_to_dir(style: &str) -> &str {
    match style {
        "outline" => "24/outline",
        "solid" => "24/solid",
        "mini" => "20/solid",
        _ => panic!("unknown style"),
    }
}

fn main() {
    let args = Args::parse();

    let mut src_dir = args.heroicons.clone();
    src_dir.push("src");

    for style in &["outline", "solid", "mini"] {
        let mut src_dir = src_dir.clone();
        src_dir.push(style_to_dir(style));

        let icons = make_icons(&src_dir);

        let mut to = args.to.clone();
        to.push(&format!("{}.rs", style));

        write_icons_file(&icons, &to);
    }
}

fn make_icons(src_dir: &PathBuf) -> Vec<Icon> {
    let mut icons: Vec<Icon> = vec![];

    let svg_sel = Selector::parse("svg").unwrap();
    let path_sel = Selector::parse("path").unwrap();
    for entry in WalkDir::new(src_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file() && e.file_name().to_string_lossy().ends_with(".svg"))
        .sorted_by(|a, b| Ord::cmp(a.file_name(), b.file_name()))
    {
        let name = entry
            .file_name()
            .to_str()
            .unwrap()
            .trim_end_matches(".svg")
            .to_upper_camel_case();

        let content = fs::read_to_string(entry.path()).unwrap();
        let frag = Html::parse_fragment(&content);
        let svg = frag.select(&svg_sel).next().unwrap();

        icons.push(Icon {
            name,
            viewbox: svg.value().attr("viewBox").unwrap().to_string(),
            path: svg
                .select(&path_sel)
                .map(|e| e.value().attr("d").unwrap().to_string())
                .collect::<Vec<_>>()
                .join(" "),
            clip_rule: svg
                .select(&path_sel)
                .find_map(|e| e.value().attr("clip-rule"))
                .map(|r| r.to_string()),
            fill_rule: svg
                .select(&path_sel)
                .find_map(|e| e.value().attr("fill-rule"))
                .map(|r| r.to_string()),
        });
    }

    icons
}

const TEMPLATE: &str = r#"
use dioxus::prelude::*;

pub(crate) const VIEW_BOX: &str = "{VIEWBOX}";

/// All available icon shapes
///
/// See the enum variants for the shape names. These names are always the
/// CamelCase version of the original heroicon name. So for example,
/// "arrow-narrow-left" becomes `ArrowNarrowLeft`.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Shape {
    {NAMES}
}

impl crate::IconShape for Shape {
    fn view_box(&self) -> &str {
        VIEW_BOX
    }

    #[allow(clippy::too_many_lines)]
    fn path(&self) -> Element {
        match self {
            {PATHS}
        }
    }
}
"#;

const PATH_TEMPLATE: &str = r#"
Shape::{NAME} => rsx! {
    path {
        {ATTRS}
    },
},"#;

fn write_icons_file(icons: &[Icon], to: &PathBuf) {
    let names = icons
        .iter()
        .map(|i| i.name.as_str())
        .collect::<Vec<_>>()
        .join(",\n");

    let paths = icons
        .iter()
        .map(|i| {
            let attrs = &[
                attr("d", Some(i.path.as_ref()), false),
                attr("clip_rule", i.clip_rule.as_deref(), true),
                attr("fill_rule", i.fill_rule.as_deref(), true),
            ]
            .iter()
            .filter_map(|a| a.as_deref())
            .collect::<Vec<_>>()
            .join("\n");
            PATH_TEMPLATE
                .replace("{NAME}", &i.name)
                .replace("{ATTRS}", attrs)
        })
        .collect::<Vec<_>>()
        .join("");

    let code = TEMPLATE
        .replace("{VIEWBOX}", &icons[0].viewbox)
        .replace("{NAMES}", &names)
        .replace("{PATHS}", &paths);

    fs::write(to, code).unwrap();
    Command::new("rustfmt").arg(to).output().unwrap();
}

// rustfmt gets confused about indentation in rsx! blocks and will indent the
// first attribute properly, but not the following, so we have to indent all
// but the first attribute manually.
fn attr(name: &str, value: Option<&str>, indent: bool) -> Option<String> {
    value.map(|v| {
        format!(
            r#"{}{}: "{}","#,
            if indent { "        " } else { "" },
            name,
            v,
        )
    })
}
