pub mod information;
pub mod options;

#[derive(Debug, serde::Serialize, serde::Deserialize, Copy, Clone, Eq, PartialEq, Hash)]
pub enum StorageType {
    #[cfg(debug_assertions)]
    Mocker,
    Lanzou,
    // FIXME: More types...
}

impl StorageType {
    #[inline]
    pub const fn is_private(&self) -> bool {
        unimplemented!()
    }

    #[inline]
    pub const fn is_share(&self) -> bool {
        unimplemented!()
    }

    #[inline]
    pub fn allowed_suffixes(&self) -> &'static hashbrown::HashSet<&'static str> {
        unimplemented!()
    }

    #[inline]
    pub fn disallowed_suffixes(&self) -> &'static hashbrown::HashSet<&'static str> {
        unimplemented!()
    }

    #[inline]
    pub const fn allow_no_suffix(&self) -> bool {
        unimplemented!()
    }

    #[inline]
    pub fn allowed_characters(&self) -> &'static hashbrown::HashSet<char> {
        unimplemented!()
    }

    #[inline]
    pub fn disallowed_characters(&self) -> &'static hashbrown::HashSet<char> {
        unimplemented!()
    }

    #[inline]
    pub const fn max_filename_length(&self) -> usize {
        unimplemented!()
    }

    pub fn check_filename(&self, name: &std::sync::Arc<String>, is_directory: bool) -> anyhow::Result<()> {
        // Not unimplemented! because this is an example function to display how the functions above be used.
        use std::sync::Arc;
        use crate::common::exceptions::{IllegalSuffixError, InvalidFilenameError, NameTooLongError};
        #[inline]
        /// "1.txt" => Some("txt")
        /// "1txt" => None
        fn get_suffix(name: &str) -> Option<&str> {
            name.rsplit_once(".").map(|(_, s)| s)
        }

        if name.len() > self.max_filename_length() {
            return Err(NameTooLongError::new(Arc::clone(name), Some(self.max_filename_length() as u64)).into());
        }
        let allowed_characters = self.allowed_characters();
        let disallowed_characters = self.disallowed_characters();
        for character in name.as_str().chars() {
            if !allowed_characters.is_empty() && !allowed_characters.contains(&character) {
                return Err(InvalidFilenameError::new(Arc::clone(name), Some(character)).into());
            }
            if disallowed_characters.contains(&character) {
                return Err(InvalidFilenameError::new(Arc::clone(name), Some(character)).into());
            }
        }
        if !is_directory {
            match get_suffix(name.as_str()) {
                Some(suffix) => {
                    let allowed_suffixes = self.allowed_suffixes();
                    if !allowed_suffixes.is_empty() && !allowed_suffixes.contains(suffix) {
                        return Err(IllegalSuffixError::new(suffix.to_string().into()).into());
                    }
                    if let Some(suffix) = self.disallowed_suffixes().get(suffix) {
                        return Err(IllegalSuffixError::new((*suffix).into()).into());
                    }
                },
                None => if !self.allow_no_suffix() {
                    return Err(IllegalSuffixError::new("".into()).into());
                },
            }
        }
        Ok(())
    }
}
