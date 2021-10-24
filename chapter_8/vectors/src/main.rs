
fn main() {
    let v1 = vec![1,2,3];
    let v2: Vec<i32> = Vec::new();

    dbg!(&v1);
    dbg!(&v2);

    let two = &v1[1];
    dbg!(two);

}
