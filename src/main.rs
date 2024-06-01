pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    nums.dedup();
    nums.len() as i32
}

fn main() {
    let mut nums = vec![1,1,2];
    remove_duplicates(&mut nums);
}
