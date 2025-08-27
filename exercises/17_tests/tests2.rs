// Calculates the power of 2 using a bit shift.
// `1 << n` is equivalent to "2 to the power of n".
fn two_to_the_power_of(n: u8) -> u64 {
    1 << n
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_can_assert_eq() {
        assert_eq!(two_to_the_power_of(2), 4);
        assert_eq!(two_to_the_power_of(32), 4294967296);
        assert_eq!(two_to_the_power_of(63), 9223372036854775808);
        assert_eq!(two_to_the_power_of(13), 8192);
    }
}
