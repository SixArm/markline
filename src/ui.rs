use cursive::{view::{Nameable, Resizable, Selector}};
use crate::views::mark_view::MarkView;

pub fn initialize(c: &mut cursive::Cursive) -> usize {
    initialize_keys(c);
    initialize_theme(c);
    let row_count = initialize_rows(c);
    row_count
}

pub fn initialize_keys(c: &mut cursive::Cursive) {
    c.add_global_callback(cursive::event::Key::Esc, |c| c.quit());
    c.add_global_callback(cursive::event::Key::Enter, |c| c.quit());
}

pub fn initialize_theme(c: &mut cursive::Cursive) {
    let mut theme = c.current_theme().clone();
    theme.shadow = false;
    theme.borders = cursive::theme::BorderStyle::None;
    theme.palette[cursive::theme::PaletteColor::Background] = cursive::theme::Color::TerminalDefault;
    theme.palette[cursive::theme::PaletteColor::View] = cursive::theme::Color::TerminalDefault;
    c.set_theme(theme);
}

pub fn initialize_rows(c: &mut cursive::Cursive) -> usize {
    let mut row_index: usize = 0;
    let mut grid = cursive::views::LinearLayout::vertical();
    for line in std::io::stdin().lines() {
        let line = line.expect("line");
        if line.len() > 0 {
            grid.add_child(initialize_row(&line, row_index).full_width());
            row_index += 1;
        }
    }
    c.add_fullscreen_layer(grid.with_name("grid").full_screen());
    row_index
}

pub fn initialize_row(line_str: &str, row_index: usize) ->  cursive::views::LinearLayout {
    let mut row = cursive::views::LinearLayout::horizontal();
    let markbox = MarkView { button: markbox_as_button() };
    let text_view = cursive::views::TextView::new(line_str.to_owned());
    row.add_child(markbox.with_name(format!("row_{}_markbox", row_index)).fixed_width(1));
    row.add_child(cursive::views::DummyView);
    row.add_child(text_view.with_name(format!("row_{}_text_view", row_index)).full_width());
    row
}

pub fn markbox_as_button() -> cursive::views::Button {
    let button = cursive::views::Button::new_raw("•", |_| {});
    button
}

pub fn print_output(c: &mut cursive::Cursive, row_count: usize) {
    let mut output = String::new();
    for row_index in 0..row_count {
        c.call_on(&Selector::Name(&format!("row_{}_markbox", row_index)), |markbox: &mut MarkView| {
            output.push_str(markbox.button.label());
            output.push_str(" ");
        });
        c.call_on(&Selector::Name(&format!("row_{}_text_view", row_index)), |text_view: &mut cursive::views::TextView| {
            output.push_str(&format!("{}\n", text_view.get_content().source()));
        });
    }
    if output.len() == 0 { output = String::from("\n"); }
    print!("{}", output);
}
