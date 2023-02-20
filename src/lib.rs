#![feature(ptr_mask)]
#![feature(strict_provenance)]
#![forbid(unsafe_op_in_unsafe_fn)]

pub unsafe fn hexdump(mut addr: *const u8, mut len: usize) {
    println!(
        "@{s:#016x}..{e:#016x}",
        s = addr.addr(),
        e = addr.addr() + len
    );
    while len > 0 {
        let paddr = addr.mask(!0b1111).addr();
        let cstart = addr.addr() - paddr;
        let clen = usize::min(16 - cstart, len);
        let mut pbuf = [' '; 16];
        let mut bs = [0u8; 16];

        let dst = (&mut bs[cstart..cstart + clen]).as_mut_ptr();
        unsafe { core::ptr::copy(addr, dst, clen) };
        print!("{:#x}:", paddr);
        for _ in 0..cstart {
            print!("   ");
        }
        for k in cstart..cstart + clen {
            let b = bs[k];
            print!(" {:02x}", bs[k]);
            pbuf[k] = if b < 32 || b >= 127 { '.' } else { b as char };
        }
        for _ in (cstart + clen)..16 {
            print!("   ");
        }
        //let s = pbuf.iter().collect::<String>();
        println!(" [{pbuf:?}]");
        addr = addr.wrapping_add(clen);
        len -= clen;
    }
}
