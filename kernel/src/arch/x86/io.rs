pub unsafe fn out8(address: u16, value: u8) {
    asm!(
    "out dx, al", in("dx") address, in("al") value
    );
}

pub unsafe fn in8(address: u16) -> u8 {
    let result: u8;
    asm!(
    "in al, dx", in("dx") address, out("al") result
    );

    result
}
