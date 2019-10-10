use cmoose::{FlowCWin,GUIBox,AdvWidgetCycler};
use gmoose;
use lmoose::{Place,Lifeform,Dungeon};
use smoose::*;
use conrod_support::{convert_event,EventLoop};

use conrod;
use glium;
use std::sync::Mutex;

//A function to handle the event loop.
pub fn event_loop_handler<'a,'b>(event_loop: & mut EventLoop,
                          display: &mut glium::Display,
                          mut ui: conrod::Ui,
                          events_loop: &mut  glium::glutin::EventsLoop,
                          pl: & mut (usize,usize),
                          wo: & mut FlowCWin,
                          pause: & mut bool,
                          p_loc: & mut Place,
                          dungeon_pointer: & mut usize,
                          idungeon: & mut Option<usize>,
                          world: & Vec<[Place;19]>,
                          coords: & mut [i32;2],
                          timer:usize,
                          freeze_timer: & mut usize,
                          comm_text: & mut String,
                          gui_box: & mut GUIBox<'a>,
                          gui_box_previous: & mut GUIBox<'a>,
                          widget_cycler: &mut Mutex<AdvWidgetCycler>,
                          party: & mut Vec<(Lifeform,usize)>,
                          enemies: & mut Vec<(Lifeform,usize)>,
                          encounter: & mut Vec<(Lifeform,usize,[Option<[usize;2]>;2])>,
                          battle_ifast: & mut usize,
                          mons: & Vec<Lifeform>,
                          dungeons: & Vec<Dungeon>)
    -> conrod::Ui
{
    let mut ui = ui;
    for mut event in event_loop.next(events_loop) {

        // Use the `winit` backend feature to convert the winit event to a conrod one.
        if let Some(mut event) = convert_event(event.clone(), display) {
            ui.handle_event(event);
            event_loop.needs_update();
        }

        match event {
            glium::glutin::Event::WindowEvent { event, .. } => match event {
                // Break from the loop upon `Escape`.
                glium::glutin::WindowEvent::Closed |
                glium::glutin::WindowEvent::KeyboardInput {
                    input: glium::glutin::KeyboardInput {
                        virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                        ..
                    },
                    ..
                } => {*gui_box = GUIBox::MainQuit(false);},

                //TEST: Adjusts srgb_linear correction.
                glium::glutin::WindowEvent::KeyboardInput {
                    input: glium::glutin::KeyboardInput {
                        virtual_keycode: Some(glium::glutin::VirtualKeyCode::Tab),
                        ..
                    },
                    ..
                } => {if wo.bgc<=0.95 { wo.bgc+=0.05 }else{ wo.bgc = 1.0 };
                    //println!("Pageup pressed,bgc2={}",bgc2);
                    },

                //TEST: Adjusts srgb_linear correction.
                glium::glutin::WindowEvent::KeyboardInput {
                    input: glium::glutin::KeyboardInput {
                        virtual_keycode: Some(glium::glutin::VirtualKeyCode::LShift),
                        ..
                    },
                    ..
                } => {if wo.bgc>=-0.95 { wo.bgc-=0.05 }else{ wo.bgc = -1.0 };
                    //println!("Pageup pressed,wo.bgc={}",wo.bgc);
                    },
                //TEST: Increment widget pointed to.
                glium::glutin::WindowEvent::KeyboardInput {
                    input: glium::glutin::KeyboardInput {
                        virtual_keycode: Some(glium::glutin::VirtualKeyCode::PageUp),
                        ..
                    },
                    ..
                } => {  if timer>*freeze_timer+1 {
							widget_cycler.lock().unwrap().advance();
							if !*pause {*freeze_timer = timer;};
						};
                    //println!("Pageup pressed,wo.wo.ifc={}",wo.wo.ifc);
                    },
                //TEST: Decrement widget pointed to..
                glium::glutin::WindowEvent::KeyboardInput {
                    input: glium::glutin::KeyboardInput {
                        virtual_keycode: Some(glium::glutin::VirtualKeyCode::PageDown),
                        ..
                    },
                    ..
                } => {  if timer>*freeze_timer+1 {
							widget_cycler.lock().unwrap().regress();
							if !*pause {*freeze_timer = timer;};
						};
                    //println!("Pagedown pressed,wo.ifc={}",wo.ifc);
                    },

                //travel down if needed.
                glium::glutin::WindowEvent::KeyboardInput {
                    input: glium::glutin::KeyboardInput {
                        virtual_keycode: Some(glium::glutin::VirtualKeyCode::Down),
                        ..
                    },
                    ..
                } =>  {
                    match gui_box.clone() {
                        GUIBox::GameTravel => {
                            let (a,b) = gmoose::travel_down(pl,p_loc,
                                                world,coords,
                                                timer,freeze_timer,
                                                comm_text,
                                                gui_box.clone(),
                                                gui_box_previous.clone(),
                                                party, enemies, encounter,
                                                mons);
                            *gui_box = a;
                            *gui_box_previous = b;
                        },
                        _ => {},
                    };
                },
                //travel up if needed.
                glium::glutin::WindowEvent::KeyboardInput {
                    input: glium::glutin::KeyboardInput {
                        virtual_keycode: Some(glium::glutin::VirtualKeyCode::Up),
                        ..
                    },
                    ..
                } => {
                    match gui_box.clone() {
                        GUIBox::GameTravel => {
                            let (a,b) = gmoose::travel_up(pl,p_loc,
                                                world,coords,
                                                timer,freeze_timer,
                                                comm_text,
                                                gui_box.clone(),
                                                gui_box_previous.clone(),
                                                party, enemies, encounter,
                                                mons);
                            *gui_box = a;
                            *gui_box_previous = b;
                        },
                        _ => {},
                    };
                },
                //travel left if needed.
                glium::glutin::WindowEvent::KeyboardInput {
                    input: glium::glutin::KeyboardInput {
                        virtual_keycode: Some(glium::glutin::VirtualKeyCode::Left),
                        ..
                    },
                    ..
                } =>  {
                    match gui_box.clone() {
                        GUIBox::GameTravel => {
                            let (a,b) = gmoose::travel_left(pl,p_loc,
                                                world,coords,
                                                timer,freeze_timer,
                                                comm_text,
                                                gui_box.clone(),
                                                gui_box_previous.clone(),
                                                party, enemies, encounter,
                                                mons);
                            *gui_box = a;
                            *gui_box_previous = b;
                        },
                        _ => {},
                    };
                },
                //travel right if needed.
                glium::glutin::WindowEvent::KeyboardInput {
                    input: glium::glutin::KeyboardInput {
                        virtual_keycode: Some(glium::glutin::VirtualKeyCode::Right),
                        ..
                    },
                    ..
                } => {
                    match gui_box.clone() {
                        GUIBox::GameTravel => {
                            let (a,b) = gmoose::travel_right(pl,p_loc,
                                                world,coords,
                                                timer,freeze_timer,
                                                comm_text,
                                                gui_box.clone(),
                                                gui_box_previous.clone(),
                                                party, enemies, encounter,
                                                mons);
                            *gui_box = a;
                            *gui_box_previous = b;
                        },
                        _ => {},
                    };
                },
                //if pause = true, continue to next turn stage, if enter is pressed.
                glium::glutin::WindowEvent::KeyboardInput {
                    input: glium::glutin::KeyboardInput {
                        virtual_keycode: Some(glium::glutin::VirtualKeyCode::Return),
                        ..
                    },
                    ..
                } => {	if *pause & (*freeze_timer+1<timer) {
                            *pause = false;
                            match gui_box.clone() {
                                GUIBox::GameFight(_) => {
                                    if (encounter[*battle_ifast].1!=0) & (encounter[*battle_ifast].0.HP_shade>0.0) {
                                        *comm_text = format!("{} ponders their next move...",encounter[*battle_ifast].0.name);
                                    };
                                },
                                GUIBox::GameCastSage(y,x) => {
                                    if x==GREETING1 {
                                        *gui_box = GUIBox::GameCastSage(y,GREETING2);
                                    }else if x==GOODBYE {
                                        *gui_box = GUIBox::GameCastPre;
                                    };
                                },
                                GUIBox::GameStory(_,_,_) => {
                                    *gui_box = GUIBox::GameTravel;
                                },
                                _ => {},
                            };
                        }else if idungeon.is_some() & (*freeze_timer+2<timer) {
                            if *dungeon_pointer==dungeons[idungeon.unwrap()].scenes.len()+2 {
                                *gui_box = GUIBox::GameTravel;
                                *dungeon_pointer = 0;
                                println!("Dungeon pointer = {}",dungeon_pointer);
                            };
                        }else{
                        //PUT THE CODE FOR PRESSING ENTER ON WIDGET HERE.
                        };
                     },
                //Go to previous button.
                glium::glutin::WindowEvent::KeyboardInput {
                    input: glium::glutin::KeyboardInput {
                        virtual_keycode: Some(glium::glutin::VirtualKeyCode::A),
                        ..
                    },
                    ..
                } => {},
                //Go to next widget
                glium::glutin::WindowEvent::KeyboardInput {
                    input: glium::glutin::KeyboardInput {
                        virtual_keycode: Some(glium::glutin::VirtualKeyCode::D),
                        ..
                    },
                    ..
                } => {},
                _ => {},
            },
            _ => (),
        }
    };
    ui
}
