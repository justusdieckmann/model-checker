#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::collections::HashMap;

use eframe::egui;
use eframe::egui::{Align, Layout, Window};
use eframe::emath::{vec2, Pos2, Vec2};
use eframe::epaint::{CubicBezierShape, Stroke};
use egui::{Align2, Color32, FontId, Frame, Key, PointerButton, Sense, Shape, Ui};
use egui_extras::{Size, StripBuilder};
use model_checker::{ltl_model_check, KripkeState, KripkeStructure};

type StateId = u64;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_min_inner_size((320.0, 240.0))
            .with_inner_size((1280.0, 720.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Model-Checker",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_zoom_factor(1.25);
            Box::<MyApp>::default()
        }),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| {
                    cc.egui_ctx.set_zoom_factor(1.25);
                    Box::<MyApp>::default()
                }),
            )
            .await
            .expect("failed to start eframe");
    });
}

const STATE_RADIUS: f32 = 20.0;
const ARROWHEAD_HALF_WIDTH: f32 = 6.0;
const ARROWHEAD_LENGTH: f32 = 12.0;

struct MyState {
    aps: Vec<String>,
    pos: Pos2,
    id: StateId,
    start: bool,
}

struct MyApp {
    states: HashMap<StateId, MyState>,
    transitions: Vec<(StateId, StateId)>,
    current_id: StateId,
    start_drag: Option<StateId>,
    selected_id: Option<StateId>,
    query: String,
    aptext: String,
    result_text: String,
    show_help: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            states: HashMap::new(),
            transitions: vec![],
            current_id: 0,
            start_drag: None,
            selected_id: None,
            query: "".to_owned(),
            aptext: "".to_owned(),
            result_text: "".to_owned(),
            show_help: false,
        }
    }
}

impl MyApp {
    fn check(&mut self) {
        let mut map = HashMap::new();

        let mut has_start = false;

        for state in self.states.values() {
            map.insert(
                state.id,
                KripkeState {
                    aps: state.aps.clone(),
                    id: state.id,
                    start: state.start,
                },
            );
            if state.start {
                has_start = true;
            }

            // Should probably happen inside the lib, and more performant.
            if self
                .transitions
                .iter()
                .find(|(s1, _)| *s1 == state.id)
                .is_none()
            {
                self.result_text = "⚠ KS ist nicht total".to_owned();
                return;
            }
        }

        if !has_start {
            self.result_text = "⚠ KS hat keinen Start".to_owned();
            return;
        }

        let ks = KripkeStructure {
            states: map,
            transitions: self.transitions.clone(),
        };

        let result = ltl_model_check(&ks, &self.query);

        if let Some(success) = result {
            if success {
                self.result_text = "✔ Erfüllt".to_owned();
            } else {
                self.result_text = "❌ Fehler!".to_owned();
            }
        } else {
            self.result_text = "⚠ Syntaxfehler".to_owned();
        }
    }

