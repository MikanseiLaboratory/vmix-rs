use crate::commands::InputNumber;

#[derive(Debug)]
pub enum ActivatorsData {
    Input(InputNumber, bool),
    InputMix2(InputNumber, bool),
    InputMix3(InputNumber, bool),
    InputMix4(InputNumber, bool),
    InputMix5(InputNumber, bool),
    InputMix6(InputNumber, bool),
    InputMix7(InputNumber, bool),
    InputMix8(InputNumber, bool),
    InputMix9(InputNumber, bool),
    InputMix10(InputNumber, bool),
    InputMix11(InputNumber, bool),
    InputMix12(InputNumber, bool),
    InputMix13(InputNumber, bool),
    InputMix14(InputNumber, bool),
    InputMix15(InputNumber, bool),
    InputMix16(InputNumber, bool),
    InputPreview(InputNumber, bool),
    InputPreviewMix2(InputNumber, bool),
    InputPreviewMix3(InputNumber, bool),
    InputPreviewMix4(InputNumber, bool),
    InputPreviewMix5(InputNumber, bool),
    InputPreviewMix6(InputNumber, bool),
    InputPreviewMix7(InputNumber, bool),
    InputPreviewMix8(InputNumber, bool),
    InputPreviewMix9(InputNumber, bool),
    InputPreviewMix10(InputNumber, bool),
    InputPreviewMix11(InputNumber, bool),
    InputPreviewMix12(InputNumber, bool),
    InputPreviewMix13(InputNumber, bool),
    InputPreviewMix14(InputNumber, bool),
    InputPreviewMix15(InputNumber, bool),
    InputPreviewMix16(InputNumber, bool),
    InputPlaying(InputNumber, bool),
    InputVolume(InputNumber, f32),
    InputHeadphones(InputNumber, f32),
    MasterVolume(f32),
    MasterHeadphones(f32),
    BusAVolume(f32),
    BusBVolume(f32),
    BusCVolume(f32),
    BusDVolume(f32),
    BusEVolume(f32),
    BusFVolume(f32),
    BusGVolume(f32),
    InputAudio(InputNumber, bool),
    InputSolo(InputNumber, bool),
    InputBusAAudio(InputNumber, bool),
    InputBusBAudio(InputNumber, bool),
    InputBusCAudio(InputNumber, bool),
    InputBusDAudio(InputNumber, bool),
    InputBusEAudio(InputNumber, bool),
    InputBusFAudio(InputNumber, bool),
    InputBusGAudio(InputNumber, bool),
    InputMasterAudio(InputNumber, bool),
    MasterAudio(bool),
    BusAAudio(bool),
    BusBAudio(bool),
    BusCAudio(bool),
    BusDAudio(bool),
    BusEAudio(bool),
    BusFAudio(bool),
    BusGAudio(bool),
    BusASolo(bool),
    BusBSolo(bool),
    BusCSolo(bool),
    BusDSolo(bool),
    BusESolo(bool),
    BusFSolo(bool),
    BusGSolo(bool),
    FadeToBlack(bool),
    Recording(bool),
    Streaming(bool),
    External(bool),
    Fullscreen(bool),
    Overlay1(InputNumber, bool),
    Overlay2(InputNumber, bool),
    Overlay3(InputNumber, bool),
    Overlay4(InputNumber, bool),
    Overlay5(InputNumber, bool),
    Overlay6(InputNumber, bool),
    Overlay7(InputNumber, bool),
    Overlay8(InputNumber, bool),
    ReplayPlaying(bool),
    /// Unknown activator type with raw data
    Unknown(Vec<String>),
}

// ActivatorsDataはマルチスレッド環境で安全に使用できる
unsafe impl Send for ActivatorsData {}
unsafe impl Sync for ActivatorsData {}

// Helper functions to extract common functionality and handle errors safely
fn parse_input_number(value: &str) -> InputNumber {
    value.parse::<InputNumber>().unwrap_or(0)
}

fn parse_float(value: &str) -> f32 {
    value.parse::<f32>().unwrap_or(0.0)
}

fn is_active(value: &str) -> bool {
    value == "1"
}

