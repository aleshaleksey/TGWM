//#![feature(use_extern_macros)]
//#![feature(libc)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![recursion_limit="512"]
#[macro_use]extern crate conrod;
#[macro_use]extern crate glium;
///	Tales of the Great White Moose (Rust GUI version)
///
///Tales of the Great White Moose (Moosequest) is a terminal RPG originally written in R (2016-2017) as a 
///[very] basic etude of machine learning algorithms. Despite all the 
///drawbacks or R.
///The AI algorithm was very basic and all battles were one on one.
///It was later rewritten in Rust with multiple "teams", still as a 
///terminal game (2017).
///This version is an attempt to rewrite it into a fully fledged 
///graphical RPG which uses the conrod package for the GUI.
///NB:conrod/examples/support is used for the boilerplating
///
///As of now the basic game engine "works", however as there was no
///"game plan" or "design documents" involved, the spec. kept evolving
///there are predictably quite a few TODOs (no particular order).
///
///TODO 1: Refactor flow control variables into a couple structures.
/// I-general GUI and Plot (see below) flow controller.
/// II-music flow controller
/// III-Battle flow controller (defacto already done, but ugly)
/// (On another note, I may leave this as this is)
///
///TODO 2: Refactor main GUI function in order to clean up.
///
///TODO 3: Implement story engine and "sages"(NPCs). Story driven event
///engine will also be used for sages.
///
///TODO 4: Implement [additional] "visual effects" for in-battle events.
///(TODO 4 is done).
///
///TODO 5: Return to the AI engine and continue with the etude.
///
///TODO 6: Make the externs and use sections pretty and stop sweeping
///warnings under the rug.
///
///Tales of the Great White Moose code is split into several packages, lmoose,imoose,
///dmoose,smoose,gmoose,omoose and the main file.
///-bmoose handles the functions governing battle logic (minus AI).
///-cmoose handles flow control structures (some also in smoose).
///-dmoose is an additional library for dungeon sequences.
///-gmoose handles the gui, graphics and game logic (merged to prevent
/// dependency hell).
///-imoose handles the AI.
///-lmoose handles the main types and monster/spell/locale data.
///-omoose handles the audio.
///-smoose is the unimplemented sage sequence library (also story and
/// story flow elements.
///-xmoose handles effects (such as attacks or spell effects.) Ass well
/// as certain background functions.
///-The main function handles global flow variables, event loop, combat
///loops, and redraw loops.
///
///
///	Alek Zholobenko 2018	

/// To compile for linux:
///		cargo build --release --features="winit glium libc"
///		cargo run --features="winit glium"
/// Try try compile for windows:
///		cargo rustc --bin q-moose --release --features="winit glium" --target=x86_64-pc-windows-gnu -- -C linker=x86_64-w64-mingw32-gcc -C link-args="-Wl,--subsystem,windows"
/// For clean compile use additional arguments (no terminal window): -C link-args="-Wl,--subsystem,windows"




//Externs
extern crate winit;
extern crate image;
extern crate num_cpus;
extern crate rand;
extern crate time;
extern crate find_folder;
extern crate num;
extern crate inflector;
mod imoose;
mod lmoose;
mod smoose;
mod gmoose;
mod omoose;
mod dmoose;
mod cmoose;
mod xmoose;
mod bmoose;
mod shared_moose;

//Imports
use shared_moose::*;
use smoose::{MyStories,Story,Sage};
#[allow(unused_imports)] use gmoose::{set_comm_text,set_widgets_rework,names_of,map_sq_col_img};
#[allow(unused_imports)] use omoose::{parse_music_config,isekai_deguchi,isekai_urusai,isekai_index};
#[allow(unused_imports)] use conrod::UiCell;
#[allow(unused_imports)] use conrod::widget::button::Interaction;
#[allow(unused_imports)] use imoose::permit_a;
#[allow(unused_imports)] use cmoose::{Landscapes,FlowCWin,GraphicsBox,SpriteBox,SpellBoxL,GUIBox};
#[allow(unused_imports)] use lmoose::{Spell,Item,Lifeform,Shade,Place,Dungeon,
									 cureL,cure,cureG,cureH,exorcism,exorcismG,exorcismH,
									 ember,fire,fireball,inferno,spark,lightning,lightningH,crystalliseL,crystallise,crystalliseH,
									 sum_reaper,teleport,teleportG,light,lightH,darkness,darknessH,slow,haste,lifestealer,curse,
									 apocalypse,timestop,dagger_of_fawn,bow_of_traveller,sword_of_perseus,
									 world,
									 goblin_dem,goblin_sco,goblin_witch,bandit,bandit_lord,dark_apprentice,
									 necromancer,necromancer_lord,skeleton,skeleton_kn,ghost,ghost_an,white_witch,beast_green,
									 beast_red,beast_great,fallen,titan,warrior,witch,wonderer,alien,loser,beast_serpent,sage_forsaken,
									 white_queen,
									 shortstaff,
									 index_arcana,
									 tree_of_life};
#[allow(unused_imports)] use lmoose::{ADVENT,ALBION,ALIEN,ANGEL,BEAST,BONE,BRIDGE,CITY,
									 DEATH,DESERT,ELF,EVIL,FALLEN,FIRE,FOREST,GIANT,GOBLIN,GRASSLAND,
									 HEALING,HIGHLAND,HOLY,HUMAN,ICE,LIGHTNING,MALACHIA,
									 MINDLESS,MOORLAND,MOOSE,RADIANT,RUIN,STEPPE,SPIRIT,
									 TELEPORTATION,TIME,TUNDRA,UNDEAD,VOID,WATER,WITCH,WHITE,NONE,
									 ANY,GROUP,GROUPS,SAME,SELF,SINGLE,TARGET,ALL,BOB,NON,PARTY};	
#[allow(unused_imports)] use dmoose::{malek_grove,monster_hall,citadel_of_spirit,elven_lake_ruins,malachia_pubcrawl,lost_lighthouse,door_to_darkness,
									  white_temple,stairway,witch_maze,way_down,wild_hunt,tower_of_bones,tower_of_flesh,tower_of_soul,hall_of_stone,
									  the_path
									  };

#[allow(unused_imports)] use conrod::color::Colour;
#[allow(unused_imports)] use conrod::{color, widget, Colorable, Labelable, Positionable, Sizeable, Widget};
#[allow(unused_imports)] use glium::Surface;
#[allow(unused_imports)] use image::GenericImage;
#[allow(unused_imports)] use rand::Rng;
#[allow(unused_imports)] use time::PreciseTime;
#[allow(unused_imports)] use time::PreciseTime as PT; //laziness at its best.

#[allow(unused_imports)] use std::sync::mpsc::{sync_channel,SyncSender,Receiver,TryRecvError};
#[allow(unused_imports)] use std::sync::{Arc,Mutex,Weak};
#[allow(unused_imports)] use std::{mem,thread,fs};
#[allow(unused_imports)] use std::path::PathBuf;
#[allow(unused_imports)] use std::ffi::OsString;
#[allow(unused_imports)] use std::collections::HashMap;

