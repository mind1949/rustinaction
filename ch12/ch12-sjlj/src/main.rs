#![feature(link_llvm_intrinsics)]
#![feature(non_camel_case_types)]
#![cfg(not(windows))]

use libc::{SIGALRM, SIGQUIT, SIGTERM, SIGHUP, SIGUSR1};
use std::mem;

const JUMP_BUF_WIDTH: usize = mem::size_of::<usize>() * 8;
type JmpBuf = [i8; JUMP_BUF_WIDTH];

static mut SHUT_DOWN: bool = false;
static mut RETURN_HERE: JmpBuf = [0; JUMP_BUF_WIDTH];
const MOCK_SIGNAL_AT: usize = 3;

extern "C" {
    #[link_name = "llvm.eh.sjlj.setjmp"]
    pub fn setjmp(_: *mut i8) -> i32;

    #[link_name = "llvm.eh.sjlj.longjump"]
    pub fn longjmp(_: *mut i8);
}

#[inline]
fn ptr_to_jmp_buf() -> *mut i8 {
    unsafe { &RETURN_HERE as *const i8 as *mut i8 }
}

#[inline]
fn return_early() {
    let franken_pointer = ptr_to_jmp_buf();
    unsafe { longjmp(franken_pointer) }
}

fn register_signal_handler() {
    unsafe {
        libc::signal(SIGUSR1, handle_signals as usize);
    }
}

#[allow(dead_code)]
fn handle_signals(sig: i32) {
    register_signal_handler();

    let should_shet_down = match sig {
        SIGHUP => false,
        SIGALRM => false,
        SIGTERM => true,
        SIGQUIT => true,
        SIGUSR1 => true,
        _ => false,
    };

    unsafe {
        SHUT_DOWN = should_shet_down;
    }

    return_early();
}

fn print_depth(depth: usize) {
    for _ in 0..depth {
        print!("#");
    }
    println!();
}

fn dive(depth: usize, max_depth: usize) {
    unsafe {
        if SHUT_DOWN {
            println!("!");
            return;
        }
    }
    print_depth(depth);

    if depth >= max_depth {
        return;
    } else {
        dive(depth + 1, max_depth);
    }
    print_depth(depth);
}

fn main() {
    const JUMP_SET: i32 = 0;

    register_signal_handler();

    let return_point = ptr_to_jmp_buf();
    let rc = unsafe { setjmp(return_point) };
    if rc == JUMP_SET {
        dive(0, 10);
    } else {
        println!("early return!");
    }

    println!("finishing");
}
