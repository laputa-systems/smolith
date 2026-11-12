use core::cmp::{PartialEq, PartialOrd};
use core::ops::{Add, Bound, Div, Mul, Neg, RangeBounds, Sub};

use crate::BaseRng;

pub(super) fn f32(rng: &mut impl BaseRng, range: impl RangeBounds<f32>) -> f32 {
    float_range_impl(rng, range)
}

pub(super) fn f64(rng: &mut impl BaseRng, range: impl RangeBounds<f64>) -> f64 {
    float_range_impl(rng, range)
}

trait FloatExt:
    Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
    + Neg<Output = Self>
    + PartialEq
    + PartialOrd
    + Copy
    + Sized
{
    const MIN: Self;
    const MAX: Self;
    const HALF: Self;

    fn is_finite(self) -> bool;
    fn next_up(self) -> Self;
    fn next_down(self) -> Self;
    /// Generate a random float in [0, 1) range.
    fn gen_close_01_open(rng: &mut impl BaseRng) -> Self;
    /// Generate a random float in (0, 1] range.
    fn gen_open_01_close(rng: &mut impl BaseRng) -> Self;
    /// Generate a random float in (0, 1) range.
    fn gen_open_01_open(rng: &mut impl BaseRng) -> Self;
    /// Get the maximum float that can be generated in [0, 1) range (i.e., the
    /// float just below 1).
    fn max_rand() -> Self;
}

macro_rules! impl_float_ext {
    ($float:ident, $bits:ident, $max_rand_bits:literal) => {
        impl FloatExt for $float {
            const MIN: Self = $float::MIN;
            const MAX: Self = $float::MAX;
            const HALF: Self = 0.5;

            #[inline]
            fn is_finite(self) -> bool {
                $float::is_finite(self)
            }
            #[inline]
            fn next_up(self) -> Self {
                let bits = self.to_bits();
                if self.is_nan() || bits == $float::INFINITY.to_bits() {
                    return self;
                }
                let magnitude = bits & ($bits::MAX >> 1);
                let next = if magnitude == 0 {
                    1
                } else if bits == magnitude {
                    bits + 1
                } else {
                    bits - 1
                };
                $float::from_bits(next)
            }
            #[inline]
            fn next_down(self) -> Self {
                if self.is_nan() || self.to_bits() == $float::NEG_INFINITY.to_bits() {
                    return self;
                }
                (-(-self).next_up())
            }
            #[inline]
            fn gen_close_01_open(rng: &mut impl BaseRng) -> Self {
                rng.$float()
            }
            #[inline]
            fn gen_open_01_close(rng: &mut impl BaseRng) -> Self {
                1.0 - rng.$float()
            }
            #[inline]
            fn gen_open_01_open(rng: &mut impl BaseRng) -> Self {
                loop {
                    let r = rng.$float();

                    if r != 0.0 {
                        return r;
                    }
                }
            }
            #[inline]
            fn max_rand() -> Self {
                <$float>::from_bits($max_rand_bits)
            }
        }
    };
}

// Max rand constant is conceptually 0.9999... in the precision of the
// particular float type. It can be determined in the following steps:
//
//   1. Fill the fractional part of the float with 1s (in binary).
//   2. Move the number to have exponent 0 (i.e., the exponent is equal to the
//      exponent bias).
//   3. Since the float representation uses implicit leading 1, subtract 1.0
//      from the number constructed in the previous steps.
//
// This is a code snippet for f32:
//
// ```
// let fraction_bits = 23; // Significand bits without the implicit leading 1.
// let exponent_bias = 127;
// let discard_bits = u32::BITS - fraction_bits;
// let max_rand = f32::from_bits(((u32::MAX >> discard_bits) | (exponent_bias << fraction_bits))) - 1.0;
// ```

impl_float_ext!(f32, u32, 0x3f7ffffe);
impl_float_ext!(f64, u64, 0x3feffffffffffffe);

/// Indication whether a range is exclusive, inclusive on either side or
/// inclusive on both sides. `Bound::Unbounded` is treated as inclusive, because
/// we use MIN and MAX constants for such bounds.
enum Inclusive {
    None,
    Left,
    Right,
    Both,
}

