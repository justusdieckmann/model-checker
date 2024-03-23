#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::collections::HashMap;
use model_checker;

use eframe::egui;
use eframe::egui::{Align2, Color32, FontDefinitions, FontId, Frame, PointerButton, Sense, Shape};
use eframe::emath::{Pos2, Vec2};
use eframe::epaint::{Stroke, TextShape};

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
            Box::<MyApp>::default()
        }),
    )

    /*let formula = parsing::parse("a U !((Xb) & c)").expect("Got no result");

    let buechi = buechi::ltl_to_buechi::ltl_to_büchi(&formula);

    dbg!(buechi);*/
}

const STATE_RADIUS: f32 = 20.0;
const ARROWHEAD_HALF_WIDTH: f32 = 6.0;
const ARROWHEAD_LENGTH: f32 = 12.0;

struct MyState {
    aps: Vec<String>,
    pos: Pos2,
    id: u64,
}

struct MyApp {
    states: HashMap<u64, MyState>,
    transitions: Vec<(u64, u64)>,
    current_id: u64,
    start_drag: Option<u64>,
    selected_id: Option<u64>,
    query: String,
    aptext: String,
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
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                let name_label = ui.label("Query: ");
                ui.text_edit_singleline(&mut self.query)
                    .labelled_by(name_label.id);
                let button = ui.button("Check");
                if button.clicked() {
                    // ...
                }
            });
            ui.horizontal(|ui| {
                let name_label = ui.label("APs: ");
                ui.text_edit_singleline(&mut self.aptext)
                    .labelled_by(name_label.id);
                let button = ui.button("Set");
                if button.clicked() {
                    if let Some(selected_id) = self.selected_id {
                        let state = self.states.get_mut(&selected_id).unwrap();
                        state.aps = self.aptext.split(",").filter(|s| {!s.trim().is_empty()})
                            .map(|a| {
                            a.trim().to_owned()
                        }).collect();
                    }
                }
            });

            Frame::canvas(ui.style()).show(ui, |ui| {
                let color = if ctx.style().visuals.dark_mode {
                    Color32::from_gray(200)
                } else {
                    Color32::from_gray(25)
                };


                let (mut response, painter) =
                    ui.allocate_painter(ui.available_size_before_wrap(), Sense::click_and_drag());

                if let Some(pointer_pos) = response.interact_pointer_pos() {
                    let canvas_pos = pointer_pos;

                    let result = self.states.values().find(|state| {
                        state.pos.distance(canvas_pos) <= STATE_RADIUS
                    });
                    if let Some(hit_state) = result {
                        if response.drag_started_by(PointerButton::Primary) {
                            self.start_drag = Some(hit_state.id);
                        } else if response.drag_released_by(PointerButton::Primary) {
                            if let Some(start_drag) = self.start_drag {
                                if !self.transitions.contains(&(start_drag, hit_state.id)) {
                                    self.transitions.push((start_drag, hit_state.id));
                                    response.mark_changed();
                                }
                            }
                        } else if response.clicked() {
                            self.selected_id = Option::from(hit_state.id);
                            response.mark_changed();
                        }
                    } else {
                        if response.clicked() {
                            self.states.insert(self.current_id, MyState {
                                aps: vec![],
                                pos: canvas_pos,
                                id: self.current_id,
                            });
                            self.selected_id = Some(self.current_id);
                            self.current_id += 1;
                            response.mark_changed();
                        }
                    }
                } else {
                    if response.drag_started_by(PointerButton::Primary) || response.drag_released_by(PointerButton::Primary) {
                        self.start_drag = None;
                    }
                }

                let shapes = self.states.values()
                    .flat_map(|state| {
                        let stroke_color = if self.selected_id == Some(state.id) {
                            Color32::from_rgb(0, 127, 255)
                        } else {
                            color
                        };
                        let shape = ctx.fonts(|f| {
                            Shape::text(f, state.pos - Vec2 { x: 120.0, y: 0.0 }, Align2::RIGHT_CENTER, "", FontId::monospace(12.0), color)
                        });
                        [
                            Shape::circle_stroke(state.pos, 20.0, Stroke::new(4.0, stroke_color)),
                            shape
                        ]
                    });

                painter.extend(shapes);

                painter.extend(self.transitions.iter()
                    .flat_map(|(id1, id2)| {
                        let state1 = self.states.get(id1).unwrap();
                        let state2 = self.states.get(id2).unwrap();
                        let dir = (state2.pos - state1.pos).normalized();
                        let cross_dir = dir.rot90();
                        let arrow_basepoint = state2.pos - dir * (STATE_RADIUS + ARROWHEAD_LENGTH);
                        return [
                            Shape::convex_polygon(vec![
                                state2.pos - dir * STATE_RADIUS,
                                arrow_basepoint + cross_dir * ARROWHEAD_HALF_WIDTH,
                                arrow_basepoint - cross_dir * ARROWHEAD_HALF_WIDTH,
                            ], color, Stroke::NONE),
                            Shape::line_segment([state1.pos + dir * STATE_RADIUS, arrow_basepoint], Stroke::new(2.0, color)),
                        ];
                    }));

                response
            });
        });
    }
}
