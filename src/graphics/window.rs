use rltk::{Rltk, SpriteSheet};

const WINDOW_TITLE: &str = "Valpondia";

pub fn create_window(width: usize, height: usize) -> Rltk {
    rltk::RltkBuilder::simple(width, height)
        .unwrap()
        .with_title(WINDOW_TITLE)
        .build()
        .unwrap()
}

pub fn create_sprite_window(width: usize, height: usize) -> Rltk {
    // let mut ss = SpriteSheet::new("rsc/sprite_sheet_8x8.png");
// 
    // let mut count = 0;
// 
    // for j in 0..10 {
    //     for i in 0..10 {
    //         if count == 18 {
    //             break;
    //         }
// 
    //         //   ss = ss.add_sprite(rltk::Rect {
    //         //       x1: i * 8,
    //         //       x2: (i + 1) * 8,
    //         //       y1: j * 8,
    //         //       y2: (j + 1) * 8,
    //         //   });
// 
    //         ss = ss.add_sprite(rltk::Rect::with_size(i * 8, j * 8, 8, 8));
// 
    //         count += 1;
    //     }
    // }



    println!("1");


    let mut context = rltk::RltkBuilder::new()
    // We specify the CONSOLE dimensions
    .with_dimensions(width , height)
    // We specify the size of the tiles
    .with_tile_dimensions(16, 16)
    // We give it a window title
    .with_title("Valpondia - Tiles");
    // We register our embedded "example_tiles.png" as a font.
    println!("2");
    context = context.with_font("sprite_sheet_16x16.png", 16, 16);
    println!("3");
    // We want a base simple console for the terrain background
    context = context.with_simple_console_no_bg(width , height, "sprite_sheet_16x16.png");
    println!("4");



    
    let c = context.with_font("terminal_16x16.png", 16, 16)
    .with_simple_console_no_bg(width, height, "terminal_16x16.png")



    // We also want a sparse console atop it to handle moving the character
    //.with_sparse_console_no_bg(WIDTH as u32, HEIGHT as u32, "example_tiles.png")
    // And we call the builder function
    .build().unwrap();



    c
}
