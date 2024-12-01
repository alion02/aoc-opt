#![allow(
    unsafe_op_in_unsafe_fn,
    clippy::missing_safety_doc,
    clippy::identity_op
)]

use std::fmt::Display;

pub fn part1(s: &str) -> impl Display {
    unsafe fn inner(s: &str) -> impl Display {
        let s = s.as_bytes();

        let mut left = [0u8; 100000];
        let mut right = [0u8; 100000];

        for i in (0..).step_by(14).take(1000) {
            let a = *s.get_unchecked(i + 0) as u32 * 10000
                + *s.get_unchecked(i + 1) as u32 * 1000
                + *s.get_unchecked(i + 2) as u32 * 100
                + *s.get_unchecked(i + 3) as u32 * 10
                + *s.get_unchecked(i + 4) as u32 * 1
                - 533328;

            *left.get_unchecked_mut(a as usize) += 1;

            let b = *s.get_unchecked(i + 8) as u32 * 10000
                + *s.get_unchecked(i + 9) as u32 * 1000
                + *s.get_unchecked(i + 10) as u32 * 100
                + *s.get_unchecked(i + 11) as u32 * 10
                + *s.get_unchecked(i + 12) as u32 * 1
                - 533328;

            *right.get_unchecked_mut(b as usize) += 1;
        }

        let mut i = 10000;
        let mut j = 10000;

        let mut sum = 0;

        'outer: loop {
            while left[i] == 0 {
                i += 1;

                if i == 100000 {
                    break 'outer;
                }
            }

            while right[j] == 0 {
                j += 1;

                if j == 100000 {
                    break 'outer;
                }
            }

            sum += left[i].abs_diff(right[j]);

            left[i] -= 1;
            right[j] -= 1;
        }

        sum
    }

    unsafe { inner(s) }
}

pub fn part2(s: &str) -> impl Display {
    unsafe fn inner(s: &str) -> impl Display {
        let s = s.as_bytes();

        let mut left = [0u8; 100000];
        let mut right = [0u8; 100000];

        for i in (0..).step_by(14).take(1000) {
            let a = *s.get_unchecked(i + 0) as u32 * 10000
                + *s.get_unchecked(i + 1) as u32 * 1000
                + *s.get_unchecked(i + 2) as u32 * 100
                + *s.get_unchecked(i + 3) as u32 * 10
                + *s.get_unchecked(i + 4) as u32 * 1
                - 533328;

            *left.get_unchecked_mut(a as usize) += 1;

            let b = *s.get_unchecked(i + 8) as u32 * 10000
                + *s.get_unchecked(i + 9) as u32 * 1000
                + *s.get_unchecked(i + 10) as u32 * 100
                + *s.get_unchecked(i + 11) as u32 * 10
                + *s.get_unchecked(i + 12) as u32 * 1
                - 533328;

            *right.get_unchecked_mut(b as usize) += 1;
        }

        let mut i = 10000;
        let mut j = 10000;

        let mut sum = 0;

        'outer: loop {
            while left[i] == 0 {
                i += 1;

                if i == 100000 {
                    break 'outer;
                }
            }

            while right[j] == 0 {
                j += 1;

                if j == 100000 {
                    break 'outer;
                }
            }

            sum += left[i].abs_diff(right[j]);

            left[i] -= 1;
            right[j] -= 1;
        }

        sum
    }

    unsafe { inner(s) }
}
