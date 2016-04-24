use libc::{size_t, c_ushort, c_uchar};

const ZOPFLI_NUM_LL: size_t = 288;
const ZOPFLI_NUM_D: size_t = 32;

// Comment from C:
// Stores lit/length and dist pairs for LZ77.
// Parameter litlens: Contains the literal symbols or length values.
// Parameter dists: Contains the distances. A value is 0 to indicate that there is
// no dist and the corresponding litlens value is a literal instead of a length.
// Parameter size: The size of both the litlens and dists arrays.
// The memory can best be managed by using ZopfliInitLZ77Store to initialize it,
// ZopfliCleanLZ77Store to destroy it, and ZopfliStoreLitLenDist to append values.

#[repr(C)]
pub struct ZopfliLZ77Store {
  litlens: *mut c_ushort,  /* Lit or len. */
  dists: *mut c_ushort,  /* If 0: indicates literal in corresponding litlens,
      if > 0: length in corresponding litlens, this is the distance. */
  size: size_t,

  data: *mut c_uchar,  /* original data */
  pos: *mut size_t,  /* position in data where this LZ77 command begins */

  ll_symbol: *mut c_ushort,
  d_symbol: *mut c_ushort,

  /* Cumulative histograms wrapping around per chunk. Each chunk has the amount
  of distinct symbols as length, so using 1 value per LZ77 symbol, we have a
  precise histogram at every N symbols, and the rest can be calculated by
  looping through the actual symbols of this chunk. */
  ll_counts: *mut size_t,
  d_counts: *mut size_t,
}

/// Appends the length and distance to the LZ77 arrays of the ZopfliLZ77Store.
/// Context must be a ZopfliLZ77Store*.
#[no_mangle]
#[allow(non_snake_case)]
pub extern fn ZopfliStoreLitLenDist(length: c_ushort, dist: c_ushort, pos: size_t, store: &mut ZopfliLZ77Store) {
    let origsize = store.size;
    let llstart = ZOPFLI_NUM_LL * (origsize / ZOPFLI_NUM_LL);
    let dstart = ZOPFLI_NUM_D * (origsize / ZOPFLI_NUM_D);
  // size_t i;
  // /* Needed for using ZOPFLI_APPEND_DATA multiple times. */
  // size_t origsize = store->size;
  // size_t llstart = ZOPFLI_NUM_LL * (origsize / ZOPFLI_NUM_LL);
  // size_t dstart = ZOPFLI_NUM_D * (origsize / ZOPFLI_NUM_D);
  //
  // /* Everytime the index wraps around, a new cumulative histogram is made: we're
  // keeping one histogram value per LZ77 symbol rather than a full histogram for
  // each to save memory. */
  // if (origsize % ZOPFLI_NUM_LL == 0) {
  //   size_t llsize = origsize;
  //   for (i = 0; i < ZOPFLI_NUM_LL; i++) {
  //     ZOPFLI_APPEND_DATA(
  //         origsize == 0 ? 0 : store->ll_counts[origsize - ZOPFLI_NUM_LL + i],
  //         &store->ll_counts, &llsize);
  //   }
  // }
  // if (origsize % ZOPFLI_NUM_D == 0) {
  //   size_t dsize = origsize;
  //   for (i = 0; i < ZOPFLI_NUM_D; i++) {
  //     ZOPFLI_APPEND_DATA(
  //         origsize == 0 ? 0 : store->d_counts[origsize - ZOPFLI_NUM_D + i],
  //         &store->d_counts, &dsize);
  //   }
  // }
  //
  // ZOPFLI_APPEND_DATA(length, &store->litlens, &store->size);
  // store->size = origsize;
  // ZOPFLI_APPEND_DATA(dist, &store->dists, &store->size);
  // store->size = origsize;
  // ZOPFLI_APPEND_DATA(pos, &store->pos, &store->size);
  // assert(length < 259);
  //
  // if (dist == 0) {
  //   store->size = origsize;
  //   ZOPFLI_APPEND_DATA(length, &store->ll_symbol, &store->size);
  //   store->size = origsize;
  //   ZOPFLI_APPEND_DATA(0, &store->d_symbol, &store->size);
  //   store->ll_counts[llstart + length]++;
  // } else {
  //   store->size = origsize;
  //   ZOPFLI_APPEND_DATA(ZopfliGetLengthSymbol(length),
  //                      &store->ll_symbol, &store->size);
  //   store->size = origsize;
  //   ZOPFLI_APPEND_DATA(ZopfliGetDistSymbol(dist),
  //                      &store->d_symbol, &store->size);
  //   store->ll_counts[llstart + ZopfliGetLengthSymbol(length)]++;
  //   store->d_counts[dstart + ZopfliGetDistSymbol(dist)]++;
  // }
}
