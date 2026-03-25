use std::cell::RefCell;

use nannou::prelude::*;
use crate::game::Game;

mod game;

pub fn main() {
    nannou::app(model).event(event).run();
}

fn model(app: &App) -> RefCell<Game> {
    app
        .new_window()
        .size(400, 400)
        .view(view)
        .build()
        .unwrap();

    RefCell::new(Game::new())
}

fn event(_app: &App, model: &mut RefCell<Game>, event: Event) {
    let game = model.replace(Game::DUMMY);
    let game = game.event(event);
    model.replace(game);
}

fn view(app: &App, model: &RefCell<Game>, frame: Frame) {
    let draw = app.draw();

    model.borrow().draw(&draw);

    draw.to_frame(app, &frame).unwrap();
}
