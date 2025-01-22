use egui::emath::TSTransform;
use egui::scene::{fit_to_rect_in_scene, Scene};
use egui::{Pos2, Rect, Sense, TextWrapMode, UiBuilder, Vec2};

#[derive(Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct SceneDemo {
    transform: Option<TSTransform>,
    drag_value: f32,
    scene_rect: Option<Rect>,
}

impl Eq for SceneDemo {}

impl crate::Demo for SceneDemo {
    fn name(&self) -> &'static str {
        "🔍 Scene"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        use crate::View as _;
        let window = egui::Window::new("Pan Zoom")
            .default_width(300.0)
            .default_height(300.0)
            .vscroll(false)
            .open(open);
        window.show(ctx, |ui| self.ui(ui));
    }
}

impl crate::View for SceneDemo {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label(
            "Pan, zoom in, and zoom out with scrolling (see the plot demo for more instructions). \
                   Double click on the background to reset.",
        );
        ui.vertical_centered(|ui| {
            ui.add(crate::egui_github_link_file!());
        });
        ui.separator();

        let (_, rect) = ui.allocate_space(ui.available_size());

        // TODO: Actually write that back
        let mut state = self.clone();

        let to_global = self.transform.get_or_insert_with(|| {
            fit_to_rect_in_scene(
                rect,
                Rect::from_min_max(Pos2::new(0., 0.), Pos2::new(120., 120.)),
            )
        });

        let scene = Scene::new();
        scene.show_scene(ui, to_global, |ui| {
            for (i, (pos, callback)) in [
                (
                    egui::Pos2::new(0.0, 0.0),
                    Box::new(|ui: &mut egui::Ui, _: &mut Self| {
                        ui.button("top left").on_hover_text("Normal tooltip")
                    })
                        as Box<dyn Fn(&mut egui::Ui, &mut Self) -> egui::Response>,
                ),
                (
                    egui::Pos2::new(0.0, 120.0),
                    Box::new(|ui: &mut egui::Ui, _| {
                        ui.button("bottom left").on_hover_text("Normal tooltip")
                    }),
                ),
                (
                    egui::Pos2::new(120.0, 120.0),
                    Box::new(|ui: &mut egui::Ui, _| {
                        ui.button("right bottom")
                            .on_hover_text_at_pointer("Tooltip at pointer")
                    }),
                ),
                (
                    egui::Pos2::new(120.0, 0.0),
                    Box::new(|ui: &mut egui::Ui, _| {
                        ui.button("right top")
                            .on_hover_text_at_pointer("Tooltip at pointer")
                    }),
                ),
                (
                    egui::Pos2::new(60.0, 60.0),
                    Box::new(|ui, state| {
                        use egui::epaint::{
                            pos2, CircleShape, Color32, QuadraticBezierShape, Stroke,
                        };
                        // Smiley face.
                        let painter = ui.painter();
                        painter.add(CircleShape::filled(pos2(0.0, -10.0), 1.0, Color32::YELLOW));
                        painter.add(CircleShape::filled(pos2(10.0, -10.0), 1.0, Color32::YELLOW));
                        painter.add(QuadraticBezierShape::from_points_stroke(
                            [pos2(0.0, 0.0), pos2(5.0, 3.0), pos2(10.0, 0.0)],
                            false,
                            Color32::TRANSPARENT,
                            Stroke::new(1.0, Color32::YELLOW),
                        ));

                        ui.add(
                            egui::Slider::new(&mut state.drag_value, 0.0..=100.0).text("My value"),
                        )
                    }),
                ),
            ]
            .into_iter()
            .enumerate()
            {
                let builder = UiBuilder::new()
                    .max_rect(Rect::from_center_size(pos, Vec2::new(200., 200.)))
                    .sense(Sense::click());

                let mut content_ui = ui.new_child(builder);
                let content_resp = callback(&mut content_ui, &mut state);
                state.scene_rect = Some(
                    state
                        .scene_rect
                        .get_or_insert(Rect::NOTHING)
                        .union(content_resp.rect),
                );
            }
        });

        // scene.register_pan_and_zoom(ui, &resp, &mut self.transform);
    }
}
