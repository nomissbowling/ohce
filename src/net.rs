//! net.rs
//!

pub mod tcp;

use std::borrow::Cow;

/// EndCode (static Cow for &amp;[u8] or Vec&lt;u8&gt;)
pub struct EndCode {
  /// c
  pub c: Cow<'static, [u8]>
}

/// EndCode
impl EndCode {
  /// dup
  pub fn dup(&self) -> Self { self.to_vec().into() }

  /// to_vec
  pub fn to_vec(&self) -> Vec<u8> { (*self.c).iter().map(|&c| c).collect() }
}

/// impl From for EndCode
impl<C> From<C> for EndCode where Cow<'static, [u8]>: From<C> {
  /// from
  fn from(c: C) -> Self { EndCode{c: c.into()} }
}
