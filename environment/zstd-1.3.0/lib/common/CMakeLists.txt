target_sources(zstd PRIVATE
    entropy_common.c
    error_private.c
    fse_decompress.c
    pool.c
    threading.c
    xxhash.c
    zstd_common.c)
