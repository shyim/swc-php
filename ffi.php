<?php

$libExtension = (PHP_OS_FAMILY == "Darwin" ? 'dylib' : 'so');

$ffi = \FFI::cdef(
    "char* compile(char* s);",
    "target/debug/libswc.$libExtension");
var_dump(FFI::string($ffi->compile('test.js')));
