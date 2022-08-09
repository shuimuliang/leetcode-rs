#[cfg(test)]
mod tests {
    use borsh::{BorshDeserialize, BorshSerialize};

    type Pubkey = [u8; 32];
    #[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
    struct BoshFoo {
        pub x: u64,    // 8
        pub y: Pubkey, // 32
    }

    #[test]
    fn test_case_1() {
        let a = BoshFoo {
            x: 3301,
            y: [
                128, 209, 54, 166, 146, 74, 68, 17, 195, 145, 227, 240, 201, 161, 20, 43, 27, 15,
                51, 229, 124, 114, 235, 205, 100, 237, 204, 198, 248, 194, 181, 9,
            ],
        };
        let encoded_a: Vec<u8> = a.try_to_vec().unwrap();
        assert_eq!(40, encoded_a.len());
        let decoded_a = BoshFoo::try_from_slice(&encoded_a).unwrap();
        assert_eq!(a, decoded_a);
    }

    #[test]
    fn test_case_from_zero_bytes() {
        let encoded_a: Vec<u8> = vec![0; 40];
        let decoded_a: BoshFoo = BoshFoo::try_from_slice(&encoded_a).unwrap();
        assert_eq!(0, decoded_a.x);
        assert_eq!(32, decoded_a.y.len());
    }
}
