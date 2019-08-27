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

/// A representation of registered country data maintained by ISO.
#[derive(Serialize, Deserialize, Debug)]
pub struct CountryInfo {
    /// The ISO-3166, part 2, 3-character identifier of the country. This
    /// is the primary identifier.
    pub code: String,
    /// The ISO-3166, part 1, 2-character identifier of the country.
    pub short_code: String,
    /// The numeric code for the `RegionInfo` that represents the country.
    pub country_code: u16,
    /// The optional numeric code for the `RegionInfo` that represents the
    /// region.
    pub region_code: Option<u16>,
    /// The optional numeric code for the `RegionInfo` that represents the
    /// sub-region.
    pub sub_region_code: Option<u16>,
    /// The optional numeric code for the `RegionInfo` that represents the
    /// intermediate region.
    pub intermediate_region_code: Option<u16>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

lazy_static! {
    static ref COUNTRIES: HashMap<String, CountryInfo> = load_countries_from_json();
    static ref LOOKUP: HashMap<String, String> = make_country_lookup();
}

/// Lookup a `CountryInfo` based on it's ISO-3166 identifier, returning
/// `None` if the name does not exist in the current ISO data set.
pub fn lookup(code: &str) -> Option<&'static CountryInfo> {
    debug!("lookup_country: {}", code);
    assert!(
        code.len() == 2 || code.len() == 3,
        "country code must be either 2, or 3, characters long."
    );
    match code.len() {
        3 => {
            debug!("lookup_country: 3-character code");
            match COUNTRIES.get(code) {
                Some(v) => Some(v),
                None => None,
            }
        }
        2 => {
            debug!("lookup_country: 2-character code");
            match LOOKUP.get(code) {
                Some(v) => lookup(v),
                None => None,
            }
        }
        _ => None,
    }
}

/// Return all the registered ISO-3166 2-character country codes.
pub fn all_codes() -> Vec<String> {
    COUNTRIES.keys().cloned().collect()
}

// ------------------------------------------------------------------------------------------------
// Generated Data
// ------------------------------------------------------------------------------------------------

fn load_countries_from_json() -> HashMap<String, CountryInfo> {
    info!("load_countries_from_json - loading JSON");
    let raw_data = include_bytes!("data/countries.json");
    let country_map: HashMap<String, CountryInfo> = serde_json::from_slice(raw_data).unwrap();
    info!(
        "load_countries_from_json - loaded {} countries",
        country_map.len()
    );
    country_map
}

fn make_country_lookup() -> HashMap<String, String> {
    info!("load_country_lookup - create from COUNTRIES");
    let mut lookup_map: HashMap<String, String> = HashMap::new();
    for country in COUNTRIES.values() {
        lookup_map.insert(country.short_code.to_string(), country.code.to_string());
    }
    info!(
        "load_country_lookup - mapped {} countries",
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

    // --------------------------------------------------------------------------------------------
    #[test]
    fn test_country_codes() {
        let codes = all_codes();
        assert!(codes.len() > 0);
    }

    #[test]
    fn test_good_country_code() {
        match lookup("DEU") {
            None => panic!("was expecting a country"),
            Some(country) => {
                assert_eq!(country.short_code, "DE");
                assert_eq!(country.country_code, 276);
            }
        }
    }

    #[test]
    fn test_good_country_short_code() {
        match lookup("DE") {
            None => panic!("was expecting a country"),
            Some(country) => {
                assert_eq!(country.code, "DEU");
                assert_eq!(country.country_code, 276);
            }
        }
    }

    #[test]
    fn test_bad_country_code() {
        match lookup("XXX") {
            None => (),
            Some(_) => panic!("was expecting a None in response"),
        }
    }
}
