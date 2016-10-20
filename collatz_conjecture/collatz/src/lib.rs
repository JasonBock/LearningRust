pub mod collatz {
    pub struct CollatzResults {
        pub value: u64,
        pub sequence: Vec<u64>,
    }

    pub fn analyze(value: u64) -> CollatzResults {
        let mut sequence: Vec<u64> = Vec::new();
        let mut current_value = value;

        while current_value > 1 {
            if current_value % 2 == 1 {
                current_value = 3 * current_value + 1;
            }
            else {
                current_value = current_value / 2;
            }

            sequence.push(current_value);
        }

        CollatzResults{ value: value, sequence: sequence }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn analyze_number_6() {
        let result = collatz::analyze(6);
        assert_eq!(6, result.value);
        assert_eq!(8, result.sequence.len());
        assert_eq!(3, result.sequence[0]);
        assert_eq!(10, result.sequence[1]);
        assert_eq!(5, result.sequence[2]);
        assert_eq!(16, result.sequence[3]);
        assert_eq!(8, result.sequence[4]);
        assert_eq!(4, result.sequence[5]);
        assert_eq!(2, result.sequence[6]);
        assert_eq!(1, result.sequence[7]);
    }

    #[test]
    fn analyze_number_1() {
        let result = collatz::analyze(1);
        assert_eq!(1, result.value);
        assert_eq!(0, result.sequence.len());
    }

    #[test]
    fn analyze_number_0() {
        let result = collatz::analyze(0);
        assert_eq!(0, result.value);
        assert_eq!(0, result.sequence.len());
    }
}