//main_loop
//main_loop
#[allow(unused_mut)]
#[allow(unused_variables)]
#[allow(unused_must_use)]
#[allow(dead_code)]
pub fn main() {
	println!("A");
	const WIDTH: u32 = 1080;
	const HEIGHT: u32 = 800;
	const MIN_W: u32 = 640;
	const MIN_H: u32 = 400;
	const SHAKE_DURATION:usize = 20;
	const TURNS_PER_MON:usize = 8;
	
	//Initiation of most flow control variables.

	// A kind of timer to time frames. freeze_timer is used to compare.
	// And background colour.
	let mut timer:usize = 0;
	let mut freeze_timer:usize = 0;

	//Initiate new option controller.
	// (silence, update backgrounds, brightness, and music paths.)
	let mut wo = FlowCWin::new();
	let mut gui_box = GUIBox::Uninitiated;
	let mut ipath:Option<(usize,String)> = None;

	//Variable to determine if extra buttons for character creation are visible or not.
	let mut mutm_box_vis = false;
	let mut new_game_init = false; //Is a game in progress.
	
	//WARNING!! tecil is now a little broken. tt_e_c_i_ll[7] controls spell inspector in player menu.
	let mut tt_e_c_i_ll: [bool;8] = [false;8];   //main menu adventure functionality
	// [0] = travel
	// [1] = travel
	// [2] = explore
	// [3] = castout of battle (not implemented).
	// [4] = inspect party
	// [5] = Dunno anymore
	// [6] = Dunno anymore
	// [7] = Dunno anymore
	// [8] = main menu in game.
	
	// Battle menu functionality
	let mut yt_adcwpe_bw:[bool;9] = [false;9];   //battle menu button functionality
	// [0] = Player turn.
	// [1] = attack pressed
	// [2] = defend pressed
	// [3] = cast pressed
	// [4] = wait pressed
	// [5] = panic pressed (yay! run in circles!)
	// [6] = escape pressed
	// [7] = black magic chosen - pick target
	// [8] = white magic chosen -pick target
	
	let mut comm_text:String = "Welcome to TGWM!".to_owned();
	let mut stage:usize = 0;
	let mut chosen_hero:usize = 0;
	
	//special battle variable for free flowing thoughts.
	//Type annotation ad nauseum.
	//Sends turn information to brain.
	let (mut thought_sender_to_brain, mut thought_receiver_to_brain):
	(SyncSender<(usize,usize,[u8;28],i32,Vec<(u8,u8)>,bool)>,
	Receiver<(usize,usize,[u8;28],i32,Vec<(u8,u8)>,bool)>)
	= std::sync::mpsc::sync_channel(1);
	
	//Sends preliminary battle information to brain
	let (mut thought_sender_to_brain2, mut thought_receiver_to_brain2):
	(SyncSender<(Vec<Vec<[u8;28]>>,Vec<(Lifeform,usize,[Option<[usize;2]>;2])>)>,
	Receiver<(Vec<Vec<[u8;28]>>,Vec<(Lifeform,usize,[Option<[usize;2]>;2])>)>)
	= std::sync::mpsc::sync_channel(1);
	
	//sends the brain's conclusion back to the main thread.
	let (mut thought_sender_to_body, mut thought_receiver_to_body) = std::sync::mpsc::sync_channel(1);
	
	//signaller for telling player function to do its job.
	let (mut b_muse_sender, mut b_muse_receiver) = std::sync::mpsc::sync_channel(1);
	let mut to_play:usize = 0;
	
	//signaller for silencing audio.
	let (mut muse_silence_sender,mut muse_silence_receiver) = std::sync::mpsc::sync_channel(1);
						
	let mut player_input:String = "Input into me.".to_owned();
	println!("B");
	// Build the window.
	// NB, it may be better to use .with_fullscreen(true)
	//Next line crashes for no reason.
	let mut events_loop = glium::glutin::EventsLoop::new();
	println!("events_loop built");
	let mut window = glium::glutin::WindowBuilder::new()
					  .with_title("TGWM")
					  .with_dimensions(WIDTH, HEIGHT)
					  .with_min_dimensions(MIN_W,MIN_H)
					  .with_visibility(true);
	println!("window build");
	let context = glium::glutin::ContextBuilder::new()
					   .with_vsync(true)
					   .with_multisampling(4);
	println!("context built");
	let mut display = glium::Display::new(window, context, &events_loop).unwrap();
	println!("display initialised");
	// construct our `Ui`.
	let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();
	println!("ui built");
	// The image map describing each of our widget->image mappings (in our case, none).
	let mut image_map = conrod::image::Map
	//::<glium::texture::Texture2d>
	::new();
	
	// Instantiate the generated list of widget identifiers.
	let ids = &mut gmoose::Ids::new(ui.widget_id_generator());
	
	// Add a `Font` to the `Ui`'s `font::Map` from file.
	let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("as").unwrap();
	let font_path = assets.join("NotoSans/NotoSans-Regular.ttf");
	ui.fonts.insert_from_file(font_path).unwrap();
	
	// A type used for converting `conrod::render::Primitives` into `Command`s that can be used
	// for drawing to the glium `Surface`.
	let mut renderer = Renderer::new(&display).unwrap();
	
	//preliminary draw
	{
		let primitives = ui.draw();
		renderer.fill(&display, primitives, &mut image_map);
		let mut target = display.draw();
		target.clear_color(0.0, 0.0, 0.0, 1.0);
		renderer.draw(&mut display, &mut target, &mut image_map,1.0,1.0).unwrap();
		target.finish().unwrap();
	}
	
	// Load images for monsters. NB must be loaded into the vector of images in the order of monster ids (see lmoose)
	// NB currently not generic over monster numbers. Perhaps do after world tree. 
	let mut mons_faces: Vec<[conrod::image::Id;3]> = Vec::with_capacity(27);	
	let mut mons_facesz: Vec<[conrod::Scalar;2]> = Vec::with_capacity(27);
	let img_path = assets.join("faces");
	
	for i in 0..27 {
		let face0 = load_image(&display, img_path.join(&format!("{}.png",i)),(false,0.0));
		let face1 = load_image(&display, img_path.join(&format!("{}a.png",i)),(false,0.0));
		let face2 = load_image(&display, img_path.join(&format!("{}b.png",i)),(false,0.0));
		mons_facesz.push([face0.get_width() as conrod::Scalar,face0.get_height().unwrap() as conrod::Scalar]);
        mons_faces.push([image_map.insert(face0),image_map.insert(face1),image_map.insert(face2)]);
	};
	
	
	//Prepare important variables
	let mut scapes:Landscapes = new_landscape();
	let sc_img_path = assets.join("scapes");
	let mut scapes_loaded = false;
	let input_sc_img_files = fs::read_dir(sc_img_path).unwrap();
	
	//cycle through image files
	for art in input_sc_img_files {
			landscape_sorter(&mut scapes,
							 &display,
							 &mut image_map,
							 art.unwrap().path(),
							 (false,0.0));
	};
	println!("length ice: {}",scapes.ice.len());
	println!("length highland: {}",scapes.highland.len());
	println!("length forest: {}",scapes.forest.len());
	println!("length grassland: {}",scapes.grassland.len());
	println!("length city: {}",scapes.city.len());
	println!("length ruin: {}",scapes.ruin.len());
	println!("length void: {}",scapes.void.len());
	println!("length moorland: {}",scapes.moorland.len());
	println!("length desert: {}",scapes.desert.len());
	println!("length steppe: {}",scapes.steppe.len());
	println!("length water: {}",scapes.water.len());
	
	// Inititate Q-ft-M variables:
	let mut battled:usize = 0;
	let mut action:u8 = 0;
	let world:Vec<[Place;19]> = world();
	let sp_list:Vec<Spell> = index_arcana();
	let mons:Vec<Lifeform> = tree_of_life();
	let mut stories:Vec<Story> = Vec::new(); //Currently placeholder.
	let mut my_stories:MyStories = MyStories::new();
	let mut diff:i32 = 0;
	let mut p_names_m:Vec<&str> = Vec::with_capacity(5);
	let mut p_names:Vec<String> = Vec::with_capacity(5);
	let mut party:Vec<(Lifeform,usize)> = Vec::with_capacity(5);
	let mut p_loc:Place = world[8][6].clone();
	let mut sages:Vec<Sage> = Vec::new();
	let mut pl:(usize,usize) = (13,5);
	let mut provisional_loc:(usize,usize) = pl.clone();
	let mut truly_quit:bool = false;
	let mut pause = true;
	let mut scenery_index:usize = 0;
	let mut centre_w:f64 = 0.0;
	let mut centre_h:f64 = 0.0;
	let mut gui_box = GUIBox::Uninitiated;
	let mut gui_box_previous = GUIBox::Uninitiated;
	
	//Initiate Q-ft-M battle variables:
	let mut dream_time:bool = false;
	let mut escaped:bool = false;
	let mut encounter: Vec<(Lifeform,usize,[Option<[usize;2]>;2])> = Vec::with_capacity(25);
	let mut enemies: Vec<(Lifeform,usize)> = Vec::with_capacity(20);
	let mut field: Place = p_loc.clone();
	let mut lore_empty = true;
	let mut aftermath:(Lifeform,Lifeform,Vec<[u8;28]>) = (ghost(),ghost(),Vec::with_capacity(1001));
	let mut sel_targets:Vec<usize> = Vec::with_capacity(25);
	let mut targets:Vec<usize> = Vec::with_capacity(25);
	let mut to_cast:String = String::new();
	let mut shaking_dam:[bool;25] = [false;25];
	let mut shaking_timer:usize = 0;
	let mut ai_turn_started:bool = false;
	let mut ai_started_thinking:bool = false;
	let mut sprite_pos:[[f64;2];25] = [[0.0;2];25];
	let mut sprite_boxer:GraphicsBox = GraphicsBox::None;
	
	
	//Create the world map
	let map_size = [1600,1200];
	let mut world_map = generate_world_map (&world,&map_size,&display);
	let world_map_id = image_map.insert(world_map);
	
	//Music:
	//Set up the song_list to display in the options menu.
	let mut gui_song_list:Vec<String> = Vec::with_capacity(19);
	for _ in 0..19 {gui_song_list.push("Standard".to_owned());}
	//read config notes.mqcfg to song_list.
	parse_music_config(&mut gui_song_list);
	
	// Set up the jukebox thread.
	//play while in battle, but not otherwise. (Also set silence = false)
	b_muse_sender.clone().send((false,to_play));
	muse_silence_sender.clone().send(false);
	
	//spawn thread.
	let assets2 = assets.clone();
	let music_thread = thread::spawn(move||{
		//initalise flow variable to avoid error upon unwrapping of b_muse_reciever.
		let mut go = (false,to_play);
		let mut silence = false;
		//loop (check if music is needed and play if true.
		loop {
			//check if music is needed (b_muse_sender status)
			go = isekai_deguchi(go.clone(),&mut b_muse_receiver);
			silence = isekai_urusai(silence, &mut muse_silence_receiver);
			//if needed loop (playback and check if play again.
			while go.0 & !silence {
				//println!("La-la-la-al.");
				//Music player goes here.
				go = omoose::play_song_rod(go,
										   &mut b_muse_receiver,
										   &mut muse_silence_receiver);
				thread::sleep(std::time::Duration::from_millis(2000));
				//check if play again.
			};
			//println!("Not being musical");
			//wait a while to avoid eating system resources.
			thread::sleep(std::time::Duration::from_millis(2000));
		};			
	});
	
	//Initiate Moosequese inner battle variables:
	//Battle startup variables.
	let mut exp_players:[f32;5] = [0.0;5];
	let mut battle_gold_pot:usize = 0;
	let mut in_battle_record:Vec<[u8;28]>= Vec::with_capacity(2000);
	let mut cms:usize = 0;
	let mut battle_timer: Vec<f32>=Vec::with_capacity(25);
	let mut battle_fast:f32 = 0.0;
	let mut battle_ifast:usize = 0;
	let mut battle_ttakes = Vec::new();
	let mut battle_omnicide:bool = false;
	let mut battle_tturns:u16 = 0;
	let mut battle_orders:Vec<(u8,u8)> = Vec::with_capacity(2000);
	let mut enc_names:Vec<String> = Vec::with_capacity(25);
	let mut recl:[u8;28] = [255;28];
	let mut to_hit:Vec<(bool,bool)> = Vec::with_capacity(25);
	let mut p_scape:u8 = FOREST;
	
	//spawn thinker thread
	//strictly easier to spawn this once than several times.
	let brain_thread = thread::spawn(move||{
		
		//loop globally looking for battles (ie lore has been sent)
		'battle:loop {
			match thought_receiver_to_brain2.try_recv() {
				Ok(scenario) => {
					let (lore,mut enc) = scenario;
					println!("enc.len() = {}, in brain",enc.len());				   
					//Initiate variables that live in this thread. and Borrow bits of lore.
					let mut differences: Vec<Vec<[i8;23]>> = Vec::with_capacity(lore.len());
					let mut last_lines:Vec<&[u8;28]> = Vec::with_capacity(lore.len());
					let mut cause_effect: HashMap<&[u8],Vec<&[i8;23]>> = HashMap::with_capacity(100000);
					let mut lore_hash_by_end: HashMap<&[u8;28],Vec<&Vec<[u8;28]>>> = HashMap::with_capacity(20000);
					let mut all_causes:Vec<&[u8]> = Vec::with_capacity(10000);
					
					//Recode AI variables for speed and sanity.
					imoose::ai_part_b1(&lore, &mut differences);
					
					let cause_effect_means = imoose::ai_accelerator_hash(&lore,
																		&differences,
																		&mut last_lines,
																		&mut lore_hash_by_end,
																		&mut cause_effect,
																		&mut all_causes);
					
					//loop within a battle, looking for specific instances to match
					//by definition, this might as well block.
					'turns:loop {
						match thought_receiver_to_brain.try_recv() {
							Ok(inst) => {
								let (a,b,c,d,e,f) = inst;
								if !f {
									//if battle over break inner loop and
									//keep looking for battles.
									println!("Breaking turns loop in battle");
									break 'turns
								}else{
									//Else do thinking and send answer.
									thought_sender_to_body.send(
										imoose::ai_part_a(
											&enc,
											a,b,
											&lore,
											&last_lines,
											&differences,
											&cause_effect,
											&cause_effect_means,
											&all_causes,
											&c,d,e
										)
									);
								};
							},
							_		 => {
								thread::sleep(std::time::Duration::from_millis(10));
							},
							
						};
					};
				},
				_ 				   => {
					thread::sleep(std::time::Duration::from_millis(10));
				},
			};
		};
	});
	
	
	//create gdugeons.
	let mut dungeons:Vec<Dungeon> = Vec::new();
	let mut idungeon:Option<usize> = None;
	let mut dungeon_pointer:usize = 0;
	
	//Initiate mooseloader variables:
	let mut rrrltxt:Vec<String> = Vec::new();
	let mut rltxt=String::new();
	let mut ltxt:Vec<&'static str> = Vec::with_capacity(1000);
	let mut rlb:Vec<u8> = Vec::with_capacity(1000);
	let mut coords:[i32;2] = [8,8];
	let blank:&'static str = "";
	let mut to_load:(Option<String>,usize) = (None,1);
	
	//rebuild display
	//redraw window
	//use next version of conrod.

	// Poll events from the window.
	let mut event_loop = EventLoop::new();
	'main: loop {
		//timer timer.
		timer+= 1;
		//if n_s_l_q_f[4] {println!("got here Z");};
		
		// Handle all events.
		for event in event_loop.next(&mut events_loop) {

			// Use the `winit` backend feature to convert the winit event to a conrod one.
			if let Some(event) = convert_event(event.clone(), &display) {
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
					} => {gui_box = GUIBox::MainQuit(false);},
					
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
					//TEST: Adjusts srgb_linear correction.
					glium::glutin::WindowEvent::KeyboardInput {
						input: glium::glutin::KeyboardInput {
							virtual_keycode: Some(glium::glutin::VirtualKeyCode::PageUp),
							..
						},
						..
					} => {if wo.ifc<=0.95 { wo.ifc+=0.05 }else{ wo.ifc = 1.0 };
						//println!("Pageup pressed,wo.wo.ifc={}",wo.wo.ifc);
						},
					//TEST: Adjusts srgb_linear correction.
					glium::glutin::WindowEvent::KeyboardInput {
						input: glium::glutin::KeyboardInput {
							virtual_keycode: Some(glium::glutin::VirtualKeyCode::PageDown),
							..
						},
						..
					} => {if wo.ifc>=-0.95  { wo.ifc-=0.05 }else{ wo.ifc = -1.0 };
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
								let (a,b) = gmoose::travel_down(&mut pl,&mut p_loc,
													&world,&mut coords,
													timer,&mut freeze_timer,
													&mut comm_text,
													gui_box,
													gui_box_previous,
													&mut party, &mut enemies, &mut encounter,
													&mons);
								gui_box = a;
								gui_box_previous = b;
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
								let (a,b) = gmoose::travel_up(&mut pl,&mut p_loc,
													&world,&mut coords,
													timer,&mut freeze_timer,
													&mut comm_text,
													gui_box,
													gui_box_previous,
													&mut party, &mut enemies, &mut encounter,
													&mons);
								gui_box = a;
								gui_box_previous = b;
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
								let (a,b) = gmoose::travel_left(&mut pl,&mut p_loc,
													&world,&mut coords,
													timer,&mut freeze_timer,
													&mut comm_text,
													gui_box,
													gui_box_previous,
													&mut party, &mut enemies, &mut encounter,
													&mons);
								gui_box = a;
								gui_box_previous = b;
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
								let (a,b) = gmoose::travel_right(&mut pl,&mut p_loc,
													&world,&mut coords,
													timer,&mut freeze_timer,
													&mut comm_text,
													gui_box,
													gui_box_previous,
													&mut party, &mut enemies, &mut encounter,
													&mons);
								gui_box = a;
								gui_box_previous = b;
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
					} => {	if pause & (freeze_timer+1<timer) {
								pause = false;
								if gui_box.is_fight() {
									if (encounter[battle_ifast].1!=0) & (encounter[battle_ifast].0.HP_shade>0.0) {
										comm_text = format!("{} ponders their next move...",encounter[battle_ifast].0.name);
									};
								};
							}else if idungeon.is_some() & (freeze_timer+2<timer)  {
								if dungeon_pointer==dungeons[idungeon.unwrap()].scenes.len()+2 {
									gui_box = GUIBox::GameTravel;
									dungeon_pointer = 0;
									println!("Dungeon pointer = {}",dungeon_pointer);
								};
							}else if gui_box.is_sage_sage() {
									gui_box = GUIBox::GameCastPre;
							};
							freeze_timer = timer;
						 },
					_ => {},
				},
				_ => (),
			}
		}
			
		
		
								  
		//if n_s_l_q_f[4] {println!("got here Z3");};
				
		//last minute comm_text corrections.
		gmoose::correct_comm_text(&mut comm_text,
								  pause,
								  &mut gui_box);
								  
		//println!("Before set widgets");
		// Instantiate all widgets in the GUI.
		// This is getting insane.
		// Getting the impression that a closure would be nicer.
		let loop_tuple = set_widgets_rework(ui.set_widgets(), ids,
					gui_box,
					gui_box_previous,
					&mons_faces,
					&mons_facesz,
					&mut comm_text,
					&mut player_input,
					&mut mutm_box_vis,
					&mut tt_e_c_i_ll,
					&mut yt_adcwpe_bw,
					&mut provisional_loc,
					&mut battled,
					&mut action,
					&world,
					&world_map_id,
					&sp_list,
					&mons,
					&mut p_names_m,
					&mut p_names,
					&mut party,
					&mut p_loc,
					&mut pl,
					&mut encounter,
					&mut enemies,
					&mut field,
					&mut lore_empty,
					&mut aftermath,
					&mut rrrltxt,
					&mut rltxt,
					&mut ltxt,
					&mut rlb,
					&mut coords,
					&mut to_load,
					timer,
					&mut freeze_timer,
					&mut sel_targets,
					&mut to_cast,
					battle_ifast,
					&mut battle_tturns,
					&mut chosen_hero,
					&mut dungeons,
					&mut idungeon,
					&mut dungeon_pointer,
					&mut truly_quit,
					&mut shaking_dam,
					&mut shaking_timer,
					&mut pause,
					&mut scenery_index,
					&scapes,
					&mut centre_h,
					&mut centre_w,
					&mut gui_song_list,
					&mut muse_silence_sender,
					&mut p_scape,
					&mut wo,
					&mut ipath,
					&mut sprite_boxer,
					&mut sprite_pos,
					&mut my_stories,
					&stories,
					sages);
								  
		//reload backgrounds if graphical settings have been changed.			
		if wo.update_bgc {		
			//Nothing implements copy... Nothing implements clone...
			landscape_unloader(&mut scapes, &mut image_map);
			scapes = new_landscape();
			let sc_img_path = assets.join("scapes");
			let input_sc_img_files = fs::read_dir(sc_img_path).unwrap();
			for art in input_sc_img_files {
				landscape_sorter(&mut scapes,
								 &display,
								 &mut image_map,
								 art.unwrap().path(),
								 (true,wo.bgc));
			};
			//Reset guiding variables so it doesn't loop.
			wo.bgc = 0.0;
			wo.update_bgc = false;
		};
		
		gui_box = loop_tuple.0;
		gui_box_previous = loop_tuple.1;
		sages = loop_tuple.2;
		//println!("after loop tuple");
		
		//if n_s_l_q_f[4] {println!("got here Z4");};
		
		// Render the `Ui` and then display it on the screen.
		let primitives = ui.draw();
		renderer.fill(&display, primitives, &mut image_map);
		let mut target = display.draw();
		
		//Thesting for display brightness adjustment.
		target.clear_color(0.0, 0.0, 0.0, 0.0);
		renderer.draw(&mut display, &mut target, &mut image_map,wo.ifc+1.0,wo.bgc+1.0).unwrap();
		target.finish().unwrap();
		
		//AI generation
		match gui_box.clone() {
			GUIBox::GameFight(x) => {
				if x {	
					gui_box = GUIBox::GameFight(false);
					
					//define song which is to be played.
					to_play = isekai_index(&party,&encounter,&dungeons,&p_loc,dungeon_pointer,&idungeon);
					
					//send signal to player to start playing.
					b_muse_sender.try_send((true,to_play));
					
					//println!("The moose must dream.");
					let cpu_n = num_cpus::get();
					let sow:usize = cpu_n;
					//println!("start of AI dreaming on {} threads.",sow);
					let s = PreciseTime::now();
					
					let mut lore:Vec<Vec<[u8;28]>> = Vec::with_capacity(500000);
					let mut battle_threads = Vec::new();
					for _ in 0..sow{
						let lim = wo.ai_mem;
						let mut d_cycle1:usize=0;
						let mut ll_c1 = if (dungeon_pointer<2) | idungeon.is_none(){
							mons.clone()
						}else{
							//dungeon updater now moved to set_widgets function.
							dungeons[idungeon.unwrap()].denizens.clone()
						};
						let s_c1 = sp_list.clone();
						let field = p_loc.clone();
						let enc_c = encounter.clone();
						let encna_c = names_of(&encounter);
						let mut n_o_d1:usize = 0;
						let mut discards:usize = 0;
						let mut tpm:usize = TURNS_PER_MON;
						let mut bytes_in_record:usize = 0;

						let battle_thr=thread::spawn(move||{
							let t0=PreciseTime::now();
							let s_c1= s_c1.clone();
							let field= field.clone();
							let encna_cc=encna_c.clone();
							let mut battle_rec = Vec::with_capacity(250000);
							let dreams_per_thread = 5000*encna_cc.len();
							'dream_looper: loop {
								d_cycle1+= 1;
								let enc_cc = enc_c.clone();
								let cms = enc_cc.len();
								let mut encna_cc = encna_c.clone();
								let bl = bmoose::battle_rand(enc_cc,
												&s_c1,
												&field,
												&encna_cc,
												n_o_d1);
								
								if d_cycle1%1000==0	{
									if discards<2*n_o_d1 {tpm-=1}else{tpm+=1};
								};
								if d_cycle1%20000==0{
									println!("D-cycle={}",d_cycle1);
									if bytes_in_record*cpu_n>lim {
										println!("Sizeof battle_rec fragment: {}\nNOD = {}",bytes_in_record,n_o_d1);
										break 'dream_looper;
									};
								};
								
								if bl.len()<(cms*TURNS_PER_MON){
									bytes_in_record+= bl.len()*28*3;
									battle_rec.push(bl);
									n_o_d1+=1;
								}else{
									discards+= 1;
								};
								let t1=PreciseTime::now();
								if t0.to(t1)>time::Duration::seconds(10){
									println!("Size of battle_rec fragment: {}",bytes_in_record);
									break 'dream_looper;
								};
							};
							println!("\nNo. of dreams: {}\n",d_cycle1);
							println!("Number of records: {}", battle_rec.len());
							println!("Number of discarded dreams: {}",discards);
							battle_rec
						});
						battle_threads.push(battle_thr);
					};
					
					for _ in 0..battle_threads.len(){
						let x = battle_threads.pop().expect("Oh pop").join().expect("Oh join!");
						lore.extend(x);
					};
					
					//Send the base encounter information to the brain.
					// And allow initiation of battle map.
					thought_sender_to_brain2.send((lore,encounter.clone()));
					lore_empty = false;

					let e=PreciseTime::now();
					println!("Total time: {}",s.to(e));
					
					//Set/Reset battle variables.
					for i in 0..party.len() {
						exp_players[i] = exp_calc(&encounter,i);
					};
					println!("Exp on victory: {:?}",exp_players);
					battle_gold_pot = 0;
					cms=encounter.len();
					for x in encounter.iter() {battle_gold_pot+= x.0.Gold};

					in_battle_record = vec!([0;28],[255;28],[255;28]);
					
					for (i,x) in encounter.iter().enumerate() {
						in_battle_record[1][6+i] = x.0.id as u8;
						in_battle_record[2][6+i] = x.1 as u8
					};

					battle_timer = vec!(0.0;cms);
					to_hit = vec![(false,false);cms];
					
					//create timer and initiate first turn.
					for (nth,x) in encounter.iter().enumerate(){
						let mut tvar:f32 = 0.0;
						tvar+= (rand::thread_rng().gen_range(-10,11)-rand::thread_rng().gen_range(-10,11)) as f32;
						let time = x.0.Speed_shade.clone()+tvar;
						battle_timer[nth] = 1.0/time
					};
					battle_fast = vnmin(battle_timer.clone());
					battle_ifast = vwhich(&battle_timer,battle_fast).unwrap_or(battle_ifast);
					println!("{} from group {} is the first to take action!",beast_name(&encounter,battle_ifast,&p_names), &encounter[battle_ifast].1);
					println!("battle_ifast = {}, encounter.len() = {}",battle_ifast,encounter.len());
					
					enc_names = Vec::with_capacity(25);
					
					for x in encounter.iter() {enc_names.push(x.0.name.to_owned());};
					for (i,x) in p_names.iter().enumerate() {
						enc_names[i] = x.to_owned();
					};
					println!("Encounter names: {:?}",enc_names);
					//main battle loop.
					battle_ttakes = vec![0;cms];
					battle_tturns = 0;
					battle_orders = Vec::with_capacity(2000);
					println!("Dungeon pointer == {}",dungeon_pointer);
					
					//send another signal to player to start playing,
					//just in case.
					b_muse_sender.try_send((true,to_play));
				
				}else if !x {
			
					//Increment shaking timer. Switch off shaking if timer elapsed.
					if shaking_timer+SHAKE_DURATION < timer {
						shaking_dam = [false;25];
					};
					let mut fight = true;
					//Take turn if game is not paused.
					if !pause & sprite_boxer.is_none() {	
						//if in battle, and time has elapsed, check for end game.	
						//println!("Got to game over");
						bmoose::game_over  (&mut encounter,
											&mut enemies,
											&mut party,
											&mut dungeons,
											&mut fight,
											&mut tt_e_c_i_ll,
											&mut exp_players,
											&mut battle_gold_pot,
											&mut comm_text,
											timer,
											&mut freeze_timer,
											&mut battle_tturns,
											&mut idungeon,
											&mut dungeon_pointer,
											escaped);
						//println!("got past game over");

						if !fight {
							
							println!("Ending battle");
							//music player controlled to off.
							b_muse_sender.try_send((false,to_play));
							//Tell the brain that the battle is over.
							thought_sender_to_brain.send((0,0,[0;28],0,Vec::new(),false));
							//if game_over functions determines end of battle, reset battle variables, level party and end battle.
							encounter = Vec::with_capacity(25);
							enemies = Vec::with_capacity(20);
							to_hit = Vec::with_capacity(25);
							sel_targets = Vec::with_capacity(25);
							targets = Vec::with_capacity(25);
							battle_timer = Vec::with_capacity(25);
							bmoose::lvlq(&party,&p_names,&mut tt_e_c_i_ll);
							yt_adcwpe_bw = [false;9];
							shaking_timer = 0;
							shaking_dam = [false;25];
							ai_turn_started = false;
							ai_started_thinking = false;
							sprite_boxer = GraphicsBox::None;
							sprite_pos = [[0.0;2];25];
							escaped = false;
							gui_box = gui_box_previous.clone();
							
							//set p_scape as needed.
							if idungeon.is_none() {
								p_scape = p_loc.scape;
							}else if (dungeon_pointer<2) | (dungeon_pointer>dungeons[idungeon.unwrap()].scenes.len()+1) {
								p_scape = p_loc.scape;
							}else{
								p_scape = dungeons[idungeon.unwrap()].scenes[dungeon_pointer-2].scape;
							};
							
							scenery_index = gmoose::scenery_setter(&scapes,p_scape,&mut centre_w,&mut centre_h);
							
							continue 'main;
						}else{
							if (encounter[battle_ifast].1!=0) & !pause {	
								//Computer turn.
								bmoose::ai_battle_turn(&mut encounter,&mut enc_names,
												if (dungeon_pointer<2) | idungeon.is_none() {
													&mut p_loc
												}else{
													&mut dungeons[idungeon.unwrap()].scenes[dungeon_pointer-2]
												},
												&sp_list,cms,
												&mut battle_fast,
												&mut battle_ifast,
												&mut battle_ttakes,
												&mut battle_tturns,
												&mut battle_orders,
												&mut battle_timer,
												&mut in_battle_record,
												timer,
												&mut freeze_timer,
												&mut comm_text,
												&mut pause,
												&mut shaking_timer,
												&mut shaking_dam,
												&mut ai_turn_started,
												&mut ai_started_thinking,
												&mut thought_sender_to_brain,
												&mut thought_receiver_to_body,
												&mut sprite_boxer,
												&mut sprite_pos,
												&mut targets);
								//println!("D");
							}else if !pause & (encounter[battle_ifast].1==0) {
								//Player tuen
								bmoose::player_battle_turn(&mut encounter,&enc_names,
												if (dungeon_pointer<2) | idungeon.is_none() {
													&mut p_loc
												}else{
													&mut dungeons[idungeon.unwrap()].scenes[dungeon_pointer-2]
												},
												&sp_list,
												cms,
												&mut battle_fast,
												&mut battle_ifast,
												&mut battle_ttakes,
												&mut battle_tturns,
												&mut battle_orders,
												&mut battle_timer,
												&mut in_battle_record,
												timer,
												&mut freeze_timer,
												&mut yt_adcwpe_bw,
												&mut recl,
												&mut comm_text,
												&mut sel_targets,
												&mut to_cast,
												&mut to_hit,
												&mut pause,
												&mut escaped,
												&mut shaking_timer,
												&mut shaking_dam,
												&mut sprite_boxer,
												&mut sprite_pos,
												&mut targets);
								//if !n_s_l_q_f[4] {b_muse_sender.try_send((false,to_play));};
								//println!("E");
							};
						};
					};
				};
			},
			GUIBox::MainQuit(x) => {if x {break 'main;};},
			_ => {},
		};
	}
}
	//	End of main main loop.
	//	End of main main loop.
	//	End of main main loop.
