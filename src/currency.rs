/*!
Codes for the representation of currencies.

Currencies can be represented in the code in two ways: a three-letter alphabetic
code and a three-digit numeric code. The most recent edition is ISO 4217:2015.
The purpose of ISO 4217:2015 is to establish internationally recognised codes
for the representation of currencies.

## Source - ISO 4217:2015

The data used here is taken from the tables in the html page
[ISO.org](https://www.iso.org/iso-4217-currency-codes.html). Additional data was taken from
[Forex](https://www.forexrealm.com/additional-info/foreign-currency-symbols.html),
and [XE](https://www.xe.com/symbols.php).
*/

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// Represents a sub-division (minor currency unit) of a currency.
/// For example, the US Dollar (USD) has a single sub-division in that
/// each 100th of a dollar is named a cent. This would be represented
/// as `Subdivision { exponent: 2, name: Somme("cent") }`. Some
/// currencies have different names for different subdivisionsm, or simply
/// different names for the same.
#[derive(Serialize, Deserialize, Debug)]
pub struct Subdivision {
    /// The exponent, or scale, of the currency unit, determining it's value.
    pub exponent: i8,
    /// The optional name of the currency unit, localized.
    pub name: Option<String>,
}

/// A representation of registered currency data that maintained by ISO.
#[derive(Serialize, Deserialize, Debug)]
pub struct CurrencyInfo {
    /// The  ISO 4217 registered 3-character currency code.
    pub alphabetic_code: String,
    /// The registered name, in English, of the currency.
    pub name: String,
    /// The registered numeric curency code, if it has one.
    pub numeric_code: Option<u16>,
    /// The localized symbol used to represent the currency, if known.
    pub symbol: Option<String>,
    /// These correspond approximately to _countries using
    ///this currency_.
    pub standards_entities: Vec<String>,
    /// The, possibly empty set of subdivisions for this currency.
    pub subdivisions: Vec<Subdivision>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

lazy_static! {
    static ref CURRENCIES: HashMap<String, CurrencyInfo> = load_currencies_from_json();
    static ref NUMERIC_LOOKUP: HashMap<u16, String> = make_currency_lookup();
}

/// Lookup a `CurrencyInfo` based on it's ISO-4217 3-character identifier,
/// returning `None` if the name does not exist in the current ISO data set.
pub fn lookup_by_alpha(alphabetic_code: &str) -> Option<&'static CurrencyInfo> {
    assert_eq!(
        alphabetic_code.len(),
        3,
        "currency code must be 3 characters long"
    );
    CURRENCIES.get(alphabetic_code)
}

/// Lookup a `CurrencyInfo` based on it's ISO-4217 numeric identifier,
/// returning `None` if the name does not exist in the current ISO data set.
pub fn lookup_by_numeric(numeric_code: &u16) -> Option<&'static CurrencyInfo> {
    match NUMERIC_LOOKUP.get(&numeric_code) {
        Some(v) => lookup_by_alpha(v),
        None => None,
    }
}

/// Lookup all `CurrencyInfo` instances that are used by the identified
/// country name.
pub fn currencies_for_country_name(name: &str) -> Vec<&'static CurrencyInfo> {
    CURRENCIES
        .values()
        .filter(|currency| currency.standards_entities.contains(&name.to_string()))
        .collect()
}

/// Return all the registered ISO-4217 3-character currency codes.
pub fn all_alpha_codes() -> Vec<String> {
    CURRENCIES.keys().cloned().collect()
}

/// Return all the registered ISO-4217 numeric currency codes.
pub fn all_numeric_codes() -> Vec<u16> {
    NUMERIC_LOOKUP.keys().cloned().collect()
}

// ------------------------------------------------------------------------------------------------
// Generated Data
// ------------------------------------------------------------------------------------------------

fn load_currencies_from_json() -> HashMap<String, CurrencyInfo> {
    info!("currencies_from_json - loading JSON");
    let raw_data = include_bytes!("data/currencies.json");
    let currency_map: HashMap<String, CurrencyInfo> = serde_json::from_slice(raw_data).unwrap();
    info!(
        "currencies_from_json - loaded {} currencies",
        currency_map.len()
    );
    currency_map
}

fn make_currency_lookup() -> HashMap<u16, String> {
    info!("load_currency_lookup - create from CURRENCIES");
    let mut lookup_map: HashMap<u16, String> = HashMap::new();
    for currency in CURRENCIES.values() {
        if let Some(numeric) = &currency.numeric_code {
            lookup_map.insert(*numeric, currency.alphabetic_code.to_string());
        }
    }
    info!(
        "load_currency_lookup - mapped {} countries",
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
    fn test_currency_loading() {
        match lookup_by_alpha(&"GBP".to_string()) {
            None => println!("lookup_by_alpha NO 'GBP'"),
            Some(c) => println!("lookup_by_alpha {:#?}", to_string_pretty(c)),
        }
    }

    // --------------------------------------------------------------------------------------------
    #[test]
    fn test_currency_codes() {
        let codes = all_alpha_codes();
        assert!(codes.len() > 0);
        let numerics = all_numeric_codes();
        assert!(numerics.len() > 0);
    }

    #[test]
    fn test_good_currency_code() {
        match lookup_by_alpha("GBP") {
            None => panic!("was expecting a currency"),
            Some(currency) => assert_eq!(currency.name.to_string(), "Pound Sterling".to_string()),
        }
    }

    #[test]
    fn test_bad_currency_code() {
        match lookup_by_alpha(&"ZZZ") {
            None => (),
            Some(_) => panic!("was expecting a None in response"),
        }
    }

    #[test]
    fn test_for_country() {
        let currencies = currencies_for_country_name("Mexico");
        assert_eq!(currencies.len(), 2);
    }
}
