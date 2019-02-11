//#![feature(use_extern_macros)]
//#![feature(libc)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(unused_assignments)]
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
///TODO 1: Finish the story engine and write a story.
///
///TODO 2: Return to the AI engine and continue with the etude.
///
///TODO 3: Make the externs and use sections pretty and stop sweeping
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
///		cargo rustc --bin q-moose --release --features="winit glium libc" --target=x86_64-pc-windows-gnu -- -C linker=x86_64-w64-mingw32-gcc -C link-args="-Wl,--subsystem,windows"
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
mod bmoose;
mod cmoose;
mod dmoose;
mod emoose;
mod gmoose;
mod imoose;
mod lmoose;
mod omoose;
mod smoose;
mod xmoose;
mod conrod_support;
mod shared_moose;
mod tales_of_the_great_white_moose;

//Imports
use emoose::event_loop_handler;
use shared_moose::*;
use smoose::{MyStories,Story,Sage};
use smoose::*;
use conrod_support::*;
use tales_of_the_great_white_moose::*;
#[allow(unused_imports)] use gmoose::{set_comm_text,set_widgets_rework,names_of,map_sq_col_img};
#[allow(unused_imports)] use omoose::{parse_music_config,isekai_deguchi,isekai_urusai,isekai_index};
#[allow(unused_imports)] use conrod::UiCell;
//#[allow(unused_imports)] use conrod::widget::button::Interaction;
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
									  the_path,petrified_shrine
									  };

