pub mod o2o;
pub mod channel;

#[inline]
pub fn convert_option_bool(o: Option<()>) -> bool {
    o.is_some()
}
