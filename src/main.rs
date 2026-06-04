use macroquad::prelude::*;
use rand::gen_range;
use rayon::prelude::*; // El botón del turbo multinúcleo

// 1. EL AGENTE (Cada bicho de la horda con su mente autónoma)
#[derive(Clone, Copy)]
struct Boid {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}

// 2. EL MOTOR DE LA COLMENA
struct SwarmEngine {
    boids: Vec<Boid>,
}

impl SwarmEngine {
    fn new() -> Self {
        Self { boids: Vec::new() }
    }

    fn spawn_swarm(&mut self, count: usize) {
        for _ in 0..count {
            self.boids.push(Boid {
                x: gen_range(100.0, screen_width() - 100.0),
                y: gen_range(100.0, screen_height() - 100.0),
                vx: gen_range(-3.0, 3.0),
                vy: gen_range(-3.0, 3.0),
            });
        }
    }

    // 🔥 EL CEREBRO DE LA COLMENA EN PARALELO
    // Cada bicho calcula su posición mirando a los demás usando todos los hilos de la CPU
    fn update_swarm(&mut self, dt: f32, width: f32, height: f32) {
        let current_boids = self.boids.clone(); // Copia de lectura segura para los hilos

        self.boids.par_iter_mut().for_each(|boid| {
            let mut close_dx = 0.0;
            let mut close_dy = 0.0;
            let mut x_vel_avg = 0.0;
            let mut y_vel_avg = 0.0;
            let mut neighboring_boids = 0;

            // Radio de visión de la IA (Optimizado para bare-metal)
            let visual_range: f32 = 40.0;
            let protected_range: f32 = 8.0;

            for other in &current_boids {
                let dx = boid.x - other.x;
                let dy = boid.y - other.y;
                let distance = (dx*dx + dy*dy).sqrt();

                if distance < visual_range && distance > 0.0 {
                    // 1. REGLA DE SEPARACIÓN: Evitar chocarse con los compañeros
                    if distance < protected_range {
                        close_dx += dx;
                        close_dy += dy;
                    }
                    // 2. REGLA DE ALINEACIÓN: Ir en la misma dirección que el grupo
                    x_vel_avg += other.vx;
                    y_vel_avg += other.vy;
                    neighboring_boids += 1;
                }
            }

            // Inyectamos las fuerzas matemáticas en el cerebro del agente
            if neighboring_boids > 0 {
                x_vel_avg /= neighboring_boids as f32;
                y_vel_avg /= neighboring_boids as f32;

                boid.vx += (x_vel_avg - boid.vx) * 0.05;
                boid.vy += (y_vel_avg - boid.vy) * 0.05;
            }

            boid.vx += close_dx * 0.1;
            boid.vy += close_dy * 0.1;

            // Añadimos una fuerza central sutil para que no se dispersen (Cohesión simulada)
            let target_x = width / 2.0;
            let target_y = height / 2.0;
            boid.vx += (target_x - boid.x) * 0.0005;
            boid.vy += (target_y - boid.y) * 0.0005;

            // Limitador de velocidad máxima para que no salgan disparados
            let speed = (boid.vx*boid.vx + boid.vy*boid.vy).sqrt();
            if speed > 5.0 {
                boid.vx = (boid.vx / speed) * 5.0;
                boid.vy = (boid.vy / speed) * 5.0;
            }

            // Aplicamos movimiento nativo masivo
            boid.x += boid.vx * dt * 60.0;
            boid.y += boid.vy * dt * 60.0;

            // Efecto pantalla infinita (Aparecen por el otro lado estilo Pac-Man)
            if boid.x < 0.0 { boid.x = width; }
            if boid.x > width { boid.x = 0.0; }
            if boid.y < 0.0 { boid.y = height; }
            if boid.y > height { boid.y = 0.0; }
        });
    }

    // DIBUJADO DE LA HORDA EN GPU
    fn render_swarm(&self) {
        for boid in &self.boids {
            // El color cambia dinámicamente según la dirección de su velocidad (Estilo Cyber-Radar)
            let color_r = (boid.vx.abs() / 5.0).min(1.0);
            let color_g = (boid.vy.abs() / 5.0).min(1.0);
            
            draw_circle(boid.x, boid.y, 2.0, Color::new(color_r, color_g, 1.0, 0.9));
        }
    }
}

// 3. EL BUCLE SOBERANO
#[macroquad::main("AXIOM SWARM AI // MENTE COLMENA PARALELA")]
async fn main() {
    let mut swarm = SwarmEngine::new();
    
    // ¡Metemos 15.000 mentes pensantes en hilos paralelos de golpe!
    swarm.spawn_swarm(15000);

    loop {
        clear_background(Color::new(0.02, 0.01, 0.03, 1.0)); // Tono búnker morado oscuro
        
        let dt = get_frame_time();

        // Actualización de la Inteligencia Artificial grupal en hilos paralelos
        swarm.update_swarm(dt, screen_width(), screen_height());
        
        // Renderizado
        swarm.render_swarm();

        // Telemetría de Axiom Systems
        draw_rectangle(10.0, 10.0, 320.0, 80.0, Color::new(0.0, 0.0, 0.0, 0.8));
        draw_text(&format!("FPS: {}", get_fps()), 20.0, 30.0, 20.0, GREEN);
        draw_text(&format!("SWARM COMPLEXITY: MULTITHREADED IA"), 20.0, 50.0, 14.0, MAGENTA);
        draw_text(&format!("ACTIVE AGENTS CEREBROS: {}", swarm.boids.len()), 20.0, 70.0, 15.0, CYAN);

        next_frame().await
    }
}
