use std::thread;
use std::time::Duration;

fn main() {
    println!("[INFO] Starting Web Automation Bot...");
    let target = "https://internal-api.base58labs.com/health";
    
    loop {
        println!("[TASK] Pinging target: {}", target);
        // Simulate health check
        thread::sleep(Duration::from_secs(60));
        println!("[SUCCESS] Service is UP. 200 OK.");
    }
}
