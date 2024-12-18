#![allow(unused)]

use std::{arch::x86_64::_pext_u64, collections::HashMap, env::var, fs::File, io::Write, path::PathBuf};

fn write_lut_d2() {
    let mut lut = vec![255; 1 << 26].into_boxed_slice();

    for (i, vec) in lut.chunks_exact_mut(32).enumerate() {
        let mut mask = (!i & 1) << 1 | i << 2 | 1 << 23;
        let mut i = 0;
        let mut j = 0;

        macro_rules! set {
            ($i:expr, $j:expr) => {
                vec[$j + $i / 16 * 16] = ($i % 16) as u8;
            };
        }

        loop {
            let num_len = mask.trailing_zeros();
            if num_len == 1 {
                set!(i, j + 1);
                i += 2;
            } else if num_len == 2 {
                set!(i, j);
                set!(i + 1, j + 1);
                i += 3;
            } else {
                break;
            }

            j += 2;
            if j == 16 {
                break;
            }

            mask >>= num_len + 1;
        }
    }

    let mut path: PathBuf = var("OUT_DIR").unwrap().into();
    path.push("day2lut.bin");

    File::create(path).unwrap().write_all(&lut).unwrap();
}

fn write_lut_d3() {
    let mut lut = vec![255; 1 << 11].into_boxed_slice();

    for (i, vec) in lut.chunks_exact_mut(16).enumerate() {
        let mut mask = i ^ 0xFF;
        let first_len = mask.trailing_zeros() as usize;
        mask &= mask - 1;
        mask >>= first_len + 1;
        let second_len = mask.trailing_zeros() as usize;
        if !(1..=3).contains(&first_len) || !(1..=3).contains(&second_len) {
            continue;
        }
        let first_num = 4;
        let comma = first_num + first_len;
        let second_num = comma + 1;
        let close = second_num + second_len;
        for i in 0..first_len {
            vec[3 - first_len + i] = i as u8 + first_num as u8;
        }
        for i in 0..second_len {
            vec[4 + 3 - second_len + i] = i as u8 + second_num as u8;
        }
        vec[8] = 0;
        vec[9] = 1;
        vec[10] = 2;
        vec[11] = 3;
        vec[12] = comma as u8;
        vec[13] = close as u8;
    }

    let mut path: PathBuf = var("OUT_DIR").unwrap().into();
    path.push("day3lut.bin");

    File::create(path).unwrap().write_all(&lut).unwrap();
}

fn write_lut_d11() {
    macro_rules! write {
        ($d:expr, $u:ty) => {
            let cache = &mut HashMap::new();
            let lut = (0..10_000_000)
                .flat_map(|s| {
                    fn process_stone(cache: &mut HashMap<(u64, u32), $u>, s: u64, d: u32) -> $u {
                        if d == 0 {
                            return 1;
                        }

                        if let Some(v) = cache.get(&(s, d)) {
                            return *v;
                        }

                        let res = if s == 0 {
                            process_stone(cache, 1, d - 1)
                        } else {
                            let digits = s.ilog10() + 1;
                            if digits % 2 == 0 {
                                let div = 10u64.pow(digits / 2);
                                process_stone(cache, s / div, d - 1) + process_stone(cache, s % div, d - 1)
                            } else {
                                process_stone(cache, s * 2024, d - 1)
                            }
                        };

                        cache.insert((s, d), res);
                        res
                    }

                    process_stone(cache, s, $d).to_ne_bytes()
                })
                .collect::<Vec<_>>();

            let mut path: PathBuf = var("OUT_DIR").unwrap().into();
            path.push(format!("day11_depth{}.bin", $d));
            File::create(path).unwrap().write_all(&lut).unwrap();
        };
    }

    write!(25, u32);
    write!(75, u64);
}

