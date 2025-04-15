use std::time::{Duration, SystemTime};
use std::{thread};
use rand::Rng;

fn main() {
    // Initialize Field
    let mut field = build_field();
    print_field(&field);
    // Start Game
    game_loop(&mut field);
}
#[derive(Clone)]
struct Field{
    width: u32,
    height: u32,
    cells: Vec<Vec<Cell>>,
}

#[derive(Clone)]
struct Cell {
    x: u32,
    y: u32,
    is_alive: bool,
    future_alive: bool,
}

fn check_neighbours(cell: &Cell, field: &Field) -> u32 {
    let mut count = 0;
    if cell.x > 0 && cell.y > 0 && field.width > cell.x && field.height > cell.y {
        count += 1;
    }
    if cell.x > 0 && cell.y > 0 && field.width > cell.x && field.height > cell.y {
        count += 1;
    }
    if cell.x > 0 && cell.y > 0 && field.width > cell.x && field.height > cell.y {
        count += 1;
    }
    if cell.x > 0 && cell.y > 0 && field.width > cell.x && field.height > cell.y {
        count += 1;
    }   
    if cell.x > 0 && cell.y > 0 && field.width > cell.x && field.height > cell.y {
        count += 1;
    }   
    if cell.x > 0 && cell.y > 0 && field.width > cell.x && field.height > cell.y {
        count += 1;
    }   
    if cell.x > 0 && cell.y > 0 && field.width > cell.x && field.height > cell.y {
        count += 1;
    }   
    if cell.x > 0 && cell.y > 0 && field.width > cell.x && field.height > cell.y {
        count += 1;
    }   
    if cell.x > 0 && cell.y > 0 && field.width > cell.x && field.height > cell.y {
        count += 1;
    }   
    if cell.x > 0 && cell.y > 0 && field.width > cell.x && field.height > cell.y {
        count += 1;
    }   
    count
}

fn apply_rules(cell: &mut Cell, alive_neighbours: u32) {
    if alive_neighbours == 2 {
        cell.future_alive = cell.is_alive;
    } else if alive_neighbours == 3 {
        cell.future_alive = true;
    } else {
        cell.future_alive = false;
    }
}   

fn update(field: &mut Field) {
    field.cells.iter_mut().for_each(
        |row: &mut Vec<Cell>| {
            row.iter_mut().for_each(|cell: &mut Cell| {
                cell.is_alive = cell.future_alive;
            });
        }
    );
}

fn game_loop(field: &mut Field){
    loop {
        // First pass: Calculate neighbor counts for all cells
        let neighbor_counts: Vec<Vec<u32>> = field.cells.iter().enumerate().map(|(i, row)| {
            row.iter().enumerate().map(|(j, cell)| {
                check_neighbours(cell, field)
            }).collect()
        }).collect();
        
        // Second pass: Apply rules using the pre-calculated neighbor counts
        field.cells.iter_mut().enumerate().for_each(|(i, row)| {
            row.iter_mut().enumerate().for_each(|(j, cell)| {
                apply_rules(cell, neighbor_counts[i][j]);
            });
        });
        
        update(field);
        print_field(field);
        thread::sleep(Duration::from_millis(100));
        print!("\x1B[2J\x1B[1;1H");
    }
}

fn create_cell_lists(width: u32, height: u32) -> Vec<Vec<Cell>> {
    let mut cells: Vec<Vec<Cell>> = Vec::new();
    let mut rng = rand::thread_rng();
    for y in 0..height as usize {
        cells.push(Vec::new());
        for x in 0..width as usize {
            cells[y].push(Cell {
                x: x as u32,
                y: y as u32,
                is_alive : rng.gen_range(0..6) == 5,
                future_alive: false
            });
        }
    }
    return cells;
}

fn build_field() -> Field {
    let width = 10;
    let height = 10;
    Field {
        width,
        height,
        cells: create_cell_lists(width, height),
    }
}

fn print_field(field: &Field) {
    for row in field.cells.iter() {
        for cell in row.iter() {
            if cell.is_alive {
                print!("👾");
            } else {
                print!("🪦");
            }
        }
        println!("");
    }
    println!("----------");
}
