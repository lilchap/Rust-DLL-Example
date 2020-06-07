use std::{thread, time};
use std::alloc::{alloc, Layout};

fn main() {
    let layout = Layout::new::<i64>();
    let address: *mut u64;

    unsafe {
        address = alloc(layout) as *mut u64;
        *address = 7331;
    }

    println!("TESTAPP: {:X}\n", address as u64);

    loop {
        thread::sleep(time::Duration::from_millis(1000));
    }
}