#[cfg(test)]
mod tests {
    #[test]
    fn smalluint_new() {
        let s0 = SmallUInt::new(8, 0);
        assert_eq!(s0.size, 8);
        assert_eq!(s0.bits, [false; 8]);

        // 128 64 32 16 8 4 2 1
        // 1   1  0  0  1 1 0 1   = 205
        // ^ ignore ^  | ^ pack ^
        let s5 = SmallUInt::new(4, 205);
        assert_eq!(s5.size, 4);
        assert_eq!(s5.bits, [true, false, true, true]);
    }

    #[test]
    fn smalluint_as_u8() {
        let s5 = SmallUInt::new(4, 205);
        assert_eq!(s5.as_u8(), 13);
    }
}
