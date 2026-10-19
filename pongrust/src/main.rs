#![windows_subsystem = "windows"]
/// Versione 1.1.5 Giorgio Ruffa (c)2026
/// Buon divertimento con Rust! 🦀🏓
use macroquad::prelude::*;
use macroquad::audio::{load_sound, play_sound_once}; 
use ::rand::Rng;  	// :: davanti forza Rust a prendere il crate esterno, non il modulo di macroquad.

const WINDOW_WIDTH: f32 = 1024.0;
const WINDOW_HEIGHT: f32 = 768.0;
const BALL_SPEED: f32 = 500.0;
const MAX_BALL_SPEED: f32 = 1500.0; // evita di raggiungere velocità così estreme da superare la capacità di rilevamento dei fotogrammi (fenomeno noto come tunneling o attraversamento dei bordi/racchette).

#[derive(PartialEq, Eq, Clone, Copy)]
enum ControlMode {
    Mouse,
    Keyboard,
    Computer,
}

#[derive(Clone, Copy)]
struct Difficulty {
    name: &'static str,
    left_speed: f32,
    right_speed: f32,
}

const DIFFICULTIES: [Difficulty; 3] = [
    Difficulty {
        name: "Facile",
        left_speed: 450.0,
        right_speed: 300.0,
    },
    Difficulty {
        name: "Media",
        left_speed: 600.0,
        right_speed: 500.0,
    },
    Difficulty {
        name: "Difficile",
        left_speed: 720.0,
        right_speed: 720.0,
    },
];

struct Paddle {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    speed: f32,
}

impl Paddle {
    fn center_y(&self) -> f32 {
        self.y + self.h / 2.0
    }

    // Movimento AI con zona morta: evita piccole oscillazioni attorno al bersaglio.
    fn follow_target(&mut self, target_y: f32, dt: f32, window_height: f32) {
        let error = target_y - self.center_y();
        let dead_zone = 14.0;

        if error.abs() > dead_zone {
            let direction = error.signum();
            let distance = (error.abs() - dead_zone).min(self.speed * dt);
            self.y += direction * distance;
        }

        self.clamp_into_screen(window_height);
    }

    fn clamp_into_screen(&mut self, window_height: f32) {
        if self.y < 0.0 {
            self.y = 0.0;
        }
        if self.y + self.h > window_height {
            self.y = (window_height - self.h).max(0.0);
        }
    }

    fn draw(&self, color: Color) {
        draw_rectangle(self.x, self.y, self.w, self.h, color);
    }
}

struct Ball {
    pos: Vec2,
    vel: Vec2,
    radius: f32,
}

impl Ball {
    fn reset(&mut self, direction: f32, window_width: f32, window_height: f32) {
        self.pos = vec2(window_width / 2.0, window_height / 2.0);
        let vx = BALL_SPEED * direction;
        let vy = macroquad::rand::gen_range(-150.0, 150.0);
        self.vel = vec2(vx, vy);
    }

    fn draw(&self, color: Color) {
        draw_circle(self.pos.x, self.pos.y, self.radius, color);
    }
}

