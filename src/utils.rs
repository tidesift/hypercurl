#[cfg(feature = "serde")]
pub(crate) mod serde_arc_array {
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

#[cfg(feature = "serde")]
pub(crate) mod serde_optional_arc_array {
  use serde::{Deserialize, Deserializer, Serialize, Serializer};

  pub fn serialize<T, S>(
    value: &Option<triomphe::Arc<[T]>>,
    serializer: S,
  ) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
    T: Serialize,
  {
    if let Some(value) = value {
      let slice: &[T] = value.as_ref();
      slice.serialize(serializer)
    } else {
      serializer.serialize_none()
    }
  }

  pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Option<triomphe::Arc<[T]>>, D::Error>
  where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
  {
    Option::<Vec<T>>::deserialize(deserializer).map(|opt| opt.map(Into::into))
  }
}

#[cfg(feature = "serde")]
pub(crate) mod serde_seconds {
  use core::time::Duration;
  use serde::{Deserialize, Deserializer, Serializer};

  pub fn serialize<S>(value: &Duration, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_u64(value.as_secs())
  }

  pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
  where
    D: Deserializer<'de>,
  {
    let secs = u64::deserialize(deserializer)?;
    Ok(Duration::from_secs(secs))
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
