use std::io::Read;
use std::ptr::copy;

fn exec_fn(){
    println!("meow hacker");
}

#[repr(C)]
struct meow {
    buffer: [u8; 16],
    point: *const fn()
}

fn main() {
    println!("exec_fn address is {:p}", exec_fn as *const usize);
    let mut i = [0u8; 32];
    let mut stdin = std::io::stdin();
    let _ = stdin.read(&mut i[..]);

    let mut meow = meow{
        buffer: [0; 16],
        point: 0 as *const fn() -> ()
    };

    unsafe { copy(i.as_ptr(), meow.buffer.as_mut_ptr(), i.len()); }

    println!("meow.point address is {:p}", meow.point);
    println!("meow.point after strcpy is {:?}", (meow.point as usize as u64).to_le_bytes().into_iter().map(|x| char::from(x)).collect::<String>());

    if meow.point as usize == 0{ println!("meow.point is null retry"); }
    else {
        println!("meow.point is not null");
        let f: fn() = unsafe {
            std::mem::transmute(meow.point)
        };
        f();
    }
}