// Introduciamo un fattore di casualità nella velocità di rimbalzo della pallina (da aggiungere nei punti desiderati)
fn rumore() -> f32 {
    let mut rng = ::rand::thread_rng();
    rng.gen_range(1.00..1.26)
}
// Introduciamo un fattore di casualità nell'angolo di rimbalzo della pallina col muro (da aggiungere nei punti desiderati)
fn rumore_angolo() -> f32 {
    let mut rng = ::rand::thread_rng();
    rng.gen_range(0.95..1.05)
}
#[macroquad::main(window_conf)]
async fn main() {
    // Oggetti di gioco
    let mut left = Paddle {
        x: 30.0,
        y: WINDOW_HEIGHT / 2.0 - 50.0,
        w: 16.0,
        h: 100.0,
        speed: 600.0,
    };
    let mut right = Paddle {
        x: WINDOW_WIDTH - 30.0 - 16.0,
        y: WINDOW_HEIGHT / 2.0 - 50.0,
        w: 16.0,
        h: 100.0,
        speed: 420.0,
    };

    let mut ball = Ball {
        pos: vec2(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0),
        vel: vec2(-300.0, 100.0),
        radius: 10.0,
    };

    let mut left_score = 0;
    let mut right_score = 0;
    let mut suono = true;
    
    // --- CARICAMENTO DEI SUONI DALLA CARTELLA ASSETS ---
    let sound_ping = load_sound("assets/ping.wav").await.unwrap();
    let sound_bordo = load_sound("assets/pong.wav").await.unwrap();
    let sound_out = load_sound("assets/vitapersa.wav").await.unwrap();   
    let sound_welcome = load_sound("assets/gameplay.ogg").await.unwrap(); 

    ball.reset(-1.0, screen_width(), screen_height());

    // --- SCELTA MODO CONTROLLO ---
    let mut control_mode: Option<ControlMode> = None;

    while control_mode.is_none() {
        clear_background(BLACK);
		
        let title = "Rust Pong";
		let autore = "Versione 1.1.5 - Giorgio Ruffa (c)2026";
        let instr1 = "Scegli la modalita' di gioco:";
        let instr2 = "M: Mouse   K: Frecce (Up/Down)   A: Computer vs Computer";
        let instr3 = "Premi Esc per uscire";
		if suono {
			play_sound_once(&sound_welcome);    // Musica di apertura gioco
			suono = false;
			}		
        let title_size = 64.0;
        let small = 28.0;

        let tdim = measure_text(title, None, title_size as u16, 1.0);
        draw_text(title, WINDOW_WIDTH / 2.0 - tdim.width / 2.0, 160.0, title_size, YELLOW);

        let d0 = measure_text(autore, None, small as u16, 1.0);
        draw_text(autore, WINDOW_WIDTH / 2.0 - d0.width / 2.0, 200.0, small, GREEN);

        let d1 = measure_text(instr1, None, small as u16, 1.0);
        draw_text(instr1, WINDOW_WIDTH / 2.0 - d1.width / 2.0, 260.0, small, GRAY);

        let d2 = measure_text(instr2, None, 22, 1.0);
        draw_text(instr2, WINDOW_WIDTH / 2.0 - d2.width / 2.0, 300.0, 22.0, LIGHTGRAY);

        let d3 = measure_text(instr3, None, 20, 1.0);
        draw_text(instr3, WINDOW_WIDTH / 2.0 - d3.width / 2.0, 340.0, 20.0, GRAY);

        // controllo tasti
        if is_key_pressed(KeyCode::M) {
            control_mode = Some(ControlMode::Mouse);
        } else if is_key_pressed(KeyCode::K) {
            control_mode = Some(ControlMode::Keyboard);
        } else if is_key_pressed(KeyCode::A) {
            control_mode = Some(ControlMode::Computer);
        } else if is_key_pressed(KeyCode::Escape) {
            return;
        }

        next_frame().await;
    }

    let control_mode = control_mode.unwrap();

    // --- Difficoltà ---
    let mut difficulty_index: Option<usize> = None;

    while difficulty_index.is_none() {
        clear_background(BLACK);

        let title = "Seleziona la difficolta'";
        let title_size = 44.0;
        let title_dim = measure_text(title, None, title_size as u16, 1.0);
        draw_text(
            title,
            screen_width() / 2.0 - title_dim.width / 2.0,
            180.0,
            title_size,
            YELLOW,
        );

        let options = [
            "1 - Facile",
            "2 - Media",
            "3 - Difficile",
        ];
        for (i, option) in options.iter().enumerate() {
            let y = 270.0 + i as f32 * 55.0;
            let dim = measure_text(option, None, 30, 1.0);
            draw_text(
                option,
                screen_width() / 2.0 - dim.width / 2.0,
                y,
                30.0,
                LIGHTGRAY,
            );
        }

        let help = "Premi 1, 2 o 3 per scegliere  -  Esc per uscire";
        let help_dim = measure_text(help, None, 20, 1.0);
        draw_text(
            help,
            screen_width() / 2.0 - help_dim.width / 2.0,
            470.0,
            20.0,
            GRAY,
        );

        if is_key_pressed(KeyCode::Key1) {
            difficulty_index = Some(0);
        } else if is_key_pressed(KeyCode::Key2) {
            difficulty_index = Some(1);
        } else if is_key_pressed(KeyCode::Key3) {
            difficulty_index = Some(2);
        } else if is_key_pressed(KeyCode::Escape) {
            return;
        }

        next_frame().await;
    }

    let difficulty = DIFFICULTIES[difficulty_index.unwrap()];
    left.speed = difficulty.left_speed;
    right.speed = difficulty.right_speed;

    // --- CICLO DI GIOCO PRINCIPALE ---
    loop {
        let dt = get_frame_time();
        let window_width = screen_width();
        let window_height = screen_height();

        // Adatta le posizioni orizzontali delle racchette alla larghezza corrente.
        left.x = 30.0;
        right.x = (window_width - 30.0 - right.w).max(left.x + left.w);

        // Input: mouse o tastiera, non entrambi
        if control_mode == ControlMode::Keyboard {
            if is_key_down(KeyCode::Up) {
                left.y -= left.speed * dt;
            }
            if is_key_down(KeyCode::Down) {
                left.y += left.speed * dt;
            }
        } else if control_mode == ControlMode::Mouse {
            // Modalità mouse: segui il mouse Y (solo se all'interno della finestra)
            let (_mx, my) = mouse_position();
            if my >= 0.0 && my <= window_height {
                left.y = my - left.h / 2.0;
            }
        } else {
            // Computer vs Computer: anche la racchetta sinistra segue la pallina.
            left.follow_target(ball.pos.y, dt, window_height);
        }

        left.clamp_into_screen(window_height);

        // La racchetta destra e' controllata dal computer in tutte le modalita'.
        right.follow_target(ball.pos.y, dt, window_height);

        // aggiorna pallina
        ball.pos += ball.vel * dt;

		// collisioni con i muri (parte superiore/inferiore) con piccolo disturbo
        if ball.pos.y - ball.radius <= 0.0 {
            ball.pos.y = ball.radius;
            ball.vel.y = -ball.vel.y * rumore_angolo(); // Leggero disturbo sull'angolo/velocità verticale
            play_sound_once(&sound_bordo);
        } else if ball.pos.y + ball.radius >= window_height {
            ball.pos.y = window_height - ball.radius;
            ball.vel.y = -ball.vel.y * rumore_angolo();
            play_sound_once(&sound_bordo);
        }

        // Collisioni con le racchette
        if circle_rect_collision(ball.pos, ball.radius, left.x, left.y, left.w, left.h) {
            ball.pos.x = left.x + left.w + ball.radius;
            // random speedup
            ball.vel.x = (-ball.vel.x * rumore()).clamp(-MAX_BALL_SPEED, MAX_BALL_SPEED);     
            let hit_pos = (ball.pos.y - left.center_y()) / (left.h / 2.0);
            ball.vel.y += hit_pos * 200.0;
            play_sound_once(&sound_ping);
        } else if circle_rect_collision(ball.pos, ball.radius, right.x, right.y, right.w, right.h) {
            ball.pos.x = right.x - ball.radius;
            ball.vel.x = (-ball.vel.x * rumore()).clamp(-MAX_BALL_SPEED, MAX_BALL_SPEED);
            let hit_pos = (ball.pos.y - right.center_y()) / (right.h / 2.0);
            ball.vel.y += hit_pos * 200.0;
            play_sound_once(&sound_ping);
        }

        // controllo del punteggio
        if ball.pos.x < 0.0 {
            right_score += 1;
            ball.reset(1.0, screen_width(), screen_height());
            play_sound_once(&sound_out);
        } else if ball.pos.x > window_width {
            left_score += 1;
            ball.reset(-1.0, screen_width(), screen_height());
            play_sound_once(&sound_out);
        }

        // tracciamento oggetti
        clear_background(BLACK);

        // linea tratteggiata al centro
        let dash_h = 20.0;
        let gap = 10.0;
        let mut y = 0.0;
        while y < window_height {
            draw_rectangle(window_width / 2.0 - 2.0, y, 4.0, dash_h, GRAY);
            y += dash_h + gap;
        }

        // paddles and ball
        left.draw(RED);
        right.draw(GREEN);
//      ball.draw(WHITE);
// 		ball.draw(RED);    // Pallina rossa
 		ball.draw(YELLOW); // Pallina gialla
// 		ball.draw(GREEN);  // Pallina verde
// 		ball.draw(BLUE);   // Pallina blu

        // aggiorna punteggio
        let score_text = format!("{}    {}", left_score, right_score);
        let font_size = 48.0;
        let text_dim = measure_text(&score_text, None, font_size as u16, 1.0);
        draw_text(
            &score_text,
            window_width / 2.0 - text_dim.width / 2.0,
            60.0,
            font_size,
            WHITE,
        );

        // piccolo testo di aiuto (mostra il controllo scelto)
        let mode_text = match control_mode {
            ControlMode::Keyboard => "Controllo: Frecce Up/Down (R: reset punteggi)",
            ControlMode::Mouse => "Controllo: Mouse (R: reset punteggi)",
            ControlMode::Computer => "Computer vs Computer (R: reset punteggi - Q: esci)",
        };
        draw_text(mode_text, 10.0, window_height - 34.0, 20.0, GRAY);
        let difficulty_text = format!("Difficolta': {}", difficulty.name);
        draw_text(&difficulty_text, 10.0, window_height - 10.0, 20.0, GRAY);

        // reset punteggio
        if is_key_pressed(KeyCode::R) {
            left_score = 0;
            right_score = 0;
            ball.reset(-1.0, screen_width(), screen_height());
        }

        // uscita (da sviluppare)
        if is_key_pressed(KeyCode::Q) {
            // ricomincia la scelta del controllo
            // (per semplicità, usciamo dal programma: l'utente può riavviare per scegliere di nuovo)
            return;
        }

        next_frame().await
    }
}

/// Collisione tra cerchio e rettangolo allineato agli assi (test del punto più vicino)
fn circle_rect_collision(circle_pos: Vec2, radius: f32, rx: f32, ry: f32, rw: f32, rh: f32) -> bool {
    let closest_x = circle_pos.x.clamp(rx, rx + rw);
    let closest_y = circle_pos.y.clamp(ry, ry + rh);
    let dx = circle_pos.x - closest_x;
    let dy = circle_pos.y - closest_y;
    dx * dx + dy * dy <= radius * radius
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Rust Pong (Giorgio Ruffa)".to_owned(),
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        window_resizable: true,
        ..Default::default()
    }
}