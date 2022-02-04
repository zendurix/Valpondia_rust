use rltk::Rltk;

const WINDOW_TITLE: &str = "ROGUELIKE";

pub const SPRITE_16X16_CONSOLE_INDEX: usize = 1;
pub const SPRITE_32X32_CONSOLE_INDEX: usize = 0;
pub const CHAR_CONSOLE_INDEX: usize = 2;

pub fn create_window(width: usize, height: usize) -> Rltk {
    rltk::RltkBuilder::simple(width, height)
        .unwrap()
        .with_title(WINDOW_TITLE)
        .build()
        .unwrap()
}

pub fn create_sprite_window(width: usize, height: usize) -> Rltk {
    let context = rltk::RltkBuilder::new()
        .with_dimensions(width / 2, height / 2)
        .with_tile_dimensions(32, 32)
        .with_title(WINDOW_TITLE)
        .with_font("sprite_sheet_32x32.png", 32, 32)
        .with_simple_console(width / 2, height / 2, "sprite_sheet_32x32.png")
        .with_font("sprite_sheet_16x16.png", 16, 16)
        .with_simple_console_no_bg(width, height, "sprite_sheet_16x16.png")
        .with_font("terminal_16x16.png", 16, 16)
        .with_simple_console_no_bg(width, height, "terminal_16x16.png")
        .build()
        .unwrap();

    context
}
