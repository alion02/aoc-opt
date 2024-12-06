use std::arch::{asm, x86_64::_mm_testc_si128};

use super::*;

// 5-23 numbers in list
// all numbers 2 digit

static mut MATRIX: [u8x32; 50] = [u8x32::from_array([0; 32]); 50];

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
#[allow(unreachable_code)]
unsafe fn inner1(s: &[u8]) -> u32 {
    let matrix = &mut MATRIX;
    // {
    //     MATRIX[0] = Simd::splat(0);
    //     MATRIX[1] = Simd::splat(0);
    //     MATRIX[2] = Simd::splat(0);
    //     MATRIX[3] = Simd::splat(0);
    //     MATRIX[4] = Simd::splat(0);
    //     MATRIX[5] = Simd::splat(0);
    //     MATRIX[6] = Simd::splat(0);
    //     MATRIX[7] = Simd::splat(0);
    //     MATRIX[8] = Simd::splat(0);
    //     MATRIX[9] = Simd::splat(0);
    //     MATRIX[10] = Simd::splat(0);
    //     MATRIX[11] = Simd::splat(0);
    //     MATRIX[12] = Simd::splat(0);
    //     MATRIX[13] = Simd::splat(0);
    //     MATRIX[14] = Simd::splat(0);
    //     MATRIX[15] = Simd::splat(0);
    //     MATRIX[16] = Simd::splat(0);
    //     MATRIX[17] = Simd::splat(0);
    //     MATRIX[18] = Simd::splat(0);
    //     MATRIX[19] = Simd::splat(0);
    //     MATRIX[20] = Simd::splat(0);
    //     MATRIX[21] = Simd::splat(0);
    //     MATRIX[22] = Simd::splat(0);
    //     MATRIX[23] = Simd::splat(0);
    //     MATRIX[24] = Simd::splat(0);
    //     MATRIX[25] = Simd::splat(0);
    //     MATRIX[26] = Simd::splat(0);
    //     MATRIX[27] = Simd::splat(0);
    //     MATRIX[28] = Simd::splat(0);
    //     MATRIX[29] = Simd::splat(0);
    //     MATRIX[30] = Simd::splat(0);
    //     MATRIX[31] = Simd::splat(0);
    //     MATRIX[32] = Simd::splat(0);
    //     MATRIX[33] = Simd::splat(0);
    //     MATRIX[34] = Simd::splat(0);
    //     MATRIX[35] = Simd::splat(0);
    //     MATRIX[36] = Simd::splat(0);
    //     MATRIX[37] = Simd::splat(0);
    //     MATRIX[38] = Simd::splat(0);
    //     MATRIX[39] = Simd::splat(0);
    //     MATRIX[40] = Simd::splat(0);
    //     MATRIX[41] = Simd::splat(0);
    //     MATRIX[42] = Simd::splat(0);
    //     MATRIX[43] = Simd::splat(0);
    //     MATRIX[44] = Simd::splat(0);
    // }

    asm!(
        "vmovdqa ymmword ptr[{table} + 160], {zero}",
        "vmovdqa ymmword ptr[{table} + 192], {zero}",
        "vmovdqa ymmword ptr[{table} + 224], {zero}",
        "vmovdqa ymmword ptr[{table} + 256], {zero}",
        "vmovdqa ymmword ptr[{table} + 288], {zero}",
        "vmovdqa ymmword ptr[{table} + 320], {zero}",
        "vmovdqa ymmword ptr[{table} + 352], {zero}",
        "vmovdqa ymmword ptr[{table} + 384], {zero}",
        "vmovdqa ymmword ptr[{table} + 416], {zero}",
        "vmovdqa ymmword ptr[{table} + 448], {zero}",
        "vmovdqa ymmword ptr[{table} + 480], {zero}",
        "vmovdqa ymmword ptr[{table} + 512], {zero}",
        "vmovdqa ymmword ptr[{table} + 544], {zero}",
        "vmovdqa ymmword ptr[{table} + 576], {zero}",
        "vmovdqa ymmword ptr[{table} + 608], {zero}",
        "vmovdqa ymmword ptr[{table} + 640], {zero}",
        "vmovdqa ymmword ptr[{table} + 672], {zero}",
        "vmovdqa ymmword ptr[{table} + 704], {zero}",
        "vmovdqa ymmword ptr[{table} + 736], {zero}",
        "vmovdqa ymmword ptr[{table} + 768], {zero}",
        "vmovdqa ymmword ptr[{table} + 800], {zero}",
        "vmovdqa ymmword ptr[{table} + 832], {zero}",
        "vmovdqa ymmword ptr[{table} + 864], {zero}",
        "vmovdqa ymmword ptr[{table} + 896], {zero}",
        "vmovdqa ymmword ptr[{table} + 928], {zero}",
        "vmovdqa ymmword ptr[{table} + 960], {zero}",
        "vmovdqa ymmword ptr[{table} + 992], {zero}",
        "vmovdqa ymmword ptr[{table} + 1024], {zero}",
        "vmovdqa ymmword ptr[{table} + 1056], {zero}",
        "vmovdqa ymmword ptr[{table} + 1088], {zero}",
        "vmovdqa ymmword ptr[{table} + 1120], {zero}",
        "vmovdqa ymmword ptr[{table} + 1152], {zero}",
        "vmovdqa ymmword ptr[{table} + 1184], {zero}",
        "vmovdqa ymmword ptr[{table} + 1216], {zero}",
        "vmovdqa ymmword ptr[{table} + 1248], {zero}",
        "vmovdqa ymmword ptr[{table} + 1280], {zero}",
        "vmovdqa ymmword ptr[{table} + 1312], {zero}",
        "vmovdqa ymmword ptr[{table} + 1344], {zero}",
        "vmovdqa ymmword ptr[{table} + 1376], {zero}",
        "vmovdqa ymmword ptr[{table} + 1408], {zero}",
        "vmovdqa ymmword ptr[{table} + 1440], {zero}",
        "vmovdqa ymmword ptr[{table} + 1472], {zero}",
        "vmovdqa ymmword ptr[{table} + 1504], {zero}",
        "vmovdqa ymmword ptr[{table} + 1536], {zero}",
        "vmovdqa ymmword ptr[{table} + 1568], {zero}",
        table = in(reg) matrix,
        zero = in(ymm_reg) u8x32::splat(0),
    );

    let mut sum = 0;
    let mut i = 0;
    let table: &mut [u8; 1600] = transmute(matrix);

    macro_rules! parse {
        ($off:expr$(, $on_newline:expr)?) => {{
            let v1 = *s.get_unchecked(i + $off) as u32;
            if v1 == b'\n' as u32 {
                $($on_newline)?
            }
            let v2 = *s.get_unchecked(i + $off + 1) as u32;
            v1 * 10 + v2 - 528
        }};
    }

    loop {
        let x = parse!(0, break);
        let y = parse!(3);
        *table.get_unchecked_mut((x * 16 + y / 8) as usize) |= 1u8.wrapping_shl(y);
        i += 6;
    }

    i += 1;

    'outer: loop {
        let [a, b, c, d, e] = [parse!(0), parse!(3), parse!(6), parse!(9), parse!(12)];
        let mut j = i + 6;
        i += 15;
        let mut wrong_order =
            *table.get_unchecked((b * 16 + a / 8) as usize) & 1u8.wrapping_shl(a) != 0;
        wrong_order |= *table.get_unchecked((c * 16 + b / 8) as usize) & 1u8.wrapping_shl(b) != 0;
        wrong_order |= *table.get_unchecked((d * 16 + c / 8) as usize) & 1u8.wrapping_shl(c) != 0;
        wrong_order |= *table.get_unchecked((e * 16 + d / 8) as usize) & 1u8.wrapping_shl(d) != 0;

        let mut p = e;
        loop {
            if *s.get_unchecked(i - 1) == b'\n' {
                if i == s.len() {
                    break 'outer;
                }
                break;
            }
            let n1 = parse!(0);
            let n2 = parse!(3);
            i += 6;
            j += 3;
            wrong_order |=
                *table.get_unchecked((n1 * 16 + p / 8) as usize) & 1u8.wrapping_shl(p) != 0;
            wrong_order |=
                *table.get_unchecked((n2 * 16 + n1 / 8) as usize) & 1u8.wrapping_shl(n1) != 0;
            p = n2;
        }

        if !wrong_order {
            sum += {
                let v1 = *s.get_unchecked(j) as u32;
                let v2 = *s.get_unchecked(j + 1) as u32;
                v1 * 10 + v2 - 528
            };
        }
    }

    sum
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn inner2(s: &[u8]) -> u32 {
    0
}

pub fn part1(s: &str) -> impl Display {
    unsafe { inner1(s.as_bytes()) }
}

pub fn part2(s: &str) -> impl Display {
    unsafe { inner2(s.as_bytes()) }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test() {
        let s = read_to_string("./inputs/5.txt").unwrap();
        let s = s.as_str();

        assert_eq!(
            part1(s).to_string(),
            read_to_string("./outputs/5p1.txt").unwrap(),
        );
        assert_eq!(
            part2(s).to_string(),
            read_to_string("./outputs/5p2.txt").unwrap(),
        );
    }
}
