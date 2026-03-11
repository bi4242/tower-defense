use nannou::prelude::*;

use crate::game::Game;

pub fn run() {
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

fn update(_app: &App, model: &mut Game, _update: Update) {
    
}

fn view(app: &App, model: &Game, frame: Frame) {
    let draw = app.draw();

    draw.background().color(REBECCAPURPLE);

    draw.to_frame(app, &frame).unwrap();
}