//function to make a new container for landscapes.
fn new_landscape()->Landscapes {
	Landscapes {
		city: Vec::with_capacity(5),
		desert: Vec::with_capacity(5),
		forest: Vec::with_capacity(5),
		grassland: Vec::with_capacity(5),
		highland: Vec::with_capacity(5),
		moorland: Vec::with_capacity(5),
		ruin: Vec::with_capacity(5),
		steppe: Vec::with_capacity(5),
		tundra: Vec::with_capacity(5),
		void: Vec::with_capacity(5),
		water: Vec::with_capacity(5),
		ice:Vec::with_capacity(5),
	}	
}

//function to load landscape image_ids into the right directory.
//nb file names must be in the format of "scapeXXXXX.png"
fn landscape_sorter(mut container: &mut Landscapes,
					display: &glium::Display,
					mut image_map: &mut conrod::image::Map<glium::texture::SrgbTexture2d>, 
					path:PathBuf,adjust:(bool,f32)) {
	//get filename
	let mut file_name:String = path.clone().file_stem().unwrap().to_owned().into_string().unwrap();
	//load image
	let image:glium::texture::SrgbTexture2d = load_image(&display,path,adjust);
	let dims:[conrod::Scalar;2] = [image.get_width() as conrod::Scalar, image.get_height().unwrap() as conrod::Scalar];
	
	//Place loaded image and dimensions in the right category in the Landscapes container.
	match file_name.trim_right_matches(char::is_numeric) {
		"ice" => container.ice.push((image_map.insert(image),dims)),
		"tundra" => container.tundra.push((image_map.insert(image),dims)),
		"water" => container.water.push((image_map.insert(image),dims)),
		"grassland" => container.grassland.push((image_map.insert(image),dims)),
		"forest" => container.forest.push((image_map.insert(image),dims)),
		"steppe" => container.steppe.push((image_map.insert(image),dims)),
		"desert" => container.desert.push((image_map.insert(image),dims)),
		"city" => container.city.push((image_map.insert(image),dims)),
		"highland" => container.highland.push((image_map.insert(image),dims)),
		"moorland" => container.moorland.push((image_map.insert(image),dims)),
		"void" => container.void.push((image_map.insert(image),dims)),
		"ruin" => container.ruin.push((image_map.insert(image),dims)),
		_ => println!("Boop-boop! Landscape image could not be classified"), 
	};
}

