use super::game::effect::StarManager;
use super::game::game_manager::GameManager;
use super::game::game_manager::Params as GameManagerParams;
use super::game::score_holder::ScoreHolder;

use crate::framework::{AppTrait, RendererTrait, SystemTrait, VKey};
use crate::util::fps_calc::{FpsCalc, TimerTrait};
use crate::util::pad::{Pad, PadBit};

#[cfg(debug_assertions)]
use super::debug::EditTrajManager;

const KEY_HIGH_SCORE: &str = "highScore";
const DEFAULT_HIGH_SCORE: u32 = 1000;

#[derive(PartialEq)]
enum AppState {
    Title,
    Game,

    #[cfg(debug_assertions)]
    EditTraj,
}

pub struct GalanguaApp<T: TimerTrait, S: SystemTrait> {
    system: S,
    state: AppState,
    count: u32,
    pad: Pad,
    pressed_key: Option<VKey>,
    fps_calc: FpsCalc<T>,
    game_manager: Option<GameManager>,
    star_manager: StarManager,
    frame_count: u32,
    score_holder: ScoreHolder,
    prev_high_score: u32,

    #[cfg(debug_assertions)]
    paused: bool,
    #[cfg(debug_assertions)]
    edit_traj_manager: Option<EditTrajManager>,
}

impl<T: TimerTrait, S: SystemTrait> GalanguaApp<T, S> {
    pub fn new(timer: T, system: S) -> Self {
        let high_score = system.get_u32(&KEY_HIGH_SCORE).or(Some(DEFAULT_HIGH_SCORE)).unwrap();

        let star_manager = StarManager::new();
        let score_holder = ScoreHolder {
            score: 0,
            high_score: high_score,
        };

        Self {
            system,
            state: AppState::Title,
            count: 0,
            pad: Pad::new(),
            pressed_key: None,
            fps_calc: FpsCalc::new(timer),
            game_manager: None,
            star_manager,
            frame_count: 0,
            score_holder,
            prev_high_score: 0,

            #[cfg(debug_assertions)]
            paused: false,
            #[cfg(debug_assertions)]
            edit_traj_manager: None,
        }
    }

    fn update_main(&mut self) -> bool {
        if self.pressed_key == Some(VKey::Escape) {
            if self.state != AppState::Title {
                self.back_to_title();
            } else {
                return false;
            }
        }

        #[cfg(debug_assertions)]
        {
            if self.pressed_key == Some(VKey::Return) {
                self.paused = !self.paused;
            }
            if self.paused && self.pressed_key != Some(VKey::S) {
                return true;
            }
        }

        self.star_manager.update();

        match self.state {
            AppState::Title => {
                self.count = self.count.wrapping_add(1);

                if self.pad.is_trigger(PadBit::A) {
                    let mut game_manager = GameManager::new();
                    game_manager.restart();
                    self.game_manager = Some(game_manager);
                    self.prev_high_score = self.score_holder.high_score;
                    self.score_holder.reset_score();
                    self.state = AppState::Game;
                    self.frame_count = 0;
                }

                #[cfg(debug_assertions)]
                if self.pressed_key == Some(VKey::E) {
                    self.state = AppState::EditTraj;

                    let mut game_manager = GameManager::new();
                    game_manager.start_edit_mode();
                    self.game_manager = Some(game_manager);
                    self.edit_traj_manager = Some(EditTrajManager::new());
                }
            }
            AppState::Game => {
                self.frame_count += 1;
                let mut params = GameManagerParams {
                    star_manager: &mut self.star_manager,
                    pad: &self.pad,
                    score_holder: &mut self.score_holder,
                };
                let game_manager = self.game_manager.as_mut().unwrap();
                game_manager.update(&mut params, &mut self.system);
                if game_manager.is_finished() {
                    self.back_to_title();
                }
            }

            #[cfg(debug_assertions)]
            AppState::EditTraj => {
                self.frame_count += 1;

                let game_manager = self.game_manager.as_mut().unwrap();
                self.edit_traj_manager.as_mut().unwrap().update(self.pressed_key, game_manager);

                let mut params = GameManagerParams {
                    star_manager: &mut self.star_manager,
                    pad: &self.pad,
                    score_holder: &mut self.score_holder,
                };
                game_manager.update(&mut params, &mut self.system);
                if game_manager.is_finished() {
                    self.back_to_title();
                }
            }
        }
        true
    }

