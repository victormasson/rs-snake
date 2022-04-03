// https://github.com/not-fl3/macroquad/blob/master/examples/snake.rs
//
// Licencié sous MIT ou Apache 2.0, à votre option, voir les fichiers LICENSE-* ou le dépôt de macroquad.

#![doc = include_str!("../README.md")]
// Le prelude de macro quad contient tout ce qui est utile pour developper en utilisant macroquad
use macroquad::prelude::*;

// On peut aussi importer des éléments individuellement
// ici, un vecdeque <https://doc.rust-lang.org/std/collections/struct.VecDeque.html>
// un vecteur circulaire, on peut accéder facilement au début et à la fin
use std::collections::VecDeque;

/// taille de la grille
/// Note: les commenataire avec 3 / apparaissent dans la doc générée par `cargo doc`
/// (quand ilis sont placés aux bon endroits)
const SQUARES: i16 = 16;

/// Un alias de type bien pratique <https://doc.rust-lang.org/reference/items/type-aliases.htmlhttps://doc.rust-lang.org/std/collections/struct.VecDeque.html>

#[derive(PartialEq, Clone, Copy)]
struct Point(i16, i16);

impl Point {
    fn get_random_point() -> Point {
        Point {
            0: rand::gen_range(0, SQUARES),
            1: rand::gen_range(0, SQUARES),
        }
    }
}

const UP: Point = Point(0, -1);
const DOWN: Point = Point(0, 1);
const RIGHT: Point = Point(1, 0);
const LEFT: Point = Point(-1, 0);

/// Le snake. <https://doc.rust-lang.org/book/ch05-01-defining-structs.html>
struct Snake {
    head: Point,
    body: VecDeque<Point>,
    dir: Point,
}

struct Fruit {
    head: Point,
    piege: Piege,
    color: Color,
}

impl Fruit {
    fn new() -> Fruit {
        let piege = Piege::get_rand_piege();

        Fruit {
            head: Point::get_random_point(),
            color: Piege::get_color(&piege),
            piege,
        }
    }
}

#[derive(Clone, PartialEq)]
enum Piege {
    DEQUEUE,
    DEATH,
    NOTHING,
}

impl Piege {
    fn get_rand_piege() -> Piege {
        match rand::gen_range::<i16>(0, 3) {
            0 => Piege::DEATH,
            1 => Piege::DEQUEUE,
            2 => Piege::NOTHING,
            _ => Piege::NOTHING,
        }
    }

    fn get_color(piege: &Piege) -> Color {
        match *piege {
            Piege::DEQUEUE => RED,
            Piege::DEATH => BLACK,
            Piege::NOTHING => YELLOW,
        }
    }
}

struct Portal {
    door1: Point,
    door2: Point,
}

