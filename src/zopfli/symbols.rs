use libc::{c_int};

/// Gets the amount of extra bits for the given dist, cfr. the DEFLATE spec.
#[no_mangle]
#[allow(non_snake_case)]
pub extern fn ZopfliGetDistExtraBits(dist: c_int) -> c_int {
    if dist < 5 {
        0
    } else {
        match dist {
            5...8 => 1,
            9...16 => 2,
            17...32 => 3,
            33...64 => 4,
            65...128 => 5,
            129...256 => 6,
            257...512 => 7,
            513...1024 => 8,
            1025...2048 => 9,
            2049...4096 => 10,
            4097...8192 => 11,
            8193...16384 => 12,
            _ => 13,
        }
    }
}

/// Gets value of the extra bits for the given dist, cfr. the DEFLATE spec.
#[no_mangle]
#[allow(non_snake_case)]
pub extern fn ZopfliGetDistExtraBitsValue(dist: c_int) -> c_int {
    if dist < 5 {
        0
    } else if dist < 9 {
        (dist - 5) & 1
    } else if dist < 17 {
        (dist - 9) & 3
    } else if dist < 33 {
        (dist - 17) & 7
    } else if dist < 65 {
        (dist - 33) & 15
    } else if dist < 129 {
        (dist - 65) & 31
    } else if dist < 257 {
        (dist - 129) & 63
    } else if dist < 513 {
        (dist - 257) & 127
    } else if dist < 1025 {
        (dist - 513) & 255
    } else if dist < 2049 {
        (dist - 1025) & 511
    } else if dist < 4097 {
        (dist - 2049) & 1023
    } else if dist < 8193 {
        (dist - 4097) & 2047
    } else if dist < 16385 {
        (dist - 8193) & 4095
    } else {
        (dist - 16385) & 8191
    }
}

/// Gets the symbol for the given dist, cfr. the DEFLATE spec.
#[no_mangle]
#[allow(non_snake_case)]
pub extern fn ZopfliGetDistSymbol(dist: c_int) -> c_int {
    if dist < 193 {
        if dist < 13 {  /* dist 0..13. */
            if dist < 5 {
                dist - 1
            } else if dist < 7 {
                4
            } else if dist < 9 {
                5
            } else {
                6
            }
        } else {  /* dist 13..193. */
            if dist < 17 {
                7
            } else if dist < 25 {
                8
            } else if dist < 33 {
                9
            } else if dist < 49 {
                10
            } else if dist < 65 {
                11
            } else if dist < 97 {
                12
            } else if dist < 129 {
                13
            } else {
                14
            }
        }
    } else {
        if dist < 2049 {  /* dist 193..2049. */
            if dist < 257 {
                15
            } else if dist < 385 {
                16
            } else if dist < 513 {
                17
            } else if dist < 769 {
                18
            } else if dist < 1025 {
                19
            } else if dist < 1537 {
                20
            } else {
                21
            }
        } else {  /* dist 2049..32768. */
            if dist < 3073 {
                22
            } else if dist < 4097 {
                23
            } else if dist < 6145 {
                24
            } else if dist < 8193 {
                25
            } else if dist < 12289 {
                26
            } else if dist < 16385 {
                27
            } else if dist < 24577 {
                28
            } else {
                29
            }
        }
    }
}
