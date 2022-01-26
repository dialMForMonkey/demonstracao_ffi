package main

// #include <stdio.h>
// #include <stdlib.h>
import "C"

func main() {

}

//export GetPointer
func GetPointer() *C.char {
	cs := C.CString("Hello from stdio")
	println(&cs)
	return cs
}
