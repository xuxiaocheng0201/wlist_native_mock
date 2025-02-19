#![allow(dead_code)]

use std::hash::Hash;
use std::sync::Arc;

use flutter_rust_bridge::for_generated::RustAutoOpaqueInner;
use indexmap::IndexMap;
pub use o2o::o2o;

use crate::frb_generated::{MoiArcValue, RustAutoOpaque};

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
pub fn from_opaque<A, B>(data: A) -> RustAutoOpaque<B> where A: Into<B>, RustAutoOpaqueInner<B>: MoiArcValue {
    RustAutoOpaque::new(data.into())
}

#[inline]
pub fn into_opaque<A, B>(opaque: RustAutoOpaque<B>) -> A where B: Clone + Into<A>, RustAutoOpaqueInner<B>: MoiArcValue {
    opaque.blocking_read().clone().into()
}


#[inline]
pub fn map_vec<A, B>(vec: Vec<A>) -> Vec<B> where A: Into<B> {
    vec.into_iter().map(|a| a.into()).collect()
}

#[inline]
pub fn from_hash_set<A, T>(set: hashbrown::HashSet<T>) -> std::collections::HashSet<A> where T: Into<A>, A: Hash + Eq {
    set.into_iter().map(Into::into).collect()
}

#[inline]
pub fn into_hash_set<T, A>(set: std::collections::HashSet<A>) -> hashbrown::HashSet<T> where A: Into<T>, T: Hash + Eq {
    set.into_iter().map(Into::into).collect()
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
pub fn from_hash_map<A, B, K, V>(map: hashbrown::HashMap<K, V>) -> std::collections::HashMap<A, B> where K: Into<A>, V: Into<B>, A: Hash + Eq {
    map.into_iter().map(|(k, v)| (k.into(), v.into())).collect()
}

#[inline]
pub fn into_hash_map<K, V, A, B>(map: std::collections::HashMap<A, B>) -> hashbrown::HashMap<K, V> where A: Into<K>, B: Into<V>, K: Hash + Eq {
    map.into_iter().map(|(k, v)| (k.into(), v.into())).collect()
}
