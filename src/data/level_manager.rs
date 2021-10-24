use super::level::Level;





/// 
pub struct LevelManager {
    pub current_level_index: usize,
    levels: Vec<Level>,
    
}



impl LevelManager {

    pub fn new() -> LevelManager {

        LevelManager {
            current_level_index: 0,
            levels: vec![]
        }

    }


    pub fn current_level(&self) -> &Level {
        &self.levels[self.current_level_index]
    }

    


}




