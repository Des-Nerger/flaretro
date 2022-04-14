use std::os::raw::c_int;

#[no_mangle]
pub extern "C" fn add_one(i: c_int) -> c_int {
	i + 1
}
