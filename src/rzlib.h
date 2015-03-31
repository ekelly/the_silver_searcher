// C header library matching rust-zlib

#ifndef __RZLIB_H
#define __RZLIB_H

void * decompress_zlib_to_heap(const void * buf,
    int buf_len,
    const char * dir_full_path,
    int * new_buf_len);


#endif
