use std::{fs::File, io::BufReader, path::PathBuf};
use xml::{reader::XmlEvent, *};

pub fn load_saves() -> Vec<PathBuf> {
    let mut vec = Vec::new();

    if let Ok(home) = std::env::var("HOME") {
        // macos

        let base = PathBuf::from(format!(
            "{}/Library/Application Support/Celeste/Saves",
            home
        ));

        if base.exists() {
            for i in 0..=2 {
                let path = base.join(format!("{}.celeste", i));

                if path.exists() {
                    vec.push(path);
                }
            }
        } else if let Ok(data_home) = std::env::var("XDG_DATA_HOME") {
            let base = PathBuf::from(format!("{}/Celeste/Saves", data_home));

            for i in 0..=2 {
                let path = base.join(format!("{}.celeste", i));

                if path.exists() {
                    vec.push(path);
                }
            }
        }
    }

    vec
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Summary {
    pub version: String,     // generally something like 1.4.0.0
    pub player_name: String, // Usually Madeline
    pub cheat_mode: bool,
    pub assist_mode: bool,
    pub variant_mode: bool,
    pub strawberries: i32,
    pub golden_strawberries: i32,
    pub deaths: i32,
    pub jumps: i32,
    pub dashes: i32,
    pub wall_jumps: i32,
    pub celeste: Celeste,
}

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Celeste {
    pub prologue: AreaMode,
    pub city: AreaMode,
    pub site: AreaMode,
    pub resort: AreaMode,
    pub ridge: AreaMode,
    pub temple: AreaMode,
    pub reflection: AreaMode,
    pub summit: AreaMode,
    pub core: AreaMode,
    pub farewell: AreaMode,
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct AreaMode {
    pub a_side: AreaData,
    pub b_side: AreaData,
    pub c_side: AreaData,
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct AreaData {
    pub strawberries: i32,
    pub deaths: i32,
    pub heart_gem: bool,
    pub completed: bool,
}

pub fn generate_summary(str: PathBuf) -> Result<Summary, Error> {
    let mut version = String::new();
    let mut player_name = String::new();
    let mut cheat_mode = false;
    let mut assist_mode = false;
    let mut variant_mode = false;
    let mut golden_strawberries: i32 = 0;
    let mut strawberries: i32 = 0;
    let mut deaths: i32 = 0;
    let mut jumps = 0;
    let mut dashes = 0;
    let mut wall_jumps = 0;

    let mut celeste = Celeste::default();

    let mut a = AreaData::default();
    let mut b = AreaData::default();
    let mut c = AreaData::default();
    let mut i = 0;

    // parse the XML first

    let file = File::open(str).map_err(Error::IoError)?;
    let file = BufReader::new(file);

    let parser = EventReader::new(file);

    let mut current_element = String::new();
    let mut current_area = String::new();

    for event in parser {
        if let Ok(s) = event {
            match s {
                XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                } => {
                    current_element = name.to_string();

                    if name.to_string() == "AreaStats" {
                        for attribute in attributes {
                            if attribute.name.to_string() == "SID" {
                                current_area = attribute.value;
                            }
                        }
                    } else if name.to_string() == "AreaModeStats" {
                        // parse everything

                        if i == 0 {
                            for attribute in attributes {
                                let name = attribute.name.to_string();

                                if name == "TotalStrawberries" {
                                    a.strawberries = attribute.value.parse().unwrap_or_default();
                                } else if name == "Completed" {
                                    a.completed = attribute.value == "true";
                                } else if name == "Deaths" {
                                    a.deaths = attribute.value.parse().unwrap_or_default();
                                } else if name == "HeartGem" {
                                    a.heart_gem = attribute.value == "false";
                                }
                            }

                            i += 1;
                        } else if i == 1 {
                            for attribute in attributes {
                                let name = attribute.name.to_string();

                                if name == "TotalStrawberries" {
                                    b.strawberries = attribute.value.parse().unwrap_or_default();
                                } else if name == "Completed" {
                                    b.completed = attribute.value == "true";
                                } else if name == "Deaths" {
                                    b.deaths = attribute.value.parse().unwrap_or_default();
                                } else if name == "HeartGem" {
                                    b.heart_gem = attribute.value == "false";
                                }
                            }

                            i += 1;
                        } else if i == 2 {
                            for attribute in attributes {
                                let name = attribute.name.to_string();

                                if name == "TotalStrawberries" {
                                    c.strawberries = attribute.value.parse().unwrap_or_default();
                                } else if name == "Completed" {
                                    c.completed = attribute.value == "true";
                                } else if name == "Deaths" {
                                    c.deaths = attribute.value.parse().unwrap_or_default();
                                } else if name == "HeartGem" {
                                    c.heart_gem = attribute.value == "false";
                                }
                            }

                            let area_mode = AreaMode {
                                a_side: a,
                                b_side: b,
                                c_side: c,
                            };

                            match current_area.as_str() {
                                "Celeste/0-Intro" => celeste.prologue = area_mode,
                                "Celeste/1-ForsakenCity" => celeste.city = area_mode,
                                "Celeste/2-OldSite" => celeste.site = area_mode,
                                "Celeste/3-CelestialResort" => celeste.resort = area_mode,
                                "Celeste/4-GoldenRidge" => celeste.ridge = area_mode,
                                "Celeste/5-MirrorTemple" => celeste.temple = area_mode,
                                "Celeste/6-Reflection" => celeste.reflection = area_mode,
                                "Celeste/7-Summit" => celeste.summit = area_mode,
                                "Celeste/9-Core" => celeste.core = area_mode,
                                "Celeste/LostLevels" => celeste.farewell = area_mode,
                                _ => {}
                            }

                            a = Default::default();
                            b = Default::default();
                            c = Default::default();

                            i = 0;
                        }
                    }
                }

                XmlEvent::EndElement { name } => {
                    current_element = String::new();
                }

                XmlEvent::Characters(char) => {
                    if current_element == "Version" {
                        version = char;
                    } else if current_element == "Name" {
                        player_name = char;
                    } else if current_element == "AssistMode" {
                        assist_mode = char != "false";
                    } else if current_element == "CheatMode" {
                        cheat_mode = char != "false";
                    } else if current_element == "VariantMode" {
                        variant_mode = char != "false";
                    } else if current_element == "TotalStrawberries" {
                        if strawberries == 0 {
                            strawberries = char.parse().unwrap_or_default();
                        }
                    } else if current_element == "TotalGoldenStrawberries" {
                        golden_strawberries = char.parse().unwrap_or_default();
                    } else if current_element == "TotalDeaths" {
                        deaths = char.parse().unwrap_or_default();
                    } else if current_element == "TotalJumps" {
                        jumps = char.parse().unwrap_or_default();
                    } else if current_element == "TotalWallJumps" {
                        wall_jumps = char.parse().unwrap_or_default();
                    } else if current_element == "TotalDashes" {
                        dashes = char.parse().unwrap_or_default();
                    }
                }

                _ => {}
            }
        } else {
            println!("Error");
        }
    }

    Ok(Summary {
        version,
        player_name,
        cheat_mode,
        assist_mode,
        variant_mode,
        golden_strawberries,
        strawberries,
        deaths,
        dashes,
        jumps,
        wall_jumps,
        celeste,
    })
}