    fn my_update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    let name_label = ui.label("Formel: ");
                    ui.text_edit_singleline(&mut self.query)
                        .labelled_by(name_label.id);
                    let button = ui.button("Prüfen");
                    if button.clicked() {
                        self.check();
                    }
                    ui.label(&self.result_text);
                });
                ui.horizontal(|ui| {
                    let name_label = ui.label("APs: ");
                    ui.text_edit_singleline(&mut self.aptext)
                        .labelled_by(name_label.id);
                    let button = ui.button("Setzen");
                    if button.clicked() {
                        if let Some(selected_id) = self.selected_id {
                            let state = self.states.get_mut(&selected_id).unwrap();
                            state.aps = self
                                .aptext
                                .split(",")
                                .filter(|s| !s.trim().is_empty())
                                .map(|a| a.trim().to_owned())
                                .collect();
                        }
                    }
                });
            });
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.with_layout(Layout::top_down_justified(Align::Max), |ui| {
                    if ui.link("Anleitung").clicked() {
                        self.show_help = true;
                    }
                    ui.hyperlink_to(
                        "View on GitHub",
                        "https://github.com/justusdieckmann/model-checker/",
                    );
                });
            })
        });

        Window::new("Anleitung")
            .open(&mut self.show_help)
            .resizable(false)
            .collapsible(false)
            .show(ui.ctx(), |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label("LTL Syntax consists of Until ");
                    ui.code("U");
                    ui.label(", Next ");
                    ui.code("X");
                    ui.label(", Release ");
                    ui.code("R");
                    ui.label(", Weak Until ");
                    ui.code("W");
                    ui.label(", Future ");
                    ui.code("F");
                    ui.label(", Generally ");
                    ui.code("G");
                    ui.label(", And ");
                    ui.code("&");
                    ui.label(", Or ");
                    ui.code("|");
                    ui.label(", Not ");
                    ui.code("!");
                    ui.label(", true ");
                    ui.code("1");
                    ui.label(", false ");
                    ui.code("0");
                    ui.label(", parenthesis and atomic propositions, which begin with a lower-case letter.");
                });
                ui.add_space(10.0);
                ui.label("Kripke structure states can be created with a click, connected with a drag (only visible after mouse released), deleted with `del` and made a starting state with a double click.");
                ui.add_space(10.0);
                ui.label("When a state is selected (blue), its atomic propositions can be set via the text field, separated by `,`.");
            });

        Frame::canvas(ui.style()).show(ui, |ui| {
            let color = if ctx.style().visuals.dark_mode {
                Color32::from_gray(200)
            } else {
                Color32::from_gray(25)
            };

            let (mut response, painter) =
                ui.allocate_painter(ui.available_size_before_wrap(), Sense::click_and_drag());

            ctx.input(|input| {
                if input.key_pressed(Key::Delete) {
                    if let Some(selected_id) = self.selected_id {
                        self.states.remove(&selected_id).unwrap();
                        self.transitions.retain(|(state1, state2)| {
                            *state1 != selected_id && *state2 != selected_id
                        });
                        self.selected_id = None;
                        self.aptext = "".to_owned();
                    }
                }
            });

            if let Some(pointer_pos) = response.interact_pointer_pos() {
                let canvas_pos = pointer_pos;

                let result = self
                    .states
                    .values_mut()
                    .find(|state| state.pos.distance(canvas_pos) <= STATE_RADIUS);

                if let Some(hit_state) = result {
                    if response.drag_started_by(PointerButton::Primary) {
                        self.start_drag = Some(hit_state.id);
                    } else if response.drag_stopped_by(PointerButton::Primary) {
                        if let Some(start_drag) = self.start_drag {
                            if !self.transitions.contains(&(start_drag, hit_state.id)) {
                                self.transitions.push((start_drag, hit_state.id));
                                response.mark_changed();
                            }
                        }
                        self.start_drag = None;
                    } else if response.double_clicked() {
                        hit_state.start = !hit_state.start;
                    } else if response.clicked() {
                        self.selected_id = Some(hit_state.id);
                        self.aptext = hit_state.aps.join(", ");
                        response.mark_changed();
                    }
                } else {
                    if response.clicked() {
                        self.states.insert(
                            self.current_id,
                            MyState {
                                aps: vec![],
                                pos: canvas_pos,
                                id: self.current_id,
                                start: false,
                            },
                        );
                        self.selected_id = Some(self.current_id);
                        self.aptext.clear();
                        self.current_id += 1;
                        response.mark_changed();
                    }
                }
            } else {
                if response.drag_started_by(PointerButton::Primary)
                    || response.drag_stopped_by(PointerButton::Primary)
                {
                    self.start_drag = None;
                }
            }

            let shapes = ctx.fonts(|f| {
                let mut shapes = vec![];
                for state in self.states.values() {
                    let stroke_color = if self.selected_id == Some(state.id) {
                        Color32::from_rgb(0, 127, 255)
                    } else {
                        color
                    };

                    if state.start {
                        shapes.push(Shape::circle_filled(
                            state.pos,
                            20.0,
                            stroke_color.clone().gamma_multiply(0.25),
                        ));
                    }
                    shapes.push(Shape::circle_stroke(
                        state.pos,
                        20.0,
                        Stroke::new(4.0, stroke_color),
                    ));
                    shapes.push(Shape::text(
                        f,
                        state.pos - Vec2 { x: 25.0, y: 0.0 },
                        Align2::RIGHT_CENTER,
                        state.aps.join("\n"),
                        FontId::monospace(12.0),
                        color,
                    ));
                }
                shapes
            });

            painter.extend(shapes);

            painter.extend(self.transitions.iter().flat_map(|(id1, id2)| {
                let state1 = self.states.get(id1).unwrap();
                let state2 = self.states.get(id2).unwrap();

                return if id1 == id2 {
                    let dir1 = vec2(1.0, -3.0).normalized();
                    let dir2 = vec2(-dir1.x, dir1.y);
                    let dir = -dir2;
                    let cross_dir = dir.rot90();
                    let arrow_basepoint = state1.pos + dir2 * (STATE_RADIUS + ARROWHEAD_LENGTH);
                    [
                        Shape::CubicBezier(CubicBezierShape::from_points_stroke(
                            [
                                state1.pos + dir1 * STATE_RADIUS,
                                state1.pos + dir1 * STATE_RADIUS * 3.0,
                                state1.pos + dir2 * STATE_RADIUS * 3.0,
                                state1.pos + dir2 * STATE_RADIUS,
                            ],
                            false,
                            Color32::TRANSPARENT,
                            Stroke::new(2.0, color),
                        )),
                        Shape::convex_polygon(
                            vec![
                                state1.pos - dir * STATE_RADIUS,
                                arrow_basepoint + cross_dir * ARROWHEAD_HALF_WIDTH,
                                arrow_basepoint - cross_dir * ARROWHEAD_HALF_WIDTH,
                            ],
                            color,
                            Stroke::NONE,
                        ),
                    ]
                } else {
                    let dir = (state2.pos - state1.pos).normalized();
                    let cross_dir = dir.rot90();
                    let arrow_basepoint = state2.pos - dir * (STATE_RADIUS + ARROWHEAD_LENGTH);
                    [
                        Shape::line_segment(
                            [state1.pos + dir * STATE_RADIUS, arrow_basepoint],
                            Stroke::new(2.0, color),
                        ),
                        Shape::convex_polygon(
                            vec![
                                state2.pos - dir * STATE_RADIUS,
                                arrow_basepoint + cross_dir * ARROWHEAD_HALF_WIDTH,
                                arrow_basepoint - cross_dir * ARROWHEAD_HALF_WIDTH,
                            ],
                            color,
                            Stroke::NONE,
                        ),
                    ]
                };
            }));

            response
        });
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.my_update(ctx, _frame, ui);
        });
    }
}
