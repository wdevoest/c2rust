extern crate sdl2;
extern crate rand;

use std::cell::RefCell;
use std::collections::LinkedList;
use std::rc::Rc;
use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::{Window, WindowContext};
use rand::Rng;

const SCREEN_WIDTH: u32 = 1024;
const SCREEN_HEIGHT: u32 = 576;
const TILE_SIZE: u32 = 16;
const BOX_X: u32 = 128;
const BOX_Y: u32 = 72;
const BOX_W: u32 = 768;
const BOX_H: u32 = 432;

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
struct Box {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

#[derive(Clone)]
struct SnakeCell {
    x: u32,
    y: u32,
    direction: Direction,
    next: Option<Rc<RefCell<SnakeCell>>>,
}

#[derive(Clone)]
struct Apple {
    x: u32,
    y: u32,
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let window_context = sdl_context.video().unwrap();

    let window = window_context
        .window("Serpent", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut snake = create_snake();
    let mut apple = create_apple();

    let fps: u32 = 60;
    let frame_delay = 1000 / fps;

    let mut running = true;

    let mut prev_frame_time = std::time::Instant::now();

    while running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => running = false,
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    if snake.direction != Direction::Down {
                        update_snake(&mut snake, Direction::Up, &apple);
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    if snake.direction != Direction::Up {
                        update_snake(&mut snake, Direction::Down, &apple);
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    if snake.direction != Direction::Right {
                        update_snake(&mut snake, Direction::Left, &apple);
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    if snake.direction != Direction::Left {
                        update_snake(&mut snake, Direction::Right, &apple);
                    }
                },
                _ => (),
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        render_snake(&mut canvas, &snake);
        render_apple(&mut canvas, &apple);

        canvas.present();

        let frame_time = std::time::Instant::now();
        let frame_elapsed = frame_time.duration_since(prev_frame_time);
        prev_frame_time = frame_time;

        if frame_elapsed < Duration::from_millis(frame_delay) {
            std::thread::sleep(Duration::from_millis(frame_delay) - frame_elapsed);
        }
    }
}

fn create_snake() -> Rc<RefCell<SnakeCell>> {
    let head = Rc::new(RefCell::new(SnakeCell {
        x: TILE_SIZE * 30,
        y: TILE_SIZE * 20,
        direction: Direction::Up,
        next: None,
    }));

    increase_snake(&head);
    increase_snake(&head);

    head
}

fn increase_snake(head: &Rc<RefCell<SnakeCell>>) {
    let mut tail = head.clone();
    while let Some(next) = &tail.borrow().next {
        tail = next.clone();
    }

    let new_cell = Rc::new(RefCell::new(SnakeCell {
        x: tail.borrow().x,
        y: tail.borrow().y,
        direction: tail.borrow().direction,
        next: None,
    }));

    let mut new_tail = tail.borrow_mut();
    new_tail.direction = match new_tail.direction {
        Direction::Up => Direction::Down,
        Direction::Down => Direction::Up,
        Direction::Left => Direction::Right,
        Direction::Right => Direction::Left,
    };
    move_snake_cell(&mut new_tail);

    new_tail.next = Some(new_cell);
}

fn render_snake(canvas: &mut Canvas<Window>, snake: &Rc<RefCell<SnakeCell>>) {
    let head_color = Color::RGB(55, 175, 175);
    let body_color = Color::RGB(0, 200, 0);

    let head = snake.borrow();

    let head_rect = Rect::new(head.x as i32, head.y as i32, TILE_SIZE, TILE_SIZE);
    canvas.set_draw_color(head_color);
    canvas.fill_rect(head_rect).unwrap();

    let mut tail = head.next.clone();
    while let Some(tail_ref) = tail {
        let tail = tail_ref.borrow();

        let tail_rect = Rect::new(tail.x as i32, tail.y as i32, TILE_SIZE, TILE_SIZE);
        canvas.set_draw_color(body_color);
        canvas.fill_rect(tail_rect).unwrap();

        tail = tail.next.clone();
    }
}

fn update_snake(snake: &mut Rc<RefCell<SnakeCell>>, new_direction: Direction, apple: &Apple) {
    let mut head = snake.borrow_mut();
    let prev_x = head.x;
    let prev_y = head.y;
    let prev_d = head.direction;

    head.direction = new_direction;

    let moved = move_snake_cell(&mut head, apple);

    if head.x == apple.x && head.y == apple.y {
        update_apple(&mut *apple, snake);
        increase_snake(snake);
    } else {
        let mut tail = head.next.clone();
        while let Some(tail_ref) = tail {
            let mut tail = tail_ref.borrow_mut();
            let prev_x = tail.x;
            let prev_y = tail.y;
            let prev_d = tail.direction;
            tail.direction = prev_d;
            tail.x = prev_x;
            tail.y = prev_y;
            tail = tail.next.clone();
        }
        let mut head = snake.borrow_mut();
        head.x = prev_x;
        head.y = prev_y;
        head.direction = prev_d;
    }
}

fn move_snake_cell(cell: &mut SnakeCell, apple: &Apple) -> i32 {
    let new_x = match cell.direction {
        Direction::Up => cell.x,
        Direction::Down => cell.x,
        Direction::Left => {
            if cell.x <= apple.x + TILE_SIZE {
                cell.x + TILE_SIZE
            } else {
                cell.x - TILE_SIZE
            }
        },
        Direction::Right => {
            if cell.x >= apple.x {
                cell.x - TILE_SIZE
            } else {
                cell.x + TILE_SIZE
            }
        },
    };
    let new_y = match cell.direction {...
        Direction::Up => cell.y,
        Direction::Down => {
            if cell.y >= apple.y {
                cell.y - TILE_SIZE
            } else {
                cell.y + TILE_SIZE
            }
        },
        Direction::Left => cell.y,
        Direction::Right => cell.y,
    };

    cell.x = new_x;
    cell.y = new_y;

    0
}

fn render_apple(canvas: &mut Canvas<Window>, apple: &Apple) {canvas.set_draw_color(Color::RGB(255, 0, 0));
canvas.fill_rect(Rect::new(apple.x as i32, apple.y as i32, TILE_SIZE, TILE_SIZE)).unwrap();
}

fn create_apple() -> Apple {
    let mut rng = rand::thread_rng();
    let mut apple = Apple {
        x: 0,
        y: 0,
    };

    loop {
        apple.x = rng.gen_range(BOX_X + TILE_SIZE..BOX_X + BOX_W - TILE_SIZE);
        apple.y = rng.gen_range(BOX_Y + TILE_SIZE..BOX_Y + BOX_H - TILE_SIZE);

        if !is_on_snake(&apple, &snake.next) {
            break;
        }
    }

    apple
}

fn is_on_snake(apple: &Apple, snake: &SnakeCell) -> bool {
    let mut snake = snake;

    while let Some(next) = &snake.next {
        if apple.x == snake.x && apple.y == snake.y {
            return true;
        }

        snake = next;
    }

    false
}

fn update_apple(apple: &mut Apple, snake: &SnakeCell) {
    let mut rng = rand::thread_rng();

    loop {
        apple.x = rng.gen_range(snake.x..snake.x + snake.w - TILE_SIZE);
        apple.y = rng.gen_range(snake.y..snake.y + snake.h - TILE_SIZE);

        if !is_on_snake(&apple, &snake.next) {
            break;
        }
    }
}
