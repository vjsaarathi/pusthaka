use alloc::string::String;

/// Stable identifier for a localized UI string.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StrId(pub u16);

/// Runtime localization service. Production data should be compiled into compact offset tables.
pub struct I18nService {
    pub language: String,
}

impl I18nService {
    pub fn get(&self, _id: StrId) -> &str { "" }
    pub fn character_set(&self) -> crate::Result<alloc::vec::Vec<u32>> { Ok(alloc::vec::Vec::new()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i18n() {
        let i18n = I18nService { language: String::from("en") };
        assert_eq!(i18n.get(StrId(0)), "");
        assert!(i18n.character_set().unwrap().is_empty());
    }
}
