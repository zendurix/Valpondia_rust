use rltk::Rltk;

const WINDOW_TITLE: &str = "Valpondia";

pub const SPRITE_CONSOLE_INDEX: usize = 0;
pub const CHAR_CONSOLE_INDEX: usize = 1;

pub fn create_window(width: usize, height: usize) -> Rltk {
    rltk::RltkBuilder::simple(width, height)
        .unwrap()
        .with_title(WINDOW_TITLE)
        .build()
        .unwrap()
}

pub fn create_sprite_window(width: usize, height: usize) -> Rltk {
    let context = rltk::RltkBuilder::new()
        .with_dimensions(width, height)
        .with_tile_dimensions(16, 16)
        .with_title("Valpondia")
        .with_font("sprite_sheet_16x16.png", 16, 16)
        .with_simple_console(width, height, "sprite_sheet_16x16.png")
        .with_font("terminal_16x16.png", 16, 16)
        .with_simple_console_no_bg(width, height, "terminal_16x16.png")
        .build()
        .unwrap();

    context
}
