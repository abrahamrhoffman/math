use hashbrown::HashMap;

fn main() {
    let mut a_dict = HashMap::new();
    a_dict.insert("A", "0");
    a_dict.insert("B", "1");
    a_dict.insert("C", "2");
    for (k, v) in &a_dict {
        if 
        println!("({},{})", k, v);
    }
}
