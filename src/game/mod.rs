use winit::keyboard::KeyCode;

use crate::engine::texture::load_sprites;
use crate::engine::{engine::Engine, input::Input, renderer::Renderer};
use crate::game::dialogue::{DialogueBox, DialogueData, MinigameTrigger};
use crate::game::game::MyGame;
use crate::game::main_menu::MainMenu;
use crate::game::minigame::{Minigame, MinigameOutcome, MinigameResult, PlaceholderMinigame};

pub mod dialogue;
pub mod game;
pub mod main_menu;
pub mod minigame;
pub mod minigame_color_switch;
pub mod minigame_vaccine;
pub mod minigame_virus;
pub mod object;
pub mod player;
pub mod tile;

/// The minigames that must all be won to complete the game.
const REQUIRED_MINIGAMES: [MinigameTrigger; 3] = [
    MinigameTrigger::CatchVirus,
    MinigameTrigger::VaccineTiming,
    MinigameTrigger::ColorSwitch,
];

/// Communicated back from `GameState::update` so the context can track progress.
pub enum Progress {
    None,
    MinigameWon(MinigameTrigger),
}

pub struct GameContext {
    pub state: GameState,
    /// Distinct required minigames the player has beaten this run.
    won: Vec<MinigameTrigger>,
}

impl GameContext {
    pub fn init(engine: &Engine, renderer: &mut Renderer) -> Self {
        Self {
            state: GameState::init(engine, renderer),
            won: Vec::new(),
        }
    }

    pub fn update(&mut self, input: &Input, dt: f32) {
        if let Some((next, progress)) = self.state.update(input, dt) {
            self.state = next;

            if let Progress::MinigameWon(trigger) = progress {
                if REQUIRED_MINIGAMES.contains(&trigger) && !self.won.contains(&trigger) {
                    self.won.push(trigger);
                }
            }

            // Returning to the menu starts a fresh run.
            if matches!(self.state, GameState::MainMenu(_)) {
                self.won.clear();
            }
        }

        // Debug: instantly complete every required minigame.
        if input.is_just_pressed(KeyCode::F12) {
            self.won = REQUIRED_MINIGAMES.to_vec();
        }

        if self.all_won() && !matches!(self.state, GameState::Won) {
            self.state = GameState::Won;
        }
    }

    fn all_won(&self) -> bool {
        REQUIRED_MINIGAMES.iter().all(|t| self.won.contains(t))
    }

    /// Fraction of required minigames completed, for the progress bar.
    fn progress(&self) -> f32 {
        self.won.len() as f32 / REQUIRED_MINIGAMES.len() as f32
    }

    pub fn render(&self, renderer: &mut Renderer) {
        self.state.render(renderer);
        if !self.state.is_main_menu()
            && !matches!(self.state, GameState::Won | GameState::Intro { .. })
        {
            Self::render_health_bar(renderer, self.progress());
        }
    }

    pub fn on_resize(&mut self, w: f32, h: f32) {
        self.state.on_resize(w, h);
    }

    fn render_health_bar(renderer: &mut Renderer, health: f32) {
        let cam_x = renderer.camera.position.x;
        let cam_y = renderer.camera.position.y;
        let bar_x = cam_x + 6.0;
        let bar_y = cam_y + 12.0;
        let bar_w = 80.0;
        let bar_h = 5.0;
        renderer.draw_sprite_ui("debug_red", bar_x, bar_y, bar_w, bar_h);
        renderer.draw_sprite_ui("health_green", bar_x, bar_y, bar_w * health, bar_h);
    }
}

pub enum GameState {
    MainMenu(MainMenu),

    /// Intro / story screen shown after the menu, before gameplay starts.
    /// Holds the (not-yet-active) overworld so it can be shown in the background.
    Intro {
        game: MyGame,
    },

    Playing(MyGame),
    Paused(MyGame),

    Dialogue {
        game: MyGame,
        data: DialogueData,
        dialogue_box: DialogueBox,
    },

    /// A minigame is running. Overworld suspended.
    Minigame {
        game: MyGame,
        minigame: Box<dyn Minigame>,
        trigger: MinigameTrigger,
        outcome_title: &'static str,
        outcome_fact: &'static str,
        object_index: Option<usize>,
    },

    /// Win/lose screen shown after a minigame ends.
    Results {
        game: MyGame,
        outcome: MinigameOutcome,
    },

    Won,
}

