pub mod hasher {
    use crate::api::core::client::upload::ConstU8;

    macro_rules! define_hasher {
        ($hash: literal, $hasher: ident) => {
            #[flutter_rust_bridge::frb(opaque)]
            #[doc = concat!(" Native hasher for ", $hash, ", fast and async.")]
            pub struct $hasher {
                hasher: wlist_native::core::helper::hasher::$hasher,
            }

            impl $hasher {
                #[doc = concat!(" Creates a new hasher for ", $hash, ".")]
                /// This is the initial step.
                pub fn new() -> Self {
                    Self { hasher: wlist_native::core::helper::hasher::$hasher::new(), }
                }

                /// Updates the hasher with data.
                /// This can be called multiple times.
                pub async fn update(&self, data: &ConstU8, data_size: usize) {
                    let buffer = unsafe { wlist_native::core::helper::buffer::new_read_buffer(data.0, data_size) };
                    self.hasher.update(buffer).await;
                }

                /// Returns the hash as a lowercase hex string.
                /// This is the final step.
                ///
                /// After called this method, the hasher cannot be reused.
                pub async fn finalize(self) -> String {
                    self.hasher.finalize().await
                }
            }
        };
    }

    define_hasher!("md5", Md5Hasher);
    define_hasher!("sha256", Sha256Hasher);
}
