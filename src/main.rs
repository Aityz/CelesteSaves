mod loader;

fn main() {
    println!("Loading Save Files!");

    let saves = loader::load_saves();

    println!("Loaded {} saves!\n", saves.len());

    for (i, save) in saves.into_iter().enumerate() {
        let summary = loader::generate_summary(save);

        if summary.is_err() {
            println!("Failed to read file!");

            return;
        }

        let summary = summary.unwrap();

        println!("****************************************");
        println!("Save Number: {}", i + 1);
        println!("Game Version: {}", summary.version);
        println!("Player Name: {}\n", summary.player_name);

        if summary.cheat_mode {
            println!("\x1b[0;31mCheat mode is enabled.\x1b[0m");
        }

        if summary.assist_mode {
            println!("\x1b[0;33mAssist mode is enabled.\x1b[0m");
        }

        if summary.variant_mode {
            println!("\x1b[0;32mVariant mode is enabled.\x1b[0m");
        }

        println!(
            "Strawberries: {}+{} ({})",
            summary.strawberries - summary.golden_strawberries,
            summary.golden_strawberries,
            summary.strawberries
        );

        println!("Deaths: {}\n", summary.deaths);

        println!("Total Jumps: {}", summary.jumps);
        println!("Total Dashes: {}", summary.dashes);
        println!("Total Wall-jumps: {}\n", summary.wall_jumps);

        let chapters = [
            summary.celeste.prologue,
            summary.celeste.city,
            summary.celeste.site,
            summary.celeste.resort,
            summary.celeste.ridge,
            summary.celeste.temple,
            summary.celeste.reflection,
            summary.celeste.summit,
            summary.celeste.core,
            summary.celeste.farewell,
        ];

        let mut i = 0;
        for chapter in chapters {
            fn return_tick(bool: bool) -> char {
                if bool {
                    '✅'
                } else {
                    '❌'
                }
            }

            println!("Chapter {}: A: {}, {} deaths, B: {}, {} deaths, C: {}, {} deaths, Strawberries: {}", i, return_tick(chapter.a_side.completed), chapter.a_side.deaths, return_tick(chapter.b_side.completed), chapter.b_side.deaths, return_tick(chapter.c_side.completed), chapter.c_side.deaths, chapter.a_side.strawberries);

            i += 1;
        }

        println!("****************************************");

        let _ = std::io::stdin().read_line(&mut String::new());
    }
}
