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

use crate::alphabet::Alphabet;
use crate::isocode::{IsoCode639_1, IsoCode639_3};
use maplit::hashset;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString};

/// This enum specifies the so far 75 supported languages which can be detected by *Lingua*.
#[derive(
    Clone, Debug, Serialize, Deserialize, EnumIter, Eq, PartialEq, Hash, Ord, PartialOrd, EnumString,
)]
#[serde(rename_all(serialize = "UPPERCASE", deserialize = "UPPERCASE"))]
#[strum(ascii_case_insensitive)]
pub enum Language {
    English,

    French,

    German,

    Italian,
}

impl Language {
    pub fn all() -> HashSet<Language> {
        Language::iter().collect()
    }

    pub fn all_spoken_ones() -> HashSet<Language> {
        Language::iter()
            .filter(|it| {
                if cfg!(feature = "latin") {
                    it != &Language::from_str("Latin").unwrap()
                } else {
                    true
                }
            })
            .collect()
    }

    pub fn all_with_arabic_script() -> HashSet<Language> {
        Language::iter()
            .filter(|it| it.alphabets().contains(&Alphabet::Arabic))
            .collect()
    }

    pub fn all_with_cyrillic_script() -> HashSet<Language> {
        Language::iter()
            .filter(|it| it.alphabets().contains(&Alphabet::Cyrillic))
            .collect()
    }

    pub fn all_with_devanagari_script() -> HashSet<Language> {
        Language::iter()
            .filter(|it| it.alphabets().contains(&Alphabet::Devanagari))
            .collect()
    }

    pub fn all_with_latin_script() -> HashSet<Language> {
        Language::iter()
            .filter(|it| it.alphabets().contains(&Alphabet::Latin))
            .collect()
    }

    pub fn from_iso_code_639_1(iso_code: &IsoCode639_1) -> Language {
        Language::iter()
            .find(|it| &it.iso_code_639_1() == iso_code)
            .unwrap()
    }

    pub fn from_iso_code_639_3(iso_code: &IsoCode639_3) -> Language {
        Language::iter()
            .find(|it| &it.iso_code_639_3() == iso_code)
            .unwrap()
    }

    pub fn iso_code_639_1(&self) -> IsoCode639_1 {
        match self {
            Language::English => IsoCode639_1::EN,

            Language::French => IsoCode639_1::FR,

            Language::German => IsoCode639_1::DE,

            Language::Italian => IsoCode639_1::IT,
        }
    }

    pub fn iso_code_639_3(&self) -> IsoCode639_3 {
        match self {
            Language::English => IsoCode639_3::ENG,

            Language::French => IsoCode639_3::FRA,

            Language::German => IsoCode639_3::DEU,

            Language::Italian => IsoCode639_3::ITA,
        }
    }

    pub fn alphabets(&self) -> HashSet<Alphabet> {
        match self {
            Language::English => hashset!(Alphabet::Latin),

            Language::French => hashset!(Alphabet::Latin),

            Language::German => hashset!(Alphabet::Latin),

            Language::Italian => hashset!(Alphabet::Latin),
        }
    }

    pub fn unique_characters(&self) -> Option<&str> {
        match self {
            Language::German => Some("ß"),

            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_language_serializer() {
        let serialized = serde_json::to_string(&Language::English).unwrap();
        assert_eq!(serialized, "\"ENGLISH\"");
    }

    #[test]
    fn test_language_deserializer() {
        let deserialized = serde_json::from_str::<Language>("\"ENGLISH\"").unwrap();
        assert_eq!(deserialized, Language::English);
    }

    #[test]
    fn test_from_str() {
        let language = Language::from_str("english").unwrap();
        assert_eq!(language, Language::English);
    }
}
