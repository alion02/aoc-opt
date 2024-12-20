use super::*;

const ROW: usize = 141;
const SIDE: usize = ROW - 1;
const BYTES: usize = ROW * SIDE;
const MARGIN: usize = ROW + 32;
const TABLE_WITH_MARGINS: usize = BYTES + MARGIN * 2;

unsafe fn inner1(s: &[u8]) -> u32 {
    static mut EDGES: [i8; TABLE_WITH_MARGINS] = [-1; TABLE_WITH_MARGINS];

    let edges = EDGES.as_mut_ptr().add(MARGIN).cast::<i8x32>();
    let ptr = s.as_ptr().cast::<i8x32>();

    for off in (0..SIDE).step_by(32) {
        let off = off.min(SIDE - 32);
        let mut up = Simd::splat(0);
        let mut mid = ptr.byte_add(off).read_unaligned();
        for row in 0..SIDE {
            let off = off + row * ROW;
            let left = if off == 0 {
                simd_swizzle!(ptr.read_unaligned(), Simd::splat(0), [
                    32, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
                    26, 27, 28, 29, 30,
                ])
            } else {
                ptr.byte_add(off - 1).read_unaligned()
            };
            let right = ptr.byte_add(off + 1).read_unaligned();
            let down = if row + 1 == SIDE {
                Simd::splat(0)
            } else {
                ptr.byte_add(off + ROW).read_unaligned()
            };
            let up_mask = mid.simd_eq(up).to_int() & Simd::splat(8);
            let left_mask = mid.simd_eq(left).to_int() & Simd::splat(4);
            let right_mask = mid.simd_eq(right).to_int() & Simd::splat(1);
            let down_mask = mid.simd_eq(down).to_int() & Simd::splat(2);
            edges
                .byte_add(off)
                .write_unaligned(up_mask | left_mask | right_mask | down_mask);
            up = mid;
            mid = down;
        }
    }

    let mut total = 0;
    for i in 0..BYTES - 1 {
        let (mut area, mut len) = (0, 0);
        asm!(
            "call 20f",
            "jmp 99f",
        "20:",
            "movzx {cell:e}, byte ptr[{ptr}]",
            "xor {cell:l}, 15",
            "jns 21f",
            "ret",
        "21:",
            "mov byte ptr[{ptr}], -1",
            "inc {area:e}",
            "popcnt {tmp:e}, {cell:e}",
            "add {len:e}, {tmp:e}",
            "push {cell}",
            "inc {ptr}",
            "test {cell:l}, 1",
            "jnz 30f",
            "call 20b",
        "30:",
            "add {ptr}, 140",
            "test byte ptr[rsp], 2",
            "jnz 30f",
            "call 20b",
        "30:",
            "add {ptr}, -142",
            "test byte ptr[rsp], 4",
            "jnz 30f",
            "call 20b",
        "30:",
            "add {ptr}, -140",
            "test byte ptr[rsp], 8",
            "jnz 30f",
            "call 20b",
        "30:",
            "add {ptr}, 141",
            "pop {tmp}",
            "ret",
        "99:",
            tmp = out(reg) _,
            cell = out(reg) _,
            area = inout(reg) area,
            len = inout(reg) len,
            ptr = inout(reg) edges.cast::<i8>().add(i) => _,
        );
        total += area * len;
    }

    total
}

