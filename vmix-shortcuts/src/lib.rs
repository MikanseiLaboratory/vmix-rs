//! vMix shortcut functions, embedded from the help reference.
//!
//! The committed JSON is produced by `cargo run -p vmix-shortcuts --features scrape --bin scrape`.
//! Runtime code never downloads it.

use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

/// Help revision the committed catalog was generated from.
pub const HELP_VERSION: u32 = 29;

/// Raw catalog JSON (`Name` / `Description` / `Parameters`), same shape as vmix-utility.
pub const JSON: &str = include_str!("../assets/shortcuts.json");

/// A parameter name from the reference, split on commas.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Parameter {
    Input,
    Value,
    Channel,
    Mix,
    Duration,
    Other(String),
}

impl Parameter {
    pub fn parse(raw: &str) -> Self {
        match raw.trim() {
            "Input" => Self::Input,
            "Value" => Self::Value,
            "Channel" => Self::Channel,
            "Mix" => Self::Mix,
            "Duration" => Self::Duration,
            other => Self::Other(other.to_string()),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Input => "Input",
            Self::Value => "Value",
            Self::Channel => "Channel",
            Self::Mix => "Mix",
            Self::Duration => "Duration",
            Self::Other(name) => name,
        }
    }
}

/// One shortcut function.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Shortcut {
    pub name: String,
    pub description: String,
    pub parameters: Vec<Parameter>,
}

/// JSON object written to `shortcuts.json`. Field names match vmix-utility.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RawShortcut {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Description")]
    description: String,
    #[serde(rename = "Parameters")]
    parameters: Option<Vec<String>>,
}

/// Every function in the embedded catalog, overrides first.
pub fn all() -> &'static [Shortcut] {
    catalog().as_slice()
}

/// Exact function name lookup.
pub fn find(name: &str) -> Option<&'static Shortcut> {
    all().iter().find(|shortcut| shortcut.name == name)
}

fn catalog() -> &'static Vec<Shortcut> {
    static CACHE: OnceLock<Vec<Shortcut>> = OnceLock::new();
    CACHE.get_or_init(|| decode_json(JSON).expect("embedded shortcuts.json is valid"))
}

fn decode_json(json: &str) -> Result<Vec<Shortcut>, serde_json::Error> {
    let raw: Vec<RawShortcut> = serde_json::from_str(json)?;
    Ok(raw.into_iter().map(Shortcut::from).collect())
}

impl From<RawShortcut> for Shortcut {
    fn from(raw: RawShortcut) -> Self {
        Self {
            name: raw.name,
            description: raw.description,
            parameters: raw
                .parameters
                .unwrap_or_default()
                .into_iter()
                .filter(|name| !name.trim().is_empty())
                .map(|name| Parameter::parse(&name))
                .collect(),
        }
    }
}

/// Functions missing from the official table. Matches the vmix-utility scraper.
pub fn overrides() -> Vec<RawShortcut> {
    vec![
        raw("Cut", "Cut", &["Input", "Mix"]),
        raw("Fade", "Fade", &["Input", "Mix", "Duration"]),
        raw("Merge", "Merge", &["Input", "Duration"]),
    ]
}

fn raw(name: &str, description: &str, parameters: &[&str]) -> RawShortcut {
    RawShortcut {
        name: name.to_string(),
        description: description.to_string(),
        parameters: Some(parameters.iter().map(|item| (*item).to_string()).collect()),
    }
}

/// Read the Shortcut Function Reference HTML.
///
/// Green category rows (`background-color: #ccffcc`) and the column header are skipped.
/// Override functions are prepended and win over a same-named row.
pub fn parse_reference_html(html: &str) -> Vec<RawShortcut> {
    let mut shortcuts = overrides();
    let overridden: Vec<String> = shortcuts.iter().map(|item| item.name.clone()).collect();
    for row in html_rows(html) {
        let cells = html_cells(&row);
        if cells.is_empty() {
            continue;
        }
        if cells[0].to_ascii_lowercase().contains("background-color: #ccffcc")
            || cells[0].to_ascii_lowercase().contains("background-color:#ccffcc")
        {
            continue;
        }
        let name = cell_text(&cells[0]);
        if name.is_empty() || overridden.iter().any(|item| item == &name) {
            continue;
        }
        let description = cells.get(1).map(|cell| cell_text(cell)).unwrap_or_default();
        if name == "Name" && description == "Description" {
            continue;
        }
        let parameters = cells.get(2).map(|cell| cell_text(cell)).unwrap_or_default();
        shortcuts.push(RawShortcut {
            name,
            description,
            parameters: split_parameters(&parameters),
        });
    }
    shortcuts
}

/// Serialize shortcuts the same way the committed asset is stored.
pub fn to_json(shortcuts: &[RawShortcut]) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(shortcuts)
}

fn split_parameters(raw: &str) -> Option<Vec<String>> {
    let trimmed = raw.trim();
    if trimmed.is_empty() || trimmed.eq_ignore_ascii_case("none") {
        return None;
    }
    Some(
        trimmed
            .split(',')
            .map(|part| part.trim().to_string())
            .filter(|part| !part.is_empty())
            .collect(),
    )
}

fn html_rows(html: &str) -> Vec<String> {
    let lower = html.to_ascii_lowercase();
    let bytes = lower.as_bytes();
    let mut rows = Vec::new();
    let mut search = 0;
    while let Some(start) = find_at(&lower, "<tr", search) {
        let Some(open_end) = lower[start..].find('>') else {
            break;
        };
        let content_start = start + open_end + 1;
        let Some(end_rel) = lower[content_start..].find("</tr>") else {
            break;
        };
        let content_end = content_start + end_rel;
        rows.push(html[content_start..content_end].to_string());
        let _ = bytes;
        search = content_end + 5;
    }
    rows
}

