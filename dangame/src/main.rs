use std::vec;

use macroquad::prelude::*;
use macroquad::rand::srand;

fn window_conf() -> Conf {
    Conf {
        window_title: "Dan game".to_string(),
        window_width: 800,
        window_height: 600,
        window_resizable: false,
        ..Default::default()
    }
}

fn draw_cell(x: f32, y: f32, w: f32, h: f32, radius: f32, color: Color, text: Option<&str>) {
    draw_rectangle(x + radius, y, w - radius * 2.0, h, color);
    draw_rectangle(x, y + radius, w, h - radius * 2.0, color);

    draw_circle(x + radius, y + radius, radius, color);
    draw_circle(x + w - radius, y + radius, radius, color);
    draw_circle(x + radius, y + h - radius, radius, color);
    draw_circle(x + w - radius, y + h - radius, radius, color);

    if let Some(t) = text {
        let text_size = 30.0;
        let text_width = measure_text(t, None, text_size as u16, 1.0).width;
        draw_text(
            t,
            x + (w - text_width) / 2.0,
            y + (h + text_size) / 2.0 - 5.0,
            text_size,
            BLACK,
        );
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Cell {
    Empty,
    InsufficientEvidence,
    EmergingUnderstanding,
    PartialMastery,
    Mastery,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn spawn_new(grid: &Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
    let mut empty_cells = Vec::new();

    let mut g = grid.clone();

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == Cell::Empty {
                empty_cells.push((i, j));
            }
        }
    }

    if !empty_cells.is_empty() {
        let index = macroquad::rand::gen_range(0, empty_cells.len() as i32) as usize;
        let (i, j) = empty_cells[index];
        if macroquad::rand::gen_range(0, 10) >= 8 {
            g[i][j] = Cell::EmergingUnderstanding;
        } else {
            g[i][j] = Cell::InsufficientEvidence;
        }
    }

    g
}

const GRID_SIZE: usize = 4;
const CELL_SIZE: f32 = 100.0;
const OFFSET: (f32, f32) = (
    (800.0 - CELL_SIZE * GRID_SIZE as f32) / 2.0,
    (600.0 - CELL_SIZE * GRID_SIZE as f32) / 2.0,
);

fn draw_grid(grid: &Vec<Vec<Cell>>) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let x = j as f32 * CELL_SIZE + OFFSET.0;
            let y = i as f32 * CELL_SIZE + OFFSET.1;
            match grid[i][j] {
                Cell::Empty => draw_cell(x, y, CELL_SIZE, CELL_SIZE, 10.0, GRAY, None),
                Cell::InsufficientEvidence => {
                    draw_cell(x, y, CELL_SIZE, CELL_SIZE, 10.0, ORANGE, Some("IE"))
                }
                Cell::EmergingUnderstanding => {
                    draw_cell(x, y, CELL_SIZE, CELL_SIZE, 10.0, YELLOW, Some("EU"))
                }
                Cell::PartialMastery => {
                    draw_cell(x, y, CELL_SIZE, CELL_SIZE, 10.0, LIME, Some("PM"))
                }
                Cell::Mastery => draw_cell(x, y, CELL_SIZE, CELL_SIZE, 10.0, GREEN, Some("M")),
            }
        }
    }
}

fn get_input() -> Option<Direction> {
    if is_key_pressed(KeyCode::Up) {
        Some(Direction::Up)
    } else if is_key_pressed(KeyCode::Down) {
        Some(Direction::Down)
    } else if is_key_pressed(KeyCode::Left) {
        Some(Direction::Left)
    } else if is_key_pressed(KeyCode::Right) {
        Some(Direction::Right)
    } else {
        None
    }
}

fn rot90(grid: &Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
    let mut new_grid = vec![vec![Cell::Empty; grid.len()]; grid[0].len()];
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            new_grid[j][grid.len() - 1 - i] = grid[i][j];
        }
    }
    new_grid
}

#[derive(PartialEq, Clone)]
enum GameState {
    Start,
    Playing,
    GoalReached,
    Lost,
}

fn handle_input(grid: &Vec<Vec<Cell>>, direction: Direction) -> Vec<Vec<Cell>> {
    let mut g = grid.clone();
    match direction {
        Direction::Up => g = rot90(&rot90(&g)),
        Direction::Down => (),
        Direction::Right => g = rot90(&g),
        Direction::Left => g = rot90(&rot90(&rot90(&g))),
    }
    // 2048 movement algorithm
    // compress-merge-compress
    // step 1. Compress. Move all non-empty cells in the direction of movement, filling in empty spaces. Start scanning in the opposite direction as the direction of movement.

    // lets just code this for downards movement and then rotate it after the fact for other directions.
    for row in (0..g.len()).rev() {
        for col in 0..g[row].len() {
            if g[row][col] != Cell::Empty {
                let mut new_row = row;
                while new_row < g.len() - 1 && g[new_row + 1][col] == Cell::Empty {
                    new_row += 1;
                }
                if new_row != row {
                    g[new_row][col] = g[row][col];
                    g[row][col] = Cell::Empty;
                }
            }
        }
    }

    // step 2. Merge. If two adjacent cells in the direction of movement have the same value, merge them into one cell with the next level of value. Start scanning in the opposite direction as the direction of movement.
    for row in (0..g.len() - 1).rev() {
        for col in 0..g[row].len() {
            if g[row][col] != Cell::Empty && g[row][col] == g[row + 1][col] {
                g[row + 1][col] = match g[row][col] {
                    Cell::InsufficientEvidence => Cell::EmergingUnderstanding,
                    Cell::EmergingUnderstanding => Cell::PartialMastery,
                    Cell::PartialMastery => Cell::Mastery,
                    _ => g[row][col],
                };
                g[row][col] = Cell::Empty;
            }
        }
    }

    // step 3. compress, just paste the same code
    for row in (0..g.len()).rev() {
        for col in 0..g[row].len() {
            if g[row][col] != Cell::Empty {
                let mut new_row = row;
                while new_row < g.len() - 1 && g[new_row + 1][col] == Cell::Empty {
                    new_row += 1;
                }
                if new_row != row {
                    g[new_row][col] = g[row][col];
                    g[row][col] = Cell::Empty;
                }
            }
        }
    }

    match direction {
        Direction::Up => rot90(&rot90(&g)),
        Direction::Down => g,
        Direction::Right => rot90(&rot90(&rot90(&g))),
        Direction::Left => rot90(&g),
    }
}

