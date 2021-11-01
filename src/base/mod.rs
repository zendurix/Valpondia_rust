mod nav;

pub use nav::Dir;

/// (x1, y1) - start, (x2, y2) - end
pub fn start_to_end_as_dir(x1: usize, y1: usize, x2: usize, y2: usize) -> Dir {
    let left = x2 as i32 - x1 as i32 == -1;
    let right = x2 as i32 - x1 as i32 == 1;
    let up = y2 as i32 - y1 as i32 == -1;
    let down = y2 as i32 - y1 as i32 == 1;

    if up && right {
        Dir::UpRight
    } else if up && left {
        Dir::UpLeft
    } else if down && left {
        Dir::DownLeft
    } else if down && right {
        Dir::DownRight
    } else if up {
        Dir::Up
    } else if down {
        Dir::Down
    } else if left {
        Dir::Left
    } else if right {
        Dir::Right
    } else {
        Dir::Center
    }
}