impl GameState {
    pub fn init(engine: &Engine, renderer: &mut Renderer) -> Self {
        load_sprites(engine, renderer);
        let (sw, sh) = engine.screen_size();
        GameState::MainMenu(MainMenu::new(sw, sh))
    }

    pub fn update(&mut self, input: &Input, dt: f32) -> Option<(GameState, Progress)> {
        match self {
            GameState::MainMenu(menu) => menu.update(input, dt).map(|s| (s, Progress::None)),

            GameState::Intro { game } => {
                if input.is_just_pressed(KeyCode::Space) || input.is_just_pressed(KeyCode::Enter) {
                    let game = std::mem::replace(game, MyGame::new(0.0, 0.0));
                    return Some((GameState::Playing(game), Progress::None));
                }
                None
            }

            GameState::Playing(game) => {
                // Debug shortcut: launch the color-switch minigame directly
                // (also reachable in-world via the ColorGate object).
                if input.is_just_pressed(KeyCode::F10) {
                    let trigger = MinigameTrigger::ColorSwitch;
                    let (minigame, title, fact) = make_minigame(trigger);
                    let game = std::mem::replace(game, MyGame::new(0.0, 0.0));
                    return Some((
                        GameState::Minigame {
                            game,
                            minigame,
                            trigger,
                            outcome_title: title,
                            outcome_fact: fact,
                            object_index: None,
                        },
                        Progress::None,
                    ));
                }
                game.update(input, dt).map(|s| (s, Progress::None))
            }
            GameState::Paused(game) => game.update_paused(input).map(|s| (s, Progress::None)),

            GameState::Dialogue { game, data, .. } => {
                if input.is_just_pressed(KeyCode::Enter) {
                    let next = if let Some(trigger) = data.minigame {
                        let (minigame, title, fact) = make_minigame(trigger);
                        let game = std::mem::replace(game, MyGame::new(0.0, 0.0));
                        let object_index = game.objects.iter().position(|obj| {
                            obj.interaction
                                .as_ref()
                                .and_then(|i| i.minigame)
                                .map(|t| t == trigger)
                                .unwrap_or(false)
                        });
                        GameState::Minigame {
                            game,
                            minigame,
                            trigger,
                            outcome_title: title,
                            outcome_fact: fact,
                            object_index,
                        }
                    } else {
                        let game = std::mem::replace(game, MyGame::new(0.0, 0.0));
                        GameState::Playing(game)
                    };
                    return Some((next, Progress::None));
                }
                None
            }

            GameState::Minigame {
                game,
                minigame,
                trigger,
                outcome_title,
                outcome_fact,
                object_index,
            } => match minigame.update(input, dt) {
                MinigameResult::Running => None,
                MinigameResult::Won { score } => {
                    let outcome = MinigameOutcome {
                        won: true,
                        score,
                        title: outcome_title,
                        fact: outcome_fact,
                        object_index: *object_index,
                        trigger: *trigger,
                    };
                    let game = std::mem::replace(game, MyGame::new(0.0, 0.0));
                    Some((GameState::Results { game, outcome }, Progress::None))
                }
                MinigameResult::Lost => {
                    let outcome = MinigameOutcome {
                        won: false,
                        score: 0,
                        title: outcome_title,
                        fact: outcome_fact,
                        object_index: *object_index,
                        trigger: *trigger,
                    };
                    let game = std::mem::replace(game, MyGame::new(0.0, 0.0));
                    Some((GameState::Results { game, outcome }, Progress::None))
                }
            },

            GameState::Results { game, outcome } => {
                if input.is_just_pressed(KeyCode::Enter) || input.is_just_pressed(KeyCode::Space) {
                    let progress = if outcome.won {
                        if let Some(idx) = outcome.object_index {
                            game.objects[idx].completed = true;
                            game.rebuild_collision_map();
                        }
                        Progress::MinigameWon(outcome.trigger)
                    } else {
                        Progress::None
                    };

                    let game = std::mem::replace(game, MyGame::new(0.0, 0.0));
                    return Some((GameState::Playing(game), progress));
                }
                None
            }

            GameState::Won => {
                if input.is_just_pressed(KeyCode::Enter) || input.is_just_pressed(KeyCode::Space) {
                    return Some((GameState::MainMenu(MainMenu::new(0.0, 0.0)), Progress::None));
                }
                None
            }
        }
    }