fn get_new_goal() -> Vec<Cell> {
    let mut has_mastery = false;
    let mut goal = vec![Cell::Empty; 4];
    while !has_mastery {
        goal = vec![Cell::Empty; 4];
        for i in 0..goal.len() {
            let r = macroquad::rand::gen_range(0, 100);
            goal[i] = if r < 25 {
                Cell::InsufficientEvidence
            } else if r < 50 {
                Cell::EmergingUnderstanding
            } else if r < 75 {
                Cell::PartialMastery
            } else {
                has_mastery = true;
                Cell::Mastery
            };
        }
    }
    goal
}

fn check_goal(grid: &Vec<Vec<Cell>>, goal: &Vec<Cell>) -> i32 {
    for i in 0..grid.len() {
        let mut flag = true;
        for j in 0..grid[i].len() {
            if grid[i][j] != goal[j] {
                flag = false;
                break;
            }
        }
        if flag {
            return i as i32;
        }
    }
    -1
}

#[macroquad::main(window_conf)]
async fn main() {
    srand(miniquad::date::now() as u64);

    let mut grid = vec![vec![Cell::Empty; GRID_SIZE]; GRID_SIZE];
    let mut state = GameState::Playing;

    grid = spawn_new(&grid);

    let mut goal = vec![Cell::Empty; GRID_SIZE];
    goal = get_new_goal();

    let mut counter: usize = 0;

    let mut prev_state = state.clone();

    let mut score = 0;

    loop {
        counter += 1;
        clear_background(BLACK);

        draw_grid(&grid);
        if state == GameState::Playing || state == GameState::GoalReached {
            
            for i in 0..goal.len() {
                let x = i as f32 * CELL_SIZE/2.0 + OFFSET.0 + 80.0;
                let y = OFFSET.1 - 20.0 - CELL_SIZE/2.0;
                match goal[i] {
                    Cell::Empty => draw_cell(x, y, CELL_SIZE/2.0, CELL_SIZE/2.0, 10.0, GRAY, None),
                    Cell::InsufficientEvidence => {
                        draw_cell(x, y, CELL_SIZE/2.0, CELL_SIZE/2.0, 10.0, ORANGE, Some("IE"))
                    }
                    Cell::EmergingUnderstanding => {
                        draw_cell(x, y, CELL_SIZE/2.0, CELL_SIZE/2.0, 10.0, YELLOW, Some("EU"))
                    }
                    Cell::PartialMastery => {
                        draw_cell(x, y, CELL_SIZE/2.0, CELL_SIZE/2.0, 10.0, LIME, Some("PM"))
                    }
                    Cell::Mastery => draw_cell(x, y, CELL_SIZE/2.0, CELL_SIZE/2.0, 10.0, GREEN, Some("M")),
                }
            }
            draw_text("Goal:", OFFSET.0, OFFSET.1 - 50.0, 30.0, WHITE);

            draw_text(&format!("Evals graded: {}", score), OFFSET.0, OFFSET.1 + CELL_SIZE * GRID_SIZE as f32 + 40.0, 20.0, WHITE);

            draw_text("Arrows to move. Try to reach the goal! Space to skip.", OFFSET.0, OFFSET.1 + CELL_SIZE * GRID_SIZE as f32 + 20.0, 20.0, WHITE);

        }
        if state == GameState::Playing {
            
            
            if let Some(direction) = get_input() {
                grid = handle_input(&grid, direction);
                grid = spawn_new(&grid);
            }
        }

        let goal_res = check_goal(&grid, &goal);
        if goal_res != -1 {
            state = GameState::GoalReached;
            if prev_state != state {
                score += 1;
            }

            draw_rectangle_lines(
                OFFSET.0,
                OFFSET.1 + goal_res as f32 * CELL_SIZE,
                CELL_SIZE * GRID_SIZE as f32,
                CELL_SIZE,
                5.0,
                GREEN,
            );
        }

        
        if prev_state != state {
            counter = 0;
            prev_state = state.clone();
        }

        if state == GameState::GoalReached && counter > (macroquad::time::get_fps() as i32 * 2.0 as i32) as usize || is_key_pressed(KeyCode::Space) {
            goal = get_new_goal();
            state = GameState::Playing;
            grid = vec![vec![Cell::Empty; GRID_SIZE]; GRID_SIZE];
            grid = spawn_new(&grid);
        }

        next_frame().await;
    }
}
