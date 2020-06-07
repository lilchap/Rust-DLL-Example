use std::os::raw::*;

type QWORD = c_ulonglong;

pub unsafe fn write<T>(address: QWORD, value: T) {
    *(address as *mut T) = value;
}

pub unsafe fn read<T: Copy>(address: QWORD) -> T {
    *(address as *const T)
}