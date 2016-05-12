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