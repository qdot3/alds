fn main() {}

trait FromAscii: Sized {
    type Err;

    fn from_ascii(ascii: &[u8]) -> Result<Self, Self::Err>;
}

impl FromAscii for u64 {
    type Err = ();

    fn from_ascii(ascii: &[u8]) -> Result<Self, Self::Err> {
        todo!()
    }
}

fn from_ascii_8(digits: [u8; 8]) -> Result<u64, ()> {
    let mut reversed = u64::from_le_bytes(digits);

    // validation
    // b'0' = 0x30, b'9' = 0x39
    if reversed & 0xf0f0_f0f0_f0f0_f0f0 != 0x3030_3030_3030_3030 {
        return Err(());
    }
    // 8 = 0b1000, 9 = 0b1001, 0xa = 0b1010, ..
    if (reversed & 0x0808_0808_0808_0808) & ((reversed & 0x0404_0404_0404_0404) << 1) != 0 {
        return Err(());
    }
    if (reversed & 0x0808_0808_0808_0808) & ((reversed & 0x0202_0202_0202_0202) << 2) != 0 {
        return Err(());
    }

    // |h|g|f|e|d|c|b|a| -> |gh|ef|cd|ba|
    reversed += (reversed & 0x0f0f_0f0f_0f0f_0f0f) * (10 << 8);
    reversed >>= 8;
    reversed &= 0x00ff_00ff_00ff_00ff;
    // |gh|ef|cd|ba| -> |efgh|abcd|
    reversed += reversed * (10 << 16);
    reversed >>= 16;
    reversed &= 0x0000_ffff_0000_ffff;
    // |efgh|abcd| -> |abcdefgh|
    reversed += reversed * (10 << 32);
    reversed >>= 32;
    reversed &= 0x0000_0000_ffff_ffff;

    Ok(reversed)
}
