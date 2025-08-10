macro_rules! read {
  ($reader: expr, $field_name: ident, $ty: tt) => {{
    #[allow(unused_imports)]
    use error_stack::ResultExt as _;
    $crate::macros::_impl_read!($reader, $ty).change_context($crate::error::ReadError(format!(
      "Failed to read the `{}` field",
      stringify!($field_name)
    )))
  }};
}

macro_rules! _impl_read {
  ($reader: expr, u8) => {{
    use byteorder::ReadBytesExt as _;
    $reader.read_u8()
  }};
  ($reader: expr, i8) => {{
    #[allow(unused_imports)]
    use byteorder::ReadBytesExt as _;
    $reader.read_i8()
  }};
  ($reader: expr, u16) => {{
    #[allow(unused_imports)]
    use byteorder::{LittleEndian, ReadBytesExt as _};
    $reader.read_u16::<LittleEndian>()
  }};
  ($reader: expr, i16) => {{
    #[allow(unused_imports)]
    use byteorder::{LittleEndian, ReadBytesExt as _};
    $reader.read_i16::<LittleEndian>()
  }};
  ($reader: expr, u24) => {{
    #[allow(unused_imports)]
    use byteorder::{LittleEndian, ReadBytesExt as _};
    $reader.read_u24::<LittleEndian>()
  }};
  ($reader: expr, i24) => {{
    #[allow(unused_imports)]
    use byteorder::{LittleEndian, ReadBytesExt as _};
    $reader.read_i24::<LittleEndian>()
  }};
  ($reader: expr, u32) => {{
    #[allow(unused_imports)]
    use byteorder::{LittleEndian, ReadBytesExt as _};
    $reader.read_u32::<LittleEndian>()
  }};
  ($reader: expr, i32) => {{
    #[allow(unused_imports)]
    use byteorder::{LittleEndian, ReadBytesExt as _};
    $reader.read_i32::<LittleEndian>()
  }};
  ($reader: expr, u48) => {{
    #[allow(unused_imports)]
    use byteorder::{LittleEndian, ReadBytesExt as _};
    $reader.read_u48::<LittleEndian>()
  }};
  ($reader: expr, i48) => {{
    #[allow(unused_imports)]
    use byteorder::{LittleEndian, ReadBytesExt as _};
    $reader.read_i48::<LittleEndian>()
  }};
  ($reader: expr, u64) => {{
    #[allow(unused_imports)]
    use byteorder::{LittleEndian, ReadBytesExt as _};
    $reader.read_u64::<LittleEndian>()
  }};
  ($reader: expr, i64) => {{
    #[allow(unused_imports)]
    use byteorder::{LittleEndian, ReadBytesExt as _};
    $reader.read_i64::<LittleEndian>()
  }};
  ($reader: expr, u128) => {{
    #[allow(unused_imports)]
    use byteorder::{LittleEndian, ReadBytesExt as _};
    $reader.read_u128::<LittleEndian>()
  }};
  ($reader: expr, i128) => {{
    #[allow(unused_imports)]
    use byteorder::{LittleEndian, ReadBytesExt as _};
    $reader.read_i128::<LittleEndian>()
  }};
  ($reader: expr, f32) => {{
    #[allow(unused_imports)]
    use byteorder::{LittleEndian, ReadBytesExt as _};
    $reader.read_f32::<LittleEndian>()
  }};
  ($reader: expr, f64) => {{
    #[allow(unused_imports)]
    use byteorder::{LittleEndian, ReadBytesExt as _};
    $reader.read_f64::<LittleEndian>()
  }};
  ($reader: expr, $buf: expr) => {{
    #[allow(unused_imports)]
    use std::io::Read as _;
    $reader.read_exact(&mut $buf)
  }};
}

pub(crate) use _impl_read;
pub(crate) use read;
