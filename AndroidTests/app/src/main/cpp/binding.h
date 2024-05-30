#ifndef binding_h
#define binding_h

#include<stdint.h>

extern "C" int32_t test_read_from_png_file_to_memory(const char *png_path);
extern "C" int32_t test_read_png_from_memory(const void *buffer, size_t len);

#endif /* binding_h */