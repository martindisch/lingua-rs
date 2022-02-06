/*
 * Copyright © 2020-today Peter M. Stahl pemistahl@gmail.com
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either expressed or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::fmt::{Debug, Display, Formatter, Result};
use strum_macros::EnumString;

/// This enum specifies the ISO 639-1 code representations for the supported languages.
///
/// ISO 639 is a standardized nomenclature used to classify languages.
#[derive(Debug, Eq, PartialEq, EnumString)]
#[allow(clippy::upper_case_acronyms)]
#[strum(ascii_case_insensitive)]
pub enum IsoCode639_1 {
    /// The ISO 639-1 code for [`German`](crate::language::Language::German)
    DE,

    /// The ISO 639-1 code for [`English`](crate::language::Language::English)
    EN,

    /// The ISO 639-1 code for [`French`](crate::language::Language::French)
    FR,

    /// The ISO 639-1 code for [`Italian`](crate::language::Language::Italian)
    IT,
}

/// This enum specifies the ISO 639-3 code representations for the supported languages.
///
/// ISO 639 is a standardized nomenclature used to classify languages.
#[derive(Debug, Eq, PartialEq, EnumString)]
#[allow(clippy::upper_case_acronyms)]
#[strum(ascii_case_insensitive)]
pub enum IsoCode639_3 {
    /// The ISO 639-3 code for [`German`](crate::language::Language::German)
    DEU,

    /// The ISO 639-3 code for [`English`](crate::language::Language::English)
    ENG,

    /// The ISO 639-3 code for [`French`](crate::language::Language::French)
    FRA,

    /// The ISO 639-3 code for [`Italian`](crate::language::Language::Italian)
    ITA,
}

impl Display for IsoCode639_1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let debug_repr = format!("{:?}", self);
        write!(f, "{}", debug_repr.to_lowercase())
    }
}

impl Display for IsoCode639_3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let debug_repr = format!("{:?}", self);
        write!(f, "{}", debug_repr.to_lowercase())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn assert_iso_code_639_1_string_representation_is_correct() {
        assert_eq!(IsoCode639_1::EN.to_string(), "en");
    }

    #[test]
    fn assert_iso_code_639_3_string_representation_is_correct() {
        assert_eq!(IsoCode639_3::ENG.to_string(), "eng");
    }

    #[test]
    fn assert_string_to_iso_code_639_1_is_correct() {
        assert_eq!(IsoCode639_1::from_str("en").unwrap(), IsoCode639_1::EN);
    }

    #[test]
    fn assert_string_to_iso_code_639_3_is_correct() {
        assert_eq!(IsoCode639_3::from_str("eng").unwrap(), IsoCode639_3::ENG);
    }
}
