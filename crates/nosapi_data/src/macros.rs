#[macro_export]
macro_rules! ensure {
  ($cond:expr, $err:expr $(,)?) => {
    if !$cond {
      return core::result::Result::Err($err);
    }
  };
}
