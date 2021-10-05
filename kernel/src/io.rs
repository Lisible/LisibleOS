pub unsafe fn out8(address: u16, value: u8) {
    asm!(
        "out dx, al", in("dx") address, in("al") value
    )
}
