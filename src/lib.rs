#![feature(ptr_mask)]
#![feature(strict_provenance)]
#![forbid(unsafe_op_in_unsafe_fn)]

pub unsafe fn hexdump(mut addr: *const u8, mut len: usize) {
    println!(
        "Dumping {s:#016x}..{e:#016x}",
        s = addr.addr(),
        e = addr.addr().wrapping_add(len)
    );
    const PAD: &str = "";
    while len > 0 {
        let base = addr.mask(!0b1111).addr();
        let start = addr.addr() - base;
        let clen = usize::min(16 - start, len);

        print!("0x{base:016x}:");
        print!("{PAD:>pad$}", pad = start * 3);
        for k in 0..clen {
            print!(" {:02x}", unsafe { core::ptr::read(addr.wrapping_add(k)) });
        }
        print!("{PAD:>pad$}", pad = (16 - (clen + start)) * 3);
        print!("{PAD:>start$}");
        print!(" [");
        for k in 0..clen {
            let b = unsafe { core::ptr::read(addr.wrapping_add(k)) };
            if b.is_ascii_graphic() || b == b' ' {
                print!("{b}", b = b as char);
            } else {
                print!(".");
            }
        }
        println!("]");
        addr = addr.wrapping_add(clen);
        len -= clen;
    }
}