//function to load landscape image_ids into the right directory.
//nb file names must be in the format of "scapeXXXXX.png"
fn landscape_unloader(mut container: &mut Landscapes,
					  mut image_map: &mut conrod::image::Map<glium::texture::SrgbTexture2d>) {
	
	for &(x,_) in container.ice.iter() {
		image_map.remove(x);
	};
	for &(x,_) in container.tundra.iter() {
		image_map.remove(x);
	};
	for &(x,_) in container.water.iter() {
		image_map.remove(x);
	};
	for &(x,_) in container.grassland.iter() {
		image_map.remove(x);
	};
	for &(x,_) in container.forest.iter() {
		image_map.remove(x);
	};
	for &(x,_) in container.steppe.iter() {
		image_map.remove(x);
	};
	for &(x,_) in container.desert.iter() {
		image_map.remove(x);
	};
	for &(x,_) in container.city.iter() {
		image_map.remove(x);
	};
	for &(x,_) in container.highland.iter() {
		image_map.remove(x);
	};
	for &(x,_) in container.moorland.iter() {
		image_map.remove(x);
	};
	for &(x,_) in container.void.iter() {
		image_map.remove(x);
	};
	for &(x,_) in container.ruin.iter() {
		image_map.remove(x);
	};
}

// Load an image from our assets folder as a texture we can draw to the screen.
//NB Borrowed function from image_button.rs example in conrod/examples.
fn load_image<P>(display: &glium::Display, path: P,adjust:(bool,f32)) -> glium::texture::SrgbTexture2d
	where P: AsRef<std::path::Path>,
{
	let path = path.as_ref();
	
	let rgba_image;
	
	//If adjust.0==true, multiply by this.
	if !adjust.0 {
		rgba_image = image::open(&std::path::Path::new(&path)).unwrap().to_rgba();
	}else{
		rgba_image = image::open(&std::path::Path::new(&path)).unwrap().brighten((adjust.1*40.0) as i32).to_rgba();
	};
	
	let image_dimensions = rgba_image.dimensions();
	let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&rgba_image.into_raw(), image_dimensions);
	let texture = glium::texture::SrgbTexture2d::new(display, raw_image).unwrap();
	texture
}

// A function to generate the overland world map as an image based on world configuration.
// This function increases overhead on game startup but decreases graphical load afterwords
// As it obviates the need for a button matrix. Therefore the graphics workload gets purer.
// Currently this doesn't do things in the most efficient way possible.
fn generate_world_map(world:&Vec<[Place;19]>,
					  size:&[usize;2],		//[h,w] ->h%19==0, w%19==0 if not the world ends.
					  display:&glium::Display)->  glium::texture::SrgbTexture2d{

	let a = PreciseTime::now();					  
	//Create vector of pixels and correct image size.
	let mut size:[usize;2] = [size[0],size[1]]; //[h,w]
	let world_len:usize = world.len();
	while size[0]%19!=0 {size[0]+= 1};
	while size[1]%world_len != 0 {size[1]+= 1};
	
	//define image vector length and the size of each map square in pixels.
	let pixel_count:usize = size[0]*size[1];
	let tile_size:[usize;2] = [size[0]/19,size[1]/world_len];  //[h,w]

	let mut pixels:Vec<[u8;4]> = vec![[0;4];pixel_count];
	
	// Iterate over longitudes (outer by x)
	// ...and then latitutes (inner by y)
	for (x,long) in world.iter().enumerate() {
		let x = world_len-1-x;
		for y in 0..19 {
			let sqs:usize = tile_size[0]*(18-y);
			for row in sqs..(sqs+tile_size[0]) {
				for pixel in (row*size[1]+x*tile_size[1])..(row*size[1]+x*tile_size[1]+tile_size[1]) {
					pixels[pixel] = map_sq_col_img(&long[y]);
				}
			}
		}
	}
	
	//function to refine (blur the boundaries)
	gmoose::refine_map(&mut pixels,size[1],size[0]);
	
	// Transmute the map from [u8;4] to u8,u8,u8,u8...
	let mut pix_rgba:Vec<u8> = unsafe
	{
		let pixel_count = pixels.len();
		let p = std::mem::transmute::<*mut [u8;4],*mut u8>(pixels.as_mut_ptr());
		std::mem::forget(pixels);
		Vec::from_raw_parts(p,pixel_count*4,pixel_count*4)
	};
	//for x in pixels.into_iter() {
		//pix_rgba.extend_from_slice(&x);
	//}
	
	println!("Map size: {:?}",size);
	
	//This is important for the final conversion.
	let raw_image = glium::texture::RawImage2d::from_raw_rgba(pix_rgba, (size[1] as u32,size[0] as u32));
	let texture = glium::texture::SrgbTexture2d::new(display, raw_image).unwrap();
	let b = PreciseTime::now();
	println!("TIme to generate map: {}",a.to(b));
	texture
}

//NB the rest of this is the support.rs from the support module of conrod/examples. (with some small modifications)
//NB the rest of this is the support.rs from the support module of conrod/examples. (with some small modifications)
//NB the rest of this is the support.rs from the support module of conrod/examples. (with some small modifications)
//NB the rest of this is the support.rs from the support module of conrod/examples. (with some small modifications)
//NB the rest of this is the support.rs from the support module of conrod/examples. (with some small modifications)
//NB the rest of this is the support.rs from the support module of conrod/examples. (with some small modifications)
//NB the rest of this is the support.rs from the support module of conrod/examples. (with some small modifications)
//NB the rest of this is the support.rs from the support module of conrod/examples. (with some small modifications)
//NB the rest of this is the support.rs from the support module of conrod/examples. (with some small modifications)
//NB the rest of this is the support.rs from the support module of conrod/examples. (with some small modifications)
//NB the rest of this is the support.rs from the support module of conrod/examples. (with some small modifications)
//NB the rest of this is the support.rs from the support module of conrod/examples. (with some small modifications)
//NB the rest of this is the support.rs from the support module of conrod/examples. (with some small modifications)
//NB the rest of this is the support.rs from the support module of conrod/examples. (with some small modifications)
//NB the rest of this is the support.rs from the support module of conrod/examples. (with some small modifications)
//NB the rest of this is the support.rs from the support module of conrod/examples. (with some small modifications)
//NB the rest of this is the support.rs from the support module of conrod/examples. (with some small modifications)

#[allow(dead_code)]
fn interaction_and_times_triggered(button_id: widget::Id, ui: &UiCell) -> (Interaction, u16) {
    let input = ui.widget_input(button_id);
    let interaction = input.mouse().map_or(Interaction::Idle, |mouse| {
        let is_pressed =
            mouse.buttons.left().is_down()
            || ui.global_input().current.touch.values()
                 .any(|t| t.start.widget == Some(button_id));
        if is_pressed { Interaction::Press } else { Interaction::Hover }
    });
    let times_triggered = (input.clicks().left().count() + input.taps().count()) as u16;
    (interaction, times_triggered)
}

pub struct EventLoop {
    ui_needs_update: bool,
    last_update: std::time::Instant,
}

impl EventLoop {

