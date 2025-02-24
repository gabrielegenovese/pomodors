use eframe::egui::{Layout, Response, Ui};
use eframe::{egui, NativeOptions};
use notify_rust::Notification;
use std::time::Instant;

const POMTIME: i32 = 25 * 60;
const BREAKTIME: i32 = 5 * 60;

enum PomodoStatus {
    INIT,
    STUDY,
    PAUSEST,
    PAUSEBT,
    WAITING,
    BREAK,
}

struct TimerApp {
    status: PomodoStatus,
    start: Instant,
    notified: bool,
    tot_st_left: i32,
    tot_bt_left: i32,
}

impl Default for TimerApp {
    fn default() -> Self {
        Self {
            status: PomodoStatus::INIT,
            start: Instant::now(),
            notified: false,
            tot_st_left: POMTIME,
            tot_bt_left: BREAKTIME,
        }
    }
}

fn show_timer(ui: &mut Ui, timer: i32) -> Response {
    let min = timer / 60;
    let sec_left = timer % 60;
    ui.label(format!("{:02}:{:02}", min, sec_left))
}

impl eframe::App for TimerApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        // egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        //     ui.horizontal(|ui| {
        //         ui.label("Pomodoro Timer");
        //         ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
        //             if ui.button("❌").clicked() {
        //                 ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
        //             }
        //         });
        //     });
        // });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::left_to_right(egui::Align::Center), |ui| {
                if ui.button("▶").clicked() {
                    match self.status {
                        PomodoStatus::STUDY => self.status = PomodoStatus::PAUSEST,
                        PomodoStatus::WAITING => {
                            self.start = Instant::now();
                            self.notified = false;
                            self.tot_bt_left = BREAKTIME;
                            self.status = PomodoStatus::BREAK;
                        }
                        PomodoStatus::BREAK => self.status = PomodoStatus::PAUSEBT,
                        PomodoStatus::PAUSEST => self.status = PomodoStatus::STUDY,
                        PomodoStatus::PAUSEBT => self.status = PomodoStatus::BREAK,
                        PomodoStatus::INIT => {
                            self.start = Instant::now();
                            self.notified = false;
                            self.tot_st_left = POMTIME;
                            self.status = PomodoStatus::STUDY;
                        }
                    }
                }
                match self.status {
                    PomodoStatus::INIT => ui.label("Study time! 25:00"),
                    PomodoStatus::WAITING => ui.label("Break time! 5:00"),
                    PomodoStatus::STUDY | PomodoStatus::PAUSEST => show_timer(ui, self.tot_st_left),
                    PomodoStatus::PAUSEBT | PomodoStatus::BREAK => show_timer(ui, self.tot_bt_left),
                };
            });
        });

        match self.status {
            PomodoStatus::STUDY => {
                if self.tot_st_left > 0 {
                    let elapsed = Instant::now().duration_since(self.start);
                    if elapsed.as_secs() >= 1 {
                        self.start = Instant::now();
                        self.tot_st_left -= 1;
                    }
                } else {
                    self.status = PomodoStatus::WAITING;
                    if !self.notified {
                        Notification::new()
                            .summary("Time's up!")
                            .body("Study time is over! Do a 5-minute break.")
                            .show()
                            .unwrap();
                        self.notified = true;
                    }
                }
            }
            PomodoStatus::BREAK => {
                if self.tot_bt_left > 0 {
                    let elapsed = Instant::now().duration_since(self.start);
                    if elapsed.as_secs() >= 1 {
                        self.start = Instant::now();
                        self.tot_bt_left -= 1;
                    }
                } else {
                    self.status = PomodoStatus::INIT;
                    if !self.notified {
                        Notification::new()
                            .summary("Break's up!")
                            .body("Break time is over! Restart the pomodoro if you want.")
                            .show()
                            .unwrap();
                        self.notified = true;
                    }
                }
            }
            _ => {}
        }
        ctx.request_repaint();
    }
}

fn main() {
    env_logger::init();
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([250.0, 120.0]),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "Pomodors",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(TimerApp::default()))
        }),
    );
}