impl Inclusive {
    fn from_bounds<T>(range: &impl RangeBounds<T>) -> Self {
        match (range.start_bound(), range.end_bound()) {
            (Bound::Excluded(_), Bound::Excluded(_)) => Inclusive::None,
            (Bound::Included(_), Bound::Excluded(_)) | (Bound::Unbounded, Bound::Excluded(_)) => {
                Inclusive::Left
            }
            (Bound::Excluded(_), Bound::Included(_)) | (Bound::Excluded(_), Bound::Unbounded) => {
                Inclusive::Right
            }
            (Bound::Included(_), Bound::Included(_))
            | (Bound::Included(_), Bound::Unbounded)
            | (Bound::Unbounded, Bound::Included(_))
            | (Bound::Unbounded, Bound::Unbounded) => Inclusive::Both,
        }
    }
}

fn float_range_impl<T: FloatExt>(rng: &mut impl BaseRng, range: impl RangeBounds<T>) -> T {
    let low = match range.start_bound() {
        Bound::Included(&low) | Bound::Excluded(&low) => low,
        Bound::Unbounded => T::MIN,
    };
    let high = match range.end_bound() {
        Bound::Included(&high) | Bound::Excluded(&high) => high,
        Bound::Unbounded => T::MAX,
    };
    if !low.is_finite() || !high.is_finite() {
        panic!("float range bounds must be finite");
    }
    let lower = match range.start_bound() {
        Bound::Excluded(_) => {
            let next = low.next_up();
            if next.is_finite() {
                Some(next)
            } else {
                None
            }
        }
        _ => Some(low),
    };
    let upper = match range.end_bound() {
        Bound::Excluded(_) => {
            let next = high.next_down();
            if next.is_finite() {
                Some(next)
            } else {
                None
            }
        }
        _ => Some(high),
    };
    let (Some(lower), Some(upper)) = (lower, upper) else {
        panic!("float range is empty");
    };
    if lower > upper {
        panic!("float range is empty");
    }
    if lower == upper {
        return lower;
    }
    for _ in 0..128 {
        let sample = sample_float_range(rng, &range);
        if lower <= sample && sample <= upper {
            return sample;
        }
    }
    lower
}

fn sample_float_range<T: FloatExt, R: RangeBounds<T>>(rng: &mut impl BaseRng, range: &R) -> T {
    let low = match range.start_bound() {
        Bound::Included(&low) | Bound::Excluded(&low) => low,
        Bound::Unbounded => T::MIN,
    };

    let high = match range.end_bound() {
        Bound::Included(&high) | Bound::Excluded(&high) => high,
        Bound::Unbounded => T::MAX,
    };

    let inclusive = Inclusive::from_bounds(range);

    // Our generator is able to generate floats with one or both sides of the
    // range open. However, it can't generate a float from the range closed on
    // both sides. For this case, we divide the scale by maximum random number
    // which "stretches" the range to include both sides. This is the approach
    // used in rand crate:
    // https://github.com/rust-random/rand/blob/f3dd0b885c4597b9617ca79987a0dd899ab29fcb/src/distributions/uniform.rs#L953
    let scale = match inclusive {
        Inclusive::None | Inclusive::Left | Inclusive::Right => high - low,
        Inclusive::Both => (high - low) / T::max_rand(),
    };

    if scale.is_finite() {
        // Generate a random number between 0 and 1, where the bounds are
        // included based on the desired range inclusiveness.
        let r = match inclusive {
            Inclusive::None => T::gen_open_01_open(rng),
            Inclusive::Right => T::gen_open_01_close(rng),
            Inclusive::Left => T::gen_close_01_open(rng),
            // Inclusiveness on both sides is achieved by stretching the scale
            // above.
            Inclusive::Both => T::gen_close_01_open(rng),
        };

        r * scale + low
    } else {
        // Scale not being finite means that the range is wider than the float
        // type can represent (or that at least one side is not finite). In such
        // case, we need to fall back into the following technique which does a
        // bit more work but can handle such ranges. Source:
        // https://medium.com/analytics-vidhya/random-floats-in-any-range-9b40d30b637b
        let high_half = T::HALF * high;
        let low_half = T::HALF * low;
        let mid_point = high_half + low_half;

        // Decide if we generate the value to the right or left from the middle
        // point. We always want to have a chance that the middle point is
        // sampled, so we can't use the (0, 1] trick with one-side inclusive
        // ranges. That is why we stretch those in appropriate cases.
        let (r, stretch) = if rng.bool() {
            let stretch = match inclusive {
                Inclusive::None | Inclusive::Left => false,
                Inclusive::Right | Inclusive::Both => true,
            };
            let r = T::gen_close_01_open(rng);
            (r, stretch)
        } else {
            let stretch = match inclusive {
                Inclusive::None | Inclusive::Right => false,
                Inclusive::Left | Inclusive::Both => true,
            };
            let r = -T::gen_close_01_open(rng);
            (r, stretch)
        };

        let half_scale = if stretch {
            let half_scale = (high_half - low_half) / T::max_rand();
            if half_scale.is_finite() {
                half_scale
            } else {
                // If the range is so extreme that it can't be stretched,
                // use the standard scale.
                high_half - low_half
            }
        } else {
            high_half - low_half
        };

        r * half_scale + mid_point
    }
}

