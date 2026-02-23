struct Sum {
    nums: Vec<u32>,
    carry: u32,
}

impl Sum {
    fn new(nums: Vec<u32>, carry: u32) -> Self {
        Self { nums, carry }
    }

    fn summation(nums: Vec<u32>) -> u32 {
        let sum = Sum::new(nums, 0);
        let result_xor = sum.nums.iter().copied().reduce(|a, b| a ^ b);
        let result_and = sum
            .nums
            .iter()
            .copied()
            .reduce(|a, b| a & b)
            .unwrap_or_default();

        let carry: u32 = result_and << 1;

        if carry == 0 {
            result_xor.unwrap_or_default()
        } else {
            Sum::summation(vec![result_xor.unwrap_or_default(), carry])
        }
    }
}
