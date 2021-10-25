use rltk::Rltk;

const WINDOW_TITLE: &str = "Valpondia";

pub fn create_window(width: usize, height: usize) -> Rltk {
    rltk::RltkBuilder::simple(width, height)
        .unwrap()
        .with_title(WINDOW_TITLE)
        .build()
        .unwrap()
}