    pub fn new() -> Self {
        EventLoop {
            last_update: std::time::Instant::now(),
            ui_needs_update: true,
        }
    }

    /// Produce an iterator yielding all available events.
    //Alek note: modified loop and loop speed, will make frame rate controllable in future.
    pub fn next(&mut self, events_loop: &mut glium::glutin::EventsLoop) -> Vec<glium::glutin::Event> {
        // We don't want to loop any faster than 60 FPS, so wait until it has been at least 16ms
        // since the last yield.
        let last_update = self.last_update;
        let sixteen_ms = std::time::Duration::from_millis(40);
        let duration_since_last_update = std::time::Instant::now().duration_since(last_update);
        if duration_since_last_update < sixteen_ms {
            std::thread::sleep(sixteen_ms - duration_since_last_update);
        }

        // Collect all pending events.
        let mut events = Vec::new();
        events_loop.poll_events(|event| events.push(event));
		
		
		//Alek's notes: Display ALWAYS nedds updating.
        // If there are no events and the `Ui` does not need updating, wait for the next event.
        //if events.is_empty() && !self.ui_needs_update {
            //events_loop.run_forever(|event| {
                //events.push(event);
                //glium::glutin::ControlFlow::Break
            //});
        //}

        self.ui_needs_update = false;
        self.last_update = std::time::Instant::now();

        events
    }

    /// Notifies the event loop that the `Ui` requires another update whether or not there are any
    /// pending events.
    ///
    /// This is primarily used on the occasion that some part of the `Ui` is still animating and
    /// requires further updates to do so.
    pub fn needs_update(&mut self) {
        self.ui_needs_update = true;
    }

}

#[allow(dead_code)]
fn marker_of_winit_import(){}

use conrod::Scalar;
use conrod::event::Input;
use conrod::input;
use conrod::cursor;


/// Types that have access to a `winit::Window` and can provide the necessary dimensions and hidpi
/// factor for converting `winit::Event`s to `conrod::event::Input`, as well as set the mouse
/// cursor.
///
/// This allows users to pass either `glium::Display`, `glium::glutin::Window` or `winit::Window`
/// to the `conrod::backend::winit::convert` function defined below.
pub trait WinitWindow {
    /// Return the inner size of the window.
    fn get_inner_size(&self) -> Option<(u32, u32)>;
    /// Return the window's DPI factor so that we can convert from pixel values to scalar values.
    fn hidpi_factor(&self) -> f32;
}

impl WinitWindow for winit::Window {
    fn get_inner_size(&self) -> Option<(u32, u32)> {
        winit::Window::get_inner_size(self)
    }
    fn hidpi_factor(&self) -> f32 {
        winit::Window::hidpi_factor(self)
    }
}

#[cfg(feature = "glium")]
impl WinitWindow for glium::Display {
    fn get_inner_size(&self) -> Option<(u32, u32)> {
        self.gl_window().get_inner_size()
    }
    fn hidpi_factor(&self) -> f32 {
        self.gl_window().hidpi_factor()
    }
}


/// A function for converting a `winit::Event` to a `conrod::event::Input`.
///
/// This can be useful for single-window applications.
pub fn convert_event<W>(e: winit::Event, window: &W) -> Option<Input>
    where W: WinitWindow,
{
    match e {
        winit::Event::WindowEvent { event, .. } => convert_window_event(event, window),
        _ => None,
    }
}

/// A function for converting a `winit::WindowEvent` to a `conrod::event::Input`.
///
/// This is useful for multi-window applications.
pub fn convert_window_event<W>(e: winit::WindowEvent, window: &W) -> Option<Input>
    where W: WinitWindow,
{
    // The window size in points.
    let (win_w, win_h) = match window.get_inner_size() {
        Some((w, h)) => (w as Scalar, h as Scalar),
        None => return None,
    };

    // The "dots per inch" factor. Multiplying this by `win_w` and `win_h` gives the framebuffer
    // width and height.
    let dpi_factor = window.hidpi_factor() as Scalar;

    // Translate the coordinates from top-left-origin-with-y-down to centre-origin-with-y-up.
    //
    // winit produces input events in pixels, so these positions need to be divided by the widht
    // and height of the window in order to be DPI agnostic.
    let tx = |x: Scalar| (x / dpi_factor) - win_w / 2.0;
    let ty = |y: Scalar| -((y / dpi_factor) - win_h / 2.0);

    match e {

        winit::WindowEvent::Resized(w, h) => {
            let w = (w as Scalar / dpi_factor) as u32;
            let h = (h as Scalar / dpi_factor) as u32;
            Some(Input::Resize(w, h).into())
        },

        winit::WindowEvent::ReceivedCharacter(ch) => {
            let string = match ch {
                // Ignore control characters and return ascii for Text event (like sdl2).
                '\u{7f}' | // Delete
                '\u{1b}' | // Escape
                '\u{8}'  | // Backspace
                '\r' | '\n' | '\t' => "".to_string(),
                _ => ch.to_string()
            };
            Some(Input::Text(string).into())
        },

        winit::WindowEvent::Focused(focused) =>
            Some(Input::Focus(focused).into()),

        winit::WindowEvent::KeyboardInput { input, .. } => {
            input.virtual_keycode.map(|key| {
                match input.state {
                    winit::ElementState::Pressed =>
                        Input::Press(input::Button::Keyboard(map_key(key))).into(),
                    winit::ElementState::Released =>
                        Input::Release(input::Button::Keyboard(map_key(key))).into(),
                }
            })
        },

        winit::WindowEvent::Touch(winit::Touch { phase, location: (x, y), id, .. }) => {
            let phase = match phase {
                winit::TouchPhase::Started => input::touch::Phase::Start,
                winit::TouchPhase::Moved => input::touch::Phase::Move,
                winit::TouchPhase::Cancelled => input::touch::Phase::Cancel,
                winit::TouchPhase::Ended => input::touch::Phase::End,
            };
            let xy = [tx(x), ty(y)];
            let id = input::touch::Id::new(id);
            let touch = input::Touch { phase: phase, id: id, xy: xy };
            Some(Input::Touch(touch).into())
        }

        winit::WindowEvent::CursorMoved { position: (x, y), .. } => {
            let x = tx(x as Scalar);
            let y = ty(y as Scalar);
            let motion = input::Motion::MouseCursor { x: x, y: y };
            Some(Input::Motion(motion).into())
        },

        winit::WindowEvent::MouseWheel { delta, .. } => match delta {

            winit::MouseScrollDelta::PixelDelta(x, y) => {
                let x = x as Scalar / dpi_factor;
                let y = -y as Scalar / dpi_factor;
                let motion = input::Motion::Scroll { x: x, y: y };
                Some(Input::Motion(motion).into())
            },

            winit::MouseScrollDelta::LineDelta(x, y) => {
                // This should be configurable (we should provide a LineDelta event to allow for this).
                const ARBITRARY_POINTS_PER_LINE_FACTOR: Scalar = 10.0;
                let x = ARBITRARY_POINTS_PER_LINE_FACTOR * x as Scalar;
                let y = ARBITRARY_POINTS_PER_LINE_FACTOR * -y as Scalar;
                Some(Input::Motion(input::Motion::Scroll { x: x, y: y }).into())
            },
        },

        winit::WindowEvent::MouseInput { state, button, .. } => match state {
            winit::ElementState::Pressed =>
                Some(Input::Press(input::Button::Mouse(map_mouse(button))).into()),
            winit::ElementState::Released =>
                Some(Input::Release(input::Button::Mouse(map_mouse(button))).into()),
        },

        winit::WindowEvent::Refresh => {
            Some(Input::Redraw)
        },

        _ => None,
    }
}

/// Maps winit's key to a conrod `Key`.
pub fn map_key(keycode: winit::VirtualKeyCode) -> input::keyboard::Key {
    use input::keyboard::Key;

    match keycode {
        winit::VirtualKeyCode::Key0 => Key::D0,
        winit::VirtualKeyCode::Key1 => Key::D1,
        winit::VirtualKeyCode::Key2 => Key::D2,
        winit::VirtualKeyCode::Key3 => Key::D3,
        winit::VirtualKeyCode::Key4 => Key::D4,
        winit::VirtualKeyCode::Key5 => Key::D5,
        winit::VirtualKeyCode::Key6 => Key::D6,
        winit::VirtualKeyCode::Key7 => Key::D7,
        winit::VirtualKeyCode::Key8 => Key::D8,
        winit::VirtualKeyCode::Key9 => Key::D9,
        winit::VirtualKeyCode::A => Key::A,
        winit::VirtualKeyCode::B => Key::B,
        winit::VirtualKeyCode::C => Key::C,
        winit::VirtualKeyCode::D => Key::D,
        winit::VirtualKeyCode::E => Key::E,
        winit::VirtualKeyCode::F => Key::F,
        winit::VirtualKeyCode::G => Key::G,
        winit::VirtualKeyCode::H => Key::H,
        winit::VirtualKeyCode::I => Key::I,
        winit::VirtualKeyCode::J => Key::J,
        winit::VirtualKeyCode::K => Key::K,
        winit::VirtualKeyCode::L => Key::L,
        winit::VirtualKeyCode::M => Key::M,
        winit::VirtualKeyCode::N => Key::N,
        winit::VirtualKeyCode::O => Key::O,
        winit::VirtualKeyCode::P => Key::P,
        winit::VirtualKeyCode::Q => Key::Q,
        winit::VirtualKeyCode::R => Key::R,
        winit::VirtualKeyCode::S => Key::S,
        winit::VirtualKeyCode::T => Key::T,
        winit::VirtualKeyCode::U => Key::U,
        winit::VirtualKeyCode::V => Key::V,
        winit::VirtualKeyCode::W => Key::W,
        winit::VirtualKeyCode::X => Key::X,
        winit::VirtualKeyCode::Y => Key::Y,
        winit::VirtualKeyCode::Z => Key::Z,
        winit::VirtualKeyCode::Apostrophe => Key::Unknown,
        winit::VirtualKeyCode::Backslash => Key::Backslash,
        winit::VirtualKeyCode::Back => Key::Backspace,
        // K::CapsLock => Key::CapsLock,
        winit::VirtualKeyCode::Delete => Key::Delete,
        winit::VirtualKeyCode::Comma => Key::Comma,
        winit::VirtualKeyCode::Down => Key::Down,
        winit::VirtualKeyCode::End => Key::End,
        winit::VirtualKeyCode::Return => Key::Return,
        winit::VirtualKeyCode::Equals => Key::Equals,
        winit::VirtualKeyCode::Escape => Key::Escape,
        //winit::VirtualKeyCode::F1 => Key::F1,
        //winit::VirtualKeyCode::F2 => Key::F2,
        //winit::VirtualKeyCode::F3 => Key::F3,
        //winit::VirtualKeyCode::F4 => Key::F4,
        //winit::VirtualKeyCode::F5 => Key::F5,
        //winit::VirtualKeyCode::F6 => Key::F6,
        //winit::VirtualKeyCode::F7 => Key::F7,
        //winit::VirtualKeyCode::F8 => Key::F8,
        //winit::VirtualKeyCode::F9 => Key::F9,
        //winit::VirtualKeyCode::F10 => Key::F10,
        //winit::VirtualKeyCode::F11 => Key::F11,
       //winit::VirtualKeyCode::F12 => Key::F12,
       //winit::VirtualKeyCode::F13 => Key::F13,
        //winit::VirtualKeyCode::F14 => Key::F14,
        //winit::VirtualKeyCode::F15 => Key::F15,
        // K::F16 => Key::F16,
        // K::F17 => Key::F17,
        // K::F18 => Key::F18,
        // K::F19 => Key::F19,
        // K::F20 => Key::F20,
        // K::F21 => Key::F21,
        // K::F22 => Key::F22,
        // K::F23 => Key::F23,
        // K::F24 => Key::F24,
        // Possibly next code.
        // K::F25 => Key::Unknown,
        winit::VirtualKeyCode::Numpad0 => Key::NumPad0,
        winit::VirtualKeyCode::Numpad1 => Key::NumPad1,
        winit::VirtualKeyCode::Numpad2 => Key::NumPad2,
        winit::VirtualKeyCode::Numpad3 => Key::NumPad3,
        winit::VirtualKeyCode::Numpad4 => Key::NumPad4,
        winit::VirtualKeyCode::Numpad5 => Key::NumPad5,
        winit::VirtualKeyCode::Numpad6 => Key::NumPad6,
        winit::VirtualKeyCode::Numpad7 => Key::NumPad7,
        winit::VirtualKeyCode::Numpad8 => Key::NumPad8,
        winit::VirtualKeyCode::Numpad9 => Key::NumPad9,
        winit::VirtualKeyCode::NumpadComma => Key::NumPadDecimal,
        winit::VirtualKeyCode::Divide => Key::NumPadDivide,
        winit::VirtualKeyCode::Multiply => Key::NumPadMultiply,
        winit::VirtualKeyCode::Subtract => Key::NumPadMinus,
        winit::VirtualKeyCode::Add => Key::NumPadPlus,
        winit::VirtualKeyCode::NumpadEnter => Key::NumPadEnter,
        winit::VirtualKeyCode::NumpadEquals => Key::NumPadEquals,
        winit::VirtualKeyCode::LShift => Key::LShift,
        winit::VirtualKeyCode::LControl => Key::LCtrl,
        winit::VirtualKeyCode::LAlt => Key::LAlt,
        winit::VirtualKeyCode::LMenu => Key::LGui,
        winit::VirtualKeyCode::RShift => Key::RShift,
        winit::VirtualKeyCode::RControl => Key::RCtrl,
        winit::VirtualKeyCode::RAlt => Key::RAlt,
        winit::VirtualKeyCode::RMenu => Key::RGui,
        // Map to backslash?
        // K::GraveAccent => Key::Unknown,
        winit::VirtualKeyCode::Home => Key::Home,
        winit::VirtualKeyCode::Insert => Key::Insert,
        winit::VirtualKeyCode::Left => Key::Left,
        winit::VirtualKeyCode::LBracket => Key::LeftBracket,
        // K::Menu => Key::Menu,
        winit::VirtualKeyCode::Minus => Key::Minus,
        winit::VirtualKeyCode::Numlock => Key::NumLockClear,
        winit::VirtualKeyCode::PageDown => Key::PageDown,
        winit::VirtualKeyCode::PageUp => Key::PageUp,
        winit::VirtualKeyCode::Pause => Key::Pause,
        winit::VirtualKeyCode::Period => Key::Period,
        // K::PrintScreen => Key::PrintScreen,
        winit::VirtualKeyCode::Right => Key::Right,
        winit::VirtualKeyCode::RBracket => Key::RightBracket,
        // K::ScrollLock => Key::ScrollLock,
        winit::VirtualKeyCode::Semicolon => Key::Semicolon,
        winit::VirtualKeyCode::Slash => Key::Slash,
        winit::VirtualKeyCode::Space => Key::Space,
        winit::VirtualKeyCode::Tab => Key::Tab,
        winit::VirtualKeyCode::Up => Key::Up,
        // K::World1 => Key::Unknown,
        // K::World2 => Key::Unknown,
        _ => Key::Unknown,
    }
}

