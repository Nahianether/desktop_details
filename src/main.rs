use rdev::{listen, Event, EventType};
// use leptess::LepTess;
// use regex::Regex;

fn main() {
    // capture_log::get_chrome_history(
    //     // "C:\\Users\\AGL\\AppData\\Local\\Google\\Chrome\\User Data\\Default\\History",
    //     "/Users/intishar/Library/Application Support/Google/Chrome/Default/History",
    // );

    // capture_log::get_running_processes();

//     let image_path = "src/c.jpeg";
    
//     let mut lt = LepTess::new(None, "eng").expect("Failed to initialize Tesseract");
//     lt.set_image(image_path).expect("Failed to set image");
//     let text = lt.get_utf8_text().expect("Failed to extract text");

//     let domain_regex = Regex::new(r"(https?://)?([a-zA-Z0-9-]+\.[a-zA-Z]{2,})").unwrap();

//     let mut domains_found = Vec::new();
//     for capture in domain_regex.captures_iter(&text) {
//     if let Some(domain) = capture.get(2) {
//         domains_found.push(domain.as_str().to_string());
//     }
// }

// if !domains_found.is_empty() {
//     println!("Domains found in the image: {:?}", domains_found);
// } else {
//     println!("No domains found in the image.");
// }
if let Err(error) = listen(callback) {
    println!("Error: {:?}", error)
}

}

fn callback(event: Event) {
    match event.event_type {
        EventType::KeyPress(key) => {
            println!("Key pressed: {:?}", key);
        }
        EventType::MouseMove { x, y } => {
            println!("Mouse moved to: ({}, {})", x, y);
        }
        _ => (),
    }
}