#[cfg(test)]
mod tests {
    use core::ops::RangeBounds;

    use fastrand::Rng;

    use super::*;

    #[test]
    fn f32_range_in_bounds() {
        let mut rng = Rng::with_seed(42);

        let range = -2.0..2.0;
        for _ in 0..10000 {
            assert!(range.contains(&float_range_impl(&mut rng, range.clone())));
        }
    }

    #[test]
    fn f32_range_wide_range_in_bounds() {
        let mut rng = Rng::with_seed(42);

        let range = f32::MIN..f32::MAX;
        for _ in 0..10000 {
            assert!(range.contains(&float_range_impl(&mut rng, range.clone())));
        }
    }

    #[test]
    fn f32_range_unbounded_finite() {
        let mut rng = Rng::with_seed(42);

        let range = ..;
        for _ in 0..10000 {
            assert!(&float_range_impl::<f32>(&mut rng, range).is_finite());
        }
    }

    fn assert_in_range<T>(range: impl RangeBounds<T> + Clone, mut samples: impl FnMut() -> T)
    where
        T: PartialOrd + core::fmt::Debug,
    {
        for _ in 0..200 {
            let sample = samples();
            assert!(range.contains(&sample), "{:?}", sample);
        }
    }

    #[test]
    fn adjacent_endpoint_is_excluded() {
        let upper32 = f32::from_bits(1.0f32.to_bits() + 1);
        let upper64 = f64::from_bits(1.0f64.to_bits() + 1);
        let mut rng = Rng::with_seed(42);
        assert_in_range(1.0f32..upper32, || float_range_impl(&mut rng, 1.0..upper32));
        assert_in_range(1.0f64..upper64, || float_range_impl(&mut rng, 1.0..upper64));
    }

    #[test]
    fn signed_zero_subnormal_and_wide_ranges_stay_inside() {
        let mut rng = Rng::with_seed(7);
        assert_in_range(-1.0f32..0.0, || float_range_impl(&mut rng, -1.0..0.0));
        let low = f32::from_bits(1);
        let high = f32::from_bits(4);
        assert_in_range(low..high, || float_range_impl(&mut rng, low..high));
        assert_in_range(f64::MIN..f64::MAX, || {
            float_range_impl(&mut rng, f64::MIN..f64::MAX)
        });
        let sample = float_range_impl(&mut rng, -0.0f64..=0.0);
        assert!(sample == 0.0);
    }

    #[test]
    fn empty_and_non_finite_ranges_panic() {
        let mut rng = Rng::with_seed(1);
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            float_range_impl(&mut rng, 1.0f32..1.0)
        }))
        .is_err());
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            float_range_impl(&mut rng, f32::NAN..1.0)
        }))
        .is_err());
    }
}
