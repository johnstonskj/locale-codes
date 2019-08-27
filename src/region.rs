/*!
Codes for the representation of names of countries and their subdivisions.

The purpose of ISO 3166 is to define internationally recognised codes
of letters and/or numbers that we can use when we refer to countries
and subdivisions. However, it does not define the names of countries
– this information comes from United Nations sources (Terminology
Bulletin Country Names and the Country and Region Codes for Statistical
Use maintained by the United Nations Statistics Divisions).

## Source - ISO 3166

The data used here is taken from the page
[Github](https://github.com/lukes/ISO-3166-Countries-with-Regional-Codes).
*/

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// A representation of registered region data maintained by ISO.
#[derive(Deserialize, Serialize, Debug)]
pub struct RegionInfo {
    /// The unique numeric identifier for this region.
    pub code: u16,
    /// The name of this region.
    pub name: String,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

lazy_static! {
    static ref REGIONS: HashMap<u16, RegionInfo> = load_regions_from_json();
}

/// Lookup a `RegionInfo` based on it's ISO-3166 numeric identifier, returning
/// `None` if the name does not exist in the current ISO data set.
pub fn lookup(code: u16) -> Option<&'static RegionInfo> {
    info!("lookup_region: {}", code);
    match REGIONS.get(&code) {
        Some(v) => Some(v),
        None => None,
    }
}

/// Return all the registered ISO-3166 numeric region codes.
pub fn all_codes() -> Vec<u16> {
    REGIONS.keys().cloned().collect()
}

// ------------------------------------------------------------------------------------------------
// Generated Data
// ------------------------------------------------------------------------------------------------

fn load_regions_from_json() -> HashMap<u16, RegionInfo> {
    info!("load_regions_from_json - loading JSON");
    let raw_data = include_bytes!("data/regions.json");
    let raw_map: HashMap<String, String> = serde_json::from_slice(raw_data).unwrap();
    raw_map
        .iter()
        .map(|(code, name)| {
            (
                code.parse::<u16>().unwrap(),
                RegionInfo {
                    code: code.parse::<u16>().unwrap(),
                    name: name.to_string(),
                },
            )
        })
        .collect()
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --------------------------------------------------------------------------------------------
    #[test]
    fn test_region_codes() {
        let codes = all_codes();
        assert!(codes.len() > 0);
    }

    #[test]
    fn test_good_region_code() {
        match lookup(21) {
            None => panic!("was expecting a region"),
            Some(region) => assert_eq!(region.name, "Northern America"),
        }
    }

    #[test]
    fn test_bad_region_code() {
        match lookup(0) {
            None => (),
            Some(_) => panic!("was expecting a None in response"),
        }
    }
}
