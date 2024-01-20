mod greeting {
    pub fn greet(name: &str) {
        println!("Hello, {}!", name);
    }
}

fn main() {
    greeting::greet("Chennai Geeks");
}
