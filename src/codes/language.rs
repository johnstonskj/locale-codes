/*!
Codes for the representation of names of languages.

These codes are widely used in many different disciplines, for example for
bibliographic purposes, in the library community, as well as for computerized
systems, and the representation of different language versions on websites.

Using a code rather than the name of a language has many benefits as some
languages are referred to by different groups in different ways, and two
unrelated languages may share the same or similar name.

* Part 1 (ISO 639-1:2002) provides a 2 letter code that has been designed
  to represent most of the major languages of the world.
* Part 2 (ISO 639-2:1998) provides a 3 letter code, which gives more possible
  combinations, so ISO 639-2:1998 can cover more languages.
* Part 3 (ISO 639-3:2007) provides a 3 letter code and aims to give as complete
  a listing of languages as possible, including living, extinct and ancient languages.
* Part 4 (ISO 639-4:2010) gives the general principles of language coding and
  lays down guidelines for the use of ISO 639.
* Part 5 (ISO 639-5:2008) provides a 3 letter code for language families
  and groups (living and extinct).

## Source - ISO 639

The data used here is taken from
[SIL International](https://iso639-3.sil.org/code_tables/download_tables).

See also: [Native names for languages](https://www.omniglot.com/language/names.htm).
*/

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// The ISO 639 data identifies 3 classes of languages, each language is
/// one of these classes only.
#[derive(Serialize, Deserialize, Debug)]
pub enum LanguageClass {
    Individual,
    MacroLanguage,
    Special,
}

/// The type of the language in this this meaning is more concerning it's
/// current usage.
#[derive(Serialize, Deserialize, Debug)]
pub enum LanguageType {
    Ancient,
    Constructed,
    Extinct,
    Historical,
    Living,
    Special,
}

/// A representation of registered language data maintained by ISO.
#[derive(Serialize, Deserialize, Debug)]
pub struct LanguageInfo {
    /// The ISO 3-character language identifier
    pub code: String,
    /// The reference name, in English, used by the standard.
    pub reference_name: String,
    /// The indigenous name, if captured in the standard.
    pub indigenous_name: Option<String>,
    /// Common aliases.
    pub other_names: Option<Vec<String>>,
    pub bibliographic_code: Option<String>,
    pub terminology_code: Option<String>,
    pub short_code: Option<String>,
    pub class: LanguageClass,
    pub l_type: LanguageType,
    /// if `class` is `LanguageClass::MacroLanguage` this is
    /// a vector of family members of this language.
    pub family_members: Option<Vec<String>>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

lazy_static! {
    static ref LANGUAGES: HashMap<String, LanguageInfo> = load_languages_from_json();
    static ref LOOKUP: HashMap<String, String> = make_language_lookup();
}

pub fn lookup(code: &str) -> Option<&'static LanguageInfo> {
    debug!("language::lookup {}", code);
    assert!(
        code.len() == 2 || code.len() == 3,
        "language code must be either 2, or 3, characters long."
    );
    match code.len() {
        3 => match LANGUAGES.get(code) {
            Some(v) => Some(v),
            None => None,
        },
        2 => match LOOKUP.get(code) {
            Some(v) => {
                debug!("language::lookup {} -> {}", code, v);
                lookup(v)
            }
            None => None,
        },
        _ => None,
    }
}

pub fn all_codes() -> Vec<String> {
    LANGUAGES.keys().cloned().collect()
}

// ------------------------------------------------------------------------------------------------
// Generated Data
// ------------------------------------------------------------------------------------------------

fn load_languages_from_json() -> HashMap<String, LanguageInfo> {
    info!("languages_from_json - loading JSON");
    let raw_data = include_bytes!("data/languages.json");
    let language_map: HashMap<String, LanguageInfo> = serde_json::from_slice(raw_data).unwrap();
    info!(
        "languages_from_json - loaded {} countries",
        language_map.len()
    );
    language_map
}

fn make_language_lookup() -> HashMap<String, String> {
    info!("load_language_lookup - create from COUNTRIES");
    let mut lookup_map: HashMap<String, String> = HashMap::new();
    for language in LANGUAGES.values() {
        if let Some(short_code) = &language.short_code {
            lookup_map.insert(short_code.to_string(), language.code.to_string());
        }
    }
    info!(
        "load_language_lookup - mapped {} countries",
        lookup_map.len()
    );
    lookup_map
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::ser::to_string_pretty;

    // --------------------------------------------------------------------------------------------
    #[test]
    fn test_language_loading() {
        match lookup("aab") {
            None => println!("test_language_loading NO 'aab'"),
            Some(l) => println!("test_language_loading {:#?}", to_string_pretty(l)),
        }
    }
}
