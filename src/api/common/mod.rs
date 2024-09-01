use crate::api::common::exceptions::UniverseError;

pub mod data;
pub mod exceptions;
pub mod versions;

/// Initialize the core server.
///
/// Note that you **must** call this method before calling all the others.
/// You should ensure these directories exist and have permissions to read/write them.
///
/// The path can be absolute or relative.
/// data_directory contains the data files, such as the database.
/// cache_directory contains the support files, such as the dynamic libs.
pub fn initialize(data_directory: String, cache_directory: String) -> Result<(), UniverseError> {
    wlist_native::common::workspace::initialize(data_directory, cache_directory).map_err(anyhow::Error::from)?;
    wlist_native::common::database::initialize().map_err(Into::into)
}

pub(in crate::api) mod o2o {
    use std::hash::Hash;
    use std::sync::Arc;

    use indexmap::IndexMap;
    pub use o2o::o2o;

    #[inline]
    pub fn map<A, B>(a: A) -> B where A: Into<B> {
        a.into()
    }

    #[inline]
    pub fn map_option<A, B>(opt: Option<A>) -> Option<B> where A: Into<B> {
        opt.map(|a| a.into())
    }


    #[inline]
    pub fn from_arc<A>(arc: Arc<A>) -> A where A: Clone {
        Arc::unwrap_or_clone(arc)
    }

    #[inline]
    pub fn into_arc<A>(a: A) -> Arc<A> {
        Arc::new(a)
    }

    #[inline]
    pub fn from_option_arc<A>(a: Option<Arc<A>>) -> Option<A> where A: Clone {
        a.map(|a| from_arc(a))
    }

    #[inline]
    pub fn into_option_arc<A>(a: Option<A>) -> Option<Arc<A>> {
        a.map(|a| into_arc(a))
    }



    #[inline]
    pub fn map_vec<A, B>(vec: Vec<A>) -> Vec<B> where A: Into<B> {
        vec.into_iter().map(|a| a.into()).collect()
    }


    #[inline]
    pub fn from_index_map<A, B, K, V>(map: IndexMap<K, V>) -> Vec<(A, B)> where K: Into<A>, V: Into<B>, K: Hash {
        map.into_iter().map(|(k, v)| (k.into(), v.into())).collect()
    }

    #[inline]
    pub fn into_index_map<K, V, A, B>(vec: Vec<(A, B)>) -> IndexMap<K, V> where A: Into<K>, B: Into<V>, K: Hash + Eq {
        vec.into_iter().map(|(k, v)| (k.into(), v.into())).collect()
    }


    #[inline]
    pub fn from_hash_brown<A, B, K, V>(map: hashbrown::HashMap<K, V>) -> std::collections::HashMap<A, B> where K: Into<A>, V: Into<B>, A: Hash + Eq {
        map.into_iter().map(|(k, v)| (k.into(), v.into())).collect()
    }

    #[inline]
    pub fn into_hash_brown<K, V, A, B>(map: std::collections::HashMap<A, B>) -> hashbrown::HashMap<K, V> where A: Into<K>, B: Into<V>, K: Hash + Eq {
        map.into_iter().map(|(k, v)| (k.into(), v.into())).collect()
    }
}
