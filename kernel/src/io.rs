pub fn out8(address: u16, value: u8) {
    unsafe {
        asm!(
            "out dx, al", in("dx") address, in("al") value
        )
    }
}