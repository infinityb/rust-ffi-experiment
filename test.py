import os
import ctypes


def get_lib_filename():
    for fn in os.listdir("./target"):
        if fn.startswith('libtest-') and fn.endswith('.so'):
            return fn
    else:
        raise KeyError('no library found')

filename = "./target/{}".format(get_lib_filename())
print("loading {}".format(filename))
libtest = ctypes.CDLL(filename)

print("foo() -> {}".format(libtest.foo()))
print("doubleit(8) -> {}".format(libtest.doubleit(8)))


buf_type = ctypes.c_ubyte * 1024
buf = buf_type()
buf[4] = 1
buf[7] = 1
buf[13] = 1
buf[400] = 1


ones = libtest.count_ones(buf, len(buf))
print("count_ones(...) -> {}".format(ones))
print("list(buf) -> {}".format(list(buf)))
