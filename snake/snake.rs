use std::time::{Duration, Instant};

use sdl2::{
    event::{Event, WindowEvent},
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::{Canvas, Renderer},
    video::{Window, WindowContext},
};
use rand::random;

const SCREEN_WIDTH: u32 = 1024;
const SCREEN_HEIGHT: u32 = 576;
const TILE_SIZE: u32 = 16;
const BOX_X: u32 = 128;
const BOX_Y: u32 = 72;
const BOX_W: u32 = 768;
const BOX_H: u32 = 432;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Box {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SnakeCell {
    x: u32,
    y: u32,
    direction: Direction,
    next: Option<Box<SnakeCell>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Apple {
    x: u32,
    y: u32,
}

fn create_snake() -> Box<SnakeCell> {
    let mut head = Box::new(SnakeCell {
        x: TILE_SIZE * 30,
        y: TILE_SIZE * 20,
        direction: Direction::Up,
        next: None,
    });

    for _ in 0..2 {
        increase_snake(&mut head);
    }

    head
}

fn increase_snake(head: &mut Box<SnakeCell>) {
    let mut tmp = head;
    while tmp.next.is_some() {
        tmp = tmp.next.as_mut().unwrap();
    }
    let new_cell = Box::new(SnakeCell {
        x: match tmp.direction {
            Direction::Up => tmp.x,
            Direction::Down => tmp.x,
            Direction::Left => tmp.x + TILE_SIZE,
            Direction::Right => tmp.x - TILE_SIZE,
        },
        y: match tmp.direction {
            Direction::Up => tmp.y - TILE_SIZE,
            Direction::Down => tmp.y + TILE_SIZE,
            Direction::Left => tmp.y,
            Direction::Right => tmp.y,
        },
        direction: tmp.direction,
        next: None,
    });
    tmp.next = Some(new_cell);
}

fn render_snake(canvas: &mut Canvas<Window>, head: &Box<SnakeCell>) {
    canvas.set_draw_color(Color::RGB(55, 175, 175));
    canvas.fill_rect(Rect::new(head.x, head.y, TILE_SIZE, TILE_SIZE)).unwrap();
    let mut tmp = head.next.as_ref();
    while let Some(cell) = tmp {
        canvas.set_draw_color(Color::RGB(0, 200, 0));
        canvas.fill_rect(Rect::new(cell.x, cell.y, TILE_SIZE, TILE_SIZE)).unwrap();
        tmp = cell.next.as_ref();
    }
}

fn update_snake(
    head: &mut Box<SnakeCell>,
    new_direction: Direction,
    apple: &mut Apple,
) -> bool {
    let mut tmp = head;
    let prev_x = head.x;
    let prev_y = head.y;
    let prev_d = head.direction;

    if move_snake(head, new_direction) == false {
        head.direction = new_direction;
        let mut tmp_x;
        let mut tmp_y;
        let mut tmp_d;
        while let Some(cell) = tmp.next.as_mut() {
            tmp_x = cell.x;
            tmp_y = cell.y;
            tmp_d = cell.direction;
            cell.x = prev_x;
            cell.y = prev_y;
            cell.direction = prev_d;
            prev_x = tmp_x;
            prev_y = tmp_y;
            prev_d = tmp_d;
            tmp = cell;
        }
    }

    if head.x == apple.x && head.y == apple.y {
        update_apple(apple);
        increase_snake(head);
        true
    } else {
        false
    }
}

fn move_snake(head: &mut Box<SnakeCell>, new_direction: Direction) -> bool {
    let mut new_x;
    let mut new_y;
    if new_direction == Direction::Up && head.direction != Direction::Down {
        new_x = head.x;
        new_y = head.y - TILE_SIZE;
    } else if new_direction == Direction::Down && head.direction != Direction::Up {
        new_x = head.x;
        new_y = head.y + TILE_SIZE;
    } else if new_direction == Direction::Left && head.direction != Direction::Right {
        new_x = head.x - TILE_SIZE;
        new_y = head.y;
    } else if new_direction == Direction::Right && head.direction != Direction::Left {
        new_x = head.x + TILE_SIZE;
        new_y = head.y;
    } else {
        return true;
    }

    let mut can_move = true;
    let mut tmp = head.next.as_ref();
    while let Some(cell) = tmp {
        if cell.x == new_x && cell.y == new_y {
            can_move = false;
        }
        tmp = cell.next.as_ref();
    }
    if can_move {
        if new_direction == Direction::Up {
            head.y -= TILE_SIZE;
            false
        } else if new_direction == Direction::Down {
            head.y += TILE_SIZE;
            false
        } else if new_direction == Direction::Left {
            head.x -= TILE_SIZE;
            false
        } else if new_direction == Direction::Right {
            head.x += TILE_SIZE;
            false
        } else {
            true
        }
    } else {
        true
    }
}

fn create_box(x: u32, y: u32, w: u32, h: u32) -> Box {
    Box { x, y, w, h }
}

fn render_box(canvas: &mut Canvas<Window>, box_: &Box) {
    canvas.set_draw_color(Color::RGB(50, 50, 50));
    canvas.draw_rect(Rect::new(box_.x, box_.y, box_.w, box_.h)).unwrap();
}

fn create_apple() -> Apple {
    let mut remainder;
    let x = (random::<f32>() * (BOX_W - BOX_X)) as u32 + BOX_X;
    remainder = x % TILE_SIZE;
    if remainder < TILE_SIZE / 2 {
        x -= remainder;
    } else {
        x += TILE_SIZE - remainder;
    }
    let y = (random::<f32>() * (BOX_H - BOX_Y)) as u32 + BOX_Y;
    remainder = y % TILE_SIZE;
    if remainder < TILE_SIZE / 2 {
        y -= remainder;
    } else {
        y += TILE_SIZE - remainder;
    }
    Apple { x, y }
}

fn render_apple(canvas: &mut Canvas<Window>, apple: &Apple) {
    canvas.set_draw_color(Color::RGB(200, 0, 0));
    canvas.fill_rect(Rect::new(apple.x, apple.y, TILE_SIZE, TILE_SIZE)).unwrap();
}

fn update_apple(apple: &mut Apple) {
    let mut remainder;
    let x = (random::<f32>() * (BOX_W - BOX_X)) as u32 + BOX_X;
    remainder = x % TILE_SIZE;
    if remainder < TILE_SIZE / 2 {
        x -= remainder;
    } else {
        x += TILE_SIZE