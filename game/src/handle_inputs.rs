use gl_lib::controller;
use nalgebra::vector;
use gl_lib::na;
use gl_lib::sdl2::{self, keyboard::Keycode};
use crate::game::*;
use crate::commands::*;
use crate::math::V3;

pub fn handle_inputs(game: &mut Game, event_pump: &mut gl_lib::sdl2::EventPump) {
    let kb_map = setup_keyboard_mapping();

    // Reset per frame results
    // maybe don't do this here, but in a general per frame reset function
    game.state.command = Command::Empty;

    let mouse_state = event_pump.mouse_state();
    game.state.mouse_pos.x = mouse_state.x() as f32;
    game.state.mouse_pos.y = mouse_state.y() as f32;


    for event in event_pump.poll_iter() {
        game.camera_controller.update_events(event.clone());
        controller::on_input(event.clone(), &kb_map, game);


        if event.is_keyboard() {
            handle_keyboard(&event, game);
        }


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


fn handle_keyboard(event: &sdl2::event::Event, game: &mut Game) {


    use sdl2::event::Event::*;
    use sdl2::keyboard::Keycode::*;

    match event.clone() {
        KeyDown{keycode: Some(kc), .. } => {
             match kc {
                 S => {
                     game.state.command = Command::Stop
                 },
                 A => {
                     // should set playState to ApplyCommand attack. But we cannot use command::Attack, since that requires a target?
                     // maybe just set target to 0,0,-1??
                     //game.play_state = ApplyCommand(Command::
                 },
                 H => {
                     game.state.action = Action::Spell;
                 }
                 _ =>{}
            }
        },
        _ => {}
    }


}

fn handle_click(event: &sdl2::event::Event, game: &mut Game) {

    use sdl2::event::Event::*;
    match event.clone() {
        MouseButtonDown{mouse_btn, x, y, ..} => {
            if mouse_btn == sdl2::mouse::MouseButton::Right {

                // Do a raycast to grund plane, to see where our click is in world space
                let ray_info = RayInfo {
                    p0: game.camera.pos(),
                    v: game.camera.screen_to_ray(x as f32, (700 - y) as f32)
                };

                // maybe return RayInfo from camera??
                let hit_p = raycast_ground_plane(ray_info);
                // Update selected pos to be, lift if above the ground pplane by a little bit to avoid clipping
                game.state.select_pos = hit_p + vector![0.0, 0.0, 0.1];
                let target = Target::Position(hit_p.x, hit_p.y);
                game.state.command = Command::DefaultRightClick(target);

            }


            if mouse_btn == sdl2::mouse::MouseButton::Middle {
                game.state.command = Command::Stop;
            }

        },
        _ => {}
    }
}

#[derive(Debug, Clone, Copy)]
struct RayInfo {
    p0: V3,
    v: V3,
}

// logic from here https://www.cs.princeton.edu/courses/archive/fall00/cs426/lectures/raycast/sld017.htm
fn raycast_ground_plane(RayInfo { p0, v }: RayInfo) -> V3 {

    let norm = na::Vector3::new(0.0, 0.0, 1.0);
    let plane_p = na::Vector3::new(1.0, 0.0, 0.0);

    let d =  -plane_p.dot(&norm);
    let t = -(p0.dot(&norm) + d) / (v.dot(&norm));
    p0 + t * v
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
                    let single_click = sb.min_x() == sb.max_x() && sb.min_y() == sb.max_y();

                    game.tmp_buffer.clear();

                    sb.current.x = x;
                    sb.current.y = y;

                    let count = game.state.entities.positions.len();

                    for i in 0..count {
                        if game.state.entities.team[i] != 1 {
                            continue;
                        }
                        let sp = game.camera.world_pos_to_screen(game.state.entities.positions[i]);

                        let screen_pos_i = ScreenPos {x: sp.x as i32, y: sp.y as i32};


                        let radius = 30;

                        if screen_pos_i.x >= sb.min_x() - radius &&
                            screen_pos_i.x <= sb.max_x() + radius &&
                            screen_pos_i.y >= sb.min_y() - radius &&
                            screen_pos_i.y <= sb.max_y() + radius {
                                game.tmp_buffer.push(i);

                            }

                        if single_click && game.tmp_buffer.len() > 0 {
                            // find the closest to click
                            let mut cur_min_d = 10000.0;
                            let mut cur_index = 0;
                            for &index in &game.tmp_buffer {
                                let d = (sp - na::Vector2::new(sb.min_x() as f32, sb.min_y() as f32)).magnitude();
                                if d < cur_min_d {
                                    cur_index = index;
                                    cur_min_d = d
                                }
                            }

                            game.tmp_buffer.clear();
                            game.tmp_buffer.push(cur_index);

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
