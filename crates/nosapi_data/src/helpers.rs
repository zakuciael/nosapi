macro_rules! impl_into_iter {
  {
    ty: $for_struct: ident,
    iter: $iter_ty: ty,
    item: $item_ty: ty,
    expr: |$self: ident| $iter_expr: expr
  } => {
    pub struct IntoIter {
      iter: <$iter_ty as core::iter::IntoIterator>::IntoIter,
    }

    impl core::iter::Iterator for IntoIter {
      type Item = $item_ty;

      fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
      }
    }

    impl core::iter::IntoIterator for $for_struct {
      type Item = $item_ty;
      type IntoIter = IntoIter;

      fn into_iter($self) -> Self::IntoIter {
        IntoIter { iter: $iter_expr }
      }
    }
  };
}

pub(crate) use impl_into_iter;
