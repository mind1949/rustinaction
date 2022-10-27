fn main() {
    // 
    let fn_ptr = noop as usize;
    // noop 的类型就是 *const fn() -> ()
    // 常量函数指针，意味着不可变
    let typed_fn_ptr = noop as *const fn() -> ();

    println!("noop as usize: 0x{:x}", fn_ptr);
    println!("noop as *const T: {:p}", typed_fn_ptr);
}

fn noop() {}
