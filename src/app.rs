use ratatui::style::Color;
use rand::Rng;
use rodio::Source;
use std::time::{Duration, Instant};

pub enum AppState {
    Question,
    AcceptedYes,
    AcceptedMaybe,
    TryingNo,
}

pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub char: char,
    pub color: Color,
    pub lifetime: u32,
}

pub struct Star {
    pub x: u16,
    pub y: u16,
    pub brightness: u8,
    pub twinkle_speed: u8,
}

pub struct App {
    pub state: AppState,
    pub no_button_offset: (i16, i16),
    pub attempt_count: u32,
    pub frame_count: u64,
    pub start_time: Instant,
    pub particles: Vec<Particle>,
    pub stars: Vec<Star>,
    pub stars_initialized: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: AppState::Question,
            no_button_offset: (0, 0),
            attempt_count: 0,
            frame_count: 0,
            start_time: Instant::now(),
            particles: Vec::new(),
            stars: Vec::new(),
            stars_initialized: false,
        }
    }

    pub fn init_stars(&mut self, width: u16, height: u16) {
        if !self.stars_initialized {
            let mut rng = rand::thread_rng();
            self.stars = (0..100).map(|_| Star {
                x: rng.gen_range(0..width),
                y: rng.gen_range(0..height),
                brightness: rng.gen_range(0..10),
                twinkle_speed: rng.gen_range(1..5),
            }).collect();
            self.stars_initialized = true;
        }
    }

    pub fn handle_yes(&mut self) {
        self.state = AppState::AcceptedYes;
        self.spawn_celebration_particles();
        std::thread::spawn(|| {
            if let Ok((_stream, handle)) = rodio::OutputStream::try_default() {
                let sink = rodio::Sink::try_new(&handle).unwrap();
                sink.append(rodio::source::SineWave::new(523.25)
                    .take_duration(Duration::from_millis(200)).amplify(0.20));
                sink.append(rodio::source::SineWave::new(659.25)
                    .take_duration(Duration::from_millis(200)).amplify(0.20));
                sink.append(rodio::source::SineWave::new(783.99)
                    .take_duration(Duration::from_millis(400)).amplify(0.20));
                sink.sleep_until_end();
            }
        });
    }

    pub fn handle_maybe(&mut self) {
        self.state = AppState::AcceptedMaybe;
        std::thread::spawn(|| {
            if let Ok((_stream, handle)) = rodio::OutputStream::try_default() {
                let sink = rodio::Sink::try_new(&handle).unwrap();
                sink.append(rodio::source::SineWave::new(440.0)
                    .take_duration(Duration::from_millis(300)).amplify(0.15));
                sink.sleep_until_end();
            }
        });
    }

    pub fn handle_no(&mut self) {
        self.attempt_count += 1;
        self.no_button_offset = (
            (self.attempt_count as i16 * 7) % 30 - 15,
            (self.attempt_count as i16 * 11) % 20 - 10,
        );
        self.state = AppState::TryingNo;
        std::thread::spawn(|| {
            if let Ok((_stream, handle)) = rodio::OutputStream::try_default() {
                let sink = rodio::Sink::try_new(&handle).unwrap();
                sink.append(rodio::source::SineWave::new(200.0)
                    .take_duration(Duration::from_millis(150)).amplify(0.15));
                sink.sleep_until_end();
            }
        });
    }

    pub fn update(&mut self) {
        self.frame_count += 1;
        self.particles.retain_mut(|p| {
            p.x += p.vx;
            p.y += p.vy;
            p.vy += 0.15;
            p.vx *= 0.99;
            p.lifetime = p.lifetime.saturating_sub(1);
            p.lifetime > 0 && p.y < 50.0
        });
        for star in &mut self.stars {
            star.brightness = ((
                    self.frame_count / star.twinkle_speed as u64) % 10
            ) as u8;
        }
    }

    fn spawn_celebration_particles(&mut self) {
        let chars = ['*', '+', 'o', '.', '~', '^', '#', '@'];
        let colors = [
            Color::Red, Color::LightRed, Color::Magenta, Color::LightMagenta, 
            Color::Yellow, Color::LightYellow, Color::Cyan, Color::LightCyan
        ];
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let angle = rng.gen::<f32>() * std::f32::consts::PI * 2.0;
            let speed = 1.5 + rng.gen::<f32>() * 3.0;
            self.particles.push(Particle {
                x: 0.0, y: 0.0,
                vx: angle.cos() * speed,
                vy: angle.sin() * speed - 3.0,
                char: chars[rng.gen_range(0..chars.len())],
                color: colors[rng.gen_range(0..colors.len())],
                lifetime: 100,
            });
        }
    }

    pub fn get_heart_beat_scale(&self) -> f32 {
        let time = self.start_time.elapsed().as_secs_f32();
        1.0 + (time * 2.5).sin() * 0.2
    }
}
