// Snake game

// import some crates
extern crate glutin_window; // backend for piston game engine
extern crate graphics;
extern crate opengl_graphics;
extern crate piston; // game engine

extern crate rand; // random

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

use std::collections::LinkedList;
use std::iter::FromIterator;

// Game struct
pub struct Game {
    gl: GlGraphics,
    rows: u32, // how many rows
    cols: u32, // how many columns
    snake: Snake,
    just_eaten: bool, // true or false
    square_width: u32, // size of square
    food: Food,
    score: u32,
}

// how the game is played
impl Game {
    // function render adds things to the screen
    fn render(&mut self, args: &RenderArgs) {
        

        const BLUE: [f32; 4] = [0.0, 1.0, 1.0, 1.0]; // background color of blue

        self.gl.draw(args.viewport(), |c, gl| {
            graphics::clear(BLUE, gl); // how we draw that color
        });

        self.snake.render(args); // add the snake
        self.food.render(&mut self.gl, args, self.square_width); // at the food
    }

    // function update updates the screen when certain things happen
    fn update(&mut self, args: &UpdateArgs) -> bool {
        if !self.snake.update(self.just_eaten, self.cols, self.rows) {
            return false;
        }

        // if the snake gets a piece of food at 1 to the score
        if self.just_eaten {
            self.score += 1;
            self.just_eaten = false;
        }

        // if the snake successfully eats we need to elongate its body
        self.just_eaten = self.food.update(&self.snake);
        if self.just_eaten {
            use rand::Rng;
            use rand::thread_rng;
            // try my luck
            let mut r = thread_rng();
            loop {
                let new_x = r.gen_range(0..self.cols);
                let new_y = r.gen_range(0..self.rows);
                if !self.snake.is_collide(new_x, new_y) {
                    self.food = Food { x: new_x, y: new_y };
                    break;
                }
            }
        }

        true
    }

    // function pressed is how we use the keyboard for the game
    fn pressed(&mut self, btn: &Button) {
        let last_direction = self.snake.d.clone();
        self.snake.d = match btn {
            &Button::Keyboard(Key::Up) if last_direction != Direction::DOWN => Direction::UP, // up
            &Button::Keyboard(Key::Down) if last_direction != Direction::UP => Direction::DOWN, // down
            &Button::Keyboard(Key::Left) if last_direction != Direction::RIGHT => Direction::LEFT, // left
            &Button::Keyboard(Key::Right) if last_direction != Direction::LEFT => Direction::RIGHT, // right
            _ => last_direction,
        };
    }
}

// the direction the snake moves in
#[derive(Clone, PartialEq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub struct Snake {
    gl: GlGraphics,
    snake_parts: LinkedList<SnakePiece>,
    width: u32,
    d: Direction,
}

#[derive(Clone)]
pub struct SnakePiece(u32, u32);

// impl's are new to me. The impl keyword in Rust is used to implement some functionality on types. This functionality can include both functions and costs.
impl Snake {
    // call render to put the snake on the screen
    pub fn render(&mut self, args: &RenderArgs) {
        

        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0]; // make the snake red

        // what the snake is gonna look like
        let squares: Vec<graphics::types::Rectangle> = self.snake_parts
            .iter()
            .map(|p| SnakePiece(p.0 * self.width, p.1 * self.width))
            .map(|p| graphics::rectangle::square(p.0 as f64, p.1 as f64, self.width as f64))
            .collect();

        // drawing the snake
        self.gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            squares
                .into_iter()
                .for_each(|square| graphics::rectangle(RED, square, transform, gl));
        })
    }

    // move the snake if valid, otherwise it returns false
    pub fn update(&mut self, just_eaten: bool, cols: u32, rows: u32) -> bool {
        let mut new_front: SnakePiece =
            (*self.snake_parts.front().expect("No front of snake found.")).clone(); // debugging

        //if a valid direction
        if (self.d == Direction::UP && new_front.1 == 0)
            || (self.d == Direction::LEFT && new_front.0 == 0)
            || (self.d == Direction::DOWN && new_front.1 == rows - 1)
            || (self.d == Direction::RIGHT && new_front.0 == cols - 1)
        {
            return false;
        }

        // how we make the rest of its body move in the same direction as its getting longer
        match self.d {
            Direction::UP => new_front.1 -= 1,
            Direction::DOWN => new_front.1 += 1,
            Direction::LEFT => new_front.0 -= 1,
            Direction::RIGHT => new_front.0 += 1,
        }

        if !just_eaten {
            self.snake_parts.pop_back(); // pop_back() removes the last element from a list and returns it
        }

        // checks self collision
        if self.is_collide(new_front.0, new_front.1) {
            return false;
        }

        self.snake_parts.push_front(new_front); // push_front() adds an element first in the list
        true
    }

    // collision
    fn is_collide(&self, x: u32, y: u32) -> bool {
        self.snake_parts.iter().any(|p| x == p.0 && y == p.1)
    }
}

pub struct Food {
    x: u32,
    y: u32,
}

impl Food {
    // return true if snake ate food this update
    fn update(&mut self, s: &Snake) -> bool {
        let front = s.snake_parts.front().unwrap();
        if front.0 == self.x && front.1 == self.y {
            true
        } else {
            false
        }
    }

    // render the food
    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs, width: u32) {
        

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0]; // color of the food

        let x = self.x * width;
        let y = self.y * width;

        let square = graphics::rectangle::square(x as f64, y as f64, width as f64); // its just gonna be a square

        // draw the food
        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            graphics::rectangle(BLACK, square, transform, gl)
        });
    }
}

// all rust programs need a main function
fn main() {
    
    let opengl = OpenGL::V3_2; // get the right version

    const COLS: u32 = 30; // how many columns
    const ROWS: u32 = 20; // how many rows
    const SQUARE_WIDTH: u32 = 20;

    let width = COLS * SQUARE_WIDTH; // get our width and height
    let height = ROWS * SQUARE_WIDTH;

    // get the window working
    let mut window: GlutinWindow = WindowSettings::new("Snake Game", [width, height])
        //.opengl(opengl)
        .exit_on_esc(true) // lets us hit esc key to quit
        .build()
        .unwrap();

    // make our snake game
    let mut game = Game {
        gl: GlGraphics::new(opengl),
        rows: ROWS,
        cols: COLS,
        square_width: SQUARE_WIDTH,
        just_eaten: false,
        food: Food { x: 1, y: 1 },
        score: 0,
        snake: Snake {
            gl: GlGraphics::new(opengl),
            snake_parts: LinkedList::from_iter((vec![SnakePiece(COLS / 2, ROWS / 2)]).into_iter()),
            width: SQUARE_WIDTH,
            d: Direction::DOWN,
        },
    };
    
    // rendering, updating, and making sure the buttons work
    let mut events = Events::new(EventSettings::new()).ups(10);
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r);
        }

        if let Some(u) = e.update_args() {
            if !game.update(&u) {
                break;
            }
        }

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                game.pressed(&k.button);
            }
        }
    }
    // print out final score to terminal
    println!("Congratulations, your score was: {}", game.score);
}