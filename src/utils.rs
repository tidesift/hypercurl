#[cfg(feature = "serde")]
pub(crate) mod serde_triomphe_array {
  use serde::{Deserialize, Deserializer, Serialize, Serializer};

  pub fn serialize<T, S>(value: &triomphe::Arc<[T]>, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
    T: Serialize,
  {
    let slice: &[T] = value.as_ref();
    slice.serialize(serializer)
  }
  pub fn deserialize<'de, T, D>(deserializer: D) -> Result<triomphe::Arc<[T]>, D::Error>
  where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
  {
    Vec::<T>::deserialize(deserializer).map(Into::into)
  }
}

pub(crate) trait TakeArc<T> {
  fn take_arc(&mut self) -> triomphe::Arc<[T]>;
}

impl<T> TakeArc<T> for triomphe::Arc<[T]> {
  #[inline(always)]
  fn take_arc(&mut self) -> triomphe::Arc<[T]> {
    core::mem::replace(self, triomphe::Arc::from_iter([]))
  }
}
