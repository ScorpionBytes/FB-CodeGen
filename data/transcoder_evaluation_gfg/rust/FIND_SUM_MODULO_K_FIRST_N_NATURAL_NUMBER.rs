#![feature(main)]
// Copyright (c) 2019-present, Facebook, Inc.
// All rights reserved.
//
// This source code is licensed under the license found in the
// LICENSE file in the root directory of this source tree.
//

#[no_mangle]
pub unsafe fn abs(mut x: i32)
 -> i32 {
    return x.abs();
}

#[no_mangle]
pub unsafe fn min(mut x: i32, mut y: i32)
 -> i32 {
    return if x < y { x } else { y };
}
#[no_mangle]
pub unsafe fn max(mut x: i32, mut y: i32)
 -> i32 {
    return if x > y { x } else { y };
}

pub unsafe fn f_gold(mut N: i32, mut K: i32)
 -> i32 {
    let mut ans: i32 = 0 as i32;
    let mut i: i32 = 1 as i32;
    while i <= N { ans += i % K; i += 1 }
    return ans;
}
//TOFILL
unsafe fn main_0() -> i32 {
    let mut n_success: i32 = 0 as i32;
    let mut param0: [i32; 10] =
        [11 as i32, 36 as i32, 71 as i32,
         74 as i32, 66 as i32, 38 as i32,
         2 as i32, 73 as i32, 79 as i32,
         30 as i32];
    let mut param1: [i32; 10] =
        [5 as i32, 69 as i32, 28 as i32,
         1 as i32, 84 as i32, 14 as i32,
         11 as i32, 87 as i32, 11 as i32,
         55 as i32];
    let mut i: i32 = 0 as i32;
    while i < param0.len() as i32 {
        if f_filled(param0[i as usize], param1[i as usize]) ==
               f_gold(param0[i as usize], param1[i as usize]) {
            n_success += 1 as i32
        }
        i += 1
    }
    println!("{} {} {} {} {}", "#Results:", " ", n_success, ", ", param0.len() as i32);
    return 0 as i32;
}
#[main]
pub fn main() { unsafe { ::std::process::exit(main_0() as i32) } }
