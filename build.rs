use winres::WindowsResource;
use chrono::{Datelike,Utc};

fn main() {
    if cfg!(target_os = "windows") {
        let mut resources = WindowsResource::new();
        resources.set_icon("wake.ico");
        resources.set("LegalCopyright", format!("© {} Ben Hutchison", Utc::now().year_ce().1).as_str());
        resources.set_language(0x0409); // English (US)
        resources.compile().unwrap();
    }
}