unsafe fn inner2(s: &[u8]) -> u32 {
    static mut CTRL_MASKS: [i8; TABLE_WITH_MARGINS] = [-1; TABLE_WITH_MARGINS];

    let masks = CTRL_MASKS.as_mut_ptr().add(MARGIN).cast::<i8x32>();
    let ptr = s.as_ptr().cast::<i8x32>();

    for off in (0..SIDE).step_by(32) {
        let off = off.min(SIDE - 32);
        let mut ul = Simd::splat(0);
        let mut um = Simd::splat(0);
        let mut ur = Simd::splat(0);
        let mut ml = if off == 0 {
            simd_swizzle!(ptr.read_unaligned(), Simd::splat(0), [
                32, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
                27, 28, 29, 30,
            ])
        } else {
            ptr.byte_add(off - 1).read_unaligned()
        };
        let mut mm = ptr.byte_add(off).read_unaligned();
        let mut mr = ptr.byte_add(off + 1).read_unaligned();
        for row in 0..SIDE {
            let off = off + row * ROW;
            let [dl, dm, dr] = if row + 1 == SIDE {
                [Simd::splat(0); 3]
            } else {
                [
                    ptr.byte_add(off + ROW - 1).read_unaligned(),
                    ptr.byte_add(off + ROW).read_unaligned(),
                    ptr.byte_add(off + ROW + 1).read_unaligned(),
                ]
            };

            let ul_mask = mm.simd_eq(ul).to_int();
            let um_mask = mm.simd_eq(um).to_int();
            let ur_mask = mm.simd_eq(ur).to_int();
            let ml_mask = mm.simd_eq(ml).to_int();
            let mr_mask = mm.simd_eq(mr).to_int();
            let dl_mask = mm.simd_eq(dl).to_int();
            let dm_mask = mm.simd_eq(dm).to_int();
            let dr_mask = mm.simd_eq(dr).to_int();

            macro_rules! is_corner {
                ($diag:expr, $side1:expr, $side2:expr) => {
                    (!$diag & $side1 & $side2) | (!$side1 & !$side2)
                };
            }
            let dr_corner = is_corner!(dr_mask, dm_mask, mr_mask) & Simd::splat(16);
            let dl_corner = is_corner!(dl_mask, ml_mask, dm_mask) & Simd::splat(16);
            let ul_corner = is_corner!(ul_mask, um_mask, ml_mask) & Simd::splat(16);
            let ur_corner = is_corner!(ur_mask, mr_mask, um_mask) & Simd::splat(16);

            let corners = dr_corner + dl_corner + ul_corner + ur_corner;
            let dirs = (mr_mask & Simd::splat(1))
                | (dm_mask & Simd::splat(2))
                | (ml_mask & Simd::splat(4))
                | (um_mask & Simd::splat(8));

            masks.byte_add(off).write_unaligned(corners | dirs);

            ul = ml;
            um = mm;
            ur = mr;
            ml = dl;
            mm = dm;
            mr = dr;
        }
    }

    let mut total = 0;
    for i in 0..BYTES - 1 {
        let (mut area, mut len) = (0, 0);
        asm!(
            "call 20f",
            "jmp 99f",
        "20:",
            "movzx {cell:e}, byte ptr[{ptr}]",
            "xor {cell:l}, 15",
            "jns 21f",
            "ret",
        "21:",
            "mov byte ptr[{ptr}], -1",
            "inc {area:e}",
            "shrx {tmp:e}, {cell:e}, {four:e}",
            "add {len:e}, {tmp:e}",
            "push {cell}",
            "inc {ptr}",
            "test {cell:l}, 1",
            "jnz 30f",
            "call 20b",
        "30:",
            "add {ptr}, 140",
            "test byte ptr[rsp], 2",
            "jnz 30f",
            "call 20b",
        "30:",
            "add {ptr}, -142",
            "test byte ptr[rsp], 4",
            "jnz 30f",
            "call 20b",
        "30:",
            "add {ptr}, -140",
            "test byte ptr[rsp], 8",
            "jnz 30f",
            "call 20b",
        "30:",
            "add {ptr}, 141",
            "pop {tmp}",
            "ret",
        "99:",
            tmp = out(reg) _,
            cell = out(reg) _,
            area = inout(reg) area,
            len = inout(reg) len,
            ptr = inout(reg) masks.cast::<i8>().add(i) => _,
            four = in(reg) 4,
        );
        total += area * len;
    }

    total
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
    fn p1() {
        let s = read_to_string("./inputs/12.txt").unwrap();
        let s = s.as_str();

        assert_eq!(part1(s).to_string(), read_to_string("./outputs/12p1.txt").unwrap(),)
    }

    #[test]
    fn p2() {
        let s = read_to_string("./inputs/12.txt").unwrap();
        let s = s.as_str();

        assert_eq!(part2(s).to_string(), read_to_string("./outputs/12p2.txt").unwrap(),);
    }
}
