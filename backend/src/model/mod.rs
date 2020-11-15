pub struct Model {
}

pub trait Modelable {
    fn to_rep(&self) -> Model;
}

pub mod actors;
pub mod dvds;
pub mod games;
pub mod mangas;
pub mod sites;
pub mod studios;
pub mod tags;
pub mod videos;

pub enum Entry {
    Actor(actors::Actor),
    Dvd(dvds::Dvd),
    Game(games::Game),
}
