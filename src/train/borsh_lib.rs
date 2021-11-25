

#[cfg(test)]
mod tests {
    use borsh::{BorshSerialize, BorshDeserialize};

    #[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
    struct BoshFoo {
        x: u64,
        y: String,
    }

    #[test]
    fn test_case_1() {
        let a = BoshFoo {
            x: 3301,
            y: "liber primus".to_string(),
        };
        let encoded_a: Vec<u8> = a.try_to_vec().unwrap();
        let decoded_a = BoshFoo::try_from_slice(&encoded_a).unwrap();
        assert_eq!(a, decoded_a);
    }
}
