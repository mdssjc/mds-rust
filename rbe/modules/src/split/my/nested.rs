pub fn function() {
    println!("called `split.my::nested::function()`");
}

#[allow(dead_code)]
fn private_function() {
    println!("called `split.my::nested::private_function()`");
}
