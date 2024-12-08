#![feature(thread_local, portable_simd, core_intrinsics)]
#![allow(
    clippy::pointers_in_nomem_asm_block,
    clippy::erasing_op,
    static_mut_refs,
    internal_features,
    clippy::missing_safety_doc,
    clippy::identity_op,
    clippy::zero_prefixed_literal
)]

#[allow(unused)]
use std::{
    arch::{
        asm,
        x86_64::{
            __m256i, _mm256_madd_epi16, _mm256_maddubs_epi16, _mm256_movemask_epi8,
            _mm256_shuffle_epi8, _mm_hadd_epi16, _mm_madd_epi16, _mm_maddubs_epi16,
            _mm_movemask_epi8, _mm_packus_epi32, _mm_shuffle_epi8, _mm_testc_si128, _pext_u32,
        },
    },
    fmt::Display,
    mem::{offset_of, transmute, MaybeUninit},
    simd::prelude::*,
};

#[allow(unused)]
macro_rules! black_box {
    ($thing:expr) => {{
        let mut thing = $thing;
        asm!(
            "/*{t}*/",
            t = inout(reg) thing,
            options(pure, nomem, preserves_flags, nostack)
        );
        thing
    }};
}

#[macro_use]
extern crate aoc_runner_derive;
extern crate aoc_runner;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

aoc_lib! { year = 2024 }
