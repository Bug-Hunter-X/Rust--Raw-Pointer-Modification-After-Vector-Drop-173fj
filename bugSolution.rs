fn main() {
    let mut v = vec![1, 2, 3];
    // Correct way: use a reference instead of raw pointer
    let first_element = &mut v[0];
    *first_element = 10;
    println!("First element: {}", v[0]);
} 
//Alternative:
fn main() {
    let mut v = vec![1, 2, 3];
    let index = 0;
    unsafe{
        let ptr = v.as_mut_ptr();
        ptr.add(index).write(10);
    }
    println!("First element: {}", v[0]);
}