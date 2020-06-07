mod memstuff;

use std::os::raw::*;

type DWORD = c_ulong;
type QWORD = c_ulonglong;

// lol
extern {
    fn DisableThreadLibraryCalls(a: DWORD) -> QWORD;
    fn CreateThread(a: QWORD, b: QWORD, c: QWORD, d: QWORD, e: DWORD, f: QWORD) -> QWORD;
    fn ExitProcess(a: QWORD);
}

unsafe fn main_function() -> i64 {
    println!("\nMain function! Enter the address:\n");

    // Parse address
    let mut string_input = String::new();
    std::io::stdin().read_line(&mut string_input).expect("read_line error");
    let address = u64::from_str_radix(string_input.trim(), 16).unwrap();

    // Read/Write
    println!("\nOld Value: {}", memstuff::read::<u64>(address));
    memstuff::write::<u64>(address, 1337);
    println!("Result Value: {}\n\nPress any key to continue", memstuff::read::<u64>(address));

    // Exit process
    std::io::stdin().read_line(&mut string_input).expect("read_line error");
    ExitProcess(0);

    0
}

#[no_mangle]
extern "stdcall" fn DllMain(hinst_dll: DWORD, reason: DWORD, _reserved: DWORD) -> i64 {
    unsafe { DisableThreadLibraryCalls(hinst_dll); } 

    if reason == 1 {
        println!("hinst_dll {:X}\nReason: {:X}\nmain_function address: {:X}", hinst_dll, reason, main_function as QWORD);
        unsafe { CreateThread(0, 0, main_function as QWORD, 0, 0, 0); }
    }

    true as i64
}