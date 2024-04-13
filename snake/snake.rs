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

const SCREEN_WIDTH: u32 = 1024;
const SCREEN_HEIGHT: u32 = 576;
const TILE_SIZE: u32 = 16;
const BOX_X: u32 = 128;
const BOX_Y: u32 = 72;
const BOX_W: u32 = 768;
const BOX_H: u32 = 432;

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

    let box_ref = Rc::new(RefCell::new(Box {
        x: BOX_X,
        y: BOX_Y,
        w: BOX_W,
        h: BOX_H,
    }));

    let mut snake = create_snake(&box_ref);
    let mut apple = create_apple(&box_ref);

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

        render_box(&mut canvas, &box_ref.borrow());
        render_apple(&mut canvas, &apple);
        render_snake(&mut canvas, &snake);

        canvas.present();

        let frame_time = std::time::Instant::now();
        let frame_elapsed = frame_time.duration_since(prev_frame_time);
        prev_frame_time = frame_time;

        if frame_elapsed < Duration::from_millis(frame_delay) {
            std::thread::sleep(Duration::from_millis(frame_delay) - frame_elapsed);
        }
    }
}

fn create_snake(box_ref: &Rc<RefCell<Box>>) -> Rc<RefCell<SnakeCell>> {
    let head = Rc::new(RefCell::new(SnakeCell {
        x: TILE_SIZE * 30,
        y: TILE_SIZE * 20,
        direction: Direction::Up,
        next: None,
    }));

    increase_snake(&head, box_ref);
    increase_snake(&head, box_ref);

    head
}

fn increase_snake(head: &Rc<RefCell<SnakeCell>>, box_ref: &Rc<RefCell<Box>>) {
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
    move_snake_cell(&mut new_tail, box_ref);

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

    if move_snake_cell(&mut head, &*snake.borrow()) == 0 {
        head.direction = new_direction;
        let mut tmp_x = head.x;
        let mut tmp_y = head.y;
        let mut tmp_d = head.direction;
        let mut tail = head.next.clone();

        while let Some(tail_ref) = tail {
            let mut tail = tail_ref.borrow_mut();

            std::mem::swap(&mut tmp_x, &mut tail.x);
            std::mem::swap(&mut tmp_y, &mut tail.y);
            std::mem::swap(&mut tmp_d, &mut tail.direction);

            tail = tail.next.clone();
        }
    }

    if head.x == apple.x && head.y == apple.y {
        update_apple(&mut *apple, &snake.borrow());
        increase_snake(snake, &Rc::new(RefCell::new(Box {
            x: BOX_X,
            y: BOX_Y,
            w: BOX_W,
            h: BOX_H,
        })));
    }
}

fn move_snake_cell(cell: &mut SnakeCell, box_ref: &Rc<RefCell<Box>>) -> i32 {
    let new_x = match cell.direction {Direction::Up => cell.x,
        Direction::Down => cell.x,
        Direction::Left => cell.x - TILE_SIZE,
        Direction::Right => cell.x + TILE_SIZE,
    };

    let new_y = match cell.direction {
        Direction::Up => cell.y - TILE_SIZE,
        Direction::Down => cell.y + TILE_SIZE,
        Direction::Left => cell.y,
        Direction::Right => cell.y,
    };

    let box_ref = box_ref.borrow();
    if new_x < box_ref.x || new_x >= box_ref.x + box_ref.w || new_y < box_ref.y || new_y >= box_ref.y + box_ref.h {
        return 1;
    }

    let mut can_move = true;
    let mut tail = cell.next.clone();

    while let Some(tail_ref) = tail {
        let tail = tail_ref.borrow();

        if tail.x == new_x && tail.y == new_y {
            can_move = false;
            break;
        }

        tail = tail.next.clone();
    }

    if can_move {
        cell.x = new_x;
        cell.y = new_y;
        return 0;
    }

    1
}

fn render_box(canvas: &mut Canvas<Window>, box_ref: &Box) {
    let box_color = Color::RGB(50, 50, 50);

    let box_rect = Rect::new(box_ref.x as i32, box_ref.y as i32, box_ref.w, box_ref.h);
    canvas.set_draw_color(box_color);
    canvas.draw_rect(box_rect).unwrap();
}

fn create_apple(box_ref: &Rc<RefCell<Box>>) -> Apple {
    let mut x = (rand::random::<u32>() % (box_ref.borrow().w - BOX_X + 1) + BOX_X) as i32;
    let mut y = (rand::random::<u32>() % (box_ref.borrow().h - BOX_Y + 1) + BOX_Y) as i32;

    let remainder_x = x % (TILE_SIZE as i32);
    if remainder_x < (TILE_SIZE / 2) as i32 {
        x -= remainder_x;
    } else {
        x += (TILE_SIZE - remainder_x) as i32;
    }

    let remainder_y = y % (TILE_SIZE as i32);
    if remainder_y < (TILE_SIZE / 2) as i32 {
        y -= remainder_y;
    } else {
        y += (TILE_SIZE - remainder_y) as i32;
    }

    Apple { x: x as u32, y: y as u32 }
}

fn render_apple(canvas: &mut Canvas<Window>, apple: &Apple) {
    let apple_color = Color::RGB(200, 0, 0);

    let apple_rect = Rect::new(apple.x as i32, apple.y as i32, TILE_SIZE, TILE_SIZE);
    canvas.set_draw_color(apple_color);
    canvas.fill_rect(apple_rect).unwrap();
}

fn update_apple(apple: &mut Apple, box_ref: &Box) {
    let mut x = (rand::random::<u32>() % (box_ref.w - BOX_X + 1) + BOX_X) as i32;
    let mut y = (rand::random::<u32>() % (box_ref.h - BOX_Y + 1) + BOX_Y) as i32;

    let remainder_x = x % (TILE_SIZE as i32);
    if remainder_x < (TILE_SIZE / 2) as i32 {
        x -= remainder_x;
    } else {
        x += (TILE_SIZE - remainder_x) as i32;
    }

    let remainder_y = y % (TILE_SIZE as i32);
    if remainder_y < (TILE_SIZE / 2) as i32 {
        y -= remainder_y;
    } else {
        y += (TILE_SIZE - remainder_y) as i32;
    }

    apple.x = x as u32;
    apple.y = y as u32;
}
