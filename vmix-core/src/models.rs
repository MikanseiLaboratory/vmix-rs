use serde::{Deserialize, Serialize};

#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Vmix {
    #[serde(rename = "version")]
    pub version: String,

    #[serde(rename = "edition")]
    pub edition: String,

    #[serde(rename = "preset", default)]
    pub preset: Option<String>,

    #[serde(rename = "inputs")]
    pub inputs: Inputs,

    #[serde(rename = "outputs", default)]
    pub outputs: Option<Outputs>,

    #[serde(rename = "overlays")]
    pub overlays: Overlays,

    #[serde(rename = "preview")]
    pub preview: String,

    #[serde(rename = "active")]
    pub active: String,

    #[serde(rename = "fadeToBlack", with = "xml_bool")]
    pub fade_to_black: bool,

    #[serde(rename = "transitions")]
    pub transitions: Transitions,

    #[serde(rename = "recording", with = "xml_bool")]
    pub recording: bool,

    #[serde(rename = "external", with = "xml_bool")]
    pub external: bool,

    #[serde(rename = "streaming", with = "xml_bool")]
    pub streaming: bool,

    #[serde(rename = "playList", with = "xml_bool")]
    pub play_list: bool,

    #[serde(rename = "multiCorder", with = "xml_bool")]
    pub multi_corder: bool,

    #[serde(rename = "fullscreen", with = "xml_bool")]
    pub fullscreen: bool,

    #[serde(rename = "mix", default)]
    pub mix: Vec<Mix>,

    #[serde(rename = "audio")]
    pub audio: Audio,

    #[serde(rename = "dynamic")]
    pub dynamic: Dynamic,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Audio {
    #[serde(rename = "master")]
    pub master: AudioBus,

    #[serde(rename = "busA", default)]
    pub bus_a: Option<AudioBus>,

    #[serde(rename = "busB", default)]
    pub bus_b: Option<AudioBus>,

    #[serde(rename = "busC", default)]
    pub bus_c: Option<AudioBus>,

    #[serde(rename = "busD", default)]
    pub bus_d: Option<AudioBus>,

    #[serde(rename = "busE", default)]
    pub bus_e: Option<AudioBus>,

    #[serde(rename = "busF", default)]
    pub bus_f: Option<AudioBus>,

    #[serde(rename = "busG", default)]
    pub bus_g: Option<AudioBus>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AudioBus {
    #[serde(rename = "@volume")]
    pub volume: f64,

    #[serde(rename = "@muted", with = "xml_bool")]
    pub muted: bool,

    #[serde(rename = "@meterF1")]
    pub meter_f1: f64,

    #[serde(rename = "@meterF2")]
    pub meter_f2: f64,

    #[serde(rename = "@headphonesVolume", default)]
    pub headphones_volume: Option<f64>,

    #[serde(rename = "@solo", default, with = "xml_bool_option")]
    pub solo: Option<bool>,

    #[serde(rename = "@sendToMaster", default, with = "xml_bool_option")]
    pub send_to_master: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Dynamic {
    #[serde(rename = "input1")]
    pub input1: String,

    #[serde(rename = "input2")]
    pub input2: String,

    #[serde(rename = "input3")]
    pub input3: String,

    #[serde(rename = "input4")]
    pub input4: String,

    #[serde(rename = "value1")]
    pub value1: String,

    #[serde(rename = "value2")]
    pub value2: String,

    #[serde(rename = "value3")]
    pub value3: String,

    #[serde(rename = "value4")]
    pub value4: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Inputs {
    #[serde(rename = "input")]
    pub input: Vec<Input>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Input {
    // 属性
    #[serde(rename = "@key")]
    pub key: String,

    #[serde(rename = "@number")]
    pub number: String,

    #[serde(rename = "@type")]
    pub input_type: String,

    #[serde(rename = "@title")]
    pub title: String,

    #[serde(rename = "@shortTitle")]
    pub short_title: String,

    #[serde(rename = "@state")]
    pub state: State,

    #[serde(rename = "@position")]
    pub position: String,

    #[serde(rename = "@duration")]
    pub duration: String,

    #[serde(rename = "@loop", with = "xml_bool")]
    pub input_loop: bool,

    #[serde(rename = "@muted", default, with = "xml_bool_option")]
    pub muted: Option<bool>,

    #[serde(rename = "@volume", default)]
    pub volume: Option<f64>,

    #[serde(rename = "@balance", default)]
    pub balance: Option<f64>,

    #[serde(rename = "@solo", default, with = "xml_bool_option")]
    pub solo: Option<bool>,

    #[serde(rename = "@soloPFL", default, with = "xml_bool_option")]
    pub solo_pfl: Option<bool>,

    #[serde(rename = "@audiobusses", default)]
    pub audiobusses: Option<String>,

    #[serde(rename = "@meterF1", default)]
    pub meter_f1: Option<f64>,

    #[serde(rename = "@meterF2", default)]
    pub meter_f2: Option<f64>,

    #[serde(rename = "@gainDb", default)]
    pub gain_db: Option<f64>,

    #[serde(rename = "@selectedIndex", default)]
    pub selected_index: Option<String>,

    #[serde(rename = "@preset", default)]
    pub preset: Option<String>,

    // 子要素
    #[serde(rename = "list", default)]
    pub list: Option<List>,

    #[serde(rename = "image", default)]
    pub image: Vec<Image>,

    #[serde(rename = "replay", default)]
    pub replay: Option<Replay>,

    #[serde(rename = "overlay", default)]
    pub overlay: Vec<InputOverlay>,

    #[serde(rename = "crop", default)]
    pub crop: Option<Crop>,

    #[serde(rename = "position", default)]
    pub input_position: Option<Position>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Image {
    #[serde(rename = "index", default)]
    pub index: Option<String>,

    #[serde(rename = "name", default)]
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct List {
    #[serde(rename = "item", default)]
    pub item: Vec<ListItem>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemClass {
    #[serde(rename = "_selected")]
    pub selected: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct InputOverlay {
    #[serde(rename = "@index")]
    pub index: String,

    #[serde(rename = "@key")]
    pub key: String,

    #[serde(rename = "position", default)]
    pub position: Option<Position>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Position {
    #[serde(rename = "@panX", default)]
    pub pan_x: Option<String>,

    #[serde(rename = "@panY", default)]
    pub pan_y: Option<String>,

    #[serde(rename = "@zoomX", default)]
    pub zoom_x: Option<String>,

    #[serde(rename = "@zoomY", default)]
    pub zoom_y: Option<String>,

    #[serde(rename = "@x", default)]
    pub x: Option<String>,

    #[serde(rename = "@y", default)]
    pub y: Option<String>,

    #[serde(rename = "@width", default)]
    pub width: Option<String>,

    #[serde(rename = "@height", default)]
    pub height: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Replay {
    // 子要素
    #[serde(rename = "timecode", default)]
    pub timecode: String,

    #[serde(rename = "timecodeA", default)]
    pub timecode_a: String,

    #[serde(rename = "timecodeB", default)]
    pub timecode_b: String,

    // 属性
    #[serde(rename = "@live", default)]
    pub live: Option<String>,

    #[serde(rename = "@recording", default)]
    pub recording: Option<String>,

    #[serde(rename = "@channelMode", default)]
    pub channel_mode: String,

    #[serde(rename = "@events", default)]
    pub events: String,

    #[serde(rename = "@eventsA", default)]
    pub events_a: String,

    #[serde(rename = "@eventsB", default)]
    pub events_b: String,

    #[serde(rename = "@cameraA", default)]
    pub camera_a: String,

    #[serde(rename = "@cameraB", default)]
    pub camera_b: String,

    #[serde(rename = "@speed", default)]
    pub speed: String,

    #[serde(rename = "@speedA", default)]
    pub speed_a: String,

    #[serde(rename = "@speedB", default)]
    pub speed_b: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Overlays {
    #[serde(rename = "overlay")]
    pub overlay: Vec<OverlaysOverlay>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct OverlaysOverlay {
    #[serde(rename = "@number")]
    pub number: String,

    #[serde(rename = "$value", default)]
    pub input: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Transitions {
    #[serde(rename = "transition")]
    pub transition: Vec<Transition>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Transition {
    #[serde(rename = "@number")]
    pub number: String,

    #[serde(rename = "@effect")]
    pub effect: String,

    #[serde(rename = "@duration")]
    pub duration: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ListItem {
    #[serde(rename = "@enabled", default)]
    pub enabled: Option<String>,
    #[serde(rename = "@selected", default)]
    pub selected: Option<String>,
    #[serde(rename = "@index", default)]
    pub index: Option<String>,
    #[serde(rename = "$value", default)]
    pub text: Option<String>,
}

// Custom deserializer for XML boolean values
mod xml_bool {
    use serde::{Deserialize, Deserializer, Serializer};

    #[cfg(not(feature = "std"))]
    use alloc::{format, string::String};

    pub fn serialize<S>(value: &bool, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = if *value { "True" } else { "False" };
        serializer.serialize_str(s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "True" => Ok(true),
            "False" => Ok(false),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid boolean value: {}",
                s
            ))),
        }
    }
}

// Custom deserializer for optional XML boolean values
mod xml_bool_option {
    use serde::{Deserialize, Deserializer, Serializer};

    #[cfg(not(feature = "std"))]
    use alloc::{format, string::String};

    pub fn serialize<S>(value: &Option<bool>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(b) => {
                let s = if *b { "True" } else { "False" };
                serializer.serialize_str(s)
            }
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt = Option::<String>::deserialize(deserializer)?;
        match opt {
            Some(s) => match s.as_str() {
                "True" => Ok(Some(true)),
                "False" => Ok(Some(false)),
                _ => Err(serde::de::Error::custom(format!(
                    "Invalid boolean value: {}",
                    s
                ))),
            },
            None => Ok(None),
        }
    }
}

// Outputs structure
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Outputs {
    #[serde(rename = "output", default)]
    pub output: Vec<Output>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Output {
    #[serde(rename = "@type")]
    pub output_type: String,
    #[serde(rename = "@number")]
    pub number: String,
    #[serde(rename = "@source")]
    pub source: String,
    #[serde(rename = "@external", default)]
    pub external: Option<String>,
    #[serde(rename = "@ndi", default)]
    pub ndi: Option<String>,
    #[serde(rename = "@mix", default)]
    pub mix: Option<String>,
    #[serde(rename = "@inputNumber", default)]
    pub input_number: Option<String>,
}

// Mix structure
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Mix {
    #[serde(rename = "@number")]
    pub number: String,
    #[serde(rename = "preview")]
    pub preview: String,
    #[serde(rename = "active")]
    pub active: String,
}

// Crop structure for inputs
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Crop {
    #[serde(rename = "@X1")]
    pub x1: String,
    #[serde(rename = "@Y1")]
    pub y1: String,
    #[serde(rename = "@X2")]
    pub x2: String,
    #[serde(rename = "@Y2")]
    pub y2: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Audiobusses {
    #[serde(rename = "M")]
    M,

    #[serde(rename = "A")]
    A,

    #[serde(rename = "B")]
    B,

    #[serde(rename = "C")]
    C,

    #[serde(rename = "D")]
    D,

    #[serde(rename = "E")]
    E,

    #[serde(rename = "F")]
    F,

    #[serde(rename = "G")]
    G,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum State {
    #[serde(rename = "Paused")]
    Paused,

    #[serde(rename = "Running")]
    Running,

    #[serde(rename = "Completed")]
    Completed,
}

#[cfg(all(test, feature = "xml"))]
mod overlay_tests {
    use super::*;
    use quick_xml::de::from_str;

    #[test]
    fn parses_overlays_1_through_8_with_active_input() {
        let xml = r#"<overlays>
            <overlay number="1" />
            <overlay number="2" />
            <overlay number="3" />
            <overlay number="4" />
            <overlay number="5">2</overlay>
            <overlay number="6" />
            <overlay number="7" />
            <overlay number="8">5</overlay>
        </overlays>"#;

        let overlays: Overlays = from_str(xml).expect("overlays should parse");
        assert_eq!(overlays.overlay.len(), 8);
        assert_eq!(overlays.overlay[0].number, "1");
        assert!(overlays.overlay[0].input.as_deref().unwrap_or("").is_empty());
        assert_eq!(overlays.overlay[4].number, "5");
        assert_eq!(overlays.overlay[4].input.as_deref(), Some("2"));
        assert_eq!(overlays.overlay[7].number, "8");
        assert_eq!(overlays.overlay[7].input.as_deref(), Some("5"));
    }
}
