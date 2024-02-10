use ai_edge::run_inference;
fn main() {
    let d = vec![1.0, 2.0];
    println!("Result: {}", run_inference(d));
}