#[macroquad::main("Snake")]
async fn main() {
    // initialisation

    // mot clef let pour déclarer une variable
    // mot clef mut pour qu'elle soit mutable
    let mut snake = Snake {
        head: Point(0, 0),     // on commence en haut a gauche
        dir: RIGHT,            // en direction de la droite
        body: VecDeque::new(), // sans queue
    };

    // fruit généré aléatoirement
    let mut fruit = Fruit::new();

    // Portail
    let mut portal = Portal {
        door1: Point::get_random_point(),
        door2: Point::get_random_point(),
    };

    // note: pour score et speed les types sont inférés par le compilateur
    // et même si ce sont des nombres, ils ne peuvent pas spontanément interagir ensemble
    let mut score = 0;
    let mut speed = 0.3;
    // si la ligne suivante est décommenté, ça ne compile pas mais ça renvoit une erreur intéressante
    // score + speed;
    // en revanche

    let mut game_over = false;
    let mut is_portal_open = false;

    // on garde en mémoire la dernière fois qu'on a
    // mis à jour pour mettre à jour la logique du jeu
    // indépendamment du temps qu'une image met à s'afficher
    let mut last_update = get_time();
    let mut last_update_portail_open = get_time();
    let mut last_update_fruit = get_time();

    // boucle infinie
    // on ne s'arrete jamais de jouer !
    loop {
        // ce if contient la logique du jeu
        if !game_over {
            snake.dir = if is_key_down(KeyCode::Right) && snake.dir != LEFT {
                RIGHT
            } else if is_key_down(KeyCode::Left) && snake.dir != RIGHT {
                LEFT
            } else if is_key_down(KeyCode::Up) && snake.dir != DOWN {
                UP
            } else if is_key_down(KeyCode::Down) && snake.dir != UP {
                DOWN
            } else {
                snake.dir
            };

            // ouverture portail aléatoire
            if !is_portal_open && i32::from(1) == rand::gen_range(0, 10_00) {
                is_portal_open = true;
                last_update_portail_open = get_time();
            }
            if is_portal_open && get_time() >= last_update_portail_open + 5.0 {
                is_portal_open = false;
                portal = Portal {
                    door1: Point::get_random_point(),
                    door2: Point::get_random_point(),
                };
            }

            if get_time() >= last_update_fruit + 5.0 + rand::gen_range::<f64>(-2.0, 3.0) {
                fruit = Fruit::new();
                last_update_fruit = get_time();
            }

            // si c'est le moment de mettre à jour
            if get_time() - last_update > speed {
                last_update = get_time();

                // l'ancien emplacement de la tete passe dans la queue
                snake.body.push_front(snake.head);
                // on met a jour la nouvelle tete en fonction de la queue
                snake.head = Point(snake.head.0 + snake.dir.0, snake.head.1 + snake.dir.1);

                // si la tete est sur un fruit
                if snake.head == fruit.head {
                    if fruit.piege == Piege::NOTHING {
                        score += 100;
                        speed *= 0.9;
                    } else if fruit.piege == Piege::DEATH {
                        game_over = true;
                    } else if fruit.piege == Piege::DEQUEUE {
                        snake.body.pop_back();
                    }

                    // un nouveau fruit
                    fruit = Fruit::new();
                } else {
                    // la ou etait le dernier block de la queue, il n'y a plus rien
                    snake.body.pop_back();
                }

                let other_side = SQUARES - 1;
                if snake.head.0 < 0 {
                    snake.head.0 = other_side;
                }
                if snake.head.0 > other_side {
                    snake.head.0 = 0;
                }
                if snake.head.1 < 0 {
                    snake.head.1 = other_side;
                }
                if snake.head.1 > other_side {
                    snake.head.1 = 0;
                }

                // vérifier s'il passe par le portail
                if portal.door1.0 == snake.head.0 && portal.door1.1 == snake.head.1 {
                    snake.head = portal.door2;
                    portal = Portal {
                        door1: Point::get_random_point(),
                        door2: Point::get_random_point(),
                    };

                    is_portal_open = false;
                }
                if portal.door2.0 == snake.head.0 && portal.door2.1 == snake.head.1 {
                    snake.head = portal.door1;
                    portal = Portal {
                        door1: Point::get_random_point(),
                        door2: Point::get_random_point(),
                    };

                    is_portal_open = false;
                }

                // vérifier si on s'est mordu la queue
                for Point(x, y) in &snake.body {
                    if *x == snake.head.0 && *y == snake.head.1 {
                        game_over = true;
                    }
                }
            }
        }

        // ce if contient l'affichage
        if !game_over {
            clear_background(LIGHTGRAY);

            // quelques valeur pratiques pour l'affichage
            // pour pouvoir réajuster l'affichage en fonction de la fenêtre
            let game_size = screen_width().min(screen_height());
            let offset_x = (screen_width() - game_size) / 2. + 10.;
            let offset_y = (screen_height() - game_size) / 2. + 10.;
            let sq_size = (screen_height() - offset_y * 2.) / SQUARES as f32;

            // la ou on joue
            draw_rectangle(offset_x, offset_y, game_size - 20., game_size - 20., WHITE);

            // le quadrillage
            for i in 1..SQUARES {
                draw_line(
                    offset_x,
                    offset_y + sq_size * i as f32,
                    screen_width() - offset_x,
                    offset_y + sq_size * i as f32,
                    2.,
                    LIGHTGRAY,
                );
            }

            for i in 1..SQUARES {
                draw_line(
                    offset_x + sq_size * i as f32,
                    offset_y,
                    offset_x + sq_size * i as f32,
                    screen_height() - offset_y,
                    2.,
                    LIGHTGRAY,
                );
            }

            // dessiner la tête
            draw_rectangle(
                offset_x + snake.head.0 as f32 * sq_size,
                offset_y + snake.head.1 as f32 * sq_size,
                sq_size,
                sq_size,
                DARKGREEN,
            );

            // dessiner le corps
            for Point(x, y) in &snake.body {
                draw_rectangle(
                    offset_x + *x as f32 * sq_size,
                    offset_y + *y as f32 * sq_size,
                    sq_size,
                    sq_size,
                    LIME,
                );
            }

            // dessiner le fruit
            draw_rectangle(
                offset_x + fruit.head.0 as f32 * sq_size,
                offset_y + fruit.head.1 as f32 * sq_size,
                sq_size,
                sq_size,
                fruit.color,
            );

            if is_portal_open {
                // dessiner le portail 1
                draw_rectangle(
                    offset_x + portal.door1.0 as f32 * sq_size,
                    offset_y + portal.door1.1 as f32 * sq_size,
                    sq_size,
                    sq_size,
                    BLUE,
                );

                // dessiner le portail 2
                draw_rectangle(
                    offset_x + portal.door2.0 as f32 * sq_size,
                    offset_y + portal.door2.1 as f32 * sq_size,
                    sq_size,
                    sq_size,
                    BLUE,
                );
            }

            // écrire le score
            draw_text(
                format!("SCORE: {}", score).as_str(),
                10.,
                10.,
                20.,
                DARKGRAY,
            );
        } else {
            // si game over
            clear_background(WHITE);
            let text = "Game Over. Press [enter] to play again.";
            let font_size = 30.;
            let text_size = measure_text(text, None, font_size as _, 1.0);

            // massage de game over
            draw_text(
                text,
                screen_width() / 2. - text_size.width / 2.,
                screen_height() / 2. - text_size.height / 2.,
                font_size,
                DARKGRAY,
            );

            // reset
            if is_key_down(KeyCode::Enter) {
                snake = Snake {
                    head: Point(0, 0),
                    dir: RIGHT,
                    body: VecDeque::new(),
                };
                fruit = Fruit::new();
                score = 0;
                speed = 0.3;
                last_update = get_time();
                game_over = false;
            }
        }
        next_frame().await
    }
}
