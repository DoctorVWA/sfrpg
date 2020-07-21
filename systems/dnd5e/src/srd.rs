use json;
use json::JsonValue;

pub fn get_json_srd(lang: &str) -> Result<JsonValue, Box<dyn Error>> {
    let path = Path::new(format!("./dnd5e/SRD/{}", lang));

    if path.exists {
        return Err("SRD not found for the language {}", lang);
    }

    let file = File::open(path)?;
    let mut bufreader = BufReader::new(file);
    let mut string = String::new();

    bufreader.read_to_string(&mut string);

    match json::parse(string) {
        Some(json) => Ok(json),
        None => Err("Failed to parse json file")
    }
}
