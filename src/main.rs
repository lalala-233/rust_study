use eframe::{
    egui::{Button, CentralPanel, Context, UserAttentionType},
    CreationContext, NativeOptions,
};

use std::time::{Duration, SystemTime};

fn main() -> eframe::Result<()> {
    let native_options = NativeOptions::default();
    eframe::run_native(
        "User attention test",
        native_options,
        Box::new(|cc| Box::new(Application::new(cc))),
    )
}

fn to_string(attention: UserAttentionType) -> String {
    format!("{attention:?}")
}

struct Application {
    attention_type: UserAttentionType,
    time_of_attention: Option<SystemTime>,
    should_auto_reset: bool,
    time_of_reset: Option<SystemTime>,
}

impl Application {
    fn new(_cc: &CreationContext<'_>) -> Self {
        Self {
            attention_type: UserAttentionType::Informational,
            time_of_attention: None,
            should_auto_reset: false,
            time_of_reset: None,
        }
    }
    fn attention_reset_timeout() -> Duration {
        Duration::from_secs(1)
    }

    fn attention_request_timeout() -> Duration {
        Duration::from_secs(1)
    }

    fn repaint_max_timeout() -> Duration {
        Duration::from_secs(1)
    }
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        if let Some(request_at) = self.time_of_attention {
            if request_at < SystemTime::now() {
                self.time_of_attention = None;
                frame.request_user_attention(self.attention_type);
                if self.should_auto_reset {
                    self.should_auto_reset = false;
                    self.time_of_reset = Some(SystemTime::now() + Self::attention_reset_timeout());
                }
            }
        }

        if let Some(reset_at) = self.time_of_reset {
            if reset_at < SystemTime::now() {
                self.time_of_reset = None;
                frame.request_user_attention(UserAttentionType::Reset);
            }
        }

        CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Attention type:");
                    eframe::egui::ComboBox::new("attention", "")
                        .selected_text(to_string(self.attention_type))
                        .show_ui(ui, |ui| {
                            for kind in [
                                UserAttentionType::Informational,
                                UserAttentionType::Critical,
                            ] {
                                ui.selectable_value(
                                    &mut self.attention_type,
                                    kind,
                                    to_string(kind),
                                );
                            }
                        })
                });

                let button_enabled =
                    self.time_of_attention.is_none() && self.time_of_reset.is_none();
                let button_text = if button_enabled {
                    format!(
                        "Request in {} seconds",
                        Self::attention_request_timeout().as_secs()
                    )
                } else {
                    match self.time_of_reset {
                        None => "Unfocus the window, fast!".to_owned(),
                        Some(t) => {
                            if let Ok(elapsed) = t.duration_since(SystemTime::now()) {
                                format!("Resetting attention in {} s...", elapsed.as_secs())
                            } else {
                                "Resetting attention...".to_owned()
                            }
                        }
                    }
                };

                let resp = ui
                    .add_enabled(button_enabled, Button::new(button_text))
                    .on_hover_text_at_pointer(
                        "After clicking, unfocus the application's window to see the effect",
                    );
                ui.checkbox(
                    &mut self.should_auto_reset,
                    format!(
                        "Reset after {} seconds",
                        Self::attention_reset_timeout().as_secs()
                    ),
                );
                eprintln!("{}", self.should_auto_reset);

                if resp.clicked() {
                    self.time_of_attention =
                        Some(SystemTime::now() + Self::attention_request_timeout());
                }
            });
        });

        ctx.request_repaint_after(Self::repaint_max_timeout());
    }
}
