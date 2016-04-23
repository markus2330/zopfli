use libc::{size_t, c_ushort};

pub enum ZopfliLZ77Store {}

/// Appends the length and distance to the LZ77 arrays of the ZopfliLZ77Store.
/// Context must be a ZopfliLZ77Store*.
#[no_mangle]
#[allow(non_snake_case)]
pub extern fn ZopfliStoreLitLenDist(length: c_ushort, dist: c_ushort, pos: size_t, store: *mut ZopfliLZ77Store) {
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
