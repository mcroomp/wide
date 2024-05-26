use std::array;

use wide::Simd;

pub fn simd_tests<
  S: Simd<T, N>,
  const N: usize,
  T: PartialEq + std::fmt::Debug + Copy,
  F: Fn(usize) -> T,
>(
  init: F,
) {
  let array: [T; N] = array::from_fn(|i| init(i));

  let s = S::from_array(array);
  assert_eq!(s.to_array(), array);

  // Test that we can splat a value
  let splat = S::splat(init(0));
  for i in 0..N {
    assert_eq!(splat.as_array()[i], init(0));
  }

  // Test that we can mutate the splat
  let mut splat_mut = S::splat(init(0));
  for i in 0..N {
    splat_mut.as_mut_array()[i] = init(i);
  }
  assert_eq!(splat_mut.to_array(), array);
}
