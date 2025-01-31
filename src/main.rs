#![windows_subsystem = "windows"]

use std::fs;
use std::path::PathBuf;

use eframe::egui;
use rfd::FileDialog;

struct NarcFixerApp {
    narc_path: Option<PathBuf>,
    status: String,
}

impl Default for NarcFixerApp {
    fn default() -> Self {
        Self {
            narc_path: None,
            status: "No file loaded".to_string(),
        }
    }
}

impl eframe::App for NarcFixerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(&self.status);
                if ui.button("Import NARC").clicked() {
                    if let Some(path) = FileDialog::new().add_filter("NARC Files", &["narc"]).pick_file() {
                        self.narc_path = Some(path.clone());
                        self.status = format!("Loaded: {:?}", path);
                    }
                }
                if let Some(ref path) = self.narc_path {
                    if ui.button("Fix and Export").clicked() {
                        match fix_narc_file(path) {
                            Ok(new_path) => {
                                self.status = format!("Fixed and saved: {:?}", new_path);
                            }
                            Err(e) => {
                                self.status = format!("Error: {}", e);
                            }
                        }
                    }
                }
            });

            ui.separator();

            ui.vertical_centered(|ui| {
                ui.label("This tool ensures that mmodel.narc/1.narc (contains BTX OW sprites) packed with Kiwi.ds can be properly read by Tinke.\n\
                          It only modifies bytes at offset 0x1B1C - 0x1B23 and does not affect gameplay.\n\
                          The modified NARC will be saved as a new file with '_fixed' appended.");
            });
        });
    }
}

fn fix_narc_file(path: &PathBuf) -> std::io::Result<PathBuf> {
    let mut data = fs::read(path)?;
    if data.len() < 0x1B24 {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "File too small"));
    }

    data[0x1B1C..=0x1B23].copy_from_slice(&[0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00]);

    let new_path = path.with_extension("fixed.narc");
    fs::write(&new_path, &data)?;
    Ok(new_path)
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 210.0])
            .with_resizable(false),
        ..Default::default()
    };

    eframe::run_native(
        "NARC BTNF Fixer v0.1 (by hushrom)",
        options,
        Box::new(|_cc| Box::new(NarcFixerApp::default())),
    )
}
