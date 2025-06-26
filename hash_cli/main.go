package main

/*
#cgo LDFLAGS: -L. -lhashlib
#include <stdlib.h>

char* hash_sha256(const char* input);
void free_string(char* s);
*/
import "C"

import (
	"fmt"
	"unsafe"
)

func main() {
	input := "This is Go!"
	cInput := C.CString(input)
	defer C.free(unsafe.Pointer(cInput))

	hashed := C.hash_sha256(cInput)
	defer C.free_string(hashed)

	fmt.Println("SHA-256:", C.GoString(hashed))
}
