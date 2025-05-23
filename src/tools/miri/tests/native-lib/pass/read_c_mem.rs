// Only works on Unix targets
//@ignore-target: windows wasm
//@only-on-host

fn main() {
    read_c_mem();
}

/// Test it!
fn read_c_mem() {
    extern "C" {
        fn allocate() -> *mut u8;
    }

    let ptr = unsafe { allocate() };
    unsafe { 
        *ptr = 10;
        println!("{}", *ptr);
    }
}