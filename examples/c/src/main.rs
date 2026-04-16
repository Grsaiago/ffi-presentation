use std::os::raw::c_int;

#[link(name = "mylib")]
unsafe extern "C" {
    fn get_c_pid() -> c_int;
    fn calling_another_function(func: extern "C" fn());
}

#[unsafe(no_mangle)]
extern "C" fn my_ffi_func() {
    println!("Omg rust hiiiiiiiiiiiiiiiiii");
}

fn main() {
    unsafe {
        println!("O pid é {}", get_c_pid());
        calling_another_function(my_ffi_func);
    }
}
