#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    // Main implementation will be added in issue #4
    println!("TodoMVC server starting...");
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
