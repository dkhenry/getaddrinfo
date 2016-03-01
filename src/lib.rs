#![crate_type = "dylib"]

// We need transmute to mess with the C structures
use std::mem; 
use std::ptr;

extern crate libc; 
use libc::{sockaddr,sockaddr_in,sockaddr_in6,ifaddrs};

#[repr(C, packed)]
#[derive(Debug)]
pub struct in6addrinfo {
    flags: u8,
    prefixlen: u8,
    padding: u16,
    index: u32,
    addr: [u32; 4]
}

fn htonl( n: u32 ) -> u32 {
    ((((n & 0xFF)) << 24) |
     (((n & 0xFF00)) << 8) |
     (((n & 0xFF0000)) >> 8) |
     (((n & 0xFF000000)) >> 24))
}

unsafe fn ifa_netmask_to_prefix(netmask: *mut libc::sockaddr) -> u8 {
    if netmask.is_null() {
        return 0
    }
    match (*netmask).sa_family as i32 {
        libc::AF_INET => {
            let addr: *const libc::sockaddr_in = netmask as *const libc::sockaddr_in;
            mem::transmute::<libc::in_addr,u32>((*addr).sin_addr).count_ones() as u8
        },
        libc::AF_INET6 => {
            let addr: *const libc::sockaddr_in6 = netmask as *const libc::sockaddr_in6;
            mem::transmute::<libc::in6_addr,[u8; 16]>((*addr).sin6_addr).iter().map(|v| { (v as &u8).count_ones() }).fold(0, |acc, v| { acc + v }) as u8
        },
        _ => 0 as u8,
    }
}

unsafe fn sockaddr_to_addr( sock: libc::sockaddr ) -> [u32; 4] {
    let mut rvalue: [u32; 4] = mem::zeroed();
    match sock.sa_family as i32 {
        libc::AF_INET => {
            let addr: libc::sockaddr_in = mem::transmute::<libc::sockaddr,libc::sockaddr_in>(sock);

            rvalue[2] = htonl(0xffff);
            rvalue[3] = mem::transmute::<libc::in_addr,u32>(addr.sin_addr);
            rvalue
        },
        libc::AF_INET6 => {
            let addr: sockaddr_in6 = *(mem::transmute::<*const libc::sockaddr,*const libc::sockaddr_in6>(&sock));
            ptr::copy(&mem::transmute::<libc::in6_addr,[u32;4]>(addr.sin6_addr),&mut rvalue,1);
            rvalue
        },
        _ => rvalue
    }
}

unsafe fn ifaddr_to_in6addrinfo( rhs :&libc::ifaddrs ) -> in6addrinfo {
    return in6addrinfo {
        flags: rhs.ifa_flags as u8,
        prefixlen: ifa_netmask_to_prefix(rhs.ifa_netmask),
        padding: 0u16,
        index: 0u32,
        addr: sockaddr_to_addr(*(rhs.ifa_addr))
    }
}

#[no_mangle]
pub extern "C" fn __check_pf(seen_ipv4: *mut bool, seen_ipv6: *mut bool, in6ai: *mut *mut in6addrinfo, in6ailen: *mut libc::size_t) {
    unsafe { 
        // Call libc::getifaddr
        let mut ifaddrs: *mut libc::ifaddrs = mem::zeroed(); 

        let errno = libc::getifaddrs(&mut ifaddrs);
        if errno == -1 || ifaddrs.is_null() {
            return
        }
        // Convert the ifaddrs into in6addrinfo
        let mut rvalue = Box::new(Vec::new());

        while !(*ifaddrs).ifa_next.is_null() {
            rvalue.push(ifaddr_to_in6addrinfo(&*ifaddrs));
            ifaddrs = (*ifaddrs).ifa_next;                        
        }
        rvalue.push(ifaddr_to_in6addrinfo(&*ifaddrs));

        // Format the return value
        *seen_ipv6 = true;
        *seen_ipv4 = true;
        *in6ailen = rvalue.len();
        *in6ai = rvalue.as_mut_ptr();
        mem::drop(rvalue); // release the memory
    }
}

#[test]
fn compare_check_pf() {
    unsafe {
        let mut ipv4 = false;
        let mut ipv6 = false;
        let mut in6ai: *mut in6addrinfo = mem::zeroed();
        let mut in6ailen = 0;
        __check_pf(&mut ipv4,&mut ipv6,&mut in6ai, &mut in6ailen);
        println!("{:?}",in6ai);        
    }
}

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