// Higher-level helper functions for common patterns
fn create_input_bool_variant(
    values: &[String],
    idx: usize,
) -> Result<ActivatorsData, anyhow::Error> {
    if values.len() <= idx + 1 {
        // Not enough values, store as Unknown
        return Ok(ActivatorsData::Unknown(values.to_vec()));
    }

    let input_num = parse_input_number(&values[idx]);
    let is_active_val = if values.len() > idx + 1 {
        is_active(&values[idx + 1])
    } else {
        false
    };

    match values[0].as_str() {
        "Input" => Ok(ActivatorsData::Input(input_num, is_active_val)),
        "InputMix2" => Ok(ActivatorsData::InputMix2(input_num, is_active_val)),
        "InputMix3" => Ok(ActivatorsData::InputMix3(input_num, is_active_val)),
        "InputMix4" => Ok(ActivatorsData::InputMix4(input_num, is_active_val)),
        "InputMix5" => Ok(ActivatorsData::InputMix5(input_num, is_active_val)),
        "InputMix6" => Ok(ActivatorsData::InputMix6(input_num, is_active_val)),
        "InputMix7" => Ok(ActivatorsData::InputMix7(input_num, is_active_val)),
        "InputMix8" => Ok(ActivatorsData::InputMix8(input_num, is_active_val)),
        "InputMix9" => Ok(ActivatorsData::InputMix9(input_num, is_active_val)),
        "InputMix10" => Ok(ActivatorsData::InputMix10(input_num, is_active_val)),
        "InputMix11" => Ok(ActivatorsData::InputMix11(input_num, is_active_val)),
        "InputMix12" => Ok(ActivatorsData::InputMix12(input_num, is_active_val)),
        "InputMix13" => Ok(ActivatorsData::InputMix13(input_num, is_active_val)),
        "InputMix14" => Ok(ActivatorsData::InputMix14(input_num, is_active_val)),
        "InputMix15" => Ok(ActivatorsData::InputMix15(input_num, is_active_val)),
        "InputMix16" => Ok(ActivatorsData::InputMix16(input_num, is_active_val)),
        "InputPreview" => Ok(ActivatorsData::InputPreview(input_num, is_active_val)),
        "InputPreviewMix2" => Ok(ActivatorsData::InputPreviewMix2(input_num, is_active_val)),
        "InputPreviewMix3" => Ok(ActivatorsData::InputPreviewMix3(input_num, is_active_val)),
        "InputPreviewMix4" => Ok(ActivatorsData::InputPreviewMix4(input_num, is_active_val)),
        "InputPreviewMix5" => Ok(ActivatorsData::InputPreviewMix5(input_num, is_active_val)),
        "InputPreviewMix6" => Ok(ActivatorsData::InputPreviewMix6(input_num, is_active_val)),
        "InputPreviewMix7" => Ok(ActivatorsData::InputPreviewMix7(input_num, is_active_val)),
        "InputPreviewMix8" => Ok(ActivatorsData::InputPreviewMix8(input_num, is_active_val)),
        "InputPreviewMix9" => Ok(ActivatorsData::InputPreviewMix9(input_num, is_active_val)),
        "InputPreviewMix10" => Ok(ActivatorsData::InputPreviewMix10(input_num, is_active_val)),
        "InputPreviewMix11" => Ok(ActivatorsData::InputPreviewMix11(input_num, is_active_val)),
        "InputPreviewMix12" => Ok(ActivatorsData::InputPreviewMix12(input_num, is_active_val)),
        "InputPreviewMix13" => Ok(ActivatorsData::InputPreviewMix13(input_num, is_active_val)),
        "InputPreviewMix14" => Ok(ActivatorsData::InputPreviewMix14(input_num, is_active_val)),
        "InputPreviewMix15" => Ok(ActivatorsData::InputPreviewMix15(input_num, is_active_val)),
        "InputPreviewMix16" => Ok(ActivatorsData::InputPreviewMix16(input_num, is_active_val)),
        "InputPlaying" => Ok(ActivatorsData::InputPlaying(input_num, is_active_val)),
        "InputAudio" => Ok(ActivatorsData::InputAudio(input_num, is_active_val)),
        "InputSolo" => Ok(ActivatorsData::InputSolo(input_num, is_active_val)),
        "InputBusAAudio" => Ok(ActivatorsData::InputBusAAudio(input_num, is_active_val)),
        "InputBusBAudio" => Ok(ActivatorsData::InputBusBAudio(input_num, is_active_val)),
        "InputBusCAudio" => Ok(ActivatorsData::InputBusCAudio(input_num, is_active_val)),
        "InputBusDAudio" => Ok(ActivatorsData::InputBusDAudio(input_num, is_active_val)),
        "InputBusEAudio" => Ok(ActivatorsData::InputBusEAudio(input_num, is_active_val)),
        "InputBusFAudio" => Ok(ActivatorsData::InputBusFAudio(input_num, is_active_val)),
        "InputBusGAudio" => Ok(ActivatorsData::InputBusGAudio(input_num, is_active_val)),
        "InputMasterAudio" => Ok(ActivatorsData::InputMasterAudio(input_num, is_active_val)),
        "Overlay1" => Ok(ActivatorsData::Overlay1(input_num, is_active_val)),
        "Overlay2" => Ok(ActivatorsData::Overlay2(input_num, is_active_val)),
        "Overlay3" => Ok(ActivatorsData::Overlay3(input_num, is_active_val)),
        "Overlay4" => Ok(ActivatorsData::Overlay4(input_num, is_active_val)),
        "Overlay5" => Ok(ActivatorsData::Overlay5(input_num, is_active_val)),
        "Overlay6" => Ok(ActivatorsData::Overlay6(input_num, is_active_val)),
        "Overlay7" => Ok(ActivatorsData::Overlay7(input_num, is_active_val)),
        "Overlay8" => Ok(ActivatorsData::Overlay8(input_num, is_active_val)),
        _ => Ok(ActivatorsData::Unknown(values.to_vec())),
    }
}

