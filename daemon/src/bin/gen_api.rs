fn main() {
    let content = rayhunter_daemon::ApiDocs::generate();
    std::fs::write("openapi.json", content).unwrap();
}
