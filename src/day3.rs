use std::arch::x86_64::{_mm_hadd_epi16, _mm_shuffle_epi8};

use super::*;

static LUT: [u8x16; 1 << 16] =
    unsafe { transmute(*include_bytes!(concat!(env!("OUT_DIR"), "/day3lut.bin"))) };

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn inner1(s: &str) -> u32 {
    let r = s.as_bytes().as_ptr_range();
    let mut ptr = r.start;
    let end = r.end;
    let lut = &LUT;
    let mut sum = 0;
    'chunk: loop {
        let chunk = (ptr as *const u8x32).read_unaligned();
        let is_u = chunk.simd_eq(Simd::splat(b'u'));
        let mut u_mask = is_u.to_bitmask() as u32;
        loop {
            let u_offset = u_mask.trailing_zeros();
            if u_offset == 32 {
                ptr = ptr.add(32);
                if ptr < end {
                    continue 'chunk;
                }
                return sum;
            }
            let instruction = (ptr.add(u_offset as _).sub(1) as *const u8x16).read_unaligned();
            let normalized = instruction - Simd::splat(b'0');
            let is_digit = normalized.simd_lt(Simd::splat(10));
            let digit_mask = is_digit.to_bitmask() as u32;
            let shuffle_idx = *lut.get_unchecked(digit_mask as usize);
            let discombobulated: i8x16 =
                _mm_shuffle_epi8(normalized.into(), shuffle_idx.into()).into();
            let is_correct = discombobulated.simd_eq(Simd::from_array([
                0, 0, 0, 0, 0, 0, 0, 0, 61, 69, 60, -8, -4, -7, 0, 0,
            ])) | Mask::from_array([
                true, true, true, true, true, true, true, true, //
                false, false, false, false, false, false, true, true, //
            ]);
            u_mask &= u_mask - 1;
            if !is_correct.all() {
                continue;
            }
            let two_digit = _mm_maddubs_epi16(
                discombobulated.into(),
                u8x16::from_array([100, 10, 1, 0, 100, 10, 1, 0, 100, 10, 1, 0, 100, 10, 1, 0])
                    .into(),
            );
            let three_digit: u16x8 = _mm_hadd_epi16(two_digit, two_digit).into();
            sum += three_digit[0] as u32 * three_digit[1] as u32;
        }
    }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn inner2(s: &str) -> u32 {
    0
}

pub fn part1(s: &str) -> impl Display {
    unsafe { inner1(s) }
}

pub fn part2(s: &str) -> impl Display {
    unsafe { inner2(s) }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test() {
        let s = read_to_string("./inputs/3.txt").unwrap();
        let s = s.as_str();

        assert_eq!(
            part1(s).to_string(),
            read_to_string("./outputs/3p1.txt").unwrap(),
        );
        assert_eq!(
            part2(s).to_string(),
            read_to_string("./outputs/3p2.txt").unwrap(),
        );
    }
}