fn write_lut_d17() {
    let mut lut = vec![0; 1 << 17];

    for (prog, a) in [
        ("2,4,1,0,7,5,4,0,1,4,5,5,0,3,3,0", 174271416563282u64),
        ("2,4,1,0,7,5,0,3,4,0,1,5,5,5,3,0", 202730212714147u64),
        ("2,4,1,0,7,5,0,3,1,5,4,0,5,5,3,0", 191025017546403u64),
        ("2,4,1,0,7,5,1,5,0,3,4,0,5,5,3,0", 202806544852643u64),
        ("2,4,1,0,7,5,1,5,4,0,0,3,5,5,3,0", 189432989771427u64),
        ("2,4,1,0,7,5,4,0,1,7,5,5,0,3,3,0", 259246506806003u64),
        ("2,4,1,0,7,5,4,1,1,4,5,5,0,3,3,0", 174271403980370u64),
        ("2,4,1,0,7,5,0,3,4,1,1,5,5,5,3,0", 202730346931875u64),
        ("2,4,1,0,7,5,4,1,0,3,1,5,5,5,3,0", 189120954525347u64),
        ("2,4,1,0,7,5,4,1,1,5,0,3,5,5,3,0", 189432876525219u64),
        ("2,4,1,0,7,5,0,3,1,5,4,1,5,5,3,0", 190992268420771u64),
        ("2,4,1,0,7,5,1,5,0,3,4,1,5,5,3,0", 202815134787235u64),
        ("2,4,1,0,7,5,1,5,4,1,0,3,5,5,3,0", 189171097429667u64),
        ("2,4,1,0,7,5,1,4,4,2,5,5,0,3,3,0", 174270842402386u64),
        ("2,4,1,0,7,5,0,3,4,2,1,5,5,5,3,0", 202730615367331u64),
        ("2,4,1,0,7,5,0,3,1,5,4,2,5,5,3,0", 202858726188707u64),
        ("2,4,1,0,7,5,1,5,0,3,4,2,5,5,3,0", 202832314656419u64),
        ("2,4,1,0,7,5,1,5,4,2,0,3,5,5,3,0", 189433392424611u64),
        ("2,4,1,0,7,5,4,2,1,7,5,5,0,3,3,0", 259234225883891u64),
        ("2,4,1,0,7,5,0,3,4,3,1,5,5,5,3,0", 202730883802787u64),
        ("2,4,1,0,7,5,4,3,1,5,0,3,5,5,3,0", 202744693548707u64),
        ("2,4,1,0,7,5,4,3,1,5,5,5,0,3,3,0", 203728106841763u64),
        ("2,4,1,0,7,5,0,3,1,5,4,3,5,5,3,0", 202743298942627u64),
        ("2,4,1,0,7,5,1,5,0,3,4,3,5,5,3,0", 202743898728099u64),
        ("2,4,1,0,7,5,4,3,1,7,5,5,0,3,3,0", 259246498679539u64),
        ("2,4,1,0,7,5,1,4,4,5,5,5,0,3,3,0", 174271345718866u64),
        ("2,4,1,0,7,5,0,3,4,5,1,5,5,5,3,0", 202730078496419u64),
        ("2,4,1,0,7,5,4,5,0,3,1,5,5,5,3,0", 189121157949091u64),
        ("2,4,1,0,7,5,4,5,1,5,0,3,5,5,3,0", 202744691451555u64),
        ("2,4,1,0,7,5,4,5,1,5,5,5,0,3,3,0", 203728104744611u64),
        ("2,4,1,0,7,5,1,5,0,3,4,5,5,5,3,0", 202797954918051u64),
        ("2,4,1,0,7,5,1,5,4,5,0,3,5,5,3,0", 189171231647395u64),
        ("2,4,1,0,7,5,4,6,1,4,5,5,0,3,3,0", 174271408174674u64),
        ("2,4,1,0,7,5,0,3,4,6,1,5,5,5,3,0", 202799737497251u64),
        ("2,4,1,0,7,5,4,7,1,4,5,5,0,3,3,0", 174271412368978u64),
        ("2,4,1,0,7,5,4,7,0,3,1,5,5,5,3,0", 190805422663331u64),
        ("2,4,1,0,7,5,0,3,1,5,4,7,5,5,3,0", 189136842548899u64),
        ("2,4,1,0,7,5,1,5,0,3,4,7,5,5,3,0", 189137442334371u64),
        ("2,4,1,0,7,5,1,5,4,7,0,3,5,5,3,0", 189433660860067u64),
        ("2,4,1,0,7,5,4,7,1,7,5,5,0,3,3,0", 259234217757427u64),
        ("2,4,1,0,7,5,1,7,4,7,5,5,0,3,3,0", 259233547193075u64),
        ("2,4,1,1,7,5,4,0,1,4,0,3,5,5,3,0", 202366621067818u64),
        ("2,4,1,1,7,5,0,3,1,4,4,0,5,5,3,0", 202356708354602u64),
        ("2,4,1,1,7,5,1,4,4,0,0,3,5,5,3,0", 203197932644906u64),
        ("2,4,1,1,7,5,1,5,4,0,0,3,5,5,3,0", 164541160582845u64),
        ("2,4,1,1,7,5,1,5,4,0,5,5,0,3,3,0", 164279024971453u64),
        ("2,4,1,1,7,5,4,0,0,3,1,6,5,5,3,0", 247839653009594u64),
        ("2,4,1,1,7,5,4,1,1,4,0,3,5,5,3,0", 202366625262122u64),
        ("2,4,1,1,7,5,1,4,4,1,0,3,5,5,3,0", 202992055757354u64),
        ("2,4,1,1,7,5,1,5,4,1,5,5,0,3,3,0", 164278764924605u64),
        ("2,4,1,1,7,5,4,2,1,4,0,3,5,5,3,0", 202366618446378u64),
        ("2,4,1,1,7,5,0,3,1,4,4,2,5,5,3,0", 202631586261546u64),
        ("2,4,1,1,7,5,1,5,4,2,5,5,0,3,3,0", 164278496489149u64),
        ("2,4,1,1,7,5,4,3,1,4,0,3,5,5,3,0", 202366623164970u64),
        ("2,4,1,1,7,5,1,5,0,3,4,3,5,5,3,0", 164542125272765u64),
        ("2,4,1,1,7,5,1,5,4,3,0,3,5,5,3,0", 164541017976509u64),
        ("2,4,1,1,7,5,1,5,4,3,5,5,0,3,3,0", 164278899142333u64),
        ("2,4,1,1,7,5,0,3,4,3,1,6,5,5,3,0", 247839539763386u64),
        ("2,4,1,1,7,5,4,4,1,4,0,3,5,5,3,0", 202991746427434u64),
        ("2,4,1,1,7,5,0,3,1,4,4,4,5,5,3,0", 202975183645226u64),
        ("2,4,1,1,7,5,1,4,4,4,0,3,5,5,3,0", 202991774214698u64),
        ("2,4,1,1,7,5,1,5,0,3,4,4,5,5,3,0", 164516454365621u64),
        ("2,4,1,1,7,5,1,5,4,4,0,3,5,5,3,0", 164540221058749u64),
        ("2,4,1,1,7,5,1,5,4,4,5,5,0,3,3,0", 164278228053693u64),
        ("2,4,1,1,7,5,0,3,4,5,1,4,5,5,3,0", 202404828555818u64),
        ("2,4,1,1,7,5,4,5,1,4,0,3,5,5,3,0", 202367015856682u64),
        ("2,4,1,1,7,5,0,3,1,4,4,5,5,5,3,0", 202322348616234u64),
        ("2,4,1,1,7,5,1,4,0,3,4,5,5,5,3,0", 202322936867370u64),
        ("2,4,1,1,7,5,1,4,4,5,0,3,5,5,3,0", 202992189975082u64),
        ("2,4,1,1,7,5,1,5,4,5,0,3,5,5,3,0", 164540892147389u64),
        ("2,4,1,1,7,5,1,5,4,5,5,5,0,3,3,0", 165522963263165u64),
        ("2,4,1,1,7,5,4,5,0,3,1,6,5,5,3,0", 247839661398202u64),
        ("2,4,1,1,7,5,4,6,0,3,1,4,5,5,3,0", 202972175280682u64),
        ("2,4,1,1,7,5,4,6,1,4,0,3,5,5,3,0", 202366627359274u64),
        ("2,4,1,1,7,5,0,3,1,4,4,6,5,5,3,0", 203181342075434u64),
        ("2,4,1,1,7,5,1,5,4,6,0,3,5,5,3,0", 164540489494205u64),
        ("2,4,1,1,7,5,1,5,4,6,5,5,0,3,3,0", 165523634351805u64),
        ("2,4,1,1,7,5,4,7,1,4,0,3,5,5,3,0", 202367025818154u64),
        ("2,4,1,1,7,5,1,5,4,7,0,3,5,5,3,0", 164540623711933u64),
        ("2,4,1,1,7,5,1,5,4,7,5,5,0,3,3,0", 164278630706877u64),
        ("2,4,1,1,7,5,0,3,4,7,1,6,5,5,3,0", 247839002892474u64),
        ("2,4,1,1,7,5,4,7,0,3,1,6,5,5,3,0", 247839648815290u64),
        ("2,4,1,2,7,5,4,0,1,3,5,5,0,3,3,0", 37221263785460u64),
        ("2,4,1,2,7,5,1,3,4,0,5,5,0,3,3,0", 37221871304180u64),
        ("2,4,1,2,7,5,4,1,1,3,5,5,0,3,3,0", 37221261688308u64),
        ("2,4,1,2,7,5,1,3,4,1,5,5,0,3,3,0", 37221468650996u64),
        ("2,4,1,2,7,5,0,3,1,7,4,1,5,5,3,0", 190615597431823u64),
        ("2,4,1,2,7,5,1,7,0,3,4,1,5,5,3,0", 190354906758159u64),
        ("2,4,1,2,7,5,4,2,1,3,5,5,0,3,3,0", 38886108872180u64),
        ("2,4,1,2,7,5,1,3,4,2,5,5,0,3,3,0", 37222005521908u64),
        ("2,4,1,2,7,5,0,3,4,2,1,7,5,5,3,0", 190384709385231u64),
        ("2,4,1,2,7,5,1,7,4,2,0,3,5,5,3,0", 190389435055119u64),
        ("2,4,1,2,7,5,4,3,1,3,5,5,0,3,3,0", 37221267979764u64),
        ("2,4,1,2,7,5,1,3,4,3,5,5,0,3,3,0", 37221334433268u64),
        ("2,4,1,2,7,5,0,3,4,3,1,7,5,5,3,0", 190626778856463u64),
        ("2,4,1,2,7,5,4,3,0,3,1,7,5,5,3,0", 190384609508367u64),
        ("2,4,1,2,7,5,1,7,4,3,0,3,5,5,3,0", 190389300050959u64),
        ("2,4,1,2,7,5,1,3,4,4,5,5,0,3,3,0", 37222273957364u64),
        ("2,4,1,2,7,5,4,4,0,3,1,7,5,5,3,0", 190384623401999u64),
        ("2,4,1,2,7,5,1,7,4,4,0,3,5,5,3,0", 190593310997519u64),
        ("2,4,1,2,7,5,4,5,1,3,5,5,0,3,3,0", 37221270076916u64),
        ("2,4,1,2,7,5,0,3,4,5,1,7,5,5,3,0", 190384433347599u64),
        ("2,4,1,2,7,5,4,5,0,3,1,7,5,5,3,0", 190384615275535u64),
        ("2,4,1,2,7,5,1,7,4,5,0,3,5,5,3,0", 190593446001679u64),
        ("2,4,1,2,7,5,4,6,1,3,5,5,0,3,3,0", 37221276368372u64),
        ("2,4,1,2,7,5,4,7,1,3,5,5,0,3,3,0", 37221274271220u64),
        ("2,4,1,2,7,5,1,3,4,7,5,5,0,3,3,0", 37221737086452u64),
        ("2,4,1,2,7,5,0,3,4,7,1,7,5,5,3,0", 190384113204239u64),
        ("2,4,1,2,7,5,0,3,1,7,4,7,5,5,3,0", 190624228719631u64),
        ("2,4,1,2,7,5,1,7,0,3,4,7,5,5,3,0", 190561065188367u64),
        ("2,4,1,3,7,5,0,3,1,0,4,0,5,5,3,0", 87765778397121u64),
        ("2,4,1,3,7,5,0,3,4,0,1,4,5,5,3,0", 266931276004369u64),
        ("2,4,1,3,7,5,0,3,1,4,4,0,5,5,3,0", 280189034740753u64),
        ("2,4,1,3,7,5,0,3,4,0,1,5,5,5,3,0", 236581377105517u64),
        ("2,4,1,3,7,5,0,3,1,0,4,1,5,5,3,0", 87800138135489u64),
        ("2,4,1,3,7,5,0,3,1,4,4,1,5,5,3,0", 266943355599889u64),
        ("2,4,1,3,7,5,0,3,4,1,1,5,5,5,3,0", 236581108670061u64),
        ("2,4,1,3,7,5,4,1,1,5,5,5,0,3,3,0", 233391028921023u64),
        ("2,4,1,3,7,5,0,3,1,5,4,1,5,5,3,0", 216148338630253u64),
        ("2,4,1,3,7,5,1,5,0,3,4,1,5,5,3,0", 216549846240877u64),
        ("2,4,1,3,7,5,0,3,4,2,1,0,5,5,3,0", 87768596969409u64),
        ("2,4,1,3,7,5,0,3,1,0,4,2,5,5,3,0", 87815170521025u64),
        ("2,4,1,3,7,5,0,3,4,2,1,4,5,5,3,0", 266930722356241u64),
        ("2,4,1,3,7,5,0,3,1,4,4,2,5,5,3,0", 266960535469073u64),
        ("2,4,1,3,7,5,4,2,0,3,1,5,5,5,3,0", 236555995274861u64),
        ("2,4,1,3,7,5,1,5,0,3,4,2,5,5,3,0", 216584205979245u64),
        ("2,4,1,3,7,5,0,3,4,3,1,0,5,5,3,0", 87768089491393u64),
        ("2,4,1,3,7,5,4,3,0,3,1,0,5,5,3,0", 87768066086849u64),
        ("2,4,1,3,7,5,4,3,1,0,0,3,5,5,3,0", 87780816771009u64),
        ("2,4,1,3,7,5,0,3,1,0,4,3,5,5,3,0", 87782958266305u64),
        ("2,4,1,3,7,5,0,3,1,4,4,3,5,5,3,0", 266908995861521u64),
        ("2,4,1,3,7,5,0,3,4,3,1,5,5,5,3,0", 236580836040301u64),
        ("2,4,1,3,7,5,1,5,0,3,4,3,5,5,3,0", 236548287712877u64),
        ("2,4,1,3,7,5,0,3,4,4,1,4,5,5,3,0", 266930990791697u64),
        ("2,4,1,3,7,5,0,3,1,4,4,4,5,5,3,0", 266926175730705u64),
        ("2,4,1,3,7,5,4,4,0,3,1,5,5,5,3,0", 236556005760621u64),
        ("2,4,1,3,7,5,0,3,1,5,4,4,5,5,3,0", 236539226447469u64),
        ("2,4,1,3,7,5,1,5,4,4,5,5,0,3,3,0", 233390776344173u64),
        ("2,4,1,3,7,5,4,5,0,3,1,4,5,5,3,0", 266931037159697u64),
        ("2,4,1,3,7,5,0,3,1,4,4,5,5,5,3,0", 280206214609937u64),
        ("2,4,1,3,7,5,4,5,0,3,1,5,5,5,3,0", 236555999469165u64),
        ("2,4,1,3,7,5,0,3,1,5,4,5,5,5,3,0", 236556406316653u64),
        ("2,4,1,3,7,5,1,5,4,5,5,5,0,3,3,0", 233391044779629u64),
        ("2,4,1,3,7,5,0,3,4,6,1,0,5,5,3,0", 87794366773185u64),
        ("2,4,1,3,7,5,4,6,0,3,1,5,5,5,3,0", 236556001566317u64),
        ("2,4,1,3,7,5,0,3,4,7,1,0,5,5,3,0", 87768372238273u64),
        ("2,4,1,3,7,5,0,3,1,4,4,7,5,5,3,0", 266932601404433u64),
        ("2,4,1,3,7,5,4,7,0,3,1,5,5,5,3,0", 236555997372013u64),
        ("2,4,1,5,7,5,0,3,4,0,1,6,5,5,3,0", 109019476330651u64),
        ("2,4,1,5,7,5,0,3,1,6,4,0,5,5,3,0", 105840740174234u64),
        ("2,4,1,5,7,5,1,6,0,3,4,0,5,5,3,0", 105843716614554u64),
        ("2,4,1,5,7,5,0,3,4,1,1,6,5,5,3,0", 109020013201563u64),
        ("2,4,1,5,7,5,4,1,1,6,0,3,5,5,3,0", 105734763776155u64),
        ("2,4,1,5,7,5,0,3,1,6,4,1,5,5,3,0", 105849330108826u64),
        ("2,4,1,5,7,5,1,6,0,3,4,1,5,5,3,0", 105981155568026u64),
        ("2,4,1,5,7,5,1,6,4,1,5,5,0,3,3,0", 107413700225434u64),
        ("2,4,1,5,7,5,0,3,1,6,4,2,5,5,3,0", 105832150239642u64),
        ("2,4,1,5,7,5,1,6,0,3,4,2,5,5,3,0", 106086382266778u64),
        ("2,4,1,5,7,5,1,6,4,2,5,5,0,3,3,0", 107416870455451u64),
        ("2,4,1,5,7,5,0,3,4,3,1,6,5,5,3,0", 109019619953050u64),
        ("2,4,1,5,7,5,4,3,1,6,0,3,5,5,3,0", 105734774294938u64),
        ("2,4,1,5,7,5,0,3,1,6,4,3,5,5,3,0", 105875099912602u64),
        ("2,4,1,5,7,5,1,6,0,3,4,3,5,5,3,0", 105706277661082u64),
        ("2,4,1,5,7,5,1,6,4,3,0,3,5,5,3,0", 105735268690330u64),
        ("2,4,1,5,7,5,1,6,4,3,5,5,0,3,3,0", 107416732707226u64),
        ("2,4,1,5,7,5,0,3,4,4,1,6,5,5,3,0", 109019205798043u64),
        ("2,4,1,5,7,5,4,4,1,6,0,3,5,5,3,0", 105734783666586u64),
        ("2,4,1,5,7,5,4,4,1,6,5,5,0,3,3,0", 108534028601754u64),
        ("2,4,1,5,7,5,0,3,1,6,4,4,5,5,3,0", 108056943298970u64),
        ("2,4,1,5,7,5,0,3,4,5,1,6,5,5,3,0", 109019742668955u64),
        ("2,4,1,5,7,5,4,5,0,3,1,6,5,5,3,0", 109019930331546u64),
        ("2,4,1,5,7,5,0,3,1,6,4,5,5,5,3,0", 105866509978010u64),
        ("2,4,1,5,7,5,1,6,0,3,4,5,5,5,3,0", 109541683456410u64),
        ("2,4,1,5,7,5,4,6,0,3,1,6,5,5,3,0", 109019846445466u64),
        ("2,4,1,5,7,5,4,6,1,6,0,3,5,5,3,0", 105734716557722u64),
        ("2,4,1,5,7,5,0,3,1,6,4,6,5,5,3,0", 109130685122970u64),
        ("2,4,1,5,7,5,1,6,0,3,4,6,5,5,3,0", 136904920099226u64),
        ("2,4,1,5,7,5,1,6,4,6,0,3,5,5,3,0", 136936180680859u64),
        ("2,4,1,5,7,5,4,7,1,6,0,3,5,5,3,0", 105734765873307u64),
        ("2,4,1,5,7,5,0,3,1,6,4,7,5,5,3,0", 109593014864282u64),
        ("2,4,1,5,7,5,1,6,0,3,4,7,5,5,3,0", 109550273391002u64),
        ("2,4,1,5,7,5,1,6,4,7,0,3,5,5,3,0", 105702138980762u64),
        ("2,4,1,5,7,5,1,6,4,7,5,5,0,3,3,0", 108399378442650u64),
        ("2,4,1,6,7,5,4,1,1,7,0,3,5,5,3,0", 47910082096018u64),
        ("2,4,1,6,7,5,0,3,1,7,4,1,5,5,3,0", 47951183653778u64),
        ("2,4,1,6,7,5,0,3,4,3,1,7,5,5,3,0", 47921188613010u64),
        ("2,4,1,6,7,5,4,4,1,7,0,3,5,5,3,0", 47910079998866u64),
        ("2,4,1,6,7,5,4,6,1,4,5,5,0,3,3,0", 90938893795561u64),
        ("2,4,1,6,7,5,4,7,1,4,5,5,0,3,3,0", 90938843463913u64),
        ("2,4,1,7,7,5,0,3,4,0,1,7,5,5,3,0", 258394985014171u64),
        ("2,4,1,7,7,5,0,3,1,7,4,0,5,5,3,0", 265660930925467u64),
        ("2,4,1,7,7,5,1,7,4,0,0,3,5,5,3,0", 258411949917083u64),
        ("2,4,1,7,7,5,4,1,1,4,5,5,0,3,3,0", 109685330781408u64),
        ("2,4,1,7,7,5,0,3,1,7,4,1,5,5,3,0", 265652340990875u64),
        ("2,4,1,7,7,5,1,7,0,3,4,1,5,5,3,0", 265601188299675u64),
        ("2,4,1,7,7,5,1,7,4,1,0,3,5,5,3,0", 258393608225691u64),
        ("2,4,1,7,7,5,0,3,4,2,1,7,5,5,3,0", 258400890594203u64),
        ("2,4,1,7,7,5,1,7,4,2,0,3,5,5,3,0", 258411144610715u64),
        ("2,4,1,7,7,5,1,4,4,3,5,5,0,3,3,0", 109552863562265u64),
        ("2,4,1,7,7,5,0,3,4,4,1,7,5,5,3,0", 267265166222235u64),
        ("2,4,1,7,7,5,1,7,0,3,4,4,5,5,3,0", 265618099733403u64),
        ("2,4,1,7,7,5,1,7,4,4,0,3,5,5,3,0", 258411681481627u64),
        ("2,4,1,7,7,5,0,3,4,5,1,7,5,5,3,0", 258395521885083u64),
        ("2,4,1,7,7,5,1,7,0,3,4,5,5,5,3,0", 265635548038043u64),
        ("2,4,1,7,7,5,1,7,4,5,0,3,5,5,3,0", 258411811505051u64),
        ("2,4,1,7,7,5,1,7,4,6,0,3,5,5,3,0", 265061364597659u64),
        ("2,4,1,7,7,5,1,7,4,7,0,3,5,5,3,0", 258411413046171u64),
    ] {
        let s = prog.as_ptr();
        let hash = unsafe {
            _pext_u64(
                s.add(15).cast::<u64>().read_unaligned() ^ s.add(6).read() as u64 ^ (s.add(14).read() as u64 * 65536),
                0x07_00_04_00_07_07_04_07,
            )
        };
        lut[hash as usize * 8..hash as usize * 8 + 8].copy_from_slice(&a.to_ne_bytes());
    }

    let mut path: PathBuf = var("OUT_DIR").unwrap().into();
    path.push("day17.bin");
    File::create(path).unwrap().write_all(&lut).unwrap();
}

fn main() {
    println!("cargo::rerun-if-changed=build.rs");

    // write_lut_d2();
    // write_lut_d3();
    // write_lut_d11();
    // write_lut_d17();
}
