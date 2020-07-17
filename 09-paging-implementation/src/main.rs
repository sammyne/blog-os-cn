#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use blog_os::println;
use core::panic::PanicInfo;

use bootloader::{entry_point, BootInfo};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use blog_os::memory::active_level_4_table;
    use x86_64::VirtAddr;
    use x86_64::structures::paging::PageTable;

    println!("Hello World!");
    blog_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let l4_table = unsafe { active_level_4_table(phys_mem_offset) };

    for (i, entry) in l4_table.iter().enumerate() {
        if entry.is_unused() {
            continue;
        }

        println!("L4 Entry {}: {:?}", i, entry);

        // get the physical address from the entry and convert it
        let phys = entry.frame().unwrap().start_address();
        let virt = phys.as_u64() + boot_info.physical_memory_offset;
        let ptr = VirtAddr::new(virt).as_mut_ptr();
        let l3_table: &PageTable = unsafe { &*ptr };

        // print non-empty entries of the level 3 table
        for (i, entry) in l3_table.iter().enumerate() {
            if !entry.is_unused() {
                println!("  L3 Entry {}: {:?}", i, entry);
            }
        }
    }
    // as before
    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    blog_os::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    blog_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}
