use airinfo::{find_pods, Pod};
use eframe::egui;

struct MyPods {
    left_battery: f32,
    right_battery: f32,
}

impl MyPods {
    fn new()  -> Self {
        Self {
            left_battery: 0.0,
            right_battery: 0.0,
        }
    }
}

fn load_icon() -> eframe::IconData {
    eframe::IconData::try_from_png_bytes(&include_bytes!("../assets/icon.png")[..]).unwrap()
}

#[tokio::main]
async fn get_pods_info() -> Option<Vec<Pod>> {
    let pods = find_pods().await.unwrap();
    if pods.len() > 0 {
        Some(pods)
    } else {
        None
    }
}

fn main() {
    let mut my_pods: MyPods = MyPods::new();

    let mut title = String::from("Airpods battery checker v");
    title.push_str(env!("CARGO_PKG_VERSION"));
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(500.0, 90.0)),
        icon_data: Some(load_icon()),
        ..Default::default()
    };
    
    eframe::run_simple_native(&title, options, move |ctx, _frame| {
        egui::CentralPanel::default().show(&ctx, |ui| {
            if ui.button("🔌 Check battery").clicked() {
                if let Some(pods) = get_pods_info() {
                    let pod = pods.first().unwrap();
                    if pod.left.is_some() {
                        my_pods.left_battery = pod.clone().left.unwrap().battery as f32 / 100.0;
                    }
                    if pod.right.is_some() {
                        my_pods.right_battery = pod.clone().right.unwrap().battery as f32 / 100.0;
                    }
                }
                
            }

            ui.horizontal(|ui| {
                ui.label("Left battery:   ");
                ui.add(egui::ProgressBar::new(my_pods.left_battery).show_percentage());
            });

            ui.horizontal(|ui| {
                ui.label("Right battery: ");
                ui.add(egui::ProgressBar::new(my_pods.right_battery).show_percentage());
            });
        });
    }).unwrap();

}