fn html_cells(row: &str) -> Vec<String> {
    let lower = row.to_ascii_lowercase();
    let mut cells = Vec::new();
    let mut search = 0;
    while let Some(start) = find_at(&lower, "<td", search) {
        let Some(open_end) = lower[start..].find('>') else {
            break;
        };
        let tag = row[start..start + open_end + 1].to_string();
        let content_start = start + open_end + 1;
        let Some(end_rel) = lower[content_start..].find("</td>") else {
            break;
        };
        let content_end = content_start + end_rel;
        cells.push(format!("{tag}{}", &row[content_start..content_end]));
        search = content_end + 5;
    }
    cells
}

fn find_at(haystack: &str, needle: &str, from: usize) -> Option<usize> {
    haystack[from..]
        .find(needle)
        .map(|offset| from + offset)
}

fn cell_text(cell: &str) -> String {
    let without_breaks = replace_ci(cell, "<br/>", "\n");
    let without_breaks = replace_ci(&without_breaks, "<br />", "\n");
    let without_breaks = replace_ci(&without_breaks, "<br>", "\n");
    let mut text = String::new();
    let mut inside = false;
    for ch in without_breaks.chars() {
        match ch {
            '<' => inside = true,
            '>' => inside = false,
            _ if !inside => text.push(ch),
            _ => {}
        }
    }
    let decoded = decode_entities(&text);
    decoded.replace(['\n', '\r'], "").trim().to_string()
}

fn replace_ci(input: &str, needle: &str, replacement: &str) -> String {
    let lower = input.to_ascii_lowercase();
    let needle = needle.to_ascii_lowercase();
    let mut out = String::new();
    let mut rest = 0;
    while let Some(found) = lower[rest..].find(&needle) {
        let at = rest + found;
        out.push_str(&input[rest..at]);
        out.push_str(replacement);
        rest = at + needle.len();
    }
    out.push_str(&input[rest..]);
    out
}

fn decode_entities(input: &str) -> String {
    let mut out = String::new();
    let mut rest = input;
    while let Some(start) = rest.find('&') {
        out.push_str(&rest[..start]);
        let after = &rest[start..];
        if let Some(end) = after.find(';') {
            let entity = &after[..=end];
            out.push_str(match entity {
                "&amp;" => "&",
                "&lt;" => "<",
                "&gt;" => ">",
                "&quot;" => "\"",
                "&nbsp;" | "&#160;" => " ",
                other => other,
            });
            rest = &after[end + 1..];
        } else {
            out.push('&');
            rest = &rest[start + 1..];
        }
    }
    out.push_str(rest);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = r#"<table>
        <tr><td><span>Name</span></td><td><span>Description</span></td><td><span>Parameters</span></td></tr>
        <tr><td style="background-color: #ccffcc;"><span>General</span></td><td></td><td></td></tr>
        <tr><td><span>Audio</span></td><td><span>Toggle Audio Mute On/Off</span></td><td><span>Input</span></td></tr>
        <tr><td><span>SetText</span></td><td><span>Change Text<br/>Value = Text</span></td><td><span>Value, Input</span></td></tr>
        <tr><td><span>FadeToBlack</span></td><td><span>Toggle FTB On/Off</span></td><td><span>None</span></td></tr>
    </table>"#;

    #[test]
    fn parser_skips_heading_rows_and_splits_parameters() {
        let rows = parse_reference_html(FIXTURE);
        assert!(rows.iter().any(|row| row.name == "Cut"));
        assert!(rows.iter().any(|row| row.name == "Fade" && row.parameters.as_ref().unwrap().len() == 3));
        assert!(!rows.iter().any(|row| row.name == "Name" || row.name == "General"));
        let audio = rows.iter().find(|row| row.name == "Audio").unwrap();
        assert_eq!(audio.parameters.as_deref(), Some(&["Input".to_string()][..]));
        let text = rows.iter().find(|row| row.name == "SetText").unwrap();
        assert!(text.description.contains("Value = Text"));
        assert!(!text.description.contains('\n'));
        let ftb = rows.iter().find(|row| row.name == "FadeToBlack").unwrap();
        assert!(ftb.parameters.is_none());
    }

    #[test]
    fn embedded_catalog_contains_help29_overlay_and_stinger_ranges() {
        assert!(all().len() > 700, "help29 catalog should be hundreds of functions");
        for name in [
            "Cut",
            "Fade",
            "Merge",
            "OverlayInput5",
            "OverlayInput6",
            "OverlayInput7",
            "OverlayInput8",
            "Stinger5",
            "Stinger6",
            "Stinger7",
            "Stinger8",
            "PreviewOverlayInput8",
        ] {
            assert!(find(name).is_some(), "missing {name}");
        }
        assert_eq!(
            find("Fade").unwrap().parameters,
            vec![Parameter::Input, Parameter::Mix, Parameter::Duration]
        );
        assert_eq!(
            find("Cut").unwrap().parameters,
            vec![Parameter::Input, Parameter::Mix]
        );
        assert_eq!(
            find("Merge").unwrap().parameters,
            vec![Parameter::Input, Parameter::Duration]
        );
        assert_eq!(
            find("OverlayInput1Off").unwrap().parameters,
            Vec::<Parameter>::new()
        );
        assert_eq!(
            find("PreviewOverlayInput1").unwrap().parameters,
            vec![Parameter::Input]
        );
        assert_eq!(HELP_VERSION, 29);
    }
}
