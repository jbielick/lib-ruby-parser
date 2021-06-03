#ifndef LIB_RUBY_PARSER_EXTERNAL_C_MAYBE_PTR_H
#define LIB_RUBY_PARSER_EXTERNAL_C_MAYBE_PTR_H

#include <stddef.h>
#include <stdlib.h>
#include "declare_blob.h"

typedef int DUMMY_MAYBE_PTR_VALUE;
typedef void(DropMaybePtr)(void *);

// MaybePtr<T>
typedef DUMMY_MAYBE_PTR_VALUE *MAYBE_PTR;
_Static_assert(sizeof(MAYBE_PTR) == 8, "wrong sizeof(MAYBE_PTR)");
DECLARE_BLOB_FOR(MAYBE_PTR);

MAYBE_PTR_BLOB_DATA lib_ruby_parser_containers_make_maybe_ptr_blob(void *ptr)
{
    return PACK_MAYBE_PTR(ptr);
}

void lib_ruby_parser_containers_free_maybe_ptr_blob(MAYBE_PTR_BLOB_DATA blob, DropMaybePtr drop)
{
    MAYBE_PTR maybe_ptr = UNPACK_MAYBE_PTR(blob);
    if (maybe_ptr != NULL)
    {
        drop(maybe_ptr);
        free(maybe_ptr);
    }
}

void *lib_ruby_parser_containers_raw_ptr_from_maybe_ptr_blob(MAYBE_PTR_BLOB_DATA blob)
{
    return UNPACK_MAYBE_PTR(blob);
}

MAYBE_PTR_BLOB_DATA lib_ruby_parser_containers_null_maybe_ptr_blob()
{
    return PACK_MAYBE_PTR(NULL);
}

#endif // LIB_RUBY_PARSER_EXTERNAL_C_MAYBE_PTR_H