/// Maps winit's mouse button to conrod's mouse button.
pub fn map_mouse(mouse_button: winit::MouseButton) -> input::MouseButton {
    use input::MouseButton;
    match mouse_button {
        winit::MouseButton::Left => MouseButton::Left,
        winit::MouseButton::Right => MouseButton::Right,
        winit::MouseButton::Middle => MouseButton::Middle,
        winit::MouseButton::Other(0) => MouseButton::X1,
        winit::MouseButton::Other(1) => MouseButton::X2,
        winit::MouseButton::Other(2) => MouseButton::Button6,
        winit::MouseButton::Other(3) => MouseButton::Button7,
        winit::MouseButton::Other(4) => MouseButton::Button8,
        _ => MouseButton::Unknown
    }
}

/// Convert a given conrod mouse cursor to the corresponding winit cursor type.
pub fn convert_mouse_cursor(cursor: cursor::MouseCursor) -> winit::MouseCursor {
    match cursor {
        cursor::MouseCursor::Text => winit::MouseCursor::Text,
        cursor::MouseCursor::VerticalText => winit::MouseCursor::VerticalText,
        cursor::MouseCursor::Hand => winit::MouseCursor::Hand,
        cursor::MouseCursor::Grab => winit::MouseCursor::Grab,
        cursor::MouseCursor::Grabbing => winit::MouseCursor::Grabbing,
        cursor::MouseCursor::ResizeVertical => winit::MouseCursor::NsResize,
        cursor::MouseCursor::ResizeHorizontal => winit::MouseCursor::EwResize,
        cursor::MouseCursor::ResizeTopLeftBottomRight => winit::MouseCursor::NwseResize,
        cursor::MouseCursor::ResizeTopRightBottomLeft => winit::MouseCursor::NeswResize,
        _ => winit::MouseCursor::Arrow,
    }
}

#[allow(dead_code)]
fn marker_of_glium_import(){}

use conrod::Rect;
use conrod::render;
use conrod::text;


/// A `Command` describing a step in the drawing process.
#[derive(Clone, Debug)]
pub enum Command<'a> {
    /// Draw to the target.
    Draw(Draw<'a>),
    /// Update the scizzor within the `glium::DrawParameters`.
    Scizzor(glium::Rect),
}

/// A `Command` for drawing to the target.
///
/// Each variant describes how to draw the contents of the vertex buffer.
#[derive(Clone, Debug)]
pub enum Draw<'a> {
    /// A range of vertices representing triangles textured with the image in the
    /// image_map at the given `widget::Id`.
    Image(conrod::image::Id, &'a [Vertex]),
    /// A range of vertices representing plain triangles.
    Plain(&'a [Vertex]),
}

enum PreparedCommand {
    Image(conrod::image::Id, std::ops::Range<usize>),
    Plain(std::ops::Range<usize>),
    Scizzor(glium::Rect),
}

/// A rusttype `GlyphCache` along with a `glium::texture::Texture2d` for caching text on the `GPU`.
pub struct GlyphCache {
    cache: text::GlyphCache<'static>,
    texture: glium::texture::Texture2d,
}

/// A type used for translating `render::Primitives` into `Command`s that indicate how to draw the
/// conrod GUI using `glium`.
pub struct Renderer {
    program: glium::Program,
    glyph_cache: GlyphCache,
    commands: Vec<PreparedCommand>,
    vertices: Vec<Vertex>,
}

/// An iterator yielding `Command`s, produced by the `Renderer::commands` method.
pub struct Commands<'a> {
    commands: std::slice::Iter<'a, PreparedCommand>,
    vertices: &'a [Vertex],
}

/// Possible errors that may occur during a call to `Renderer::new`.
#[derive(Debug)]
pub enum RendererCreationError {
    /// Errors that might occur when creating the glyph cache texture.
    Texture(glium::texture::TextureCreationError),
    /// Errors that might occur when constructing the shader program.
    Program(glium::program::ProgramChooserCreationError),
}

/// Possible errors that may occur during a call to `Renderer::draw`.
#[derive(Debug)]
pub enum DrawError {
    /// Errors that might occur upon construction of a `glium::VertexBuffer`.
    Buffer(glium::vertex::BufferCreationError),
    /// Errors that might occur when drawing to the `glium::Surface`.
    Draw(glium::DrawError),
}

/// The `Vertex` type passed to the vertex shader.
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    /// The mode with which the `Vertex` will be drawn within the fragment shader.
    ///
    /// `0` for rendering text.
    /// `1` for rendering an image.
    /// `2` for rendering non-textured 2D geometry.
    ///
    /// If any other value is given, the fragment shader will not output any color.
    pub mode: u32,
    /// The position of the vertex within vector space.
    ///
    /// [-1.0, -1.0] is the leftmost, bottom position of the display.
    /// [1.0, 1.0] is the rightmost, top position of the display.
    pub position: [f32; 2],
    /// The coordinates of the texture used by this `Vertex`.
    ///
    /// [0.0, 0.0] is the leftmost, bottom position of the texture.
    /// [1.0, 1.0] is the rightmost, top position of the texture.
    pub tex_coords: [f32; 2],
    /// A color associated with the `Vertex`.
    ///
    /// The way that the color is used depends on the `mode`.
    pub color: [f32; 4],
}

#[allow(unsafe_code)]
mod vertex_impl {
    use super::Vertex;
    implement_vertex!(Vertex, position, tex_coords, color, mode);
}

/// Draw text from the text cache texture `tex` in the fragment shader.
pub const MODE_TEXT: u32 = 0;
/// Draw an image from the texture at `tex` in the fragment shader.
pub const MODE_IMAGE: u32 = 1;
/// Ignore `tex` and draw simple, colored 2D geometry.
pub const MODE_GEOMETRY: u32 = 2;


//NB fragment shaders modified, to allow lightening and darkning of images.
/// The vertex shader used within the `glium::Program` for OpenGL.
pub const VERTEX_SHADER_120: &'static str = "
    #version 120

    attribute vec2 position;
    attribute vec2 tex_coords;
    attribute vec4 color;
    attribute float mode;

    varying vec2 v_tex_coords;
    varying vec4 v_color;
    varying float v_mode;

    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
        v_tex_coords = tex_coords;
        v_color = color;
        v_mode = mode;
    }
";

/// The fragment shader used within the `glium::Program` for OpenGL.
pub const FRAGMENT_SHADER_120: &'static str = "
    #version 120
    uniform sampler2D tex;

    varying vec2 v_tex_coords;
    varying vec4 v_color;
    varying float v_mode;

    void main() {
        // Text
        if (v_mode == 0.0) {
            gl_FragColor = v_color * vec4(1.0, 1.0, 1.0, texture2D(tex, v_tex_coords).r);

        // Image
        } else if (v_mode == 1.0) {
            gl_FragColor = v_color * texture2D(tex, v_tex_coords);

        // 2D Geometry
        } else if (v_mode == 2.0) {
            gl_FragColor = v_color;
        }
    }
";

/// The vertex shader used within the `glium::Program` for OpenGL.
pub const VERTEX_SHADER_140: &'static str = "
    #version 140

    in vec2 position;
    in vec2 tex_coords;
    in vec4 color;
    in uint mode;

    out vec2 v_tex_coords;
    out vec4 v_color;
    flat out uint v_mode;

    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
        v_tex_coords = tex_coords;
        v_color = color;
        v_mode = mode;
    }
";

/// The fragment shader used within the `glium::Program` for OpenGL.
pub const FRAGMENT_SHADER_140: &'static str = "
    #version 140
    uniform sampler2D tex;

    in vec2 v_tex_coords;
    in vec4 v_color;
    flat in uint v_mode;

    out vec4 f_color;

    void main() {
        // Text
        if (v_mode == uint(0)) {
            f_color = v_color * vec4(1.0, 1.0, 1.0, texture(tex, v_tex_coords).r);

        // Image
        } else if (v_mode == uint(1)) {
            f_color = v_color * texture(tex, v_tex_coords);

        // 2D Geometry
        } else if (v_mode == uint(2)) {
            f_color = v_color;
        }
    }
";

/// The vertex shader used within the `glium::Program` for OpenGL ES.
pub const VERTEX_SHADER_300_ES: &'static str = "
    #version 300 es
    precision mediump float;

    in vec2 position;
    in vec2 tex_coords;
    in vec4 color;
    in uint mode;

    out vec2 v_tex_coords;
    out vec4 v_color;
    flat out uint v_mode;

    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
        v_tex_coords = tex_coords;
        v_color = color;
        v_mode = mode;
    }
";

/// The fragment shader used within the `glium::Program` for OpenGL ES.
pub const FRAGMENT_SHADER_300_ES: &'static str = "
    #version 300 es
    precision mediump float;
    uniform sampler2D tex;

    in vec2 v_tex_coords;
    in vec4 v_color;
    flat in uint v_mode;

    out vec4 f_color;

    void main() {
        // Text
        if (v_mode == uint(0)) {
            f_color = v_color * vec4(1.0, 1.0, 1.0, texture(tex, v_tex_coords).r);

        // Image
        } else if (v_mode == uint(1)) {
            f_color = v_color * texture(tex, v_tex_coords);

        // 2D Geometry
        } else if (v_mode == uint(2)) {
            f_color = v_color;
        }
    }
