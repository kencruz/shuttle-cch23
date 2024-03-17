use axum::extract::Path;
use std::collections::HashMap;

pub async fn exclusive_cube(Path(params): Path<HashMap<String, String>>) -> String {
    let mut nums = params
        .get("nums")
        .unwrap()
        .split("/")
        .map(|n| n.parse::<u32>().unwrap())
        .into_iter();

    let first = nums.next().unwrap();
    let xor = nums.fold(first, |n, acc| n ^ acc);
    let cubed = xor.pow(3);

    format!("{:?}\n", cubed)
}
