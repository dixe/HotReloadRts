use crate::entity_system::EntityId;
use gl_lib::{gl, sdl2::event, na, camera, widget_gui::*, helpers, widget_gui::widgets::*, widget_gui::layout::*};


pub struct Ui {
    pub info: UiInfo,
    pub state: UiState,
    pub widget_setup: helpers::WidgetSetup
}

pub fn create_ui() -> (UiInfo, UiState) {

    let mut ui_state = UiState::new();
    let row = RowWidget {};


    let sp_widget = SelectionPanelWidget::new();

    let sp_id = ui_state.add_widget(Box::new(sp_widget), None);

    // Add dispatcher for add button
    //ui_state.set_widget_dispatcher(button_id, Box::new(button_dispatcher));


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

    fn layout(&mut self, bc: &BoxContraint, _children: &[Id], ctx: &mut LayoutContext) -> LayoutResult {
        LayoutResult::Size(Size {
            pixel_w: 300,
            pixel_h: 100
        })
    }

    fn render(&self, geom: &Geometry, ctx: &mut render::RenderContext) {
        render::render_rect(geom, ctx);
    }

    fn dispatcher(&self) -> Dispatcher {
        Box::new(selection_panel_dispatcher)
    }

    fn handle_sdl_event(&mut self, event: &event::Event, geom: &Geometry, self_id: Id, queue: &mut DispatcherQueue) {

        println!("Selection got event {:?}", event);

    }

}



fn selection_panel_dispatcher(event: &event::Event, self_id: Id, queue: &mut DispatcherQueue) {
    use event::Event::*;
    match event {
        MouseButtonDown {..} => {
            queue.push_back(DispatcherEvent { target_id: self_id, event: Box::new(())});
        },
        _ => {}
    };
}