";

/// Glium textures that have two dimensions.
pub trait TextureDimensions {
    /// The width and height of the texture.
    fn dimensions(&self) -> (u32, u32);
}

impl<T> TextureDimensions for T
    where T: std::ops::Deref<Target=glium::texture::TextureAny>,
{
    fn dimensions(&self) -> (u32, u32) {
        (self.get_width(), self.get_height().unwrap_or(0))
    }
}


/// Construct the glium shader program that can be used to render `Vertex`es.
pub fn program<F>(facade: &F) -> Result<glium::Program, glium::program::ProgramChooserCreationError>
    where F: glium::backend::Facade,
{
    program!(facade,
             120 => { vertex: VERTEX_SHADER_120, fragment: FRAGMENT_SHADER_120 },
             140 => { vertex: VERTEX_SHADER_140, fragment: FRAGMENT_SHADER_140 },
             300 es => { vertex: VERTEX_SHADER_300_ES, fragment: FRAGMENT_SHADER_300_ES })
}

/// Default glium `DrawParameters` with alpha blending enabled.
pub fn draw_parameters() -> glium::DrawParameters<'static> {
    let blend = glium::Blend::alpha_blending();
    glium::DrawParameters { multisampling: true,
							blend: blend,
							 ..Default::default() }
}


/// Converts gamma (brightness) from sRGB to linear color space.
///
/// sRGB is the default color space for image editors, pictures, internet etc.
/// Linear gamma yields better results when doing math with colors.
pub fn gamma_srgb_to_linear(c: [f32; 4]) -> [f32; 4] {
    fn component(f: f32) -> f32 {
        // Taken from https://github.com/PistonDevelopers/graphics/src/color.rs#L42
        if f <= 0.04045 {
            f / 12.92
        } else {
            ((f + 0.055) / 1.055).powf(2.4)
        }
    }
    [component(c[0]), component(c[1]), component(c[2]), c[3]]
}


/// Return the optimal client format for the text texture given the version.
pub fn text_texture_client_format(opengl_version: &glium::Version) -> glium::texture::ClientFormat {
    match *opengl_version {
        // If the version is greater than or equal to GL 3.0 or GLes 3.0, we can use the `U8` format.
        glium::Version(_, major, _) if major >= 3 => glium::texture::ClientFormat::U8,
        // Otherwise, we must use the `U8U8U8` format to support older versions.
        _ => glium::texture::ClientFormat::U8U8U8,
    }
}

/// Return the optimal uncompressed float format for the text texture given the version.
pub fn text_texture_uncompressed_float_format(opengl_version: &glium::Version) -> glium::texture::UncompressedFloatFormat {
    match *opengl_version {
        // If the version is greater than or equal to GL 3.0 or GLes 3.0, we can use the `U8` format.
        glium::Version(_, major, _) if major >= 3 => glium::texture::UncompressedFloatFormat::U8,
        // Otherwise, we must use the `U8U8U8` format to support older versions.
        _ => glium::texture::UncompressedFloatFormat::U8U8U8,
    }
}


impl GlyphCache {

    /// Construct a `GlyphCache` with a size equal to the given `Display`'s current framebuffer
    /// dimensions.
    pub fn new<F>(facade: &F) -> Result<Self, glium::texture::TextureCreationError>
        where F: glium::backend::Facade,
    {
        const SCALE_TOLERANCE: f32 = 0.1;
        const POSITION_TOLERANCE: f32 = 0.1;

        let context = facade.get_context();
        let (w, h) = context.get_framebuffer_dimensions();

        // Determine the optimal texture format to use given the opengl version.
        let opengl_version = context.get_opengl_version();
        let client_format = text_texture_client_format(opengl_version);
        let uncompressed_float_format = text_texture_uncompressed_float_format(opengl_version);

        // Construct the `GlyphCache`.
        let num_components = client_format.get_num_components() as u32;

        let buffer_w = num_components * w;

        // First, the rusttype `Cache` which performs the logic for rendering and laying out glyphs
        // in the cache.
        let cache = text::GlyphCache::new(w, h, SCALE_TOLERANCE, POSITION_TOLERANCE);

        // Now the texture to which glyphs will be rendered.
        let grey_image = glium::texture::RawImage2d {
            data: std::borrow::Cow::Owned(vec![128u8; buffer_w as usize * h as usize]),
            width: w,
            height: h,
            format: client_format,
        };
        let format = uncompressed_float_format;
        let no_mipmap = glium::texture::MipmapsOption::NoMipmap;
        let texture = try!(glium::texture::Texture2d::with_format(facade, grey_image, format, no_mipmap));

        Ok(GlyphCache {
            cache: cache,
            texture: texture,
        })
    }

    /// The texture used to cache the glyphs on the GPU.
    pub fn texture(&self) -> &glium::texture::Texture2d {
        &self.texture
    }

}


impl Renderer {

    /// Construct a new empty `Renderer`.
    pub fn new<F>(facade: &F) -> Result<Self, RendererCreationError>
        where F: glium::backend::Facade,
    {
        let program = try!(program(facade));
        let glyph_cache = try!(GlyphCache::new(facade));
        Ok(Renderer {
            program: program,
            glyph_cache: glyph_cache,
            commands: Vec::new(),
            vertices: Vec::new(),
        })
    }

    /// Produce an `Iterator` yielding `Command`s.
    pub fn commands(&self) -> Commands {
        let Renderer { ref commands, ref vertices, .. } = *self;
        Commands {
            commands: commands.iter(),
            vertices: vertices,
        }
    }

    /// Fill the inner vertex and command buffers by translating the given `primitives`.
    pub fn fill<P, T>(&mut self,
                      display: &glium::Display,
                      mut primitives: P,
                      image_map: &conrod::image::Map<T>)
        where P: render::PrimitiveWalker,
              T: TextureDimensions,
    {
        let Renderer { ref mut commands, ref mut vertices, ref mut glyph_cache, .. } = *self;

        commands.clear();
        vertices.clear();

        // This is necessary for supporting rusttype's GPU cache with OpenGL versions older than GL
        // 3.0 and GL ES 3.0. It is used to convert from the `U8` data format given by `rusttype`
        // to the `U8U8U8` format that is necessary for older versions of OpenGL.
        //
        // The buffer is only used if an older version was detected, otherwise the text GPU cache
        // uses the rusttype `data` buffer directly.
        let mut text_data_u8u8u8 = Vec::new();

        // Determine the texture format that we're using.
        let opengl_version = display.get_opengl_version();
        let client_format = text_texture_client_format(opengl_version);

        enum State {
            Image { image_id: conrod::image::Id, start: usize },
            Plain { start: usize },
        }

        let mut current_state = State::Plain { start: 0 };

        // Switches to the `Plain` state and completes the previous `Command` if not already in the
        // `Plain` state.
        macro_rules! switch_to_plain_state {
            () => {
                match current_state {
                    State::Plain { .. } => (),
                    State::Image { image_id, start } => {
                        commands.push(PreparedCommand::Image(image_id, start..vertices.len()));
                        current_state = State::Plain { start: vertices.len() };
                    },
                }
            };
        }

        // Framebuffer dimensions and the "dots per inch" factor.
        let (screen_w, screen_h) = display.get_framebuffer_dimensions();
        let (win_w, win_h) = (screen_w as Scalar, screen_h as Scalar);
        let half_win_w = win_w / 2.0;
        let half_win_h = win_h / 2.0;
        let dpi_factor = display.gl_window().hidpi_factor() as Scalar;

        // Functions for converting for conrod scalar coords to GL vertex coords (-1.0 to 1.0).
        let vx = |x: Scalar| (x * dpi_factor / half_win_w) as f32;
        let vy = |y: Scalar| (y * dpi_factor / half_win_h) as f32;

        let mut current_scizzor = glium::Rect {
            left: 0,
            width: screen_w,
            bottom: 0,
            height: screen_h,
        };

        let rect_to_glium_rect = |rect: Rect| {
            let (w, h) = rect.w_h();
            let left = (rect.left() * dpi_factor + half_win_w) as u32;
            let bottom = (rect.bottom() * dpi_factor + half_win_h) as u32;
            let width = (w * dpi_factor) as u32;
            let height = (h * dpi_factor) as u32;
            glium::Rect {
                left: std::cmp::max(left, 0),
                bottom: std::cmp::max(bottom, 0),
                width: std::cmp::min(width, screen_w),
                height: std::cmp::min(height, screen_h),
            }
        };

        // Draw each primitive in order of depth.
        while let Some(primitive) = primitives.next_primitive() {
            let render::Primitive { kind, scizzor, rect, .. } = primitive;

            // Check for a `Scizzor` command.
            let new_scizzor = rect_to_glium_rect(scizzor);
            if new_scizzor != current_scizzor {
                // Finish the current command.
                match current_state {
                    State::Plain { start } =>
                        commands.push(PreparedCommand::Plain(start..vertices.len())),
                    State::Image { image_id, start } =>
                        commands.push(PreparedCommand::Image(image_id, start..vertices.len())),
                }

                // Update the scizzor and produce a command.
                current_scizzor = new_scizzor;
                commands.push(PreparedCommand::Scizzor(new_scizzor));

                // Set the state back to plain drawing.
                current_state = State::Plain { start: vertices.len() };
            }

            match kind {

                render::PrimitiveKind::Rectangle { color } => {
                    switch_to_plain_state!();

                    let color = gamma_srgb_to_linear(color.to_fsa());
                    let (l, r, b, t) = rect.l_r_b_t();

                    let v = |x, y| {
                        // Convert from conrod Scalar range to GL range -1.0 to 1.0.
                        Vertex {
                            position: [vx(x), vy(y)],
                            tex_coords: [0.0, 0.0],
                            color: color,
                            mode: MODE_GEOMETRY,
                        }
                    };

                    let mut push_v = |x, y| vertices.push(v(x, y));

                    // Bottom left triangle.
                    push_v(l, t);
                    push_v(r, b);
                    push_v(l, b);

                    // Top right triangle.
                    push_v(l, t);
                    push_v(r, b);
                    push_v(r, t);
                },

                render::PrimitiveKind::TrianglesSingleColor { color, triangles } => {
                    if triangles.is_empty() {
                        continue;
                    }

                    switch_to_plain_state!();

                    let colour = gamma_srgb_to_linear(color.into());

                    let v = |p: [Scalar; 2]| {
                        Vertex {
                            position: [vx(p[0]), vy(p[1])],
                            tex_coords: [0.0, 0.0],
                            color: colour,
                            mode: MODE_GEOMETRY,
                        }
                    };

                    for triangle in triangles {
                        vertices.push(v(triangle[0]));
                        vertices.push(v(triangle[1]));
                        vertices.push(v(triangle[2]));
                    }
                },

                render::PrimitiveKind::TrianglesMultiColor { triangles } => {
                    if triangles.is_empty() {
                        continue;
                    }

                    switch_to_plain_state!();

                    let v = |(p, c): ([Scalar; 2], color::Rgba)| {
                        Vertex {
                            position: [vx(p[0]), vy(p[1])],
                            tex_coords: [0.0, 0.0],
                            color: gamma_srgb_to_linear(c.into()),
                            mode: MODE_GEOMETRY,
                        }
                    };

                    for triangle in triangles {
                        vertices.push(v(triangle[0]));
                        vertices.push(v(triangle[1]));
                        vertices.push(v(triangle[2]));
                    }
                },

                render::PrimitiveKind::Text { color, text, font_id } => {
                    switch_to_plain_state!();

                    let positioned_glyphs = text.positioned_glyphs(dpi_factor as f32);

                    let GlyphCache { ref mut cache, ref mut texture } = *glyph_cache;

                    // Queue the glyphs to be cached.
                    for glyph in positioned_glyphs.iter() {
                        cache.queue_glyph(font_id.index(), glyph.clone());
                    }

                    // Cache the glyphs on the GPU.
                    cache.cache_queued(|rect, data| {
                        let w = rect.width();
                        let h = rect.height();
                        let glium_rect = glium::Rect {
                            left: rect.min.x,
                            bottom: rect.min.y,
                            width: w,
                            height: h,
                        };

                        let data = match client_format {
                            // `rusttype` gives data in the `U8` format so we can use it directly.
                            glium::texture::ClientFormat::U8 => std::borrow::Cow::Borrowed(data),
                            // Otherwise we have to convert to the supported format.
                            glium::texture::ClientFormat::U8U8U8 => {
                                text_data_u8u8u8.clear();
                                for &b in data.iter() {
                                    text_data_u8u8u8.push(b);
                                    text_data_u8u8u8.push(b);
                                    text_data_u8u8u8.push(b);
                                }
                                std::borrow::Cow::Borrowed(&text_data_u8u8u8[..])
                            },
                            // The text cache is only ever created with U8 or U8U8U8 formats.
                            _ => unreachable!(),
                        };

                        let image = glium::texture::RawImage2d {
                            data: data,
                            width: w,
                            height: h,
                            format: client_format,
                        };
                        texture.main_level().write(glium_rect, image);
                    }).unwrap();

                    let color = gamma_srgb_to_linear(color.to_fsa());

                    let cache_id = font_id.index();

                    let origin = text::rt::point(0.0, 0.0);
                    let to_gl_rect = |screen_rect: text::rt::Rect<i32>| text::rt::Rect {
                        min: origin
                            + (text::rt::vector(screen_rect.min.x as f32 / screen_w as f32 - 0.5,
                                          1.0 - screen_rect.min.y as f32 / screen_h as f32 - 0.5)) * 2.0,
                        max: origin
                            + (text::rt::vector(screen_rect.max.x as f32 / screen_w as f32 - 0.5,
                                          1.0 - screen_rect.max.y as f32 / screen_h as f32 - 0.5)) * 2.0
                    };

                    for g in positioned_glyphs {
                        if let Ok(Some((uv_rect, screen_rect))) = cache.rect_for(cache_id, g) {
                            let gl_rect = to_gl_rect(screen_rect);
                            let v = |p, t| Vertex {
                                position: p,
                                tex_coords: t,
                                color: color,
                                mode: MODE_TEXT,
                            };
                            let mut push_v = |p, t| vertices.push(v(p, t));
                            push_v([gl_rect.min.x, gl_rect.max.y], [uv_rect.min.x, uv_rect.max.y]);
                            push_v([gl_rect.min.x, gl_rect.min.y], [uv_rect.min.x, uv_rect.min.y]);
                            push_v([gl_rect.max.x, gl_rect.min.y], [uv_rect.max.x, uv_rect.min.y]);
                            push_v([gl_rect.max.x, gl_rect.min.y], [uv_rect.max.x, uv_rect.min.y]);
                            push_v([gl_rect.max.x, gl_rect.max.y], [uv_rect.max.x, uv_rect.max.y]);
                            push_v([gl_rect.min.x, gl_rect.max.y], [uv_rect.min.x, uv_rect.max.y]);
                        }
                    }
                },

                render::PrimitiveKind::Image { image_id, color, source_rect } => {

                    // Switch to the `Image` state for this image if we're not in it already.
                    let new_image_id = image_id;
                    match current_state {

                        // If we're already in the drawing mode for this image, we're done.
                        State::Image { image_id, .. } if image_id == new_image_id => (),

                        // If we were in the `Plain` drawing state, switch to Image drawing state.
                        State::Plain { start } => {
                            commands.push(PreparedCommand::Plain(start..vertices.len()));
                            current_state = State::Image {
                                image_id: new_image_id,
                                start: vertices.len(),
                            };
                        },

                        // If we were drawing a different image, switch state to draw *this* image.
                        State::Image { image_id, start } => {
                            commands.push(PreparedCommand::Image(image_id, start..vertices.len()));
                            current_state = State::Image {
                                image_id: new_image_id,
                                start: vertices.len(),
                            };
                        },
                    }
					
					//NB applying colour correction here seems to do nothing.
					//Could this be cackground or what?
                    //let color = 

                    let (image_w, image_h) = image_map.get(&image_id).unwrap().dimensions();
                    let (image_w, image_h) = (image_w as Scalar, image_h as Scalar);

                    // Get the sides of the source rectangle as uv coordinates.
                    //
                    // Texture coordinates range:
                    // - left to right: 0.0 to 1.0
                    // - bottom to top: 0.0 to 1.0
                    let (uv_l, uv_r, uv_b, uv_t) = match source_rect {
                        Some(src_rect) => {
                            let (l, r, b, t) = src_rect.l_r_b_t();
                            ((l / image_w) as f32,
                             (r / image_w) as f32,
                             (b / image_h) as f32,
                             (t / image_h) as f32)
                        },
                        None => (0.0, 1.0, 0.0, 1.0),
                    };

                    let v = |x, y, t| {
                        // Convert from conrod Scalar range to GL range -1.0 to 1.0.
                        let x = (x * dpi_factor as Scalar / half_win_w) as f32;
                        let y = (y * dpi_factor as Scalar / half_win_h) as f32;
                        Vertex {
                            position: [x, y],
                            tex_coords: t,
                            color: [1.0,1.0,1.0,1.0], 
                            //gamma_srgb_to_linear(correction,color.unwrap_or(color::BLACK).to_fsa()),
                            mode: MODE_IMAGE,
                        }
                    };

                    let mut push_v = |x, y, t| vertices.push(v(x, y, t));

                    let (l, r, b, t) = rect.l_r_b_t();

                    // Bottom left triangle.
                    push_v(l, t, [uv_l, uv_t]);
                    push_v(r, b, [uv_r, uv_b]);
                    push_v(l, b, [uv_l, uv_b]);

                    // Top right triangle.
                    push_v(l, t, [uv_l, uv_t]);
                    push_v(r, b, [uv_r, uv_b]);
                    push_v(r, t, [uv_r, uv_t]);
                },

                // We have no special case widgets to handle.
                render::PrimitiveKind::Other(_) => (),
            }

        }

        // Enter the final command.
        match current_state {
            State::Plain { start } =>
                commands.push(PreparedCommand::Plain(start..vertices.len())),
            State::Image { image_id, start } =>
                commands.push(PreparedCommand::Image(image_id, start..vertices.len())),
        }
    }

