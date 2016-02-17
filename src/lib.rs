#![crate_type = "dylib"]

extern crate libc; 

/// Given a node and a service, which identify as an Internet host and
/// service, return one or more addrinfo structures each of which
/// contains an internet address that can be specified ina call to
/// bind or connect. 
#[no_mangle]
pub extern "C" fn getaddrinfo(node: *const libc::c_char, service: *const libc::c_char, hints: *const libc::addrinfo, res: *mut *mut libc::addrinfo) -> libc::c_int {
    // Look up DNS resolution servers

    // Send a request to DNS servers

    // Marshell Response into res
    println!("Called getaddrinfo from rust");
    unsafe {
        return libc::getaddrinfo(node,service,hints,res);
    }
        
}

#[test]
fn comapre_to_libc_getaddrinfo() {
    assert_eq!(1, 1)
}
