/*!
Codes for the representation of names of scripts

ISO 15924, Codes for the representation of names of scripts, defines two sets of
codes for a number of writing systems (scripts). Each script is given both a
four-letter code and a numeric one. Script is defined as "set of graphic characters
used for the written form of one or more languages".

Where possible the codes are derived from ISO 639-2 where the name of a script
and the name of a language using the script are identical (example: Gujarātī ISO 639
guj, ISO 15924 Gujr). Preference is given to the 639-2 Bibliographical codes, which
is different from the otherwise often preferred use of the Terminological codes.

4-letter ISO 15924 codes are incorporated into the Language Subtag Registry for
IETF language tags and so can be used in file formats that make use of such language
tags. For example, they can be used in HTML and XML to help Web browsers determine which
typeface to use for foreign text. This way one could differentiate, for example,
between Serbian written in the Cyrillic (sr-Cyrl) or Latin (sr-Latn) script, or mark
romanized text as such.

ISO appointed the Unicode Consortium as the Registration Authority (RA) for the standard.

## Source - ISO 15924

The data used here is taken from
[ISO](https://www.unicode.org/iso15924/iso15924-codes.html).

*/

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// A representation of registered script data maintained by ISO.
#[derive(Serialize, Deserialize, Debug)]
pub struct ScriptInfo {
    /// The standard 3-character identifier for this script.
    pub alphabetic_code: String,
    /// The standard numeric identifier for this script.
    pub numeric_code: u16,
    /// The script name, in English.
    pub name: String,
    /// An optional alias for this script.
    pub alias: Option<String>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

lazy_static! {
    static ref SCRIPTS: HashMap<String, ScriptInfo> = load_scripts_from_json();
    static ref NUMERIC_LOOKUP: HashMap<u16, String> = make_script_lookup();
}

/// Lookup a `ScriptInfo` based on it's ISO-15924 4-character identifier, returning
/// `None` if the name does not exist in the current ISO data set.
pub fn lookup_by_alpha(alphabetic_code: &str) -> Option<&'static ScriptInfo> {
    assert_eq!(
        alphabetic_code.len(),
        4,
        "script code is expected to be 3 characters"
    );
    SCRIPTS.get(alphabetic_code)
}

/// Lookup a `ScriptInfo` based on it's ISO-15924 numeric identifier, returning
/// `None` if the name does not exist in the current ISO data set.
pub fn lookup_by_numeric(numeric_code: &u16) -> Option<&'static ScriptInfo> {
    match NUMERIC_LOOKUP.get(&numeric_code) {
        Some(v) => lookup_by_alpha(v),
        None => None,
    }
}

/// Return all the registered ISO-15924 4-character country codes.
pub fn all_alpha_codes() -> Vec<String> {
    SCRIPTS.keys().cloned().collect()
}

/// Return all the registered ISO-15924 numeric country codes.
pub fn all_numeric_codes() -> Vec<u16> {
    NUMERIC_LOOKUP.keys().cloned().collect()
}

// ------------------------------------------------------------------------------------------------
// Generated Data
// ------------------------------------------------------------------------------------------------

fn load_scripts_from_json() -> HashMap<String, ScriptInfo> {
    info!("scripts_from_json - loading JSON");
    let raw_data = include_bytes!("data/scripts.json");
    let script_map: HashMap<String, ScriptInfo> = serde_json::from_slice(raw_data).unwrap();
    info!("scripts_from_json - loaded {} codesets", script_map.len());
    script_map
}

fn make_script_lookup() -> HashMap<u16, String> {
    info!("load_script_lookup - create from SCRIPTS");
    let mut lookup_map: HashMap<u16, String> = HashMap::new();
    for script in SCRIPTS.values() {
        debug!("{} -> {}", &script.numeric_code, &script.alphabetic_code);
        lookup_map.insert(script.numeric_code, script.alphabetic_code.to_string());
    }
    info!("load_script_lookup - mapped {} countries", lookup_map.len());
    lookup_map
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --------------------------------------------------------------------------------------------
    #[test]
    fn test_good_script_alpha_code() {
        match lookup_by_alpha("Hluw") {
            None => panic!("was expecting a script"),
            Some(script) => {
                assert_eq!(script.alphabetic_code.to_string(), "Hluw".to_string());
                assert_eq!(script.numeric_code, 80);
                //assert_eq!(script.alias.unwrap().to_string(), "Anatolian_Hieroglyphs".to_string())
            }
        }
    }

    #[test]
    fn test_bad_script_alpha_code() {
        match lookup_by_alpha(&"UTF8") {
            None => (),
            Some(_) => panic!("was expecting a None in response"),
        }
    }

    #[test]
    fn test_good_script_numeric_code() {
        match lookup_by_numeric(&80) {
            None => panic!("was expecting a script"),
            Some(script) => {
                assert_eq!(script.alphabetic_code.to_string(), "Hluw".to_string());
                assert_eq!(script.numeric_code, 80);
                //assert_eq!(script.alias.unwrap().to_string(), "Anatolian_Hieroglyphs".to_string())
            }
        }
    }

    #[test]
    fn test_bad_script_numeric_code() {
        match lookup_by_numeric(&0) {
            None => (),
            Some(_) => panic!("was expecting a None in response"),
        }
    }

    #[test]
    fn test_script_codes() {
        let codes = all_alpha_codes();
        assert!(codes.len() > 0);
        let numerics = all_numeric_codes();
        assert!(numerics.len() > 0);
    }
}
