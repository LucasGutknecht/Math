struct Sum {
    nums: Vec<u32>,
    carry: u32,
}

impl Sum {
    fn new(nums: Vec<u32>, carry: u32) -> Self {
        Self { nums, carry }
    }

    fn reverse_string(input: &str) -> String {
        input.chars().rev().collect()
    }

    fn get_binary(&mut self) -> Vec<String> {
        let mut binary_nums = Vec::new();
        let mut binary_num = String::new();
        let mut dividend: u32;
        for num in self.nums.iter().copied() {
            dividend = num;
            while dividend != 0 {
                if !dividend.is_multiple_of(2) {
                    binary_num.push_str(&1.to_string());
                } else {
                    binary_num.push_str(&0.to_string());
                }
                dividend /= 2;
            }

            binary_nums.push(Sum::reverse_string(&binary_num));
            binary_num.clear();
        }
        binary_nums
    }
}