    pub fn render(&self, renderer: &mut Renderer) {
        match self {
            GameState::MainMenu(menu) => menu.render(renderer),

            GameState::Intro { game } => {
                // Show the overworld behind the intro text.
                game.render(renderer, false);
                Self::render_overlay(renderer, "transparent_gray");

                let cam_x = renderer.camera.position.x;
                let cam_y = renderer.camera.position.y;
                let cam_w = renderer.camera.logical_width;
                let cam_h = renderer.camera.logical_height;
                let cx = cam_x + cam_w / 2.0;

                let font = crate::engine::font::Font::new("font");
                let body = "Eines Tages besuchst du mit deiner Schulklasse das Forschungslabor \
                    von Professor Nova. Dort entwickelt er eine unglaubliche Erfindung: einen \
                    Schrumpfstrahl, mit dem Menschen winzig klein werden koennen. Ploetzlich wird \
                    ein Patient schwer krank, und niemand weiss, wie sich die Krankheit ausbreitet. \
                    Du wirst auf Mikroskopgroesse geschrumpft und in seinen Koerper geschickt. \
                    Bist du bereit fuer die wichtigste Mission deines Lebens? \
                    \
                    Laufe in der Spielwelt herum und finde 3 wichtige Orte \
                    um die Mission erfolgreich abzuschließen";

                font.draw_centered_ui(renderer, "BODY QUEST", cx, cam_y + 16.0, 1.0);
                font.draw_paragraph_ui(renderer, body, cx, cam_y + 40.0, 0.45, cam_w - 24.0);
                font.draw_centered_ui(renderer, "LEERTASTE ZUM STARTEN", cx, cam_y + cam_h - 16.0, 0.6);
            }

            GameState::Playing(game) => game.render(renderer, true),
            GameState::Paused(game) => {
                game.render(renderer, false);
                Self::render_overlay(renderer, "transparent_gray");

                let cam_x = renderer.camera.position.x;
                let cam_y = renderer.camera.position.y;
                let cam_w = renderer.camera.logical_width;
                let cx = cam_x + cam_w / 2.0;

                let font = crate::engine::font::Font::new("font");
                font.draw_centered_ui(renderer, "PAUSE", cx, cam_y + 28.0, 1.0);
                font.draw_centered_ui(renderer, "GESUNDHEITSANZEIGE", cx, cam_y + 60.0, 0.6);

                let body = "Diese Leiste zeigt deinen Fortschritt und den Gesundheitszustand \
                    des Patienten. Jede abgeschlossene Aufgabe hilft dem Koerper, die Krankheit \
                    zu bekaempfen. Schaffe alle Missionen, um den Koerper vollstaendig zu heilen!";
                font.draw_paragraph_ui(renderer, body, cx, cam_y + 76.0, 0.45, cam_w - 40.0);
            }
            GameState::Dialogue {
                game,
                data,
                dialogue_box,
            } => {
                game.render(renderer, false);
                dialogue_box.render(renderer, data);
            }
            GameState::Minigame { minigame, .. } => {
                minigame.render(renderer);
            }
            GameState::Results { game, outcome } => {
                game.render(renderer, false);
                Self::render_overlay(renderer, "transparent_gray");

                let cam_x = renderer.camera.position.x;
                let cam_y = renderer.camera.position.y;
                let cam_w = renderer.camera.logical_width;
                let cam_h = renderer.camera.logical_height;
                let cx = cam_x + cam_w / 2.0;
                let cy = cam_y + cam_h / 2.0;

                let font = crate::engine::font::Font::new("font");

                if outcome.won {
                    // Topic heading + the educational closing text for this minigame.
                    font.draw_centered_ui(renderer, outcome.title, cx, cam_y + 14.0, 0.9);
                    font.draw_paragraph_ui(renderer, outcome.fact, cx, cam_y + 40.0, 0.45, cam_w - 24.0);
                    font.draw_centered_ui(renderer, "ENTER UM WEITERZUSPIELEN", cx, cam_y + cam_h - 14.0, 0.6);
                } else {
                    font.draw_centered_ui(renderer, "VERLOREN", cx, cy - 14.0, 1.0);
                    font.draw_centered_ui(renderer, "ENTER ZUM ERNEUT VERSUCHEN", cx, cy + 8.0, 0.5);
                }
            }
            GameState::Won => {
                let cam_x = renderer.camera.position.x;
                let cam_y = renderer.camera.position.y;
                let cam_w = renderer.camera.logical_width;
                let cam_h = renderer.camera.logical_height;
                let cx = cam_x + cam_w / 2.0;

                renderer.draw_sprite_ui("transparent_gray", cam_x, cam_y, cam_w, cam_h);
                let font = crate::engine::font::Font::new("font");

                let body = "Dank deiner Hilfe konnten die Krankheitserreger besiegt werden. \
                    Die weissen Blutkoerperchen haben gekaempft, die Impfung hat das Immunsystem \
                    vorbereitet und die Antikoerper haben ihren Einsatzort erreicht. Der Patient \
                    ist nun wieder gesund. Du hast gelernt, wie unser Immunsystem funktioniert und \
                    warum Impfungen wichtig sind. Herzlichen Glueckwunsch, Body Quest ist geschafft!";

                font.draw_centered_ui(renderer, "MISSION ERFOLGREICH", cx, cam_y + 16.0, 0.9);
                font.draw_paragraph_ui(renderer, body, cx, cam_y + 42.0, 0.45, cam_w - 24.0);
                font.draw_centered_ui(renderer, "ENTER ZUM MENU", cx, cam_y + cam_h - 14.0, 0.6);
            }
        }
    }

