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
    } else {
        match dist {
            5...8 => (dist - 5) & 1,
            9...16 => (dist - 9) & 3,
            17...32 => (dist - 17) & 7,
            33...64 => (dist - 33) & 15,
            65...128 => (dist - 65) & 31,
            129...256 => (dist - 129) & 63,
            257...512 => (dist - 257) & 127,
            513...1024 => (dist - 513) & 255,
            1025...2048 => (dist - 1025) & 511,
            2049...4096 => (dist - 2049) & 1023,
            4097...8192 => (dist - 4097) & 2047,
            8193...16384 => (dist - 8193) & 4095,
            _ => (dist - 16385) & 8191,
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern fn ZopfliGetDistSymbol(dist: c_int) -> c_int {
    match dist {
        0...4 => dist - 1,
        5...6 => 4,
        7...8 => 5,
        9...12 => 6,
        13...16 => 7,
        17...24 => 8,
        25...32 => 9,
        33...48 => 10,
        49...64 => 11,
        65...96 => 12,
        97...128 => 13,
        129...192 => 14,
        193...256 => 15,
        257...384 => 16,
        385...512 => 17,
        513...768 => 18,
        769...1024 => 19,
        1025...1536 => 20,
        1537...2048 => 21,
        2049...3072 => 22,
        3073...4096 => 23,
        4097...6144 => 24,
        6145...8192 => 25,
        8193...12288 => 26,
        12289...16384 => 27,
        16385...24576 => 28,
        _ => 29,
    }
}
