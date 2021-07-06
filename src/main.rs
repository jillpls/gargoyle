use dialoguer::Select;
use dialoguer::theme;
use console::Style;

mod data;

use data::Names;


fn build_theme() -> theme::ColorfulTheme {
    theme::ColorfulTheme {
        defaults_style : Style::new().white(),
        error_style : Style::new().white(),
        indicator_style : Style::new().red(),
        inactive_style : Style::new().white(),
        active_style : Style::new().white(),
        yes_style : Style::new().white(),
        no_style : Style::new().white(),
        values_style : Style::new().white(),
    }
}

fn build_selection<'a, T: ToString>(items : &[T], theme: &'a theme::ColorfulTheme) -> Select<'a> {
    let mut s = Select::with_theme(theme);
    s.items(items);
    s.default(0);
    s
}

fn main() {
    let names = Names::from_dir("./res");
    println!("{:?}", names.f_names);
    println!("Hello, world!");
    let theme = build_theme();
    let s = build_selection(&["Female", "Male", "Non-binary"], &theme);
    let r = s.interact();
    let (first_name, last_name) = {
        let first_name = match r.unwrap() {
            0 => names.get_rand_name(data::NameType::Female),
            1 => names.get_rand_name(data::NameType::Male),
            2 => names.get_rand_name(data::NameType::NonBinary),
            _ => data::Name { value: "".to_string() }
        };
        let last_name = names.get_rand_name(data::NameType::Surname);
        (first_name, last_name)
    };
    println!("First Name: {}\nLast Name: {}", first_name, last_name);
}
