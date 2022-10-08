use gl_lib::controller;
use nalgebra::vector;
use gl_lib::na;
use gl_lib::sdl2::{self, keyboard::Keycode};
use crate::game::*;
use crate::commands::*;


pub fn handle_inputs(game: &mut Game, event_pump: &mut gl_lib::sdl2::EventPump) {
    let kb_map = setup_keyboard_mapping();

    // Reset per frame results
    // maybe don't do this here, but in a general per frame reset function
    game.state.command = Command::Empty;

    let mouse_state = event_pump.mouse_state();
    game.state.mouse_pos.x = mouse_state.x() as f32;
    game.state.mouse_pos.y = mouse_state.y() as f32;

    println!("mp_ input {:?}", game.state.mouse_pos);

    for event in event_pump.poll_iter() {
        game.camera_controller.update_events(event.clone());
        controller::on_input(event.clone(), &kb_map, game);

        if event.is_mouse() {
            match game.play_state {
                PlayState::General => {
                    handle_click(&event, game);
                    handle_select_box(&event, game);
                },
                PlayState::ApplyCommand(_command) => {
                    // if left click apply the command to the target
                    // if right, reset PlayState to general

                }
            }

        }
    }
}


fn handle_click(event: &sdl2::event::Event, game: &mut Game) {

    use sdl2::event::Event::*;
    match event.clone() {
        MouseButtonDown{mouse_btn, x, y, ..} => {
            if mouse_btn == sdl2::mouse::MouseButton::Right {

                // Do a raycast to grund plane, to see where our click is in world space
                // logic from here https://www.cs.princeton.edu/courses/archive/fall00/cs426/lectures/raycast/sld017.htm

                let norm = na::Vector3::new(0.0, 0.0, 1.0);
                let p0 =  game.camera.pos();

                // our y 0 is top of screen, camera assumes y=0 is bottom
                let v = game.camera.screen_to_ray(x as f32, (700 - y) as f32);


                let plane_p = na::Vector3::new(1.0, 0.0, 0.0);
                let d =  -plane_p.dot(&norm);
                let t = -(p0.dot(&norm) + d) / (v.dot(&norm));
                let hit_p = p0 + t * v;


                // Update selected pos to be, lift if above the ground pplane by a little bit to avoid clipping
                game.state.select_pos = hit_p + vector![0.0, 0.0, 0.01];

                let target = Target::Position(hit_p.x, hit_p.y);
                game.state.command = Command::DefaultRightClick(target);

            }
        },
        _ => {}
    }
}


fn handle_select_box(event: &sdl2::event::Event, game: &mut Game) {
    use sdl2::event::Event::*;
    match event.clone() {
        MouseButtonDown{mouse_btn, x, y, ..} => {
            if game.state.select_box.is_none() && mouse_btn == sdl2::mouse::MouseButton::Left {
                game.state.select_box = Some(SelectBox {
                    start: ScreenPos { x,y },
                    current: ScreenPos { x,y },
                });
            }

            if mouse_btn ==  sdl2::mouse::MouseButton::Right {
                // default action, units is move, unless an enemy is clicked, then attack

            }
        },
        MouseMotion{x, y, .. } => {
            if let Some(sb) = &mut game.state.select_box {
                sb.current.x = x;
                sb.current.y = y;
            }
        },
        MouseButtonUp{mouse_btn, x, y, ..} => {
            if mouse_btn == sdl2::mouse::MouseButton::Left {

                if let Some(sb) = &mut game.state.select_box{
                    game.tmp_buffer.clear();

                    sb.current.x = x;
                    sb.current.y = y;

                    let count = game.state.positions.len();

                    for i in 0..count {
                        let sp = game.camera.world_pos_to_screen(game.state.positions[i]);
                        let screen_pos_i = ScreenPos {x: sp.x as i32, y: sp.y as i32};



                        if screen_pos_i.x >= sb.min_x() &&
                            screen_pos_i.x <= sb.max_x() &&
                            screen_pos_i.y >= sb.min_y() &&
                            screen_pos_i.y <= sb.max_y() {
                                game.tmp_buffer.push(i);
                            }
                    }
                }

                // only overwrite selection when something new is selected
                if game.tmp_buffer.len() > 0 {
                    game.state.selected.clear();
                    for &entity in &game.tmp_buffer {
                        game.state.selected.push(entity);
                    }
                }

                game.state.select_box = None;
            }

        },
        _ => {}
    }
}


fn setup_keyboard_mapping() -> controller::ControllerMapping<Game> {
    let mut kb_map = controller::ControllerMapping::new();

    use Keycode::*;
    kb_map.exit(Keycode::Escape);
    kb_map.add_on_press(R, reload_assets);
    kb_map.add_on_press(Q, reset);

    kb_map
}