    pub fn on_resize(&mut self, w: f32, h: f32) {
        match self {
            GameState::MainMenu(menu) => menu.on_resize(w, h),
            GameState::Intro { game } => game.on_resize(w, h),
            GameState::Playing(game) => game.on_resize(w, h),
            GameState::Paused(game) => game.on_resize(w, h),
            GameState::Dialogue { game, .. } => game.on_resize(w, h),
            GameState::Minigame { game, minigame, .. } => {
                game.on_resize(w, h);
                minigame.on_resize(w, h);
            }
            GameState::Results { game, .. } => game.on_resize(w, h),
            GameState::Won => {}
        }
    }

    fn render_overlay(renderer: &mut Renderer, texture: &str) {
        renderer.draw_sprite(
            texture,
            renderer.camera.position.x,
            renderer.camera.position.y,
            renderer.camera.logical_width,
            renderer.camera.logical_height,
        );
    }

    pub fn is_main_menu(&self) -> bool {
        matches!(self, GameState::MainMenu(_))
    }
}

fn make_minigame(trigger: MinigameTrigger) -> (Box<dyn Minigame>, &'static str, &'static str) {
    match trigger {
        MinigameTrigger::CatchVirus => (
            Box::new(minigame_virus::VirusMinigame::new()),
            "IMMUNSYSTEM",
            "Gut gemacht! Dank deiner Hilfe konnten viele Krankheitserreger beseitigt werden. Genau wie im Spiel bekaempfen weisse Blutkoerperchen jeden Tag echte Viren und Bakterien in unserem Koerper. Damit das Immunsystem stark bleibt, sind genug Schlaf, Bewegung und eine ausgewogene Ernaehrung wichtig.",
        ),
        MinigameTrigger::SortFood => (
            Box::new(PlaceholderMinigame::new("SORT THE FOOD")),
            "ERNAEHRUNG",
            "Eine ausgewogene Ernaehrung gibt deinem Koerper die Energie, die er braucht.",
        ),
        MinigameTrigger::VaccineTiming => (
            Box::new(minigame_vaccine::AaMinigame::new()),
            "IMPFUNGEN",
            "Ausgezeichnet! Durch Impfungen lernt das Immunsystem Krankheitserreger frueh kennen und kann bei einer echten Infektion viel schneller reagieren. Wenn viele Menschen geimpft sind, entsteht Herdenimmunitaet, die auch andere schuetzt. Impfstoffe werden gruendlich getestet und gehoeren zu den sichersten Methoden gegen schwere Krankheiten.",
        ),
        MinigameTrigger::DeliverOxygen => (
            Box::new(PlaceholderMinigame::new("DELIVER OXYGEN")),
            "ATMUNG",
            "Rote Blutkoerperchen transportieren Sauerstoff zu jeder Zelle des Koerpers.",
        ),
        MinigameTrigger::ColorSwitch => (
            Box::new(minigame_color_switch::ColorSwitchMinigame::new()),
            "ANTIKOERPER",
            "Geschafft! Die Antikoerper haben ihr Ziel erreicht und unterstuetzen nun die Abwehr des Koerpers. Taucht derselbe Krankheitserreger spaeter erneut auf, erkennt ihn das Immunsystem viel schneller. Dieses Gedaechtnis des Immunsystems ist einer der wichtigsten Gruende, warum Impfungen so gut funktionieren.",
        ),
    }
}