fn create_input_float_variant(
    values: &[String],
    idx: usize,
) -> Result<ActivatorsData, anyhow::Error> {
    if values.len() <= idx + 1 {
        // Not enough values, store as Unknown
        return Ok(ActivatorsData::Unknown(values.to_vec()));
    }

    let input_num = parse_input_number(&values[idx]);
    let volume = if values.len() > idx + 1 {
        parse_float(&values[idx + 1])
    } else {
        0.0
    };

    match values[0].as_str() {
        "InputVolume" => Ok(ActivatorsData::InputVolume(input_num, volume)),
        "InputHeadphones" => Ok(ActivatorsData::InputHeadphones(input_num, volume)),
        _ => Ok(ActivatorsData::Unknown(values.to_vec())),
    }
}

fn create_single_float_variant(
    values: &[String],
    idx: usize,
) -> Result<ActivatorsData, anyhow::Error> {
    if values.len() <= idx {
        // Not enough values, store as Unknown
        return Ok(ActivatorsData::Unknown(values.to_vec()));
    }

    let volume = parse_float(&values[idx]);

    match values[0].as_str() {
        "MasterVolume" => Ok(ActivatorsData::MasterVolume(volume)),
        "MasterHeadphones" => Ok(ActivatorsData::MasterHeadphones(volume)),
        "BusAVolume" => Ok(ActivatorsData::BusAVolume(volume)),
        "BusBVolume" => Ok(ActivatorsData::BusBVolume(volume)),
        "BusCVolume" => Ok(ActivatorsData::BusCVolume(volume)),
        "BusDVolume" => Ok(ActivatorsData::BusDVolume(volume)),
        "BusEVolume" => Ok(ActivatorsData::BusEVolume(volume)),
        "BusFVolume" => Ok(ActivatorsData::BusFVolume(volume)),
        "BusGVolume" => Ok(ActivatorsData::BusGVolume(volume)),
        _ => Ok(ActivatorsData::Unknown(values.to_vec())),
    }
}

fn create_single_bool_variant(
    values: &[String],
    idx: usize,
) -> Result<ActivatorsData, anyhow::Error> {
    if values.len() <= idx {
        // Not enough values, store as Unknown
        return Ok(ActivatorsData::Unknown(values.to_vec()));
    }

    let is_active_val = is_active(&values[idx]);

    match values[0].as_str() {
        "MasterAudio" => Ok(ActivatorsData::MasterAudio(is_active_val)),
        "BusAAudio" => Ok(ActivatorsData::BusAAudio(is_active_val)),
        "BusBAudio" => Ok(ActivatorsData::BusBAudio(is_active_val)),
        "BusCAudio" => Ok(ActivatorsData::BusCAudio(is_active_val)),
        "BusDAudio" => Ok(ActivatorsData::BusDAudio(is_active_val)),
        "BusEAudio" => Ok(ActivatorsData::BusEAudio(is_active_val)),
        "BusFAudio" => Ok(ActivatorsData::BusFAudio(is_active_val)),
        "BusGAudio" => Ok(ActivatorsData::BusGAudio(is_active_val)),
        "BusASolo" => Ok(ActivatorsData::BusASolo(is_active_val)),
        "BusBSolo" => Ok(ActivatorsData::BusBSolo(is_active_val)),
        "BusCSolo" => Ok(ActivatorsData::BusCSolo(is_active_val)),
        "BusDSolo" => Ok(ActivatorsData::BusDSolo(is_active_val)),
        "BusESolo" => Ok(ActivatorsData::BusESolo(is_active_val)),
        "BusFSolo" => Ok(ActivatorsData::BusFSolo(is_active_val)),
        "BusGSolo" => Ok(ActivatorsData::BusGSolo(is_active_val)),
        "FadeToBlack" => Ok(ActivatorsData::FadeToBlack(is_active_val)),
        "Recording" => Ok(ActivatorsData::Recording(is_active_val)),
        "Streaming" => Ok(ActivatorsData::Streaming(is_active_val)),
        "External" => Ok(ActivatorsData::External(is_active_val)),
        "Fullscreen" => Ok(ActivatorsData::Fullscreen(is_active_val)),
        "ReplayPlaying" => Ok(ActivatorsData::ReplayPlaying(is_active_val)),
        _ => Ok(ActivatorsData::Unknown(values.to_vec())),
    }
}

