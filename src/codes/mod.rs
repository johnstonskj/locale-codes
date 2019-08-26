/*!
Parent to a set of standard code/identifier lookup modules.

These modules are effectively registries of standard code/identifiers and
any metadate published as a part of the associated standard(s). For example,
_Codes for the representation of currencies_, or ISO 4217 is based on a
spreadsheet published directly by ISO itself with some additional fields added
from other publicly accessible sources.


While there is no formal type system or traits for modules exporting codes, there
are definitely some patterns all of the current implementations follow.

1. modules typically implement a `lookup()` function that returns an `Option`,
1. although where some standards have both alphabetic and numeric identifiers
   there are `lookup_by_alpha()` and `lookup_by_numeric()` instead, .
1. Most will also include a function `all_codes()` to retrieve a vector of all
   the known identifiers,
1. or, `all_alpha_codes()` and `all_numeric_codes()` as appropriate.

Some standards, specifically language and country, support 2-character and
3-character alphabetic identifiers, a single `lookup()` function is used to
lookup either.
*/

// ------------------------------------------------------------------------------------------------
// Public Modules
// ------------------------------------------------------------------------------------------------

pub mod codeset;

pub mod country;

pub mod currency;

pub mod language;

pub mod region;

pub mod script;
