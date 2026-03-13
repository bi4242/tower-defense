use nannou::prelude::*;
use crate::game::Game;

mod game;

pub fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Game {
    app.new_window()
        .size(400, 400)
        .view(view)
        .build()
        .unwrap();

    Game::new()
}

fn update(_app: &App, game: &mut Game, _update: Update) {
    
}

fn view(app: &App, game: &Game, frame: Frame) {
    let draw = app.draw();

    game.draw(&draw);

    draw.to_frame(app, &frame).unwrap();
}