impl TryFrom<&[String]> for ActivatorsData {
    type Error = anyhow::Error;
    fn try_from(value: &[String]) -> Result<Self, Self::Error> {
        if value.is_empty() {
            // Empty value array, store as Unknown
            return Ok(ActivatorsData::Unknown(value.to_vec()));
        }

        let activator_type = value[0].as_str();

        // Group activators by their parameter patterns
        match activator_type {
            // Input with boolean (InputNumber, bool)
            "Input" | "InputMix2" | "InputMix3" | "InputMix4" | "InputMix5" | "InputMix6"
            | "InputMix7" | "InputMix8" | "InputMix9" | "InputMix10" | "InputMix11"
            | "InputMix12" | "InputMix13" | "InputMix14" | "InputMix15" | "InputMix16"
            | "InputPreview" | "InputPreviewMix2" | "InputPreviewMix3" | "InputPreviewMix4"
            | "InputPreviewMix5" | "InputPreviewMix6" | "InputPreviewMix7" | "InputPreviewMix8"
            | "InputPreviewMix9" | "InputPreviewMix10" | "InputPreviewMix11"
            | "InputPreviewMix12" | "InputPreviewMix13" | "InputPreviewMix14"
            | "InputPreviewMix15" | "InputPreviewMix16" | "InputPlaying" | "InputAudio"
            | "InputSolo" | "InputBusAAudio" | "InputBusBAudio" | "InputBusCAudio"
            | "InputBusDAudio" | "InputBusEAudio" | "InputBusFAudio" | "InputBusGAudio"
            | "InputMasterAudio" | "Overlay1" | "Overlay2" | "Overlay3" | "Overlay4"
            | "Overlay5" | "Overlay6" | "Overlay7" | "Overlay8" => {
                create_input_bool_variant(value, 1)
            }

            // Input with float (InputNumber, f32)
            "InputVolume" | "InputHeadphones" => create_input_float_variant(value, 1),

            // Single float (f32)
            "MasterVolume" | "MasterHeadphones" | "BusAVolume" | "BusBVolume" | "BusCVolume"
            | "BusDVolume" | "BusEVolume" | "BusFVolume" | "BusGVolume" => {
                create_single_float_variant(value, 1)
            }

            // Single boolean (bool)
            "MasterAudio" | "BusAAudio" | "BusBAudio" | "BusCAudio" | "BusDAudio" | "BusEAudio"
            | "BusFAudio" | "BusGAudio" | "BusASolo" | "BusBSolo" | "BusCSolo" | "BusDSolo"
            | "BusESolo" | "BusFSolo" | "BusGSolo" | "FadeToBlack" | "Recording" | "Streaming"
            | "External" | "Fullscreen" | "ReplayPlaying" => create_single_bool_variant(value, 1),

            _ => {
                // Store unknown activators with raw data instead of failing
                Ok(ActivatorsData::Unknown(value.to_vec()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(parts: &[&str]) -> ActivatorsData {
        let values: Vec<String> = parts.iter().map(|s| s.to_string()).collect();
        ActivatorsData::try_from(values.as_slice()).unwrap()
    }

    #[test]
    fn parses_overlay1_through_8() {
        for (name, active) in [
            ("Overlay1", true),
            ("Overlay2", false),
            ("Overlay3", true),
            ("Overlay4", false),
            ("Overlay5", true),
            ("Overlay6", false),
            ("Overlay7", true),
            ("Overlay8", false),
        ] {
            let data = parse(&[name, "2", if active { "1" } else { "0" }]);
            match (name, data) {
                ("Overlay1", ActivatorsData::Overlay1(2, a)) => assert_eq!(a, active),
                ("Overlay2", ActivatorsData::Overlay2(2, a)) => assert_eq!(a, active),
                ("Overlay3", ActivatorsData::Overlay3(2, a)) => assert_eq!(a, active),
                ("Overlay4", ActivatorsData::Overlay4(2, a)) => assert_eq!(a, active),
                ("Overlay5", ActivatorsData::Overlay5(2, a)) => assert_eq!(a, active),
                ("Overlay6", ActivatorsData::Overlay6(2, a)) => assert_eq!(a, active),
                ("Overlay7", ActivatorsData::Overlay7(2, a)) => assert_eq!(a, active),
                ("Overlay8", ActivatorsData::Overlay8(2, a)) => assert_eq!(a, active),
                other => panic!("unexpected parse for {name}: {other:?}"),
            }
        }
    }
}