    fn draw_main<R>(&mut self, renderer: &mut R)
    where
        R: RendererTrait,
    {
        self.star_manager.draw(renderer);
        match self.state {
            AppState::Title => {
                renderer.set_texture_color_mod("font", 255, 255, 255);
                renderer.draw_str("font", 10 * 8, 8 * 8, "GALANGUA");

                if self.count & 32 == 0 {
                    renderer.draw_str("font", 2 * 8, 25 * 8, "PRESS SPACE KEY TO START");
                }
                draw_scores(renderer, &self.score_holder, true);
            }
            AppState::Game => {
                self.game_manager.as_mut().unwrap().draw(renderer);
                draw_scores(renderer, &self.score_holder, (self.frame_count & 31) < 16);
            }
            #[cfg(debug_assertions)]
            AppState::EditTraj => {
                let game_manager = self.game_manager.as_mut().unwrap();
                game_manager.draw(renderer);

                self.edit_traj_manager.as_mut().unwrap().draw(renderer, game_manager);
            }
        }

        #[cfg(debug_assertions)]
        {
            renderer.set_texture_color_mod("font", 128, 128, 128);
            renderer.draw_str("font", 23 * 8, 0 * 8, &format!("FPS{:2}", self.fps_calc.fps()));
        }
    }

    fn back_to_title(&mut self) {
        self.star_manager.set_stop(false);

        if self.score_holder.high_score > self.prev_high_score {
            self.on_high_score_updated();
        }

        self.state = AppState::Title;
        self.count = 0;
        self.game_manager = None;
    }

    fn on_high_score_updated(&mut self) {
        self.system.set_u32(KEY_HIGH_SCORE, self.score_holder.high_score);
    }
}

impl<R: RendererTrait, T: TimerTrait, S: SystemTrait> AppTrait<R> for GalanguaApp<T, S> {
    fn on_key(&mut self, vkey: VKey, down: bool) {
        self.pad.on_key(vkey, down);
        if down {
            self.pressed_key = Some(vkey);
        }
    }

    fn on_joystick_axis(&mut self, axis_index: u8, dir: i8) {
        self.pad.on_joystick_axis(axis_index, dir);
    }

    fn on_joystick_button(&mut self, button_index: u8, down: bool) {
        self.pad.on_joystick_button(button_index, down);
    }

    fn init(&mut self, renderer: &mut R)
    where
        R: RendererTrait,
    {
        renderer.load_textures("assets", &["chr.png", "font.png"]);
        renderer.load_sprite_sheet("assets/chr.json");
    }

    fn update(&mut self) -> bool {
        self.pad.update();
        let result = self.update_main();
        self.pressed_key = None;
        result
    }

    fn draw(&mut self, renderer: &mut R)
    where
        R: RendererTrait,
    {
        renderer.set_draw_color(0, 0, 0);
        renderer.clear();

        self.draw_main(renderer);

        self.fps_calc.update();
    }
}

fn draw_scores<R: RendererTrait>(
    renderer: &mut R, score_holder: &ScoreHolder, show_1up: bool
) {
    renderer.set_texture_color_mod("font", 255, 0, 0);
    if show_1up {
        renderer.draw_str("font", 2 * 8, 0 * 8, "1UP");
    }
    renderer.draw_str("font", 9 * 8, 0 * 8, "HIGH SCORE");
    renderer.set_texture_color_mod("font", 255, 255, 255);

    const MAX_DISP_SCORE: u32 = 9999999;
    let score = std::cmp::min(score_holder.score, MAX_DISP_SCORE);
    renderer.draw_str("font", 0 * 8, 1 * 8, &format!("{:6}0", score / 10));
    let high_score = std::cmp::min(score_holder.high_score, MAX_DISP_SCORE);
    renderer.draw_str("font", 10 * 8, 1 * 8, &format!("{:6}0", high_score / 10));
}
