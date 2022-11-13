use crate::entity_system::EntityId;
use gl_lib::{gl, sdl2::{self, event}, na, camera, widget_gui::*, helpers, widget_gui::widgets::*, widget_gui::layout::*};
use std::any::Any;
use crate::types::*;


pub struct Ui {
    pub info: UiInfo,
    pub state: UiState,
    pub widget_setup: helpers::WidgetSetup
}

pub fn create_ui() -> (UiInfo, UiState) {

    let mut ui_state = UiState::new();

    let row = RowWidget {};
    let col = ColumnWidget { };

    let col_id = ui_state.add_widget(Box::new(col), None);
    let row_id = ui_state.add_widget(Box::new(row), Some(col_id));


    let sp_widget = SelectionPanelWidget::new();

    let sp_id = ui_state.add_widget(Box::new(sp_widget), Some(row_id));
    ui_state.set_alignment_x(sp_id, AlignmentX::Center);
    ui_state.set_alignment_y(row_id, AlignmentY::Bottom);

    (UiInfo {selection_id: sp_id }, ui_state)
}


#[derive(Debug, Clone, Copy)]
pub struct UiInfo {
    pub selection_id: Id
}


#[derive(Debug, Clone)]
pub struct SelectionPanelWidget {

    pub entity_ids: Vec::<EntityId>
}


impl SelectionPanelWidget {
    pub fn new() -> Self {
        SelectionPanelWidget {
            entity_ids: vec![]
        }
    }
}

impl Widget for SelectionPanelWidget {

    fn layout(&mut self, _bc: &BoxContraint, _children: &[Id], ctx: &mut LayoutContext) -> LayoutResult {
        LayoutResult::Size(Size {
            pixel_w: 300,
            pixel_h: 100
        })
    }

    fn render(&self, geom: &Geometry, ctx: &mut render::RenderContext) {

        let text = &format!("{}", self.entity_ids.len());


        render::render_text(&text, 1.0, geom, ctx);
        render::render_rect(geom, ctx);
    }


    fn handle_widget_input(&mut self, input: Box::<dyn Any>) {

        self.entity_ids.clear();
        if let Ok(info) = input.downcast::<SliceInfo::<usize>>() {
            let slice;
            unsafe {
                slice = std::slice::from_raw_parts(info. pointer, info.len);
            }

            for id in slice {
                self.entity_ids.push(*id);
            }
        }
    }

    fn handle_sdl_event(&mut self,
                        event: &event::Event,
                        geom: &Geometry,
                        _self_id: Id,
                        _widget_output_queue: &mut WidgetOutputQueue) {


        use sdl2::event::Event::*;
        match event.clone() {
            MouseButtonDown{mouse_btn, x, y, ..} => {
                if mouse_btn == sdl2::mouse::MouseButton::Left {
                    let relative_x = x - geom.pos.x;
                    let relative_y = y - geom.pos.y;
                    println!("pressed at: {:?}", (relative_x, relative_y));
                }
            },
            _ => {}
        }

    }
}