#[allow(unused_imports)] use conrod::color::Colour;
#[allow(unused_imports)] use conrod::{color, widget, Colorable, Labelable, Positionable, Sizeable, Widget};
#[allow(unused_imports)] use glium::Surface;
#[allow(unused_imports)] use image::GenericImage;
#[allow(unused_imports)] use rand::Rng;
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
	let mut pressed_button_marker:Option<conrod::widget::Id> = None;

	//special battle variable for free flowing thoughts.
	//Type annotation ad nauseum.
	//Sends turn information to brain.
	let (mut thought_sender_to_brain, mut thought_receiver_to_brain):
	(SyncSender<(usize,usize,[u8;28],i32,Vec<(u8,u8)>,bool)>,
	Receiver<(usize,usize,[u8;28],i32,Vec<(u8,u8)>,bool)>)
	= std::sync::mpsc::sync_channel(1);

	//Sends preliminary battle information to brain
	let (mut thought_sender_to_brain2, mut thought_receiver_to_brain2):
	(SyncSender<((Vec<Vec<[u8;28]>>,Vec<(Lifeform,usize,[Option<[usize;2]>;2])>),bool)>,
	Receiver<((Vec<Vec<[u8;28]>>,Vec<(Lifeform,usize,[Option<[usize;2]>;2])>),bool)>)
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
	 //Currently placeholder.
	let mut stories:Vec<Story> = vec![void_bridge_or_black_tower(&mons_faces),ghosthunt_part_1(&mons_faces),ghosthunt_part_2a(&mons_faces)];
	println!("number of stories: {}",stories.len());
	let mut my_stories:MyStories = MyStories::new();
	let mut my_dungeons:MyDungeons = MyDungeons::new();
	let mut my_kills:KillList = KillList::new();
	let mut diff:i32 = 0;
	let mut p_names_m:Vec<&str> = Vec::with_capacity(5);
	let mut p_names:Vec<String> = Vec::with_capacity(5);
	let mut party:Vec<(Lifeform,usize)> = Vec::with_capacity(5);
	let mut p_loc:Place = world[8][6].clone();
	let mut sages:Vec<Sage> = Vec::new();
	let mut pl:(usize,usize) = (13,5);
	let mut provisional_loc:(usize,usize) = pl.clone();
	let mut truly_quit:bool = false;
	let mut pause = false;
	let mut scenery_index:usize = 0;
	let mut centre_w:f64 = 0.0;
	let mut centre_h:f64 = 0.0;
	let mut gui_box = GUIBox::Uninitiated;
	let mut gui_box_previous = GUIBox::Uninitiated;
	let mut widget_cycler = cmoose::AdvWidgetCycler::new();

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
	b_muse_sender.clone().send(((false,to_play),false));
	muse_silence_sender.clone().send(false);

	//spawn thread.
	let assets2 = assets.clone();

	let music_thread = thread::spawn(move||{
		//initalise flow variable to avoid error upon unwrapping of b_muse_reciever.
		let mut go = (false,to_play);
		let mut kill = false;
		let mut silence = false;
		//loop (check if music is needed and play if true).
		'player:loop {
			//check if music is needed (b_muse_sender status)
			let ketsuron = isekai_deguchi((go,kill).clone(),&mut b_muse_receiver);
			go = ketsuron.0;
			kill = ketsuron.1;

			//Break the thread, if the end is night.
			if kill {
				println!("Report from music_thread: Killing the music.");
				break 'player;
			};

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
				Ok((scenario,true)) => {
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
					//println!("past ai_part_b1");
					let cause_effect_means = imoose::ai_accelerator_hash(&lore,
																		&differences,
																		&mut last_lines,
																		&mut lore_hash_by_end,
																		&mut cause_effect,
																		&mut all_causes);
					//println!("past ai_accelerator");
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
				Ok((scenario,false)) => {
					println!("Report from brain: I am now shutting down.");
					break 'battle;
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

		ui = event_loop_handler(&mut event_loop,
		                          &mut display,
		                          ui,
		                          &mut events_loop,
		                          &mut pl,
		                          &mut wo,
		                          &mut pause,
		                          &mut p_loc,
		                          &mut dungeon_pointer,
		                          &mut idungeon,
		                          &world,
		                          &mut coords,
		                          timer,
		                          &mut freeze_timer,
		                          &mut comm_text,
		                          &mut gui_box,
		                          &mut gui_box_previous,
		                          &mut widget_cycler,
								  &mut party,
		                          &mut enemies,
								  &mut encounter,
		                          &mut battle_ifast,
		                          &mons,
		                          &dungeons);

		//if n_s_l_q_f[4] {println!("got here Z3");};

		//last minute comm_text corrections.
		gmoose::correct_comm_text(&mut comm_text,
								  pause,
								  &mut gui_box);

		//Check whether to enter a story dialog.
		gui_box.check_for_story(&stories,&mut my_stories,
										 &mut my_dungeons,
										 &mut my_kills,
										 &scapes,
										 &p_loc,
										 &party,
										 &mut centre_w,
										 &mut centre_h,
										 &mut scenery_index,
										 timer);
		//println!("Before set widgets");
		// Instantiate all widgets in the GUI.
		// This is getting insane.
		// Getting the impression that a closure would be nicer.
		let loop_tuple = set_widgets_rework(ui.set_widgets(), ids,
					&mut pressed_button_marker,
					gui_box,
					gui_box_previous,
					&mut widget_cycler,
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
					&in_battle_record,
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
					&mut my_dungeons,
					&mut my_kills,
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
					b_muse_sender.try_send(((true,to_play),false));

					//println!("The moose must dream.");
					let cpu_n = num_cpus::get();
					let sow:usize = cpu_n;
					//println!("start of AI dreaming on {} threads.",sow);
					let s = PT::now();

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
							let t0=PT::now();
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
								let t1=PT::now();
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
					thought_sender_to_brain2.send(((lore,encounter.clone()),true));
					lore_empty = false;

					let e=PT::now();
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
					b_muse_sender.try_send(((true,to_play),false));

				}else if !x {
					//println!("Got into general battle round circuit.");
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
											&in_battle_record,
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
							//Switch of music player unless you're in a dungeon.
							if idungeon.is_none() {
								b_muse_sender.try_send(((false,to_play),false));
								p_scape = p_loc.scape;
							}else if (dungeon_pointer<2) | (dungeon_pointer>dungeons[idungeon.unwrap()].scenes.len()+1) {
								b_muse_sender.try_send(((false,to_play),false));
								p_scape = p_loc.scape;
							}else{
								p_scape = dungeons[idungeon.unwrap()].scenes[dungeon_pointer-2].scape;
							};

							scenery_index = gmoose::scenery_setter(&scapes,p_scape,&mut centre_w,&mut centre_h);
							println!("Scenery index = {}, dungeon_pointer = {}",scenery_index,dungeon_pointer);

							//
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
								//println!("Past ai turn");
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
												&mut my_kills,
												&mut sprite_pos,
												&mut targets);
								//println!("Past player turn");
								//if !n_s_l_q_f[4] {b_muse_sender.try_send((false,to_play));};
								//println!("E");
							};
						};
					};
				};
			},
			GUIBox::MainQuit(x) => {
				if x {
					println!("Quitting main loop.");
					thought_sender_to_brain2.send(((Vec::new(),Vec::new()),false));
					b_muse_sender.send(((false,to_play),true));
					stories = Vec::new();
					my_stories= MyStories::new();
					my_dungeons = MyDungeons::new();
					my_kills = KillList::new();
					p_names_m = Vec::new();
					p_names = Vec::new();
					party = Vec::new();
					sages = Vec::new();
					gui_box = GUIBox::Uninitiated;
					gui_box_previous = GUIBox::Uninitiated;
					encounter = Vec::new();
					enemies = Vec::new();
					aftermath = (ghost(),ghost(),Vec::new());
					sprite_boxer = GraphicsBox::None;
					wo = FlowCWin::new();
					gui_box = GUIBox::Uninitiated;
					ipath = None;
					println!("Important variables reset...");
					
					brain_thread.join().expect("Could not shut down the brain.");
					println!("Brain shut down.");
					music_thread.join().expect("Could not kill the music.");
					println!("Music dead.");
					
					println!("Quitting.");
					break 'main;
				};
			},
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

	let a = PT::now();
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
	let b = PT::now();
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

//#[allow(dead_code)]
//fn interaction_and_times_triggered(button_id: widget::Id, ui: &UiCell) -> (Interaction, u16) {
    //let input = ui.widget_input(button_id);
    //let interaction = input.mouse().map_or(Interaction::Idle, |mouse| {
        //let is_pressed =
            //mouse.buttons.left().is_down()
            //|| ui.global_input().current.touch.values()
                 //.any(|t| t.start.widget == Some(button_id));
        //if is_pressed { Interaction::Press } else { Interaction::Hover }
    //});
    //let times_triggered = (input.clicks().left().count() + input.taps().count()) as u16;
    //(interaction, times_triggered)
//}
