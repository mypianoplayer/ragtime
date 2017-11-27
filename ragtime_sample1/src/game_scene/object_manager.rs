use components::*;
use game_scene::entities::player::*;
use game_scene::entities::obstacle::*;

pub struct ObjectManager {
    players: Vec<Player>,
    obstacles : Vec<Obstacle>,
}

impl ObjectManager {
    pub fn new() -> ObjectManager {
        ObjectManager {
            players: Vec::new(),
            obstacles: Vec::new(),
        }
    }
    pub fn update(&mut self) {
        let pl_upd_it = Player::updaters();
        let ob_upd_it = Obstacle::updaters();
        
        for updater in pl_upd_it.iter().zip(ob_upd_it.iter()) {
            let (pl_upd, ob_upd) = updater;
            for o in &mut self.players {
                pl_upd(o);
            }
            for o in &mut self.obstacles {
                ob_upd(o);
            }
        }
    }
}