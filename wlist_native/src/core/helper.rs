pub(crate) mod utils {
    use std::sync::Arc;

    use dashmap::DashMap;
    use rand::Rng;
    use tokio::task::{spawn_blocking, JoinError};

    pub fn generate_string(length: usize) -> String {
        const ALL: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let mut key = Vec::with_capacity(length);
        let mut rand = rand::thread_rng();
        for _ in 0..length {
            key.push(ALL.as_bytes()[rand.gen_range(0..ALL.len())]);
        }
        unsafe { String::from_utf8_unchecked(key) }
    }

    #[allow(dead_code)] // TODO
    pub async fn random_id_and_put<V: Send + Sync + 'static>(map: &'static DashMap<Arc<String>, V>, value: V) -> Result<Arc<String>, JoinError> {
        spawn_blocking(move || random_id_and_put_sync(map, value)).await
    }

    #[allow(dead_code)]
    pub fn random_id_and_put_sync<V>(map: &DashMap<Arc<String>, V>, value: V) -> Arc<String> {
        loop {
            let id = Arc::new(generate_string(32));
            let entry = map.entry(id);
            if let dashmap::Entry::Occupied(_) = &entry {
                continue;
            }
            break Arc::clone(entry.insert(value).key())
        }
    }
}

pub mod hasher {
    use std::sync::Arc;

    use bytes::Bytes;
    use faster_hex::{hex_encode, Error};
    use md5::Md5;
    use sha2::Sha256;
    use tokio::sync::Mutex;
    use tokio::task::spawn_blocking;

    /// MD5
    pub(crate) fn hex32(src: &[u8]) -> Result<String, Error> {
        let mut buffer = vec![0; 32];
        hex_encode(src, &mut buffer)?;
        Ok(unsafe { String::from_utf8_unchecked(buffer) })
    }

    /// SHA256
    pub(crate) fn hex64(src: &[u8]) -> Result<String, Error> {
        let mut buffer = vec![0; 64];
        hex_encode(src, &mut buffer)?;
        Ok(unsafe { String::from_utf8_unchecked(buffer) })
    }

    macro_rules! define_hasher {
        ($vis: vis $name: ident($hasher: ty) $digest: ident -> $hex: ident ? $h: literal) => {
            $vis struct $name {
                inner: Arc<Mutex<$hasher>>,
            }

            impl $name {
                $vis fn new() -> Self {
                    Self { inner: Arc::new(Mutex::new($digest::Digest::new())), }
                }

                $vis async fn update(&self, data: &Bytes) {
                    let mut inner = self.inner.clone().lock_owned().await;
                    let data = data.clone();
                    spawn_blocking(move || {
                        let inner: &mut $hasher = &mut *inner;
                        $digest::Digest::update(inner, data.as_ref());
                    }).await.expect(concat!("update ", $h, " should not fail"))
                }

                $vis async fn finalize(self) -> String {
                    let hasher = Arc::try_unwrap(self.inner).expect(concat!("waiting ", $h, " update finish")).into_inner();
                    spawn_blocking(move || $hex(&$digest::Digest::finalize(hasher)).expect(concat!($h, " should be valid hex"))).await
                        .expect(concat!("finalize ", $h, " should not fail"))
                }
            }
        };
    }

    define_hasher!(pub Md5Hasher(Md5) md5 -> hex32 ? "md5");
    define_hasher!(pub Sha256Hasher(Sha256) sha2 -> hex64 ? "sha256");
}

pub mod buffer {
    use bytes::{BufMut, Bytes};
    use bytes::buf::UninitSlice;

    #[inline]
    pub unsafe fn new_read_buffer(ptr: *const u8, cap: usize) -> Bytes {
        debug_assert!(!ptr.is_null(), "ptr is null");
        Vec::from_raw_parts(ptr as _, cap, cap).into()
    }

    #[inline]
    pub unsafe fn new_write_buffer(ptr: *mut u8, cap: usize) -> WriteBuffer {
        debug_assert!(!ptr.is_null(), "ptr is null");
        WriteBuffer { ptr, len: 0, cap, }
    }

    pub struct WriteBuffer {
        ptr: *mut u8,
        len: usize,
        cap: usize,
    }

    unsafe impl BufMut for WriteBuffer {
        #[inline]
        fn remaining_mut(&self) -> usize {
            self.cap - self.len
        }

        #[inline]
        unsafe fn advance_mut(&mut self, cnt: usize) {
            let remaining = self.remaining_mut();
            if remaining < cnt {
                panic!("advance_mut: cnt({cnt}) > remaining_mut({remaining}).");
            }
            self.len += cnt;
        }

        #[inline]
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unsafe { UninitSlice::from_raw_parts_mut(self.ptr.add(self.len), self.remaining_mut()) }
        }
    }

    unsafe impl Send for WriteBuffer { }
}
