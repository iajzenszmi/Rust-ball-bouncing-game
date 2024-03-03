extern crate piston_window;

use piston_window::*;

struct Game {
    ball: Ball,
}

struct Ball {
    x: f64,
    y: f64,
    vel_x: f64,
    vel_y: f64,
    radius: f64,
}

impl Game {
    fn new() -> Game {
        Game {
            ball: Ball {
                x: 100.0,
                y: 100.0,
                vel_x: 2.0,
                vel_y: 2.0,
                radius: 10.0,
            },
        }
    }

    fn run(&mut self, window: &mut PistonWindow) {
        while let Some(event) = window.next() {
            if let Some(_) = event.render_args() {
                self.draw(window, &event);
            }

            if let Some(_) = event.update_args() {
                self.update();
            }
        }
    }

    fn draw(&self, window: &mut PistonWindow, event: &Event) {
        window.draw_2d(event, |c, g, _| {
            clear([1.0; 4], g);
            ellipse(
                [1.0, 0.0, 0.0, 1.0], // red color
                [self.ball.x, self.ball.y, self.ball.radius * 2.0, self.ball.radius * 2.0],
                c.transform,
                g,
            );
        });
    }

    fn update(&mut self) {
        self.ball.x += self.ball.vel_x;
        self.ball.y += self.ball.vel_y;

        if self.ball.x > 300.0 - self.ball.radius || self.ball.x < self.ball.radius {
            self.ball.vel_x = -self.ball.vel_x;
        }

        if self.ball.y > 300.0 - self.ball.radius || self.ball.y < self.ball.radius {
            self.ball.vel_y = -self.ball.vel_y;
        }
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Bouncing Ball", [300, 300])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new();
    game.run(&mut window);
}
