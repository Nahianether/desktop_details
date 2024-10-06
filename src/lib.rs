pub fn get_chrome_history(path: &str) -> Vec<String> {
    let query =
        "SELECT url, title, last_visit_time FROM urls ORDER BY last_visit_time DESC LIMIT 10";
    let conn = rusqlite::Connection::open(path).unwrap();
    let mut stmt = conn.prepare(query).unwrap();

    let rows = stmt.query_map([], |row| {
        Ok(format!(
            "{:?}: {:?} ({:?})",
            row.get::<usize, String>(0),
            row.get::<usize, String>(1),
            row.get::<usize, i64>(2)
        ))
    });

    let mut history = Vec::new();

    for row in rows.unwrap() {
        if let Ok(row) = row {
            println!("{}", row);
            history.push(row);
        }
    }

    history
}

pub fn get_running_processes() -> Vec<String> {
    let mut system = sysinfo::System::new_all();
    system.refresh_all();

    let mut processes = Vec::new();
    for (pid, process) in system.processes() {
        processes.push(format!(
            "{}: {:?} ({})",
            pid,
            process.name(),
            process.cpu_usage()
        ));
        println!("{}: {:?} ({})", pid, process.name(), process.cpu_usage());
    }

    processes
}

// pub fn detect_https_in_image(image_path: &str) -> bool {
//     let mut tess = Tesseract::new(None, Some("eng")).expect("Failed to initialize Tesseract");
//     tess.set_image(image_path).expect("Failed to set image");
//     let text = tess.get_text().expect("Failed to recognize text");

//     // Check if the recognized text contains "https://"
//     text.contains("https://")
// }
