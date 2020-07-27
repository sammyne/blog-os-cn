use core::marker::PhantomPinned;
use std::mem;

fn main() {
    let mut heap_value = Box::pin(SelfReferential {
        self_ptr: 0 as *const _,
        _pin: PhantomPinned,
    });
    let ptr = &*heap_value as *const SelfReferential;
    heap_value.self_ptr = ptr;
    println!("heap value at: {:p}", heap_value);
    println!("internal reference: {:p}", heap_value.self_ptr);

    let stack_value = mem::replace(
        &mut *heap_value,
        SelfReferential {
            self_ptr: 0 as *const _,
            _pin: PhantomPinned,
        },
    );
    println!("value at: {:p}", &stack_value);
    println!("internal reference: {:p}", stack_value.self_ptr);
}

struct SelfReferential {
    self_ptr: *const Self,
    _pin: PhantomPinned,
}
