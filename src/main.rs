use rand::Rng;
use std::{thread, time};

const SIZE: usize = 20;         // Taille de la grille (20x20)
const DENSITY_Q: f64 = 0.8;       // Densité initiale de la forêt (0.0 à 1.0)
const SPREAD_P: f64 = 0.8;        // Probabilité que le feu se propage à une case voisine

const TREE: char = '🌲';         // Représente un arbre vivant
const FIRE: char = '🔥';         // Représente un arbre en feu
const BURNED: char = '⬜';       // Représente une case brûlée

fn main() {
    let mut rng = rand::thread_rng();

    // Génération de la grille initiale :
    // Chaque case a une probabilité DENSITY_Q de contenir un arbre, sinon elle est vide (affichée par un espace).
    let mut grid = [[' '; SIZE]; SIZE];
    for y in 0..SIZE {
        for x in 0..SIZE {
            if rng.gen_bool(DENSITY_Q) {
                grid[y][x] = TREE;
            }
        }
    }

    // Choisir un point de départ aléatoire pour le feu parmi les arbres existants.
    let (start_x, start_y) = loop {
        let x = rng.gen_range(0..SIZE);
        let y = rng.gen_range(0..SIZE);
        if grid[y][x] == TREE {
            break (x, y);
        }
    };
    grid[start_y][start_x] = FIRE;

    // Simulation : on met à jour la grille à chaque itération
    while grid.iter().any(|row| row.contains(&FIRE)) {
        print_grid(&grid);
        grid = update_grid(&grid, SPREAD_P, &mut rng);
        thread::sleep(time::Duration::from_millis(300));
    }
}

/// Affiche la grille dans le terminal en effaçant l'écran à chaque appel.
fn print_grid(grid: &[[char; SIZE]; SIZE]) {
    // Séquence ANSI pour effacer l'écran et repositionner le curseur
    print!("\x1B[2J\x1B[1;1H");
    for row in grid.iter() {
        for &cell in row.iter() {
            print!("{}", cell);
        }
        println!();
    }
}

/// Met à jour la grille :
/// - Une case en feu (FIRE) devient brûlée (BURNED).
/// - Pour chaque case en feu, on regarde ses 4 voisins (haut, bas, gauche, droite) et, si le voisin contient un arbre (TREE),
///   il prend feu (FIRE) selon la probabilité SPREAD_P.
fn update_grid(
    grid: &[[char; SIZE]; SIZE],
    spread_p: f64,
    rng: &mut rand::rngs::ThreadRng,
) -> [[char; SIZE]; SIZE] {
    let mut new_grid = grid.clone();

    for y in 0..SIZE {
        for x in 0..SIZE {
            if grid[y][x] == FIRE {
                // L'arbre en feu devient brûlé.
                new_grid[y][x] = BURNED;

                // Propagation du feu vers les 4 voisins
                for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;
                    if nx >= 0 && ny >= 0 && nx < SIZE as isize && ny < SIZE as isize {
                        let (nx, ny) = (nx as usize, ny as usize);
                        if grid[ny][nx] == TREE && rng.gen_bool(spread_p) {
                            new_grid[ny][nx] = FIRE;
                        }
                    }
                }
            }
        }
    }

    new_grid
}