    /// Draws using the inner list of `Command`s to the given `display`.
    ///
    /// Note: If you require more granular control over rendering, you may want to use the `fill`
    /// and `commands` methods separately. This method is simply a convenience wrapper around those
    /// methods for the case that the user does not require accessing or modifying conrod's draw
    /// parameters, uniforms or generated draw commands.
    //NB: Adjusting brightness of drawn flat textures works. Adjusting brightness of images does sod-all.
    pub fn draw<F, S, T>(&mut self, facade: &mut F, surface: &mut S, image_map: &mut conrod::image::Map<T>,bgc:f32,bgci:f32)
        -> Result<(), DrawError>
        where F: glium::backend::Facade,
              S: glium::Surface,
              for<'a> glium::uniforms::Sampler<'a, T>: glium::uniforms::AsUniformValue,
    {
        let mut draw_params = draw_parameters();
        let mut no_indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        let mut uniforms = uniform! {
            tex: self.glyph_cache.texture()
                .sampled()
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
                .minify_filter(glium::uniforms::MinifySamplerFilter::Linear)
        };

        const NUM_VERTICES_IN_TRIANGLE: usize = 3;
        
        //one more try at adjusting the image colours.
		fn adj_c(f:f32,sh:[f32;4])->[f32;4] {
			let mut shi = sh;
			for i in 0..3 {
				let x = shi[i]*f;						
				shi[i] = if x<0.0 {0.0} else if x<=1.0 {x} else {1.0}
			};
			shi
		}
		
		fn adj_i(f:f32,sh:[f32;4])->[f32;4] {
			let mut shi = sh;
			for i in 0..3 {		
				let x = f;				
				shi[i] = if x<0.0 {0.0}else if x<=2.0 {x} else {2.0};
			};
			shi
		}
		
		//Iterate over commands.				
        for mut command in self.commands() {
            match command {

                // Update the `scizzor` before continuing to draw.
                Command::Scizzor(mut scizzor) => draw_params.scissor = Some(scizzor),

                // Draw to the target with the given `draw` command.
                Command::Draw(mut draw) => match draw {

                    // Draw text and plain 2D geometry.
                    //
                    // Only submit the vertices if there is enough for at least one triangle.
						
                    Draw::Plain(mut slice) => if slice.len() >= NUM_VERTICES_IN_TRIANGLE {
						
						//I could not for the life of me find a way to mutate the slice.
						let mut pseudo_slice = Vec::new();
						for mut vertex in slice.iter() {
							let mut vx = (*vertex).clone();
							vx.color = adj_c(bgc,vx.color);
							pseudo_slice.push(vx);
						};
						
                        let mut vertex_buffer = try!(glium::VertexBuffer::new(facade, &pseudo_slice[..]));
                        surface.draw(&vertex_buffer, no_indices, &self.program, &uniforms, &draw_params).unwrap();
                    },

                    // Draw an image whose texture data lies within the `image_map` at the
                    // given `id`.
                    //
                    // Only submit the vertices if there is enough for at least one triangle.
                    // NB THIS ALSO SEEMS TO HAVE NO EFFECT on image brightness

                    Draw::Image(mut image_id, mut slice) => if slice.len() >= NUM_VERTICES_IN_TRIANGLE {
						
						
						
						//I could not for the life of me find a way to mutate the slice.
						let mut pseudo_slice = Vec::new();
						for mut vertex in slice.iter() {
							let mut vx = (*vertex).clone();
							vx.color = adj_i(bgci,vx.color);
							pseudo_slice.push(vx);
						};
						
                        let mut vertex_buffer = glium::VertexBuffer::new(facade, &pseudo_slice[..]).unwrap();
                        let mut image = image_map.get(&image_id).unwrap();
                        let mut image_uniforms = uniform! {
                            tex: glium::uniforms::Sampler::new(image)
                                .wrap_function(glium::uniforms::SamplerWrapFunction::Clamp)
                                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
                        };
                        surface.draw(&vertex_buffer, no_indices, &self.program, &image_uniforms, &draw_params).unwrap();
                    },

                }
            }
        }

        Ok(())
    }

}

impl<'a> Iterator for Commands<'a> {
    type Item = Command<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        let Commands { ref mut commands, ref vertices } = *self;
        commands.next().map(|command| match *command {
            PreparedCommand::Scizzor(scizzor) => Command::Scizzor(scizzor),
            PreparedCommand::Plain(ref range) =>
                Command::Draw(Draw::Plain(&vertices[range.clone()])),
            PreparedCommand::Image(id, ref range) =>
                Command::Draw(Draw::Image(id, &vertices[range.clone()])),
        })
    }
}

impl From<glium::texture::TextureCreationError> for RendererCreationError {
    fn from(err: glium::texture::TextureCreationError) -> Self {
        RendererCreationError::Texture(err)
    }
}

impl From<glium::program::ProgramChooserCreationError> for RendererCreationError {
    fn from(err: glium::program::ProgramChooserCreationError) -> Self {
        RendererCreationError::Program(err)
    }
}

impl std::error::Error for RendererCreationError {
    fn description(&self) -> &str {
        match *self {
            RendererCreationError::Texture(ref e) => std::error::Error::description(e),
            RendererCreationError::Program(ref e) => std::error::Error::description(e),
        }
    }
}

impl std::fmt::Display for RendererCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match *self {
            RendererCreationError::Texture(ref e) => std::fmt::Display::fmt(e, f),
            RendererCreationError::Program(ref e) => std::fmt::Display::fmt(e, f),
        }
    }
}

impl From<glium::vertex::BufferCreationError> for DrawError {
    fn from(err: glium::vertex::BufferCreationError) -> Self {
        DrawError::Buffer(err)
    }
}

impl From<glium::DrawError> for DrawError {
    fn from(err: glium::DrawError) -> Self {
        DrawError::Draw(err)
    }
}

impl std::error::Error for DrawError {
    fn description(&self) -> &str {
        match *self {
            DrawError::Buffer(ref e) => std::error::Error::description(e),
            DrawError::Draw(ref e) => std::error::Error::description(e),
        }
    }
}

impl std::fmt::Display for DrawError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match *self {
            DrawError::Buffer(ref e) => std::fmt::Display::fmt(e, f),
            DrawError::Draw(ref e) => std::fmt::Display::fmt(e, f),
        }
    }
}
