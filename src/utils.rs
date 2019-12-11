// Hm, utils... because I don't care http://ralin.io/blog/oop-anti-patterns-utility-or-helper-classes.html


pub fn str_vec_to_i32_vec(input: &str, separator: &str) -> Vec<i32> {
    input
        .trim()
        .split(separator)
        .map(|m| m.trim())
        .map(|m| m.parse::<i32>().unwrap())
        .collect()
}
