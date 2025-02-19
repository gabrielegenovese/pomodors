// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
// #![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui::{Response, Ui};
use eframe::{egui, NativeOptions};
use notify_rust::Notification;
use std::time::Instant;
use std::{thread, time::Duration};

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
    start: std::time::Instant,
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
    if sec_left >= 10 {
        ui.heading(format!("{}:{}", min, sec_left))
    } else {
        ui.heading(format!("{}:0{}", min, sec_left))
    }
}

impl eframe::App for TimerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::Window::new("Pomodors")
            .title_bar(true)
            .show(ctx, |ui| {
                if ui.button("▶").clicked() {
                    match self.status {
                        PomodoStatus::STUDY => self.status = PomodoStatus::PAUSEST,
                        PomodoStatus::WAITING => {
                            self.start = Instant::now();
                            self.notified = false;
                            self.tot_bt_left = BREAKTIME;
                            self.status = PomodoStatus::BREAK
                        }
                        PomodoStatus::BREAK => self.status = PomodoStatus::PAUSEBT,
                        PomodoStatus::PAUSEST => self.status = PomodoStatus::STUDY,
                        PomodoStatus::PAUSEBT => self.status = PomodoStatus::BREAK,
                        PomodoStatus::INIT => {
                            self.start = Instant::now();
                            self.notified = false;
                            self.tot_st_left = POMTIME;
                            self.status = PomodoStatus::STUDY
                        }
                    }
                }

                match self.status {
                    PomodoStatus::INIT => ui.heading("Study time! 25:00"),
                    PomodoStatus::WAITING => ui.heading("Break time! 5:00"),
                    PomodoStatus::STUDY => show_timer(ui, self.tot_st_left),
                    PomodoStatus::PAUSEST => show_timer(ui, self.tot_st_left),
                    PomodoStatus::PAUSEBT => show_timer(ui, self.tot_bt_left),
                    PomodoStatus::BREAK => show_timer(ui, self.tot_bt_left),
                }
            });

        match self.status {
            PomodoStatus::STUDY => {
                if self.tot_st_left > 0 {
                    let new_now = Instant::now();
                    let elapsed = new_now.checked_duration_since(self.start).unwrap();
                    match elapsed.checked_sub(Duration::from_secs(1)) {
                        Some(_) => {
                            self.start = Instant::now();
                            self.tot_st_left -= 1
                        }
                        None => {}
                    }
                } else {
                    self.status = PomodoStatus::WAITING;
                    if !self.notified {
                        Notification::new()
                            .summary("Time's up!")
                            .body("Study time is over! Do a 5 minutes break :)")
                            .show()
                            .unwrap();
                        self.notified = true;
                    }
                }
            }
            PomodoStatus::WAITING => {}
            PomodoStatus::BREAK => {
                if self.tot_bt_left > 0 {
                    let new_now = Instant::now();
                    let elapsed = new_now.checked_duration_since(self.start).unwrap();
                    match elapsed.checked_sub(Duration::from_secs(1)) {
                        Some(_) => {
                            self.start = Instant::now();
                            self.tot_bt_left -= 1
                        }
                        None => {}
                    }
                } else {
                    self.status = PomodoStatus::INIT;
                    if !self.notified {
                        Notification::new()
                            .summary("Break's up!")
                            .body("Break time is over! Restart the pomodoro if you want :)")
                            .show()
                            .unwrap();
                        self.notified = true;
                    }
                }
            }
            PomodoStatus::INIT => {}
            PomodoStatus::PAUSEST => {}
            PomodoStatus::PAUSEBT => {}
        }

        ctx.request_repaint();
    }
}

fn main() {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
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
