// TODO: based on what you learned in this section, replace `todo!()` with
//  the correct value after the conversion.

#[cfg(test)]
mod tests {

    #[test]
    fn u16_to_u32() {
        let v: u32 = 47;
        assert_eq!(47u16 as u32, v);
    }

    #[test]
    fn u8_to_i8() {
        // The compiler is smart enough to know that the value 255 cannot fit
        // inside an i8, so it'll emit a hard error. We intentionally disable
        // this guardrail to make this (bad) conversion possible.
        // The compiler is only able to pick on this because the value is a
        // literal. If we were to use a variable, the compiler wouldn't be able to
        // catch this at compile time.
        #[allow(overflowing_literals)]
        let x = { 255 as i8 };

        // You could solve this by using exactly the same expression as above,
        // but that would defeat the purpose of the exercise. Instead, use a genuine
        // `i8` value that is equivalent to `255` when converted from `u8`.
        //
        // logic: when casting to signed integer, "two's complement representation" method is used for casting
        // It works as follows:
        // 1. convert 255 into bits --> `11111111`
        // 2. treat most significant bit as sign (+/-), "1" means "-" and "0" means "+" --> negative sign
        // 3. invert all bits and add 1 bit --> `11111111` becomes `00000000` + 1 --> `00000001` --> 1
        // 4. Result --> negative 1 --> -1
        let y: i8 = -1;

        assert_eq!(x, y);
    }

    #[test]
    fn bool_to_u8() {
        let v: u8 = 1;
        assert_eq!(true as u8, v);
    }
}
