use gxhash::{HashMap, HashMapExt};

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut complements = HashMap::with_capacity(nums.len());

    for (idx, num) in nums.iter().enumerate() {
        let complement = target - num;

        if let Some(complement_idx) = complements.get(&complement) {
            return vec![*complement_idx as i32, idx as i32];
        }

        complements.insert(num, idx);
    }

    vec![]
}

fn main() {
    let mut input = vec![2, 7, 11, 15];
    let mut target = 9;
    assert_eq!(two_sum(input, target), vec![0, 1]);

    input = vec![3, 2, 4];
    target = 6;
    assert_eq!(two_sum(input, target), vec![1, 2]);

    input = vec![3, 3];
    target = 6;
    assert_eq!(two_sum(input, target), vec![0, 1]);
}
