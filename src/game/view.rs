use super::Game;

pub enum ViewAction {
    None,
    Quit,
    ChangeView(Box<View>),
}

pub trait View {
    fn update(&mut self, game: &mut Game, elapsed: u32) -> ViewAction;
    fn render(&mut self, game: &mut Game, elapsed: u32);
}
