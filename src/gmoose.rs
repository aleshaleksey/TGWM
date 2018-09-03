#![allow(non_snake_case)]
#![allow(unused_must_use)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
extern crate conrod;
extern crate glium;
extern crate image;
extern crate winit;
///
///Quest for the Moose:gmoose
///
///The gmoose module is responsible for all GUI related functions
///and game logic. In the GUI version of the game they are somewhat
///difficult to seperate, which has led to a somewhat bloated module.
///
///Some functions that use glium or image are stored in the main.rs file
///as it seems that otherwise they end up using the "wrong" glium/image.
///
///Some unused functions from the original Moosequest have not been
///removed, as the code may be useful for re-introduction of features.
///
///gmoose depends on lmoose, imoose, and dmoose, when smoose is finished
///it will also formally depend on smoose.
///
///~Alek Zholobenko
///
extern crate find_folder;
#[allow(unused_imports)] extern crate num_cpus;
extern crate inflector;
#[allow(unused_imports)] extern crate num;
extern crate rand;
extern crate time;

//mod dmoose;
//mod imoose;
//mod lmoose;
//mod smoose;
use lmoose;
use imoose;
use smoose;
#[allow(unused_imports)] use inflector::Inflector;
#[allow(unused_imports)] use num::Num;
#[allow(unused_imports)] use rand::Rng;
use std;
#[allow(unused_imports)] use std::ffi::OsStr;
#[allow(unused_imports)] use std::fs::File;
#[allow(unused_imports)] use std::clone;
#[allow(unused_imports)] use std::cmp;
#[allow(unused_imports)] use std::env;
#[allow(unused_imports)] use std::fmt::{self, Formatter, Display};
#[allow(unused_imports)] use std::fs::{self, DirEntry};
#[allow(unused_imports)] use std::io;
#[allow(unused_imports)] use std::io::Read;
#[allow(unused_imports)] use std::io::Write;
#[allow(unused_imports)] use std::mem;
#[allow(unused_imports)] use std::mem::transmute;
#[allow(unused_imports)] use std::ops::Deref;
#[allow(unused_imports)] use std::path::{Component, Path, PathBuf};
#[allow(unused_imports)] use std::prelude::*;
#[allow(unused_imports)] use std::str::Split;
#[allow(unused_imports)] use std::sync::mpsc::{sync_channel,SyncSender,Receiver,TryRecvError};
#[allow(unused_imports)] use std::thread;
#[allow(unused_imports)] use std::time::Duration;
#[allow(unused_imports)] use time::{PreciseTime};
#[allow(unused_imports)] use std::f32;

#[allow(unused_imports)] use conrod::Dimensions;
#[allow(unused_imports)] use conrod::color::{Colour, Color, hsl, hsla, rgb, rgba};
#[allow(unused_imports)] use conrod::text::Justify;
#[allow(unused_imports)] use conrod::{color, widget, Borderable, Colorable, Labelable, Positionable, Sizeable, Widget};
#[allow(unused_imports)] use glium::Surface;
#[allow(unused_imports)] use conrod::widget::BorderedRectangle;
#[allow(unused_imports)] use imoose::permit_a;
#[allow(unused_imports)] use cmoose::{FlowCWin,GraphicsBox,SpriteBox,SpellBoxL,SpellBoxF,SpellBoxI,SpellBoxT,
									  SpellBoxH,SpellBoxD,SpellBoxS,SpellBoxR,SpellBoxInferno};
#[allow(unused_imports)] use cmoose::GraphicsBox::{Attack,CastL,CastF,CastH,CastD,CastI,CastS,CastR,CastT,CastInferno};
#[allow(unused_imports)] use lmoose::{Spell,Item,Lifeform,Shade,Place,Dungeon,Landscapes,
			 cureL,cure,cureG,cureH,exorcism,exorcismG,exorcismH,
			 ember,fire,fireball,inferno,spark,lightning,lightningH,crystalliseL,crystallise,crystalliseH,
			 sum_reaper,teleport,teleportG,light,lightH,darkness,darknessH,slow,haste,lifestealer,curse,
			 apocalypse,timestop,sword_of_perseus,bow_of_traveller,dagger_of_fawn,
			 world,goblin_dem,goblin_sco,goblin_witch,bandit,bandit_lord,dark_apprentice,
			 necromancer,necromancer_lord,skeleton,skeleton_kn,ghost,ghost_an,white_witch,beast_green,
			 beast_red,beast_great,fallen,titan,warrior,witch,wonderer,alien,loser,beast_serpent,sage_forsaken,
			 white_queen,
			 shortstaff,
			 convert_affinity,convert_mag_type,convert_mon_type,convert_affinity_rev};	 
#[allow(unused_imports)] use lmoose::{ADVENT,ALBION,ALIEN,ANGEL,BEAST,BONE,BRIDGE,CITY,
		     DEATH,DESERT,ELF,EVIL,FALLEN,FIRE,FOREST,GIANT,GOBLIN,GRASSLAND,
		     HEALING,HIGHLAND,HOLY,HUMAN,ICE,LIGHTNING,MALACHIA,
			 MINDLESS,MOORLAND,MOOSE,RADIANT,RUIN,STEPPE,SPIRIT,
			 TELEPORTATION,TIME,TUNDRA,UNDEAD,VOID,WATER,WITCH,WHITE,NONE,
			 ANY,GROUP,GROUPS,SAME,SELF,SINGLE,TARGET,ALL,BOB,NON,PARTY,
			 
			 S_LESSER_CURE,S_CURE,S_GREATER_CURE,S_SACRED_CURE,S_INFERNO,S_FIREBALL,S_FIRE,S_EMBER,
			 S_LESSER_CRYSTALLISE,S_CRYSTALLISE,S_TRUE_CRYSTALLISE,S_EXORCISM,S_GREATER_EXORCISM,S_SACRED_EXORCISM,
			 S_SUMMON_REAPER,S_TELEPORT,S_GREATER_TELEPORT,S_LIGHT,S_SACRED_LIGHT,S_DARKNESS,S_ABYSSAL_DARKNESS,
			 S_SLOW,S_HASTE,S_APOCALYPSE,S_GENESIS,S_SPARK,S_LIGHTNING,S_JOVIAN_LIGHTNING,S_TIMESTOP,
			 S_CURSE,S_LIFESTEALER,S_DAGGER_OF_FAWN,S_BOW_OF_TRAVELLER,S_SWORD_OF_PERSEUS};			 
			 
#[allow(unused_imports)] use dmoose::{malek_grove,monster_hall,citadel_of_spirit,elven_lake_ruins,malachia_pubcrawl,lost_lighthouse,
									  door_to_darkness,white_temple,stairway,witch_maze,way_down,wild_hunt,tower_of_bones,tower_of_flesh,
									  tower_of_soul,hall_of_stone,the_path,on_the_prairie,ice_palace};
			 
//General constacts.			 
const VOID_TEXT:&str = "You cannot travel through the void.";
const BLANK_THREAD:&str = "";
const SQUARES:[usize;3] = [20,5,2];
const TRAVEL_DELAY:usize = 15;
const ISEKAIN:usize = 19;
const BORDER:f64 = 3.0;
const BORDER_COLOUR:color::Colour = Color::Rgba(237.0/255.0, 212.0/255.0, 0.0, 128.0/255.0);
const BACKGR_COLOUR:color::Colour = color::BLACK;
const BUTTON_COLOUR:color::Colour = color::DARK_RED;
const HOLY_COLOUR:color::Colour = Color::Rgba(255.0/255.0,222.2/255.0,222.2/255.0,200.0/255.0);
const SLIDE_H:f64 = 20.0;
const AI_MEM_MIN:f64 = 10_000_000.0;
const AI_MEM_MAX:f64 = 32_000_000_000.0;
const AI_MEM_DEFAULT:usize = 500_000_000;
const ECLAIR_BALL:f64 = 25.0;
pub const FPS:f32 = 20.0; //Frame rate, will make this variable later.
pub const FPSU:usize = 20; 



//Before I knew about borderable.
pub fn canvas_border(mut can:conrod::widget::Canvas, border:Option<conrod::Scalar>) -> conrod::widget::Canvas {
	can.style.border = border;
	can
}

//Before I knew about borderable.
pub fn canvas_bord_col(mut can:conrod::widget::Canvas,
					   border:Option<conrod::Scalar>,
					   b_colour:color::Colour) -> conrod::widget::Canvas {
	can.style.border = border;
	can.style.border_color = Some(b_colour);
	can
}

// A function to chosoe the font size on labels that pertain to
// The middle column based on the column's size.
fn font_size_chooser_marker(){}
fn font_size_chooser(wh_mc: &[f64;2]) -> u32 {
	if 		(wh_mc[0]<360.1) | (wh_mc[1]<280.1) {10}
	else if (wh_mc[0]<540.1) | (wh_mc[1]<360.1) {12}
	else if (wh_mc[0]<720.1) | (wh_mc[1]<280.1) {14}
	else if (wh_mc[0]<1080.1)| (wh_mc[1]<760.1) {18}
	else 										{22}
}

fn font_size_chooser_button(w:f64) -> u32 {
	if 		w>=1080.0 {40}
	else if w>=800.0  {32}
	else if w>=640.0  {24}
	else			  {16}	
}

fn font_size_chooser_button_b(w:f64) -> u32 {
	if 		w>=800.0  {20}
	else			  {16}	
}

//Function to set the text in the comm_box.
#[allow(unused_variables)]
pub fn set_comm_text(mut comm_text:&mut String, ui: &mut conrod::UiCell, ids: & Ids)	{
	
	let font_size = font_size_chooser_button_b(ui.w_of(ids.master).unwrap_or(800.0));
	*comm_text = comm_text.to_owned().trim_left().to_owned().trim_right().to_owned();
	widget::Text::new(comm_text)
		.color(color::YELLOW)
		.font_size(font_size)
		.left_justify()
		.top_left_of(ids.comm_box)
		.padded_w_of(ids.comm_box,9.0)
		.line_spacing(5.0)
		.set(ids.comm, ui);
}

//function to pick a random scenery image for a battle background.
pub fn scenery_setter(landscapes:&Landscapes, locale:u8, centre_w:&mut f64, centre_h:&mut f64)->usize {
	println!("scenery setter locale = {}",locale);
	
	*centre_w = rand::thread_rng().gen_range(0.0,400.0);
	*centre_h = rand::thread_rng().gen_range(0.0,400.0);
	
	match locale {
		ICE => {rand::thread_rng().gen_range(0,landscapes.ice.len())},
		TUNDRA => {rand::thread_rng().gen_range(0,landscapes.tundra.len())},
		WATER => {rand::thread_rng().gen_range(0,landscapes.water.len())},
		GRASSLAND => {rand::thread_rng().gen_range(0,landscapes.grassland.len())},
		FOREST => {rand::thread_rng().gen_range(0,landscapes.forest.len())},
		STEPPE => {rand::thread_rng().gen_range(0,landscapes.steppe.len())},
		DESERT => {rand::thread_rng().gen_range(0,landscapes.desert.len())},
		CITY => {rand::thread_rng().gen_range(0,landscapes.city.len())},
		HIGHLAND => {rand::thread_rng().gen_range(0,landscapes.highland.len())},
		MOORLAND => {rand::thread_rng().gen_range(0,landscapes.moorland.len())},
		VOID => {0},
		RUIN => {rand::thread_rng().gen_range(0,landscapes.ruin.len())},
		_ => 0,	
	}
}

//function to check if scenery index exceeds number of pictures and needs to be regened.
fn scenery_l_checker(landscapes:&Landscapes, locale:u8)->usize {
	
	match locale {
		ICE => landscapes.ice.len(),
		TUNDRA => landscapes.tundra.len(),
		WATER => landscapes.water.len(),
		GRASSLAND => landscapes.grassland.len(),
		FOREST => landscapes.forest.len(),
		STEPPE => landscapes.steppe.len(),
		DESERT => landscapes.desert.len(),
		CITY => landscapes.city.len(),
		HIGHLAND => landscapes.highland.len(),
		MOORLAND => landscapes.moorland.len(),
		VOID => landscapes.void.len(),
		RUIN => landscapes.ruin.len(),
		_ => 0,	
	}
}

//function to set the background in a battle.
fn set_battle_background(ui: &mut conrod::UiCell, ids: &mut Ids,
						 landscapes: &Landscapes,
						 locale:u8,
						 i:usize,
						 centre_w:&mut f64, centre_h:&mut f64) {
							 
	//println!("scenery setter locale = {}",locale);
	let mut wh = ui.wh_of(ids.middle_column).unwrap();	
	wh = [wh[0]-6.0,wh[1]-6.0];
	let mut pic_wh = landscapes.ice[0].1;
	*centre_w = if wh[0] > pic_wh[0] + *centre_w {*centre_w-(pic_wh[0]+*centre_w-wh[0])}else{*centre_w};
	*centre_h = if wh[1] > pic_wh[1] + *centre_h {*centre_h-(pic_wh[1]+*centre_w-wh[1])}else{*centre_h};
	
	if (pic_wh[0]<wh[0]) | (pic_wh[1]<wh[1]) {
		let snap:conrod::Rect = conrod::Rect::from_xy_dim([wh[0]/2.0+*centre_w,wh[1]/2.0+*centre_h],wh);			 
		let widget = match locale {
			ICE => conrod::widget::Image::new(landscapes.ice[i].0)
										   .wh(wh)
										   .bottom_left_of(ids.middle_column),
			TUNDRA => conrod::widget::Image::new(landscapes.tundra[i].0)
										   .wh(wh)
										   .bottom_left_of(ids.middle_column),
			WATER => conrod::widget::Image::new(landscapes.water[i].0)
										   .wh(wh)
										   .bottom_left_of(ids.middle_column),
			GRASSLAND => conrod::widget::Image::new(landscapes.grassland[i].0)
										   .wh(wh)
										   .bottom_left_of(ids.middle_column),
			FOREST => conrod::widget::Image::new(landscapes.forest[i].0)
										   .wh(wh)
										   .bottom_left_of(ids.middle_column),
			STEPPE => conrod::widget::Image::new(landscapes.steppe[i].0)
										   .wh(wh)
										   .bottom_left_of(ids.middle_column),
			DESERT => conrod::widget::Image::new(landscapes.desert[i].0)
										   .wh(wh)
										   .bottom_left_of(ids.middle_column),
			CITY => conrod::widget::Image::new(landscapes.city[i].0)
										   .wh(wh)
										   .bottom_left_of(ids.middle_column),
			HIGHLAND => conrod::widget::Image::new(landscapes.highland[i].0)
										   .wh(wh)
										   .bottom_left_of(ids.middle_column),
			MOORLAND => conrod::widget::Image::new(landscapes.moorland[i].0)
										   .wh(wh)
										   .bottom_left_of(ids.middle_column),
			VOID => conrod::widget::Image::new(landscapes.ruin[0].0)  //if you somehow get here....
										   .wh(wh)
										   .bottom_left_of(ids.middle_column),
			RUIN => conrod::widget::Image::new(landscapes.ruin[i].0)
										   .wh(wh)
										   .bottom_left_of(ids.middle_column),
			_ => conrod::widget::Image::new(landscapes.void[0].0).wh(wh).bottom_left_of(ids.middle_column),	
		};
		widget.set(ids.battle_background,ui);
	}else{
		let snap:conrod::Rect = conrod::Rect::from_xy_dim([wh[0]/2.0+*centre_w,wh[1]/2.0+*centre_h],wh);			 
		let widget = match locale {
			ICE => conrod::widget::Image::new(landscapes.ice[i].0)
										   .wh(wh)
										   .bottom_left_of(ids.middle_column)
										   .source_rectangle(snap.clone()),
			TUNDRA => conrod::widget::Image::new(landscapes.tundra[i].0)
										   .wh(wh)
										   .bottom_left_of(ids.middle_column)
										   .source_rectangle(snap.clone()),
			WATER => conrod::widget::Image::new(landscapes.water[i].0)
										   .wh(wh)
										   .bottom_left_of(ids.middle_column)
										   .source_rectangle(snap.clone()),
			GRASSLAND => conrod::widget::Image::new(landscapes.grassland[i].0)
										   .wh(wh)
										   .bottom_left_of(ids.middle_column)
										   .source_rectangle(snap.clone()),
			FOREST => conrod::widget::Image::new(landscapes.forest[i].0)
										   .wh(wh)
										   .bottom_left_of(ids.middle_column)
										   .source_rectangle(snap.clone()),
			STEPPE => conrod::widget::Image::new(landscapes.steppe[i].0)
										   .wh(wh)
										   .bottom_left_of(ids.middle_column)
										   .source_rectangle(snap.clone()),
			DESERT => conrod::widget::Image::new(landscapes.desert[i].0)
										   .wh(wh)
										   .bottom_left_of(ids.middle_column)
										   .source_rectangle(snap.clone()),
			CITY => conrod::widget::Image::new(landscapes.city[i].0)
										   .wh(wh)
										   .bottom_left_of(ids.middle_column)
										   .source_rectangle(snap.clone()),
			HIGHLAND => conrod::widget::Image::new(landscapes.highland[i].0)
										   .wh(wh)
										   .bottom_left_of(ids.middle_column)
										   .source_rectangle(snap.clone()),
			MOORLAND => conrod::widget::Image::new(landscapes.moorland[i].0)
										   .wh(wh)
										   .bottom_left_of(ids.middle_column)
										   .source_rectangle(snap.clone()),
			VOID => conrod::widget::Image::new(landscapes.ruin[0].0)  //if you somehow get here....
										   .wh(wh)
										   .bottom_left_of(ids.middle_column)
										   .source_rectangle(snap.clone()),
			RUIN => conrod::widget::Image::new(landscapes.ruin[i].0)
										   .wh(wh)
										   .bottom_left_of(ids.middle_column)
										   .source_rectangle(snap.clone()),
			_ => conrod::widget::Image::new(landscapes.void[0].0).wh(wh).bottom_left_of(ids.middle_column),	
		};
		widget.set(ids.battle_background,ui);
	};
}

//function to set the ever shifting time scape. (Simple diffraction for now)
fn set_timescape(ui: &mut conrod::UiCell, ids: &mut Ids,timer:usize){

	let mut wh = ui.wh_of(ids.middle_column).unwrap();
	let mut centre:conrod::Point = ui.xy_of(ids.middle_column).unwrap();	
	wh = [wh[0]-6.0,wh[1]-6.0];
	
	
	let mut mat = widget::Matrix::new((wh[0]/75.0) as usize,(wh[1]/75.0) as usize)
					.wh(wh)
					.middle_of(ids.middle_column);
					
	let mut mat_a = mat.clone()
					   .set(ids.battle_background_time_a, ui);
	let mut mat_b = mat.clone()
					   .set(ids.battle_background_time_b, ui);
	let mut mat_c = mat.clone()
					   .set(ids.battle_background_time_c, ui);
	let mut mat_d = mat.clone()
					   .set(ids.battle_background_time_d, ui);
					
	let mut start_point = [0.0,0.0];
	let mut end_point = [0.0,0.0];
	
	let max_h = centre[1] + wh[1]/2.0;
	let min_h = centre[1] - wh[1]/2.0;
	let max_w = centre[0] + wh[0]/2.0;
	let min_w = centre[0] - wh[0]/2.0;
	
	while let Some(elem) = mat_a.next(ui) {
		border_crawler_a(centre.clone(),wh.clone(),
						 timer,1,
						 &mut start_point,&mut end_point,
						 elem.col+1,elem.row+1,
						 max_h,min_h,max_w,min_w);		
		elem.set(widget::Line::abs(start_point,end_point).color(color::rgba(elem.col as f32/9.0 - timer as f32%0.5,
																			elem.row as f32/18.0 + timer as f32%0.5,
																			(elem.col + elem.row) as f32/18.0,
																			1.0)),ui);
	}
	while let Some(elem) = mat_b.next(ui) {
		border_crawler_b(centre.clone(),wh.clone(),
						 timer,1,
						 &mut start_point,&mut end_point,
						 elem.col+1,elem.row+1,
						 max_h,min_h,max_w,min_w);		
		elem.set(widget::Line::abs(start_point,end_point).color(color::rgba(elem.col as f32/9.0 - timer as f32%0.5,
																			elem.row as f32/18.0 + timer as f32%0.5,
																			(elem.col + elem.row) as f32/18.0,
																			1.0)),ui);
	}
	while let Some(elem) = mat_c.next(ui) {
		border_crawler_c(centre.clone(),wh.clone(),
						 timer,1,
						 &mut start_point,&mut end_point,
						 elem.col+1,elem.row+1,
						 max_h,min_h,max_w,min_w);		
		elem.set(widget::Line::abs(start_point,end_point).color(color::rgba(elem.col as f32/9.0 - timer as f32%0.5,
																			elem.row as f32/18.0 + timer as f32%0.5,
																			(elem.col + elem.row) as f32/18.0,
																			1.0)),ui);
	}
	while let Some(elem) = mat_d.next(ui) {
		border_crawler_d(centre.clone(),wh.clone(),
						 timer,1,
						 &mut start_point,&mut end_point,
						 elem.col+1,elem.row+1,
						 max_h,min_h,max_w,min_w);		
		elem.set(widget::Line::abs(start_point,end_point).color(color::rgba(elem.col as f32/9.0 - timer as f32%0.5,
																			elem.row as f32/18.0 + timer as f32%0.5,
																			(elem.col + elem.row) as f32/18.0,
																			1.0)),ui);
	}
	
}

fn border_crawler_a(centre:conrod::Point, wh:conrod::Dimensions, 
				  timer:usize, time_base:usize,
				  mut start: &mut conrod::Point,
				  mut end: &mut conrod::Point,
				  c:usize,r:usize,
				  max_h:f64,min_h:f64,
				  max_w:f64,min_w:f64) {
	let progress = (timer*time_base+c*10*r) as f64 %(wh[0]+wh[1]);
	
	start[0] = if progress<wh[0] {min_w+progress}else{max_w};
	end[0] = if progress<wh[1] {min_w}else{min_w+progress-wh[1]};
	start[1] = if progress<wh[0] {max_h}else{max_h+wh[0]-progress};
	end[1] = if progress<wh[1] {max_h-progress}else{min_h};
}

fn border_crawler_b(centre:conrod::Point, wh:conrod::Dimensions, 
				  timer:usize, time_base:usize,
				  mut start: &mut conrod::Point,
				  mut end: &mut conrod::Point,
				  c:usize,r:usize,
				  max_h:f64,min_h:f64,
				  max_w:f64,min_w:f64) {
	let progress = (timer*time_base+c*10*r) as f64 %(wh[0]+wh[1]);
	
	start[0] = if progress<wh[0] {max_w-progress}else{min_w};
	end[0] = if progress<wh[1] {max_w}else{max_w+wh[1]-progress};
	start[1] = if progress<wh[0] {max_h}else{max_h+wh[0]-progress};
	end[1] = if progress<wh[1] {max_h-progress}else{min_h};
}

fn border_crawler_c(centre:conrod::Point, wh:conrod::Dimensions, 
				  timer:usize, time_base:usize,
				  mut start: &mut conrod::Point,
				  mut end: &mut conrod::Point,
				  c:usize,r:usize,
				  max_h:f64,min_h:f64,
				  max_w:f64,min_w:f64) {
	let progress = (timer*time_base+c*10*r) as f64 %(wh[0]+wh[1]);
	
	start[0] = if progress<wh[0] {max_w-progress}else{min_w};
	end[0] = if progress<wh[1] {max_w}else{max_w-wh[1]+progress};
	start[1] = if progress<wh[0] {max_h}else{max_h+wh[0]-progress};
	end[1] = if progress<wh[1] {max_h+progress}else{min_h};
}

fn border_crawler_d(centre:conrod::Point, wh:conrod::Dimensions, 
				  timer:usize, time_base:usize,
				  mut start: &mut conrod::Point,
				  mut end: &mut conrod::Point,
				  c:usize,r:usize,
				  max_h:f64,min_h:f64,
				  max_w:f64,min_w:f64) {
	let progress = (timer*time_base+c*10*r) as f64 %(wh[0]+wh[1]);
	
	start[0] = if progress<wh[1] {max_w}else{max_w+wh[0]-progress};
	end[0] = if progress<wh[0] {min_w-progress}else{max_w};
	start[1] = if progress<wh[1] {max_h-progress}else{min_h};
	end[1] = if progress<wh[0] {min_h}else{min_h-wh[0]+progress};
}

//function to make a sprite attack another.
//moves the attacker by the appropriate amount.
//needs to be inside a "match sprite_boxer {" clause.
fn sprite_approach_marker(){}
fn sprite_approach (sprite_box:&SpriteBox)-> [f64;2] {
	
	let factor = (sprite_box.turns_init-sprite_box.turns_to_go as f64)/sprite_box.turns_init;			 
	let mut dx:f64 = (sprite_box.def_coord[0] - sprite_box.att_coord[0])*factor;
	let mut dy:f64 = (sprite_box.def_coord[1] - sprite_box.att_coord[1])*factor;
	
	[sprite_box.att_coord[0]+dx,sprite_box.att_coord[1]+dy]
}

//it is imperative to decrement this at the end of each turn and make it disappear.
//and to set the sprite to vibrate if the attack actually hits.
fn sprite_box_dec_marker(){}
fn sprite_box_decrement (sprite_boxer:&mut GraphicsBox,
						 wo:&mut FlowCWin,
						 timer:usize,
						 shake_timer:&mut usize,
						 shake_damage:&mut [bool;25]) {
							 
	//Borrow checker makes this annoying.
	let mut nullify = false;
			 
	match *sprite_boxer {
		Attack(ref mut x) => {
				x.turns_to_go-= 1;
				if x.turns_to_go==0 {
					nullify = true;
					shake_damage[x.def_index] = x.damage;
				};
		},
		CastL(ref mut x) => {
				if x.turns_to_go != 0 {
					x.turns_to_go-= 1;
				}else if x.turns_to_go==0 {
					x.turns_after+= 1;
				};
				if x.turns_after>FPSU*2 {
					nullify = true;
					for t in x.targets.iter() {
						shake_damage[*t] = x.damage[*t];
					};
				};
		},
		CastI(ref mut x) => {
				if x.turns_to_go != 0 {
					x.turns_to_go-= 1;
				}else if x.turns_to_go==0 {
					x.turns_after+= 1;
				};
				if x.turns_after>FPSU*2 {
					nullify = true;
					for t in x.targets.iter() {
						shake_damage[*t] = x.damage[*t];
					};
				};
		},
		CastF(ref mut x) => {
				if x.turns_to_go != 0 {
					x.turns_to_go-= 1;
				}else if x.turns_to_go==0 {
					x.turns_after+= 1;
				};
				if x.turns_after>FPSU {
					nullify = true;
					for t in x.targets.iter() {
						shake_damage[*t] = x.damage[*t];
					};
				};
		},
		CastInferno(ref mut x) => {
				if x.turns_to_go != 0 {
					x.turns_to_go-= 1;
				}else if x.turns_after != 0 {
					x.turns_after-= 1;
				}else if x.turns_after2 != 0 {
					x.turns_after2-= 1;
				}else if (x.turns_to_go==0) & (x.turns_after2==0) {
					x.stage_four+= 1;
				};
				if x.stage_four>FPSU {
					nullify = true;
					for t in x.targets.iter() {
						shake_damage[*t] = x.damage[*t];
					};
				};
		},
		CastD(ref mut x) => {
				if x.turns_to_go != 0 {
					x.turns_to_go-= 1;
				}else if x.turns_to_go==0 {
					nullify = true;
					for t in x.targets.iter() {
						shake_damage[*t] = x.damage[*t];
					};
				};
		},
		CastH(ref mut x) => {
				if x.turns_to_go != 0 {
					x.turns_to_go-= 1;
				}else if x.turns_to_go==0 {
					nullify = true;
					for t in x.targets.iter() {
						shake_damage[*t] = x.damage[*t];
					};
				};
		},
		CastS(ref mut x) => {
				if x.turns_to_go != 0 {
					x.turns_to_go-= 1;
				}else if x.turns_to_go==0 {
					nullify = true;
					for t in x.targets.iter() {
						shake_damage[*t] = x.damage[*t];
					};
				};
		},
		CastR(ref mut x) => {
				if x.turns_to_go != 0 {
					x.turns_to_go-= 1;
				}else if x.turns_to_go==0 {
					nullify = true;
					wo.bgc+= x.lightness;
					for t in x.targets.iter() {
						shake_damage[*t] = x.damage[*t];
					};
				};
		},
		CastT(ref mut x) => {
				if x.turns_to_go != 0 {
					x.turns_to_go-= 1;
				}else if x.turns_to_go==0 {
					nullify = true;
					for t in x.targets.iter() {
						shake_damage[*t] = x.damage[*t];
					};
				};
		},
		_ => {},
	};			
	
	if nullify {
		*shake_timer = timer;
		*sprite_boxer = GraphicsBox::None;
		
	};			 
}

//Function to sill graphics box in accourdance with spell type.
fn sprite_box_filler_marker(){}
fn sprite_box_filler(magic:&Spell,
					 gx_box:&mut GraphicsBox,
					 caster:&(Lifeform,usize,[std::option::Option<[usize; 2]>;2]),
					 bifast:usize,
					 to_hit:&Vec<(bool,bool)>,
					 targets: &mut Vec<usize>,
					 sprite_pos: &[[f64;2];25],
					 shaking_dam: &[bool;25]) {
						 
	spell_targets_to_indices(to_hit,targets);

	match magic.Type {
		LIGHTNING => {
			*gx_box = GraphicsBox::CastL
			(
				SpellBoxL::new
				(
					 caster,
					 bifast,
					 targets,
					 sprite_pos,
					 (*shaking_dam).clone()
				)
			);
		},
		ICE => {
			*gx_box = GraphicsBox::CastI
			(
				SpellBoxI::new
				(
					 caster,
					 bifast,
					 targets,
					 sprite_pos,
					 (*shaking_dam).clone()
				)
			);
		},
		FIRE => {
			match magic.id {
				S_INFERNO => {
					//println!("Going for inferno!");
					*gx_box = GraphicsBox::CastInferno
					(
						SpellBoxInferno::new
						(
							caster,
							bifast,
							targets,
							sprite_pos,
							(*shaking_dam).clone()
						)
					);
						
				},
				_		=> {
					*gx_box = GraphicsBox::CastF
					(
						SpellBoxF::new
						(
							 caster,
							 bifast,
							 targets,
							 sprite_pos,
							 (*shaking_dam).clone()
						)
					);
				},
			};
		},
		DEATH => {
			*gx_box = GraphicsBox::CastD
			(
				SpellBoxD::new
				(
					 caster,
					 bifast,
					 targets,
					 sprite_pos,
					 (*shaking_dam).clone()
				)
			);
		},
		HEALING => {
			*gx_box = GraphicsBox::CastH
			(
				SpellBoxH::new
				(
					 caster,
					 bifast,
					 targets,
					 sprite_pos,
					 (*shaking_dam).clone()
				)
			);
		},
		HOLY => {
			*gx_box = GraphicsBox::CastS
			(
				SpellBoxS::new
				(
					 caster,
					 bifast,
					 targets,
					 sprite_pos,
					 (*shaking_dam).clone()
				)
			);
		},
		RADIANT => {
			*gx_box = GraphicsBox::CastR
			(
				SpellBoxR::new
				(
					 caster,
					 bifast,
					 targets,
					 sprite_pos,
					 magic.Light,
					 magic.Illumination,
					 (*shaking_dam).clone()
				)
			);
		},
		TIME => {
			*gx_box = GraphicsBox::CastT
			(
				SpellBoxT::new
				(
					 caster,
					 bifast,
					 targets,
					 sprite_pos,
					 (*shaking_dam).clone(),
					 magic.Light
				)
			);
		},
		_	=> {},
	};
} 


//Function to set spell list in battle.
fn set_battle_spell_menu(ui: &mut conrod::UiCell, ids: &mut Ids, mut comm_text: &mut String,
						spl: &Vec<Spell>,
						mut party: &mut Vec<(Lifeform,usize)>,
						mut to_cast: &mut String,
						battle_ifast: usize) {
	
	//place scroll bar on spells_can canvas.
	widget::Scrollbar::y_axis(ids.spells_can).auto_hide(true).set(ids.spells_can_scroll, ui);
	
	let mut spm_wh:[f64;2] = ui.wh_of(ids.spells_can).unwrap();
	spm_wh = [spm_wh[0]-6.0,spm_wh[1]];
	let butt_wh:[f64;2] = [spm_wh[1],50.0];
	let w_mc:f64 = ui.w_of(ids.master).unwrap_or(1080.0);
	
	//place button matrix of spells on spells_can canvas.
	let mut spell_menu = conrod::widget::Matrix::new(1,party[battle_ifast].0.Spellist.len())
			.mid_top_of(ids.spells_can)
			.wh([spm_wh[0],(party[battle_ifast].0.Spellist.len()*50) as f64])
			.set(ids.spells_mtrx, ui);
	
	//activate spell buttons.
	while let Some(spell) = spell_menu.next(ui) {
		let r  = spell.row;
		let spell_name:String = arcana_name_from_spell_id(spl,party[battle_ifast].0.Spellist[r]);
		let colour = colour_of_magic(arcana_type_from_spell_id(spl,party[battle_ifast].0.Spellist[r]).unwrap());
		let magic_butt = widget::Button::new().label(&spell_name)
											  .label_color(colour.plain_contrast())
											  .label_font_size(font_size_chooser_button_b(w_mc))
											  .wh(butt_wh)
											  .color(colour);
		for _click in spell.set(magic_butt,ui) {
			*comm_text = format!("{}\nYou prepare to cast {}...",comm_text,&spell_name);
			*to_cast = spell_name.clone();
		};
	};
}

//Colour magic buttons with a selected colour
fn colour_of_magic(spell_type: u8) -> conrod::color::Colour {
	match spell_type {
		RADIANT => color::RED.with_luminance(0.85),
		FIRE => color::RED.with_luminance(0.4),
		HEALING => color::GREEN,
		DEATH => color::DARK_GREY.with_luminance(0.1),
		TIME => color::DARK_GREY.with_luminance(0.6),
		LIGHTNING => color::BLUE.with_luminance(0.7),
		HOLY => color::YELLOW.with_luminance(0.85),
		TELEPORTATION => color::DARK_GREY.with_luminance(0.4),
		ICE => color::BLUE.with_luminance(0.8),
		_ => color::WHITE,
	}
}

//Colour a monsterreflecting its type.
fn colour_of_monster(monster_type: u8) -> conrod::color::Colour {

	match monster_type {
		ANGEL => color::YELLOW.with_luminance(0.85),
		BEAST => color::RED,
		UNDEAD => color::DARK_GREY,
		EVIL => color::DARK_GREY.with_luminance(0.1),
		FIRE => color::RED,
		GOBLIN => color::GREEN.with_luminance(0.3),
		HOLY => color::YELLOW.with_luminance(0.85),
		HUMAN => color::YELLOW.with_luminance(0.5),
		ICE => color::BLUE.with_luminance(0.85),
		LIGHTNING => color::BLUE.with_luminance(0.7),
		RADIANT => color::RED.with_luminance(0.85),
		WITCH => color::PURPLE,
		SPIRIT => color::PURPLE.with_luminance(0.8),
		_ => color::WHITE,
	}
}

//A function to set the mutant menus.
#[allow(unused_variables)]
fn set_mutant_menu (ui: &mut conrod::UiCell, ids: &mut Ids,a:&str,b:&str,c:&str,d:&str,e:&str) -> (usize,String) {
	//initiate the general button template.
	let font_size = font_size_chooser(&ui.wh_of(ids.middle_column).unwrap_or([1080.0,800.0]));
	let mut_but = widget::Button::new().color(BACKGR_COLOUR).border(BORDER).border_color(BORDER_COLOUR);
	let mut out:usize = 0;
	//instructions for button 1.
	let mut comm_text = String::new();
	for _click in mut_but.clone().label(a).label_color(color::GREEN.with_luminance(0.66))
										  .wh_of(ids.mut1_box)
										  .label_font_size(font_size)
										  .middle_of(ids.mut1_box)
										  .set(ids.mut1_but,ui){
		comm_text = format!("{}",a);
		set_comm_text(&mut comm_text,ui,ids);
		out = 1;};
	for _click in mut_but.clone().label(b).label_color(color::GREEN.with_luminance(0.66))
										  .wh_of(ids.mut2_box)
										  .label_font_size(font_size)
										  .middle_of(ids.mut2_box)
										  .set(ids.mut2_but,ui){
		comm_text = format!("{}",b);
		set_comm_text(&mut comm_text,ui,ids);
		out = 2;};
	for _click in mut_but.clone().label(c).label_color(color::GREEN.with_luminance(0.66))
										  .wh_of(ids.mut3_box)
										  .label_font_size(font_size)
										  .middle_of(ids.mut3_box)
										  .set(ids.mut3_but,ui){
		comm_text = format!("{}",c);
		set_comm_text(&mut comm_text,ui,ids);
		out = 3;};
	for _click in mut_but.clone().label(d).label_color(color::GREEN.with_luminance(0.66))
										  .wh_of(ids.mut4_box)
										  .label_font_size(font_size)
										  .middle_of(ids.mut4_box)
										  .set(ids.mut4_but,ui){
		comm_text = format!("{}",d);
		set_comm_text(&mut comm_text,ui,ids);
		out = 4;};
	for _click in mut_but.clone().label(e).label_color(color::GREEN.with_luminance(0.66))
										  .wh_of(ids.mut5_box)
										  .label_font_size(font_size)
										  .middle_of(ids.mut5_box)
										  .set(ids.mut5_but,ui){
		comm_text = format!("{}",e);
		set_comm_text(&mut comm_text,ui,ids);
		out = 5;};
	
	(out,comm_text)
}

#[allow(unused_variables)]
fn set_mutant_menu_bin (ui: &mut conrod::UiCell, ids: &mut Ids,a:&str,e:&str,comm_text:String) -> (usize,String) {
	//initiate the general button template.
	let font_size = font_size_chooser(&ui.wh_of(ids.middle_column).unwrap_or([1080.0,800.0]));
	let mut_but = widget::Button::new().color(BACKGR_COLOUR).border(BORDER).border_color(BORDER_COLOUR);
	let mut out:usize = 0;
	//instructions for button 1.
	let mut comm_text = String::new();
	for _click in mut_but.clone().label(a).label_color(color::GREEN.with_luminance(0.66))
										  .wh_of(ids.mut1_box)
										  .label_font_size(font_size)
										  .middle_of(ids.mut1_box)
										  .set(ids.mut1_but,ui){
		comm_text = comm_text;
		out = 1;};
	for _click in mut_but.clone().label(e).label_color(color::GREEN.with_luminance(0.66))
										  .wh_of(ids.mut5_box)
										  .label_font_size(font_size)
										  .middle_of(ids.mut5_box)
										  .set(ids.mut5_but,ui){
		//comm_text = format!("You pressed \"{}\".",e);
		set_comm_text(&mut comm_text,ui,ids);
		out = 5;};
	
	(out,comm_text)
}

#[allow(unused_variables)]
fn set_mutant_menu_uni (ui: &mut conrod::UiCell, ids: &mut Ids,e:&str) -> (usize,String) {
	//initiate the general button template.
	let font_size = font_size_chooser(&ui.wh_of(ids.middle_column).unwrap_or([1080.0,800.0]));
	let mut_but = widget::Button::new().color(BACKGR_COLOUR).border(BORDER).border_color(BORDER_COLOUR);
	let mut out:usize = 0;
	//instructions for button 1.
	let mut comm_text = String::new();
	for _click in mut_but.clone().label(e).label_color(color::GREEN.with_luminance(0.66))
										  .wh_of(ids.mut5_box)
										  .label_font_size(font_size)
										  .middle_of(ids.mut5_box)
										  .set(ids.mut5_but,ui){
		out = 5;
	};	
	(out,comm_text)
}

#[allow(unused_variables)]
fn set_mutant_menu_tri (ui: &mut conrod::UiCell, ids: &mut Ids,a:&str,b:&str,e:&str) -> (usize,String) {
	//initiate the general button template.
	let font_size = font_size_chooser(&ui.wh_of(ids.middle_column).unwrap_or([1080.0,800.0]));
	let mut_but = widget::Button::new().color(BACKGR_COLOUR).border(BORDER).border_color(BORDER_COLOUR);
	let mut out:usize = 0;
	//instructions for button 1.
	let mut comm_text = String::new();
	for _click in mut_but.clone().label(a).label_color(color::GREEN.with_luminance(0.66))
										  .wh_of(ids.mut1_box)
										  .middle_of(ids.mut1_box)
										  .set(ids.mut1_but,ui){
		out = 1;
	};
	for _click in mut_but.clone().label(b).label_color(color::GREEN.with_luminance(0.66))
										  .wh_of(ids.mut2_box)
										  .middle_of(ids.mut2_box)
										  .set(ids.mut2_but,ui){
		out = 2;
	};
	for _click in mut_but.clone().label(e).label_color(color::GREEN.with_luminance(0.66))
										  .wh_of(ids.mut5_box)
										  .middle_of(ids.mut5_box)
										  .set(ids.mut5_but,ui){
		out = 5;
	};	
	(out,comm_text)
}

#[allow(unused_variables)]
fn quitter_marker(){}
fn quitter (ui: &mut conrod::UiCell, ids: &mut Ids, mut n_s_l_q_f: [bool;7], mut truly_quit: &mut bool) -> [bool;7] {
			   
	//get window size and set all the sizes as is appropriate.	   
	let win_wh = ui.wh_of(ids.master).unwrap_or([1080.0,800.0]);
	
	let canvas = widget::Canvas::new().length_weight(1.0);
	//draw quit canvas
	widget::Canvas::new().flow_right(&[
				//(ids.header, widget::Canvas::new().color(color::BLUE).pad_bottom(2.0)),
				(ids.quit_true_can, canvas.clone().color(color::BLACK)
												  .pad(BORDER)),
				(ids.quit_false_can, canvas.clone().color(color::BLACK)
												   .pad(BORDER)),
			]).border(BORDER)
			  .border_color(BORDER_COLOUR)
			  .set(ids.master, ui);
	
	let mut button = widget::Button::new().label_font_size(font_size_chooser_button(win_wh[0]));
	
	for _click in button.clone().color(color::DARK_RED).label("QUIT!").label_color(color::DARK_RED.complement())
						.w_of(ids.quit_true_can).h(200.0)
						.border(BORDER)
						.border_color(BORDER_COLOUR)
						.mid_left_of(ids.quit_true_can).set(ids.quit_true_but,ui){
		*truly_quit = true;
	};
	for _click in button.color(color::DARK_GREEN).label("Please don't...").label_color(color::DARK_GREEN.complement())
						.w_of(ids.quit_false_can).h(200.0)
						.border(BORDER)
						.border_color(BORDER_COLOUR)
						.mid_right_of(ids.quit_false_can).set(ids.quit_false_but,ui){
		n_s_l_q_f[3] = false;
	};
	n_s_l_q_f	   
}

// A number of functions for making text with various settings.

fn text_maker0(text: widget::Text, col:Colour, x:u32) -> widget::Text { text.color(col).font_size(x) }

fn text_maker1(text: &str, col:Colour, x:u32) -> widget::Text {
	 widget::Text::new(text).color(col).font_size(x)
}

fn text_makert(text: &str, col:Colour, x:u32) -> widget::Text {
	 widget::Text::new(text).color(col)
							.font_size(x)
							.left_justify()
}

fn text_maker_r(text: &str, col:Colour, x:u32) -> widget::Text {
	 widget::Text::new(text).color(col)
							.font_size(x)
							.right_justify()
}

fn text_maker_m(text: &str, col:Colour, x:u32) -> widget::Text {
	 widget::Text::new(text).color(col)
							.font_size(x)
							.center_justify()
}

//Sets the lightning strike upon casting a lightning spell.
fn set_lightning_marker(){}
fn set_lightning(ids: &mut Ids, ref mut ui: &mut conrod::UiCell,
				 sbl:&mut SpellBoxL,
				 sprite_pos: &mut [[f64;2];25]) {
	
	let caster_pos:&[f64;2] = &sprite_pos[sbl.caster_indx];
	let mut eclair_matrix = widget::Matrix::new(25,1)
							  .wh([0.0;2])
							  .xy([0.0;2])
							  .set(ids.eclair_matrix, ui);
	
	//Draw lightning strikes.
	if sbl.turns_to_go>0 {
		for i in 0..sbl.targets.len() {
			//extend lightning path for each target.
			sprite_lightning_extender(&sprite_pos[sbl.targets[i]],
									  &mut sbl.paths[i],
									  sbl.turns_to_go,
									  sbl.turns_init);
			if let Some(mut thing) = eclair_matrix.next(ui) {
				
				let mut path = widget::PointPath::abs(sbl.paths[i].clone())
					.thickness((sbl.turns_to_go%3) as f64)
					.color(color::BLUE.with_luminance(0.1*(sbl.turns_to_go%5) as f32));
					
				thing.set(path,ui);
			};
		};
	//Draw hit spheres.
	}else if (sbl.turns_after<FPSU*2) & (sbl.turns_after%4 != 0) {
		let mut eclair_matrix_two = widget::Matrix::new(5,5)
							  .wh([5.0*100.0,5.0*100.0])
							  .xy([0.0;2])
							  .set(ids.eclair_matrix_two, ui);

		for i in 0..sbl.targets.len() {
			if let Some(mut thing) = eclair_matrix_two.next(ui) {
				//println!("We got a lightning rod!");
				thing.rel_x = sprite_pos[sbl.targets[i]][0];
				thing.rel_y = sprite_pos[sbl.targets[i]][1];
				let size = 20.0+2.0*(((sbl.turns_after+i)%12) as f64);
				thing.w = size;
				thing.h = size;
				
				let mut circle = widget::Circle::fill_with(size,
					color::BLUE.with_luminance(0.7).with_alpha(0.07)
				);
				
				thing.set(circle,ui);
			};
		};
	};
}

// extends lightning path from last point towards target.
fn sprite_lightning_extender_marker(){}
fn sprite_lightning_extender(target_pos:&[f64;2],
							 ref mut path:&mut Vec<[f64;2]>,
							 n:usize,i:f64){
	let var_gen = rand::thread_rng().gen_range(0,2) as f64; 
	let last:[f64;2] = *path.last().unwrap_or(target_pos);
	let factor = (i-n as f64)/i;
	let mut dx:f64 = factor*(target_pos[0]-last[0]);
	let mut dy:f64 = factor*(target_pos[1]-last[1]);
	dx+= 0.3*dx*(var_gen);
	dy+= 0.3*dy*(1.0-var_gen);
	path.push([last[0]+dx,last[1]+dy]);
}

//Set fireballs.
fn set_fire_marker(){}
fn set_fire(ids: &mut Ids, ref mut ui: &mut conrod::UiCell,
				 sbf:&mut SpellBoxF,
				 sprite_pos: &mut [[f64;2];25]) {
	
	let caster_pos:&[f64;2] = &sprite_pos[sbf.caster_indx];
	let mut fire_matrix = widget::Matrix::new(5,5)
							  //.wh_of(ids.middle_column)
							  .wh([5.0*100.0,5.0*100.0])
							  .xy([0.0;2])
							  .set(ids.fire_matrix, ui);
	
	if (sbf.turns_to_go>0) & (sbf.turns_to_go%4 != 0) {
		for i in 0..sbf.targets.len() {
			//extend lightning path for each target.
			sprite_fire_extender(&sprite_pos[sbf.targets[i]],
									  &mut sbf.tracks[i],
									  sbf.turns_to_go,
									  sbf.turns_init);
			if let Some(mut thing) = fire_matrix.next(ui) {
				
				thing.rel_x = sbf.tracks[i][0];
				thing.rel_y = sbf.tracks[i][1];
				let size = 10.0;
				thing.w = size*2.0;
				thing.h = size*2.0;
				
				let mut circle = widget::Circle::fill_with(size,
					color::RED.with_luminance(0.7).with_alpha(0.07)
				);
				thing.set(circle,ui);
			};
		};
	}else if (sbf.turns_after<FPSU) & (sbf.turns_after%4 != 0) {
		
		for i in 0..sbf.targets.len() {
			if let Some(mut thing) = fire_matrix.next(ui) {
				
				thing.rel_x = sprite_pos[sbf.targets[i]][0];
				thing.rel_y = sprite_pos[sbf.targets[i]][1];
				let size = 20.0+5.0*((sbf.turns_after%FPSU) as f64);
				thing.w = size;
				thing.h = size;
				
				let mut circle = widget::Circle::fill_with(size,
					color::RED.with_luminance(0.7).with_alpha(0.07)
				);
				thing.set(circle,ui);
			};
		};
	};
}

fn set_inferno_marker(){}
fn set_inferno(ids: &mut Ids, ref mut ui: &mut conrod::UiCell,
				 sbi:&mut SpellBoxInferno,
				 sprite_pos: &mut [[f64;2];25]) {
	
	let caster_pos:&[f64;2] = &sprite_pos[sbi.caster_indx];
	
	let mut fire_matrix = widget::Matrix::new(5,5)
							  .wh([5.0*100.0,5.0*100.0])
							  .xy([0.0;2])
							  .set(ids.fire_matrix, ui);
							  
	let mut eclair_matrix = widget::Matrix::new(25,1)
							  .wh([0.0;2])
							  .xy([0.0;2])
							  .set(ids.eclair_matrix, ui);
	
	if (sbi.turns_to_go>0) & (sbi.turns_to_go%4 != 0) {
		for i in 0..sbi.targets.len() {
			//extend lightning path for each target.
			sprite_lightning_extender(&sprite_pos[sbi.targets[i]],
									  &mut sbi.paths[i],
									  sbi.turns_to_go,
									  sbi.turns_init);
			if let Some(mut thing) = eclair_matrix.next(ui) {
				
				let mut path = widget::PointPath::abs(sbi.paths[i].clone())
					.thickness((sbi.turns_to_go%3) as f64)
					.color(color::RED.with_luminance(0.1*(sbi.turns_to_go%5) as f32));
					
				thing.set(path,ui);
			};
		};
	}else if (sbi.turns_after>0) & (sbi.turns_after%4 != 0) {
		for i in 0..sbi.targets.len() {
			//extend lightning path for each target.
			sprite_fire_extender(caster_pos,
							     &mut sbi.tracks[i],
							     sbi.turns_after,
							     sbi.turns_init);
			if let Some(mut thing) = fire_matrix.next(ui) {
				
				thing.rel_x = sbi.tracks[i][0];
				thing.rel_y = sbi.tracks[i][1];
				let size = 10.0+(sbi.turns_after%5) as f64;
				thing.w = size*2.0;
				thing.h = size*2.0;
				
				let mut circle = widget::Circle::fill_with(size,
					color::RED.with_luminance(0.7).with_alpha(0.07)
				);
				thing.set(circle,ui);
			};
		};
	}else if (sbi.turns_after2>0) & (sbi.turns_after2%4 != 0) {
		for i in 0..sbi.targets.len() {
			//extend lightning path for each target.
			sprite_fire_extender(&sprite_pos[sbi.targets[i]],
							     &mut sbi.tracks[i],
							     sbi.turns_after2,
							     sbi.turns_init);
			if let Some(mut thing) = fire_matrix.next(ui) {
				
				thing.rel_x = sbi.tracks[i][0];
				thing.rel_y = sbi.tracks[i][1];
				let size = 10.0+(sbi.turns_after%5) as f64;
				thing.w = size*2.0;
				thing.h = size*2.0;
				
				let mut circle = widget::Circle::fill_with(size,
					color::RED.with_luminance(0.7).with_alpha(0.07)
				);
				thing.set(circle,ui);
			};
		};
	}else if (sbi.stage_four<FPSU) & (sbi.stage_four%4 != 0) {
		
		for i in 0..sbi.targets.len() {
			if let Some(mut thing) = fire_matrix.next(ui) {
				
				thing.rel_x = sprite_pos[sbi.targets[i]][0];
				thing.rel_y = sprite_pos[sbi.targets[i]][1];
				let size = 40.0+5.0*((sbi.stage_four%FPSU) as f64);
				thing.w = size;
				thing.h = size;
				
				let mut circle = widget::Circle::fill_with(size,
					color::RED.with_luminance(0.7).with_alpha(0.07)
				);
				thing.set(circle,ui);
			};
		};
	};
}

//throw the "fireball" at targets.
//NB, to return it, use caster_position for target_pos.
fn sprite_fire_extender_marker(){}
fn sprite_fire_extender(target_pos:&[f64;2],
							 path:&mut [f64;2],
							 n:usize,i:f64){
	let var_gen = rand::thread_rng().gen_range(0,2) as f64; 
	let last:[f64;2] = [path[0],path[1]];
	let factor = (i-n as f64)/i;
	let mut dx:f64 = factor*(target_pos[0]-last[0]);
	let mut dy:f64 = factor*(target_pos[1]-last[1]);
	dx+= 0.2*dx*(var_gen);
	dy+= 0.2*dy*(1.0-var_gen);
	*path = [last[0]+dx,last[1]+dy];
}

//Set ice effects.
fn set_ice_marker(){}
fn set_ice(ids: &mut Ids, ref mut ui: &mut conrod::UiCell,
				 sbi:&mut SpellBoxI,
				 sprite_pos: &mut [[f64;2];25]) {
	
	let caster_pos:&[f64;2] = &sprite_pos[sbi.caster_indx];
	let mut ice_matrix = widget::Matrix::new(5,5)
							  .wh([50.0*5.0;2])
							  .xy([0.0;2])
							  .set(ids.ice_matrix, ui);
	//println!("in ice");
	//Draw lightning strikes.
	if sbi.turns_to_go>0 {
		for (i,tar) in sbi.targets.iter().enumerate() {
			//println!("in ice part 1: sbi.tracks.len()={}\nturns_to_go={}",sbi.tracks.len(),sbi.turns_to_go);
			//extend lightning path for each target.
			sprite_ice_extender(&sprite_pos[*tar],
									  &mut sbi.tracks[i],
									  sbi.turns_to_go,
									  sbi.turns_init);
			//println!("in ice part 1a: sbi.tracks.len()={}",sbi.tracks.len());					  
			if let Some(mut thing) = ice_matrix.next(ui) {
				//println!("in thingy 1 i={}",i);
				thing.rel_x = sbi.tracks[i][0];
				//println!("in thingy 1a-: about to get coordinates for poly");
				thing.rel_y = sbi.tracks[i][1];
				//println!("in thingy 1a: about to get coordinates for poly");
				let size:f64 = 20.0+2.0*(((sbi.turns_to_go+i)%12) as f64);
				thing.w = size;
				thing.h = size;
				//println!("in thingy 1b: about to get coordinates for poly");		
				let points = poly_round(size,5,&sbi.tracks[i]);
				//println!("in thingy 1c: about to poly construct");		
				let mut poly = widget::Polygon::abs_outline(points)
						.color(color::BLUE.with_luminance(0.8).with_alpha(0.6));
				//println!("in thingy 2: poly contructed");	
				thing.set(poly,ui);
				//println!("in thingy 3: poly set");
			};
		};
	//Draw hit spheres.
	}else if (sbi.turns_after<FPSU*2) & (sbi.turns_after%6 != 0) {

		for i in 0..sbi.targets.len() {
			//println!("in ice part 2");
			if let Some(mut thing) = ice_matrix.next(ui) {
				
				thing.rel_x = sprite_pos[sbi.targets[i]][0];
				thing.rel_y = sprite_pos[sbi.targets[i]][1];
				let size = 12.0+5.0*(((sbi.turns_after+i)%12) as f64);
				thing.w = size;
				thing.h = size;
				
				let points = poly_round(size,5,&sprite_pos[sbi.targets[i]]);
				
				let mut poly = widget::Polygon::abs_fill(points)
						.color(color::BLUE.with_luminance(0.8).with_alpha(0.06));
				
				thing.set(poly,ui);
			};
		};
	};
}

//Move crystals from source.
fn sprite_ice_extender_marker(){}
fn sprite_ice_extender(target_pos:&[f64;2],
							 path:&mut [f64;2],
							 n:usize,i:f64){
	//println!("extending ice");
	let var_gen = rand::thread_rng().gen_range(0,2) as f64; 
	let last:[f64;2] = [path[0],path[1]];
	let factor = (i-n as f64)/i;
	let mut dx:f64 = factor*(target_pos[0]-last[0]);
	let mut dy:f64 = factor*(target_pos[1]-last[1]);
	dx+= 0.2*dx*(var_gen);
	dy+= 0.2*dy*(1.0-var_gen);
	*path = [last[0]+dx,last[1]+dy];
	//println!("extended ice");
}

//Set Death based spells.
fn set_death_marker(){}
fn set_death(ids: &mut Ids, ref mut ui: &mut conrod::UiCell,
				 sbd:&mut SpellBoxD,
				 sprite_pos: &mut [[f64;2];25]) {
	
	let caster_pos:&[f64;2] = &sprite_pos[sbd.caster_indx];
	let mut death_matrix = widget::Matrix::new(5,5)
							  //.wh_of(ids.middle_column)
							  .wh([5.0*100.0,5.0*100.0])
							  .xy([0.0;2])
							  .set(ids.death_matrix, ui);
	
	if (sbd.turns_to_go>0) & (sbd.turns_to_go%5 != 0) {
		for i in 0..sbd.targets.len() {
			//extend lightning path for each target.
			if let Some(mut thing) = death_matrix.next(ui) {
				
				thing.rel_x = sprite_pos[sbd.targets[i]][0];
				thing.rel_y = sprite_pos[sbd.targets[i]][1];
				let size = (sbd.turns_init-sbd.turns_to_go as f64)*4.0;
				thing.w = size*2.0;
				thing.h = size*2.0;
				
				let mut circle = widget::Circle::outline(size)
					.color(color::BLACK.with_alpha(0.5));
					
				thing.set(circle,ui);
			};
		};
	};
}

//Set healing effects..
fn set_heal_marker(){}
fn set_heal(ids: &mut Ids, ref mut ui: &mut conrod::UiCell,
				 sbh:&mut SpellBoxH,
				 sprite_pos: &mut [[f64;2];25]) {
	
	let caster_pos:&[f64;2] = &sprite_pos[sbh.caster_indx];
	let mut health_matrix = widget::Matrix::new(5,5)
							  //.wh_of(ids.middle_column)
							  .wh([5.0*100.0,5.0*100.0])
							  .xy([0.0;2])
							  .set(ids.healing_matrix, ui);
	
	if (sbh.turns_to_go>0) & (sbh.turns_to_go%7 != 0) {
		for i in 0..sbh.targets.len() {
			//extend lightning path for each target.
			if let Some(mut thing) = health_matrix.next(ui) {
				
				thing.rel_x = sprite_pos[sbh.targets[i]][0];
				thing.rel_y = sprite_pos[sbh.targets[i]][1];
				let size = (sbh.turns_to_go as f64)*4.0;
				thing.w = size*2.0;
				thing.h = size*2.0;
				
				let mut circle = widget::Circle::fill(size)
					.color(color::GREEN.with_alpha(0.1));
					
				thing.set(circle,ui);
			};
		};
	};
}

fn set_time_marker(){}
fn set_time(ids: &mut Ids, ref mut ui: &mut conrod::UiCell,
				 sbt:&mut SpellBoxT,
				 sprite_pos: &mut [[f64;2];25]) {
	
	let caster_pos:&[f64;2] = &sprite_pos[sbt.caster_indx];
	let mut time_matrix = widget::Matrix::new(5,5)
							  //.wh_of(ids.middle_column)
							  .wh([5.0*10.0,5.0*10.0])
							  .xy([0.0;2])
							  .set(ids.time_matrix, ui);
	
	if (sbt.turns_to_go>0) & (sbt.turns_to_go%7 != 0) {
		for i in 0..sbt.targets.len() {
			//extend lightning path for each target.
			if let Some(mut thing) = time_matrix.next(ui) {
				
				let ratio = sbt.turns_to_go as f64/sbt.turns_init;
				let radius = if sbt.light {100.0*ratio}else{100.0*(1.0-ratio)};
				let colour = if sbt.light {
					color::GREEN.with_alpha(0.15)
				}else{
					color::BLACK.with_alpha(0.1)
				};
				
				thing.rel_x = sprite_pos[sbt.targets[i]][0]+radius*sinp(sbt.turns_to_go,FPSU);
				thing.rel_y = sprite_pos[sbt.targets[i]][1]+radius*cosp(sbt.turns_to_go,FPSU);
				let size = 10.0*ratio;
				thing.w = size*2.0;
				thing.h = size*2.0;
				
				let mut circle = widget::Circle::fill(size)
					.color(color::GREEN.with_alpha(0.2));
					
				thing.set(circle,ui);
			};
		};
	};
}

//Set SFX for holy spells.
fn set_holy_marker(){}
fn set_holy(ids: &mut Ids, ref mut ui: &mut conrod::UiCell,
				 sbh:&mut SpellBoxS,
				 sprite_pos: &mut [[f64;2];25]) {
	
	let caster_pos:&[f64;2] = &sprite_pos[sbh.caster_indx];
	let mut holy_matrix = widget::Matrix::new(5,5)
							  //.wh_of(ids.middle_column)
							  .wh([5.0*100.0,5.0*100.0])
							  .xy([0.0;2])
							  .set(ids.holy_matrix, ui);
	
	if (sbh.turns_to_go>0) & (sbh.turns_to_go%7 != 0) {
		for i in 0..sbh.targets.len() {
			//extend lightning path for each target.
			if let Some(mut thing) = holy_matrix.next(ui) {
				
				thing.rel_x = sprite_pos[sbh.targets[i]][0];
				thing.rel_y = sprite_pos[sbh.targets[i]][1];
				let size = (sbh.turns_to_go as f64)*3.0;
				thing.w = size*2.0;
				thing.h = size*2.0;
				let points = poly_star(size,&sprite_pos[sbh.targets[i]]);
				
				let mut star = widget::Polygon::abs_outline(points)
									 .color(HOLY_COLOUR);
					
				thing.set(star,ui);
			};
		};
	};
}

//Set SFX for radiant spells.
fn set_radiant_marker(){}
fn set_radiant(ids:&mut Ids, ref mut ui:&mut conrod::UiCell,
				 sbr:&mut SpellBoxR,
				 sprite_pos:&mut [[f64;2];25]) {
	
	let caster_pos:&[f64;2] = &sprite_pos[sbr.caster_indx];
	let mut radiant_matrix = widget::Matrix::new(25,5)
							  //.wh_of(ids.middle_column)
							  .wh([0.0,0.0])
							  .xy([0.0;2])
							  .set(ids.radiant_matrix, ui);
	
	if sbr.turns_to_go>0 {
		
		let colour:color::Colour = if !sbr.light {
			color::PURPLE.with_luminance(0.05*(sbr.turns_to_go%6) as f32).with_alpha(0.6)
		}else{
			HOLY_COLOUR
		};
		
		for i in 0..sbr.destinations.len() {
			//extend radiance (basically lightning)
			sprite_lightning_extender(&sbr.destinations[i],
									  &mut sbr.paths[i],
									  sbr.turns_to_go,
									  sbr.turns_init);
			if let Some(mut thing) = radiant_matrix.next(ui) {
				//println!("In here, dest:{:?},path:{:?}",sbr.destinations[i],sbr.paths[i]);
				let mut path = widget::PointPath::abs(sbr.paths[i].clone())
					.thickness((sbr.turns_to_go%3) as f64)
					.color(colour);
					
				thing.set(path,ui);
			};
		};
	};
}

// Takes reference to GraphicsBox. Depending on what it contains,
// sets the appropriate spell effect.
// NB this function will be expanded as things get finished.
fn spell_setter_marker(){}
fn spell_setter(ids: &mut Ids, ref mut ui: &mut conrod::UiCell,
				 sprite_boxer:&mut GraphicsBox,
				 sprite_pos: &mut [[f64;2];25]) {
	match sprite_boxer {
		&mut GraphicsBox::None	  => {return},
		&mut CastL(ref mut eclair)=> {set_lightning(ids,ui,eclair,sprite_pos);},
		&mut CastF(ref mut flair) => {set_fire(ids,ui,flair,sprite_pos);},
		&mut CastD(ref mut death) => {set_death(ids,ui,death,sprite_pos);},
		&mut CastH(ref mut health)=> {set_heal(ids,ui,health,sprite_pos);},
		&mut CastI(ref mut ice)   => {set_ice(ids,ui,ice,sprite_pos);},
		&mut CastS(ref mut holy)  => {set_holy(ids,ui,holy,sprite_pos);},
		&mut CastR(ref mut rad)   => {set_radiant(ids,ui,rad,sprite_pos);},
		&mut CastT(ref mut time)  => {set_time(ids,ui,time,sprite_pos);},
		&mut CastInferno(ref mut inf)  => {set_inferno(ids,ui,inf,sprite_pos);},
		_			  	  		  => {return},
	};			 
}

//set up the vertically aligned groups.
//ie group east and west. (See battle_line_h for logic).
fn battle_line_v (ids: &mut Ids, ref mut ui: &mut conrod::UiCell,
				mon_faces: &Vec<[conrod::image::Id;3]>,
				mut mtrx: widget::matrix::Elements,
				group: &Vec<(Lifeform,usize)>,
				mut comm_text: &mut String,
				timer:usize,
				mut yt_adcwpe_bw: &mut [bool;9],
				font_size:u32,
				mut sel_targets:&mut Vec<usize>,
				bh: f64,
				shaking_dam: &mut [bool;25],
				shaking_timer: usize,
				battle_ifast: usize,
				mut pos_bif:&mut (Option<conrod::Point>,[conrod::position::Scalar;2]),
				mut sp: &mut Option<conrod::Point>,
				sprite_boxer:&mut GraphicsBox,
				sprite_pos: &mut [[f64;2];25],
				mtrxp: [f64;2]) {
					
	//println!("Inside blv");				
	let mut map_size = ui.wh_of(ids.middle_column).unwrap();	
	while let Some(mut renegade) = mtrx.next(ui) {
		let r = renegade.row as usize;
		if r < group.len() {
			
			//get absolute position of thingies.
			sprite_pos[group[r].1] = [mtrxp[0]+renegade.rel_x,mtrxp[1]+renegade.rel_y];
			
			let rel_pos = 5.0*shake_pos_a(timer,shaking_timer,shaking_dam[group[r].1]);
			renegade.rel_x += rel_pos;
			
			//If attacker is attackingm modify the position.
			match *sprite_boxer {
				Attack(ref mut x) => {
					if x.att_index==group[r].1 {
						let pos = sprite_approach(&x);
						renegade.rel_x = pos[0]-mtrxp[0];
						renegade.rel_y = pos[1]-mtrxp[1];
					};
				},
				_ => {},
			};
					
			if group[r].1==battle_ifast {
				pos_bif.0 = *sp;
				pos_bif.1[0] = renegade.rel_x;
				pos_bif.1[1] = renegade.rel_y;
			};
			
			let x = format!("{}",group[r].0.name);
			//let y = &x.iwpl();
			let b = widget::Button::image( if group[r].0.HP_shade/group[r].0.HP>0.0 {mon_faces[group[r].0.id][0]}else{mon_faces[group[r].0.id][2]} )
											.hover_image(mon_faces[group[r].0.id][1])
											.press_image(mon_faces[group[r].0.id][1])
											.label(&x)
											.label_color(sm_retc(&group[r].0,timer))
											.label_font_size(font_size)
											.label_y(conrod::position::Relative::Scalar(bh/2.0))
											.label_x(conrod::position::Relative::Scalar(-rel_pos as f64));
			for _click in renegade.set(b,ui) {
				if !yt_adcwpe_bw[0] {
					*comm_text = format!("This {} is {}.",x,sm_rets(&group[r].0));
					set_comm_text(comm_text,ui,ids);
				}else if yt_adcwpe_bw[1]{
					*comm_text = format!("You attack {}!",x);
					set_comm_text(comm_text,ui,ids);
					*sel_targets = vec![group[r].1];
				}else if yt_adcwpe_bw[3] & (!yt_adcwpe_bw[7]|!yt_adcwpe_bw[8]){
					*sel_targets = vec![group[r].1];
				}else if yt_adcwpe_bw[2] {
					*sel_targets = vec![group[r].1];
				};
			};
		};
		//println!("finished element {}",r);
	};
	//println!("Exiting blv");					
}


//Set the threee potential horizontal battle lines of...
//Grous centre, north and south.
fn battle_line_h (ids: &mut Ids, ref mut ui: &mut conrod::UiCell,
				mon_faces: &Vec<[conrod::image::Id;3]>,
				mut mtrx: widget::matrix::Elements,
				group: &Vec<(Lifeform,usize)>,
				mut comm_text: &mut String,
				timer:usize,
				mut yt_adcwpe_bw: &mut [bool;9],
				font_size:u32,
				mut sel_targets:&mut Vec<usize>,
				bh: f64,
				shaking_dam: &mut [bool;25],
				shaking_timer: usize,
				battle_ifast: usize,
				mut pos_bif:&mut (Option<conrod::Point>,[conrod::position::Scalar;2]),
				mut sp: &mut Option<conrod::Point>,
				sprite_boxer:&mut GraphicsBox,
				sprite_pos: &mut [[f64;2];25],
				mtrxp: [f64;2]) {
	
	//Set the button sin the button matrix.
	while let Some(mut renegade) = mtrx.next(ui) {
		
		let c = renegade.col as usize;
		//Don't try to set monster buttons for monsters that don't exist.
		if c < group.len() {			
			
			//get absolute position of thingies.
			sprite_pos[group[c].1] = [mtrxp[0]+renegade.rel_x,mtrxp[1]+renegade.rel_y];
			
			//If hit, shake the button to show damage.
			let rel_pos = 5.0*shake_pos_a(timer,shaking_timer,shaking_dam[group[c].1]);
			renegade.rel_x += rel_pos; 
			
			//If attacker is attackingm modify the position.
			match *sprite_boxer {
				Attack(ref mut x) => {
					if x.att_index==group[c].1 {
						let pos = sprite_approach(&x);
						renegade.rel_x = pos[0]-mtrxp[0];
						renegade.rel_y = pos[1]-mtrxp[1];
					};
				},
				_ => {},
			};
			
			//If turn of this one, put the circle marker here.			
			if group[c].1==battle_ifast {
				pos_bif.0 = *sp;
				pos_bif.1[0] = renegade.rel_x;
				pos_bif.1[1] = renegade.rel_y;
			};
			
			//set the monster, name and all.
			let x = format!("{}",group[c].0.name);
			let y = &x.x_chr_pl2(12);
			let b = widget::Button::image( if group[c].0.HP_shade/group[c].0.HP>0.0 {mon_faces[group[c].0.id][0]}else{mon_faces[group[c].0.id][2]} )
											.hover_image(mon_faces[group[c].0.id][1])
											.press_image(mon_faces[group[c].0.id][1])
											.label(&y)
											.label_color(sm_retc(&group[c].0,timer))
											.label_font_size(font_size)
											.label_y(conrod::position::Relative::Scalar(bh/2.0+4.0))
											.label_x(conrod::position::Relative::Scalar(-rel_pos as f64));
			
			//depending on phase of player turn, decide what to do on click. 
			for _click in renegade.set(b,ui) {
				if !yt_adcwpe_bw[0] { //If just started
					//println!("d1");
					*comm_text = format!("This {} is {}.",x,sm_rets(&group[c].0));
					set_comm_text(comm_text,ui,ids);
				}else if yt_adcwpe_bw[1]{ //If selecting attack target
					//println!("d2");
					*comm_text = format!("You attack {}!",x);
					set_comm_text(comm_text,ui,ids);
					*sel_targets = vec![group[c].1];
				}else if yt_adcwpe_bw[3] & (!yt_adcwpe_bw[7]|!yt_adcwpe_bw[8]){ //if casting
					//println!("d3");
					*sel_targets = vec![group[c].1];
				}else if yt_adcwpe_bw[2] { //if choosing defence target.
					//println!("d4");
					*sel_targets = vec![group[c].1];
				};
			};
			//println!("e");				
		};
		//println!("finished element {}",c);
	};
	//println!("Exiting blh");					
}


fn marker_of_set_battle_map(){}
//Layout parties on battlefield: Maybe not too awful.
fn set_battle_map(ids: &mut Ids, ref mut ui: &mut conrod::UiCell,
				  mon_faces: &Vec<[conrod::image::Id;3]>,
				  mon_facesz: &Vec<[conrod::Scalar;2]>,
				  world: &Vec<[Place;19]>,
				  mut diff:i32,
				  p_names:&mut Vec<String>,
				  mut encounter:&mut Vec<(Lifeform,usize,[Option<[usize;2]>;2])>,
				  sprite_boxer:&mut GraphicsBox,
				  wo:&mut FlowCWin,
				  p_loc:&mut Place,
				  mut comm_text: &mut String,
				  timer:usize,
				  mut yt_adcwpe_bw: &mut [bool;9],
				  mut sel_targets:&mut Vec<usize>,
				  shaking_dam: &mut [bool;25],
				  sprite_pos: &mut [[f64;2];25],
				  shaking_timer: &mut usize,
				  battle_ifast: usize,
				  pause: bool) {
					  
	//println!("Inside set_battle_map");

	//get measurements of middle column and hence make width and height.
	let mut wh_mc = ui.wh_of(ids.middle_column).unwrap();
	let xy_mc = ui.xy_of(ids.middle_column).unwrap();
	let factor = if wh_mc[0]/760.0 > wh_mc[1]/540.0 {wh_mc[1]/540.0}else{wh_mc[0]/760.0};
	let base_h:f64 = mon_facesz[0][1] as f64 * factor;
	let base_w:f64 = mon_facesz[0][0] as f64 * factor;
	
	let font_size = font_size_chooser(&wh_mc);
	
	let pad_w:f64 = base_h/5.0; 
	let pad_h:f64 = base_w/2.5;
	let bh:f64 = base_h + pad_h;
	let bh2:f64 = base_h + pad_h/2.0;
	let bw:f64 = base_w + pad_w;
	let mut pos_bif:(Option<conrod::Point>,[conrod::position::Scalar;2]) = (None,[0.0,0.0]);
	let mut spare_point:Option<conrod::Point> = None;
	//set up variables:
	let enc_l = encounter.len();
	let mut enc_c:Vec<(Lifeform,usize)> = Vec::with_capacity(5);
	let mut enc_n:Vec<(Lifeform,usize)> = Vec::with_capacity(5);
	let mut enc_e:Vec<(Lifeform,usize)> = Vec::with_capacity(5);
	let mut enc_s:Vec<(Lifeform,usize)> = Vec::with_capacity(5);
	let mut enc_w:Vec<(Lifeform,usize)> = Vec::with_capacity(5);
	for i in 0..encounter.len() {	
		match encounter[i].1 {
			0 => enc_c.push((encounter[i].0.clone(),i)),
			1 => enc_n.push((encounter[i].0.clone(),i)),
			2 => enc_e.push((encounter[i].0.clone(),i)),
			3 => enc_s.push((encounter[i].0.clone(),i)),
			_ => enc_w.push((encounter[i].0.clone(),i)),
		};	
	};
	
	let enc_cl = enc_c.len(); let enc_clf:f64 = enc_cl as f64;
	let enc_nl = enc_n.len(); let enc_nlf:f64 = enc_nl as f64;
	let enc_el = enc_e.len(); let enc_elf:f64 = enc_el as f64;
	let enc_sl = enc_s.len(); let enc_slf:f64 = enc_sl as f64;
	let enc_wl = enc_w.len(); let enc_wlf:f64 = enc_wl as f64;
	//create canvases for all four parties.
	
	//println!("Inside set_battle_map A");
	//set up obligatory matrices and canvases.
	canvas_border(widget::Canvas::new().middle_of(ids.middle_column)
						 .wh([bw*enc_clf,bh])
						 .color(rgba(0.0,0.0,0.0,0.0)),
						 //.label_color(color::YELLOW)
						 //.label("Party"),
				  Some(0.0)).set(ids.partyc_can,ui);
		
	let mut enc_c_matrix = widget::Matrix::new(enc_cl,1)
			.wh([bw*enc_clf,base_h])
			.mid_bottom_of(ids.partyc_can)
			.cell_padding(pad_w,0.0)
			.set(ids.partyc_mtrx, ui);
			
	let wh_enc_cm = ui.xy_of(ids.partyc_mtrx).unwrap_or([0.0,0.0]);
			
	//println!("Inside set_battle_map B");
	//NB: Party canvas is inlined directly. After all, why not.		
	while let Some(mut renegade) = enc_c_matrix.next(ui) {
		let c = renegade.col as usize;
		//NB CURRENTLY ONLY REL POS. THIS NEEDS FIXED.
		sprite_pos[enc_c[c].1] = [renegade.rel_x+wh_enc_cm[0],renegade.rel_y+wh_enc_cm[1]];
		
		//If attacker is attackingm modify the position.
		match *sprite_boxer {
				Attack(ref mut x) => {
					if x.att_index==enc_c[c].1 {
						let pos = sprite_approach(&x);
						renegade.rel_x = pos[0]-wh_enc_cm[0];
						renegade.rel_y = pos[1]-wh_enc_cm[1];
					};
				},
				_ => {},
			};
		
		let rel_pos = 5.0*shake_pos_a(timer,*shaking_timer,shaking_dam[enc_c[c].1]);
		renegade.rel_x += rel_pos; 
		
		if c<enc_c.len(){
			if enc_c[c].1==battle_ifast {
				pos_bif.0 = ui.xy_of(ids.partyc_mtrx);
				pos_bif.1[0] = renegade.rel_x;
				pos_bif.1[1] = renegade.rel_y;
			};
		};
		
		let x = format!("\n{}",p_names[c]);
		let y = &x.x_chr_pl(8);
		let b = widget::Button::image( if enc_c[c].0.HP_shade/enc_c[c].0.HP>0.0 {mon_faces[enc_c[c].0.id][0]}else{mon_faces[enc_c[c].0.id][2]} )
										.label(&y)
										.hover_image(mon_faces[enc_c[c].0.id][1])
										.press_image(mon_faces[enc_c[c].0.id][1])
										.label_color(sm_retc(&enc_c[c].0,timer))
										.label_font_size(font_size)
										.label_y(conrod::position::Relative::Scalar(bh/2.0))
										.label_x(conrod::position::Relative::Scalar(-rel_pos as f64));
		for _click in renegade.set(b,ui) {
			if !yt_adcwpe_bw[0] {
				*comm_text = format!("{} is {}.",p_names[c],sm_rets(&enc_c[c].0));
				set_comm_text(comm_text,ui,ids);
			}else if yt_adcwpe_bw[1]{
					*comm_text = format!("You attack {}!",p_names[c]);
					set_comm_text(comm_text,ui,ids);
					*sel_targets = vec![enc_c[c].1];
			}else if yt_adcwpe_bw[3] & (!yt_adcwpe_bw[7]|!yt_adcwpe_bw[8]){
					*sel_targets = vec![enc_c[c].1];
					//println!("{}",c);
			}else if yt_adcwpe_bw[2] {
				*sel_targets = vec![enc_c[c].1];
			};
		};
	};
	
	//println!("Inside set_battle_map C");
	if enc_nl>0 {
		canvas_border(widget::Canvas::new()
							 .mid_top_with_margin_on(ids.middle_column,5.0)
							 .wh([bw*enc_nlf,bh])
							 .color(rgba(0.0,0.0,0.0,0.0)),
							 //.label_color(color::YELLOW)
							 //.label("Party"),
							 Some(0.0)).set(ids.enemyn_can,ui);
		let mut enc_n_matrix = widget::Matrix::new(enc_nl,1)
			.wh([bw*enc_nlf,base_h])
			.mid_bottom_of(ids.enemyn_can)
			.cell_padding(pad_w,0.0)
			.set(ids.enemyn_mtrx, ui);
			
		//println!("enc_nl={}",enc_nl);
		//println!("enc_n.len={}",enc_n.len());
		spare_point = ui.xy_of(ids.enemyn_mtrx);
		let point = ui.xy_of(ids.enemyn_mtrx).unwrap_or([0.0;2]);
			
		battle_line_h(ids,ui,mon_faces,enc_n_matrix,
					  &enc_n,comm_text,timer,&mut yt_adcwpe_bw,font_size,
					  &mut sel_targets,bh,shaking_dam,*shaking_timer,battle_ifast,
					  &mut pos_bif,&mut spare_point,sprite_boxer,sprite_pos,point);
	};
	if enc_el>0 {
		canvas_border(widget::Canvas::new()
							 .mid_right_with_margin_on(ids.middle_column,pad_w*2.0)
							 .wh([bw,enc_elf*bh2])
							 .color(rgba(0.0,0.0,0.0,0.0)),
							 //.label_color(color::YELLOW)
							//.label("Party"),
					Some(0.0)).set(ids.enemye_can,ui);
		let mut enc_e_matrix = widget::Matrix::new(1,enc_el)
			.wh([bw,bh2*enc_elf])
			.mid_bottom_of(ids.enemye_can)
			.cell_padding(pad_w,pad_h/4.0)
			.set(ids.enemye_mtrx, ui);
			
		spare_point = ui.xy_of(ids.enemye_mtrx);
		let point = ui.xy_of(ids.enemye_mtrx).unwrap_or([0.0;2]);
		
		battle_line_v(ids,ui,mon_faces,enc_e_matrix,
					  &enc_e,comm_text,timer,&mut yt_adcwpe_bw,font_size,
					  &mut sel_targets,bh2,shaking_dam,*shaking_timer,battle_ifast,
					  &mut pos_bif,&mut spare_point,sprite_boxer,sprite_pos,point);
	};
	if enc_sl>0 {
		canvas_border(widget::Canvas::new()
							 .mid_bottom_of(ids.middle_column)
							 .wh([bw*enc_slf,bh])
							 .color(rgba(0.0,0.0,0.0,0.0)),
							//.label_color(color::YELLOW)
							//.label("Party"),
						Some(0.0)).set(ids.enemys_can,ui);
		let mut enc_s_matrix = widget::Matrix::new(enc_sl,1)
			.wh([bw*enc_slf,base_h])
			.mid_bottom_of(ids.enemys_can)
			.cell_padding(pad_w,0.0)
			.set(ids.enemys_mtrx, ui);
			
		spare_point= ui.xy_of(ids.enemys_mtrx);
		let point = ui.xy_of(ids.enemys_mtrx).unwrap_or([0.0;2]);
		
		//println!("enc_sl={}",enc_sl);
		//println!("enc_s.len={}",enc_s.len());	
		battle_line_h(ids,ui,mon_faces,enc_s_matrix,
					  &enc_s,comm_text,timer,&mut yt_adcwpe_bw,font_size,
					  &mut sel_targets,bh,shaking_dam,*shaking_timer,battle_ifast,
					  &mut pos_bif,&mut spare_point,sprite_boxer,sprite_pos,point);
	};
	if enc_wl>0 {
		canvas_border(widget::Canvas::new()
							 .mid_left_with_margin_on(ids.middle_column,pad_w*2.0)
							 .wh([bw,bh2*enc_wlf])
							 .color(rgba(0.0,0.0,0.0,0.0)),
							//.label_color(color::YELLOW)
							//.label("Party"),
						Some(0.0)).set(ids.enemyw_can,ui);
		let mut enc_w_matrix = widget::Matrix::new(1,enc_wl)
			.wh([bw,bh2*enc_wlf])
			.mid_bottom_of(ids.enemyw_can)
			.cell_padding(pad_w,pad_h/4.0)
			.set(ids.enemyw_mtrx, ui);
			
		spare_point = ui.xy_of(ids.enemyw_mtrx);
		let point = ui.xy_of(ids.enemyw_mtrx).unwrap_or([0.0;2]);
		
		battle_line_v(ids,ui,mon_faces,enc_w_matrix,
					  &enc_w,comm_text,timer,&mut yt_adcwpe_bw,font_size,
					  &mut sel_targets,bh2,shaking_dam,*shaking_timer,battle_ifast,
					  &mut pos_bif,&mut spare_point,sprite_boxer,sprite_pos,point);
	};
	
	//Set the wondering circle if target has not been murderified and game is not paused.
	//println!("Inside set_battle_map D");
	if (encounter[battle_ifast].0.HP_shade>0.0) & !pause {
		set_marker_of_go(ids,ui,pos_bif,timer,battle_ifast,encounter,base_h);	
	};	 
	
	//Set spell effects if needs be.
	spell_setter(ids,ui,sprite_boxer,sprite_pos);
	
	//decrement sprite box.
	sprite_box_decrement(sprite_boxer,wo,timer,shaking_timer,shaking_dam);
	//println!("Exiting set_battle_map C"); 
	//println!("{:?}",sprite_boxer);
}



//function to set two circles which circle the Lifeform whose turn it is,
//with a speed proportional to the lifeform speed. 
//Uses parabolic trig function- faster than normal trig..
fn set_marker_of_go_marker(){}
fn set_marker_of_go(ids: &mut Ids, ref mut ui: &mut conrod::UiCell,
					pos_bif: (Option<conrod::Point>,[conrod::position::Scalar;2]),
					timer:usize,
					battle_ifast: usize,
					encounter: &mut Vec<(Lifeform,usize,[Option<[usize;2]>;2])>,
					size:f64) {
	
	//At speed of 50, the period of rotation will equal 2 seconds.
	//There may be a problem with odd vs even numbers (now fixed).					
	let half_period:usize = (FPS * 50.0/encounter[battle_ifast].0.Speed_shade) as usize*2;
	let period:usize = half_period*2;
						
	conrod::widget::RoundedRectangle::fill_with([10.0,10.0],5.0,colour_of_monster(encounter[battle_ifast].0.Type).with_alpha(0.3))
									 .x(pos_bif.0.clone().unwrap()[0]+pos_bif.1[0]+size*0.62*sinp(timer,period))
									 .y(pos_bif.0.clone().unwrap()[1]+pos_bif.1[1]+size*0.62*cosp(timer,period))
									 .set(ids.marker_shape,ui);
	conrod::widget::RoundedRectangle::fill_with([10.0,10.0],5.0,colour_of_monster(encounter[battle_ifast].0.Type).with_alpha(0.3))
									 .x(pos_bif.0.clone().unwrap()[0]+pos_bif.1[0]+size*0.62*sinp(timer+half_period,period))
									 .y(pos_bif.0.clone().unwrap()[1]+pos_bif.1[1]+size*0.62*cosp(timer+half_period,period))
									 .set(ids.marker_shape2,ui);
}


//Put the world map where it's meant to be.
//This method uses a button matrix to represent the world map.
//A better way in the second version uses an image of the world map.
fn marker_of_set_init_world_map(){}
#[allow(unused_variables)]
fn set_init_world_map (	ids: &mut Ids, ref mut ui: &mut conrod::UiCell,
						mut n_s_l_q_f:&mut [bool;7],
						world: &Vec<[Place;19]>,
						mut diff:i32,
						p_names:&mut Vec<String>,
						party:&mut Vec<(Lifeform,usize)>,
						p_loc:&mut Place,
						pl:&mut (usize,usize),
						coords:&mut [i32;2],
						mut comm_text: &mut String,
						timer:usize,
						mut tt_e_c_i_ll: &mut [bool;8],
						mut provisional_loc: &mut (usize,usize)) {
		//set tteci[0] to true.
		
		*tt_e_c_i_ll = [true,tt_e_c_i_ll[1],false,false,false,tt_e_c_i_ll[5],tt_e_c_i_ll[6],false];
		
		//get number of rows. and the map size.
		let world_len = world.len();
		let wml = world_len-1;
		let mut map_size = ui.wh_of(ids.middle_column).unwrap();
		map_size = [map_size[0]-6.0,map_size[1]-6.0];
		
		let mut world_matrix = widget::Matrix::new(world_len,19)
			.wh(map_size)
			.middle_of(ids.middle_column)
			.set(ids.global_map, ui);
			
		let mut button = widget::Button::new();
		let square_size = [map_size[0]/(world_len as f64),map_size[1]/19.0];
		
		//Find a way to fix this clusterfuck.
        while let Some(mut square) = world_matrix.next(ui) {
            let (r, c) = (square.row as usize, square.col as usize);
			//if !tt_e_c_i_ll[0]{println!("{:?}",(r,c));};
			let butt_col = map_sq_colour(&world[wml-c][r]);
			let butt_txc = map_tx_colour(&world[wml-c][r]);
            if &(c,r)==pl {
				for _click in square.set(button.clone().color(butt_col.with_luminance(sync_t(timer)))
											.label(&world[wml-c][r].name[0..1])
											.label_color(butt_txc)
										,ui) {
					//println!("Hey! {:?}", world[wml-c][r]);
					*comm_text = format!("You are here: {}", world[wml-c][r]);
					set_comm_text(comm_text,ui,ids);
				}
			}else if within_one(r,c,&pl) {
				for _click in square.set(button.clone().color(butt_col)
											.label(&world[wml-c][r].name[0..1])
											.label_color(butt_txc)
										,ui) {
					//println!("Hey! {:?}", world[wml-c][r]);
					*comm_text = if world[wml-c][r].scape==VOID{
						format!("Ho! You can see: {}{}", world[wml-c][r],VOID_TEXT)	
					}else{
						format!("Ho! You can see: {}", world[wml-c][r])					
					};
					*provisional_loc = (c,r);	
					set_comm_text(comm_text,ui,ids);
					tt_e_c_i_ll[1] = true;
				}
            }else{
				let mut distant = square.set(
						widget::Button::new()
							.color(butt_col)
						,ui);
			};
        };
}


// Alternative version of set_init_world_map()
// Uses pregenerated world_map instead of widget matrix.
fn marker_of_set_init_world_map2(){}
#[allow(unused_variables)]
fn set_init_world_map2 (ids: &mut Ids, ref mut ui: &mut conrod::UiCell,
						mut n_s_l_q_f:&mut [bool;7],
						world: &Vec<[Place;19]>,
						map: &conrod::image::Id,
						mon_faces: &Vec<[conrod::image::Id;3]>,
						mut diff:i32,
						p_names:&mut Vec<String>,
						party:&mut Vec<(Lifeform,usize)>,
						p_loc:&mut Place,
						pl:&mut (usize,usize),
						coords:&mut [i32;2],
						mut comm_text: &mut String,
						timer:usize,
						mut tt_e_c_i_ll: &mut [bool;8],
						mut provisional_loc: &mut (usize,usize)) {
		
		//set tteci[0] to true.
		*tt_e_c_i_ll = [true,tt_e_c_i_ll[1],false,false,false,tt_e_c_i_ll[5],tt_e_c_i_ll[6],false];
		
		//get number of rows. and the map size.
		let world_len = world.len();
		let wml = world_len-1;
		
		let mut map_size = ui.wh_of(ids.middle_column).unwrap();
		let mut bp = ui.xy_of(ids.middle_column).unwrap();
	
		map_size = [map_size[0]-6.0,map_size[1]-6.0];
		let square_size = [map_size[0]/(world_len as f64),map_size[1]/19.0];
		bp = [bp[0]-(map_size[0]-square_size[0])/2.0,bp[1]-(map_size[1]-square_size[1])/2.0];
		
		widget::Image::new(*map)
			.middle_of(ids.middle_column)
			.wh(map_size)
			.set(ids.global_map_image, ui);
		
		//initiate the five buttons.
		let (c,r) = pl.clone();
		let butt_col = color::BLACK.with_alpha(0.0);
		let mut butt_txc = map_tx_colour(&world[wml-c][r]);
		
		//initiate the invisible buttons.
		let mut button_n = widget::Button::new().wh(square_size).color(butt_col);								  
		let mut button_e = button_n.clone(); let mut button_s = button_n.clone(); let mut button_w = button_n.clone();
		
		//initate pulse variable
		let s_sync = sync_s(timer);
		
		//TODO(DONE): Replace with pulsing image button.
		//button_c = button_c.wh([square_size[0]*0.8,square_size[1]*0.8])
					 	   //.color(butt_col.with_luminance(sync_t(timer)))
						   //.label(&world[wml-c][r].name[0..1])
						   //.label_color(butt_txc)
						   //.border(0.0)
						   //.border_color(butt_col.with_luminance(sync_t(timer)))
						   //.bottom_left_of(ids.middle_column);
						   
		let mut button_c = widget::Button::image(mon_faces[party[0].0.id][0])
										.wh([square_size[0]*s_sync,square_size[0]*s_sync])
										.hover_image(mon_faces[party[0].0.id][1])
										.press_image(mon_faces[party[0].0.id][1]);
						   
		button_c = reposition_geography_button(button_c,c,r,wml,&square_size,&bp);	
		
		for _click in button_c.set(ids.center_button,ui) {
			//println!("Hey! {:?}", world[wml-c][r]);
			*comm_text = format!("You are here: {}", world[wml-c][r]);
			set_comm_text(comm_text,ui,ids);
		}
		
		let (a,b) = geographical_button(wml,c,r,'N');
		
		butt_txc = map_tx_colour(&world[wml-a][b]);
	
		button_n = button_n.color(butt_col)
						   //.label(&world[wml-a][b].name[0..1])
						   .label_color(butt_txc)
						   .border_color(butt_col)
				           .bottom_left_of(ids.middle_column);
		button_n = reposition_geography_button(button_n,a,b,wml,&square_size,&bp);	
		
		for _click in button_n.set(ids.north_button,ui) { 
					//println!("Hey! {:?}", world[wml-c][r]);
			*comm_text = if world[wml-a][b].scape==VOID{
				format!("Ho! You can see: {}{}", world[wml-a][b],VOID_TEXT)	
			}else{
				format!("Ho! You can see: {}", world[wml-a][b])					
			};
			*provisional_loc = (a,b);	
			set_comm_text(comm_text,ui,ids);
			tt_e_c_i_ll[1] = true;
		}
		
		let (a,b) = geographical_button(wml,c,r,'E');
		butt_txc = map_tx_colour(&world[wml-a][b]);
	
		button_e = button_e.color(butt_col)
							//.label(&world[wml-a][b].name[0..1])
							.label_color(butt_txc)
							.border_color(butt_col)
							.mid_right_of(ids.middle_column);
		button_e = reposition_geography_button(button_e,a,b,wml,&square_size,&bp);	
		
		for _click in button_e.set(ids.east_button,ui) { 
					//println!("Hey! {:?}", world[wml-c][r]);
			*comm_text = if world[wml-a][b].scape==VOID{
				format!("Ho! You can see: {}{}", world[wml-a][b],VOID_TEXT)	
			}else{
				format!("Ho! You can see: {}", world[wml-a][b])					
			};
			*provisional_loc = (a,b);	
			set_comm_text(comm_text,ui,ids);
			tt_e_c_i_ll[1] = true;
		}
		
		let (a,b) = geographical_button(wml,c,r,'S');
		butt_txc = map_tx_colour(&world[wml-a][b]);
	
		button_s = button_s.color(butt_col)
						   //.label(&world[wml-a][b].name[0..1])
						   .label_color(butt_txc)
						   .border_color(butt_col)
						   .mid_bottom_of(ids.middle_column);							
		button_s = reposition_geography_button(button_s,a,b,wml,&square_size,&bp);	
		
		for _click in button_s.set(ids.south_button,ui) { 
					//println!("Hey! {:?}", world[wml-c][r]);
			*comm_text = if world[wml-a][b].scape==VOID{
				format!("Ho! You can see: {}{}", world[wml-a][b],VOID_TEXT)	
			}else{
				format!("Ho! You can see: {}", world[wml-a][b])					
			};
			*provisional_loc = (a,b);	
			set_comm_text(comm_text,ui,ids);
			tt_e_c_i_ll[1] = true;
		}
		
		let (a,b) = geographical_button(wml,c,r,'W');
		butt_txc = map_tx_colour(&world[wml-a][b]);
		
		button_w = button_w.color(butt_col)
						   //.label(&world[wml-a][b].name[0..1])
						   .label_color(butt_txc)
						   .border_color(butt_col)
						   .mid_left_of(ids.middle_column);
		button_w = reposition_geography_button(button_w,a,b,wml,&square_size,&bp);
		
		for _click in button_w.set(ids.west_button,ui) { 
					//println!("Hey! {:?}", world[wml-c][r]);
			*comm_text = if world[wml-a][b].scape==VOID{
				format!("Ho! You can see: {}{}", world[wml-a][b],VOID_TEXT)	
			}else{
				format!("Ho! You can see: {}", world[wml-a][b])					
			};
			*provisional_loc = (a,b);	
			set_comm_text(comm_text,ui,ids);
			tt_e_c_i_ll[1] = true;
		}
}

// sets the coordinates of the other 4 buttons relative to centre.
fn geographical_button(wml:usize,c:usize,r:usize,direction:char) -> (usize,usize) {
	let new_c:usize;
	let new_r:usize;
	
	if direction=='N' {
		new_c = c;
		new_r = if r<2 {0}else{r-1};
	}else if direction=='E' {
		new_r = r;
		new_c = if c==wml {0}else{c+1};
	}else if direction=='S' {
		new_c = c;
		new_r = if r>16 {18}else{r+1};
	}else{ //else West! (TODO, add extra directions... maybe)
		new_r = r;
		new_c = if c==0 {wml}else{c-1};
	};
	(new_c,new_r)
}

//Function to reposition the geography buttons.
fn reposition_geography_button<T:Positionable> (button_c:T,
								c:usize,
								r:usize,
								wml:usize,
								sq_size:&[f64;2],
								base_pos:&[f64;2]) -> T  {
	
	button_c.xy([base_pos[0]+sq_size[0]*(c) as f64, base_pos[1]+sq_size[1]*(18-r) as f64])
}


//color of map squares.
fn map_sq_colour(square:&Place)-> color::Colour {
	match square.scape {
		ICE => color::LIGHT_BLUE,
		TUNDRA => color::LIGHT_PURPLE,
		WATER => color::DARK_BLUE,
		GRASSLAND => color::DARK_GREEN.with_luminance(0.3),
		FOREST => color::DARK_GREEN.with_luminance(0.2),
		STEPPE => color::LIGHT_BROWN.with_luminance(0.4),
		DESERT => color::DARK_YELLOW.with_luminance(0.3),
		CITY => color::DARK_GREY.with_luminance(0.3),
		HIGHLAND => Color::Rgba(0.12,0.15,0.3,1.0),
		MOORLAND => color::DARK_GREEN.with_luminance(0.3),
		VOID => color::BLACK,
		RUIN => color::GREY.with_luminance(0.1),
		_ => color::GREY.with_luminance(0.1),
	}
}

//color of map text.
fn map_tx_colour(square:&Place)-> color::Colour {
	match square.scape {
		WATER => color::YELLOW,
		VOID => color::WHITE,
		_ => color::BLACK,
	}
}

//To see if adjacent square to our square is selected.
fn within_one(r:usize,c:usize,pl:&(usize,usize)) ->bool {
	if (&(c+1,r)==pl)|(&(c,r+1)==pl)
	|| ((c,r)==(pl.0+1,pl.1))|((c,r)==(pl.0,pl.1+1)) {true}else{false}
}

fn marker_of_set_widgets(){}
//Output is "mutm_box_vis" (bool), "comm_text" (String), "new_game_init" (bool), "new,load,save,quit tuple".
//this is followed by a large number of variables from Q-ft-M itself.
#[allow(unused_variables)]
pub fn set_widgets (ref mut ui: conrod::UiCell, ids: &mut Ids,
					mon_faces: &Vec<[conrod::image::Id;3]>,
					mon_facesz: &Vec<[conrod::Scalar;2]>,
					mut comm_text:String,
					player_input:&mut String,
					mut mutm_box_vis:bool,
					mut new_game_init:bool,
					mut1_text:&str,mut2_text:&str,mut3_text:&str,mut4_text:&str,
					main_vis:bool,adv_vis:bool,fight_vis:bool,
					mut n_s_l_q_f:[bool;7],
					mut tt_e_c_i_ll: &mut [bool;8],
					mut provisional_loc: &mut (usize,usize),
					mut battled:usize,
					mut action:u8,
					world: &Vec<[Place;19]>,
					world_map: &conrod::image::Id,
					spl:&Vec<Spell>,
					mons:&Vec<Lifeform>,
					mut diff:i32,
					p_names_m:&mut Vec<&str>,
					mut p_names:&mut Vec<String>,
					mut party:&mut Vec<(Lifeform,usize)>,
					p_loc:&mut Place,
					pl:&mut (usize,usize),
					encounter:&mut Vec<(Lifeform,usize,[Option<[usize;2]>;2])>,
					enemies:&mut Vec<(Lifeform,usize)>,
					field:&mut Place,
					lore_empty:&mut bool,
					aftermath:&mut (Lifeform,Lifeform,Vec<[u8;28]>),
					rrrltxt:&mut Vec<String>,
					rltxt:&mut String,
					ltxt:&mut Vec<&'static str> ,
					rlb:&mut Vec<u8>,
					coords:&mut [i32;2],
					mut stage:usize,
					mut to_load:&mut (Option<String>,usize),
					mut dream_time:&mut bool,
					timer:usize,
					mut freeze_timer: &mut usize,
					mut yt_adcwpe_bw: &mut [bool;9],
					mut sel_targets: &mut Vec<usize>,
					mut to_cast: &mut String,
					battle_ifast: usize,
					battle_ttakes: &mut u16,
					mut chosen_hero: &mut usize,
					mut dungeons: &mut Vec<Dungeon>,
					mut idungeon: &mut Option<usize>,
					mut dungeon_pointer: &mut usize,
					mut truly_quit: &mut bool,
					mut shaking_dam: &mut [bool;25],
					shaking_timer: &mut usize,
					pause:bool,
					mut scenery_index: &mut usize,
					landscapes: &Landscapes,
					mut centre_h: &mut f64,
					mut centre_w: &mut f64,
					mut gui_song_list: &mut Vec<String>,
					mut silent_sender: &mut SyncSender<bool>,
					mut p_scape: &mut u8,
					wo: &mut FlowCWin,
					ipath:&mut Option<(usize,String)>,
					sprite_boxer: &mut GraphicsBox,
					sprite_pos: &mut [[f64;2];25]) -> (bool,String,bool,[bool;7],usize,u8,i32,usize) {
	
	//if tt_e_c_i_ll[2] {println!("tecill[2], In sset_widgets A");};
	//create an initial backup of comm_text					
	let comm_text_bckup1:String = comm_text.clone();
	
	//if quit==true, make dialog with two buttons (quit and cancel), which lets you finally quit!
	if n_s_l_q_f[3] {
		n_s_l_q_f = quitter(ui,ids,n_s_l_q_f,truly_quit);			   
		return (mutm_box_vis,comm_text,new_game_init,n_s_l_q_f,battled,action,diff,stage);
	};
	//get canvas size:
	let win_wh = ui.wh_of(ids.master).unwrap_or([1080.0,800.0]);
	
	let wml = world.len()-1;
	
	let hide_all_but = if n_s_l_q_f[0] | n_s_l_q_f[1] | n_s_l_q_f[2] {
		mutm_box_vis = true;
		true
	}else{
		mutm_box_vis = if tt_e_c_i_ll[1] & (world[wml-provisional_loc.0][provisional_loc.1].scape!=VOID) {
			true
		}else if tt_e_c_i_ll[2]
		 & (*dungeon_pointer>1)
		  & idungeon.is_some() {
			  //println!("mb1, dp: {}\ndunlen: {}",dungeon_pointer,dungeons[idungeon.unwrap()].scenes.len());
			if *dungeon_pointer<dungeons[idungeon.unwrap()].scenes.len()+2 {
				//println!("mb2");
				mutm_box_vis = false;
				false
			}else{
				//println!("mb3");
				mutm_box_vis = true;
				true
			}
		}else if tt_e_c_i_ll[2] {
			mutm_box_vis
		}else{
			*dungeon_pointer = 0;
			false};
		false
	};
	
	//show the right menus depending on whether there's a fight or not.
	let right_menu_l = if !n_s_l_q_f[4] | hide_all_but {0.0}else{220.0};
	let left_menu_l = if n_s_l_q_f[4] | hide_all_but {0.0}else{220.0};
	
	let wml = world.len()-1;
	
	let men_wh = [214.0,win_wh[1]];
									
	let mut right_menus_canvas = widget::Canvas::new()
												//.color(color::DARK_GREY
												.length(right_menu_l)
												.h(win_wh[1]-6.0)
												.pad(BORDER);
									
	let mut comm_box = canvas_bord_col(widget::Canvas::new()
											  .color(color::BLACK)
											  .scroll_kids_vertically()
											  .length_weight(0.5),
									   Some(BORDER),
									   BORDER_COLOUR);
	let mut coml_box = widget::Canvas::new()
									.color(BACKGR_COLOUR)
									.scroll_kids_vertically()
									.pad(BORDER)
									.border(BORDER)
									.border_color(BORDER_COLOUR)
									.length(if !mutm_box_vis {0.0}else{36.0});
	
	let background_colour:Color = if idungeon.is_none() {
		*p_scape = p_loc.scape;
		map_sq_colour(p_loc)
	}else if (*dungeon_pointer<2) | (*dungeon_pointer>dungeons[idungeon.unwrap()].scenes.len()+1) {
		*p_scape = p_loc.scape;
		map_sq_colour(p_loc)
	}else{
		*p_scape = dungeons[idungeon.unwrap()].scenes[*dungeon_pointer-2].scape;
		map_sq_colour(&dungeons[idungeon.unwrap()].scenes[*dungeon_pointer-2])
	};
									
	let mut map_column = canvas_bord_col(widget::Canvas::new()
												.color(background_colour)
												.scroll_kids_vertically()
												.length_weight(1.5)
												.pad(BORDER),
												Some(BORDER),
												BORDER_COLOUR);
									
	let mutm_box_l:f64 = if mutm_box_vis {50.0}else{0.0};
	let butt_h:f64 = if new_game_init {men_wh[1]/10.0}else{0.0};
	let but_hb:f64 = men_wh[1]/10.0;
	let mut mutm_minibox = widget::Canvas::new().color(BACKGR_COLOUR).length(mutm_box_l);
	
	widget::Canvas::new().flow_down(&[
				//(ids.header, widget::Canvas::new().color(color::BLUE).pad_bottom(2.0)),
				(ids.body, widget::Canvas::new()
				//.length(300.0)
				.flow_right(&[
					(ids.far_left_column, canvas_bord_col(widget::Canvas::new()
						.length(left_menu_l)
						.pad(BORDER)
					    .h(win_wh[1]-6.0),
						Some(BORDER),
						BORDER_COLOUR)
					),
					(ids.map_and_word, widget::Canvas::new().flow_down(&[
						(ids.mid_label_can, widget::Canvas::new().pad(BORDER)
																 .color(color::DARK_BLUE.with_luminance(0.1))
																 .length(if n_s_l_q_f[4] {30.0}else{0.0})
						),
						(ids.middle_column, map_column),
						(ids.coml_box, coml_box.pad(BORDER)),
						(ids.mutant_menu, widget::Canvas::new()
									.flow_right(&[
										((ids.mut1_box),(mutm_minibox.length_weight(1.0))),
										((ids.mut2_box),(mutm_minibox.length_weight(1.0))),
										((ids.mut3_box),(mutm_minibox.length_weight(1.0))),
										((ids.mut4_box),(mutm_minibox.length_weight(1.0))),
										((ids.mut5_box),(mutm_minibox.length_weight(1.0))),
									])
									.color(BACKGR_COLOUR)
									.length(mutm_box_l)
									.pad(BORDER)),
						(ids.comm_box, comm_box.pad(BORDER)),
					]).pad(6.0)
					  .length(win_wh[0]-left_menu_l-right_menu_l-6.0)
					  .h(win_wh[1]-6.0)),
					(ids.fight_menu_canvas, right_menus_canvas),
				]),
			)]).pad(6.0).set(ids.master, ui);
			
	//println!("{:?}",ui.wh_of(ids.middle_column));
	
	
	//Layout the comment box and make the scrollbar:
	set_comm_text(&mut comm_text,ui,ids);
	widget::Scrollbar::y_axis(ids.comm_box).auto_hide(true).set(ids.comm_scroll, ui);
	
	let text_input = widget::TextEdit::new(player_input)
		.color(color::DARK_RED)
		.font_size(24)
		.top_left_of(ids.coml_box)
		.padded_w_of(ids.coml_box,5.0)
		.line_spacing(5.0)
		.restrict_to_height(false)
		.set(ids.comm_link, ui);
	
	
	//Layout the input box and make the scrollbar:
	widget::Scrollbar::y_axis(ids.coml_box).auto_hide(true).set(ids.coml_scroll, ui);
	                  
	//Alternative model fight menu buttons matrix.
	let game_menu_button = widget::Button::new().color(color::DARK_RED)
												.w_h(men_wh[0]-6.0,butt_h)
												.label_font_size(font_size_chooser_button_b(win_wh[0]));
	let main_menu_button = widget::Button::new().color(color::DARK_RED)
												.w_h(men_wh[0]-6.0,but_hb)
												.label_font_size(font_size_chooser_button_b(win_wh[0]));
     
    //if tt_e_c_i_ll[2] {println!("tecill[2], In sset_widgets B");};
    //fight menu buttons.
    if (n_s_l_q_f[4]) & (!hide_all_but) {
		//Place title.
		text_maker_m(if (*dungeon_pointer>1) & idungeon.is_some() {
											dungeons[idungeon.unwrap()].scenes[*dungeon_pointer-2].name
										}else{
											p_loc.name
										},
		color::YELLOW,20).middle_of(ids.mid_label_can).set(ids.mid_label,ui);
		//println!("gmoose731!");
		//Tell the world you're in a fight.
		// Menu in fight_menu_canvas
		widget::Canvas::new()
			.w_h(right_menu_l,men_wh[1])
			//.color(color::DARK_BLUE)
			.label_color(color::YELLOW)
			.mid_right_of(ids.fight_menu_canvas)
			.set(ids.fight_menu,ui);
		//Fight menu button canvases.
		 let long_l:f64 = men_wh[1]/10.0-BORDER;
		 let short_l:f64 = men_wh[1]-butt_h*6.0-BORDER*7.0;
		 let fight_buttons = canvas_bord_col(widget::Canvas::new()
													.mid_top_of(ids.fight_menu)
													.wh_of(ids.fight_menu_canvas)
													.pad(BORDER),
													Some(BORDER),
													BORDER_COLOUR)
							.set(ids.fight_menu_buttons, ui);
		//Fight menu buttons.
		let attack_button = game_menu_button.clone().label("Attack").w(right_menu_l-6.0).mid_top_of(ids.fight_menu).set(ids.at_button,ui);		
		let defend_button = game_menu_button.clone().label("Defend").w(right_menu_l-6.0).down_from(ids.at_button,0.0).set(ids.de_button,ui);
		let cast_button = game_menu_button.clone().label("Cast a spell").w(right_menu_l-6.0).down_from(ids.de_button,0.0).set(ids.ca_button,ui);
		let wait_button = game_menu_button.clone().label("Wait..").w(right_menu_l-6.0).down_from(ids.ca_button,0.0).set(ids.wa_button,ui);
		let panic_button = game_menu_button.clone().label("Panic!").w(right_menu_l-6.0).down_from(ids.wa_button,0.0).set(ids.pa_button,ui);
		let escape_button = game_menu_button.clone().label("Escape!").w(right_menu_l-6.0).down_from(ids.pa_button,0.0).set(ids.es_button,ui);
		
		//Prepare battle spell menu. Excess work here?
		let mut battle_spell_menu = widget::Canvas::new().scroll_kids_vertically()
										 .w_of(ids.es_button)
										 .x(ui.xy_of(ids.es_button).unwrap()[0])
										 .h(short_l)
										 .down_from(ids.es_button,0.0);
		 
		if *dream_time {
			//println!("gmoose765");
			*freeze_timer = timer;
			
			//scenery index is moved here.
			*scenery_index = scenery_setter(&landscapes,*p_scape,centre_w,centre_h);
			println!("p_scape = {}, scenery_index = {}",p_scape,scenery_index);
			if (*dungeon_pointer<2) | idungeon.is_none() {
				comm_text = "Well now you've gone and picked a fight.\nThe Great White Moose is dreaming of what this world has become...".to_owned()
			}else if idungeon.is_some() {
				comm_text = format!("You proceed to {} of {}...",dungeons[idungeon.unwrap()].scenes[*dungeon_pointer-2].name,dungeons[idungeon.unwrap()].name);
			};
			set_comm_text(&mut comm_text,ui,ids);
			if (*p_scape != VOID) & (*p_scape != TIME) {
				set_battle_background(ui,ids,&landscapes,*p_scape,*scenery_index,centre_w,centre_h);
			};
							
		}else if !*lore_empty {
			if (*battle_ttakes==0) & (*dungeon_pointer>1) & idungeon.is_some() {
				comm_text = "May the Great White Moose protect you!\n***Press Enter to Continue***".to_owned();
			}else if *battle_ttakes==0 {
				comm_text = "The Great White Moose has seen how this can end, but not how this will end...\n***Press Enter to Continue***".to_owned();
			};
			//println!("gmoose 1114-entering set_battle_background");
			if (*p_scape != VOID) & (*p_scape != TIME) {
				set_battle_background(ui,ids,&landscapes,*p_scape,*scenery_index,centre_w,centre_h);
			}else if *p_scape==TIME {
				set_timescape(ui,ids,timer);
			};
			//println!("gmoose 1114-entering set_battle_map");
			set_battle_map(ids,ui,
						mon_faces,mon_facesz,
						world,
						diff,
						p_names,
						encounter,
						sprite_boxer,
						wo,
						if (*dungeon_pointer<2) | idungeon.is_none() {
							p_loc
						}else{
							&mut dungeons[idungeon.unwrap()].scenes[*dungeon_pointer-2]
						},
						&mut comm_text,
						timer,
						&mut yt_adcwpe_bw,
						&mut sel_targets,
						shaking_dam,
						sprite_pos,
						shaking_timer,
						battle_ifast,
						pause);
			//println!("gmoose1046-exiting set_battle_map");
			//Activate fight menu buttons.
			if yt_adcwpe_bw[0] {
				comm_text = comm_text_bckup1;
				for _click in attack_button{
					//println!("Attack in battle Button Pressed");
					yt_adcwpe_bw[1] = true;
				};
				for _click in defend_button{
					//println!("Defend in battle Button Pressed");
					yt_adcwpe_bw[2] = true;
				};
				for _click in cast_button{
						//println!("Cast in battle Button Pressed");
						yt_adcwpe_bw[3] = !yt_adcwpe_bw[3];
				}
				for _click in wait_button{
					//println!("Wait in battle Button Pressed");
					yt_adcwpe_bw[4] = true;
				};
				for click in panic_button{
					//println!("Panic in battle Button Pressed");
					comm_text = format!("{} loses it and starts running in circles...",p_names[battle_ifast]);
					yt_adcwpe_bw[5] = true;
				};
				for _click in escape_button{
					yt_adcwpe_bw[6] = true;
				};
				if yt_adcwpe_bw[3] {
					//set the battle spell canvas...
					battle_spell_menu.border(BORDER)
							 .border_color(color::BLUE.with_luminance(0.66))
							 .set(ids.spells_can,ui);
					//...and enter the battle spell menu function.
					set_battle_spell_menu( ui,ids,&mut comm_text,
														spl,party,
														&mut to_cast,
														battle_ifast);
				};
			};	
			set_comm_text(&mut comm_text,ui,ids);
		};
		//println!("gmoose838!");
		//if tt_e_c_i_ll[2] {println!("tecill[2], Coming out of set_widgets B");};
		return (mutm_box_vis,comm_text,new_game_init,n_s_l_q_f,battled,action,diff,stage)	
	};
		
	//if tt_e_c_i_ll[2] {println!("tecill[2], In sset_widgets C");};	
	//Set up the main and adventure menus.
	if left_menu_l>0.0 {
		let short_l:f64 = left_menu_l-6.0;
		//create main menu burrons
		if !new_game_init || n_s_l_q_f[5] {
			//If game is not started, or menu entered voluntarily, activate main menu.
			let ng_button = main_menu_button.clone().label("New Moose").w(short_l).mid_top_of(ids.far_left_column).set(ids.ng_button,ui);		
			let lg_button = main_menu_button.clone().label("Load Moose").w(short_l).down_from(ids.ng_button,0.0).set(ids.lg_button,ui);
			let sg_button = main_menu_button.clone().label("Save Moose").w(short_l).down_from(ids.lg_button,0.0).set(ids.sg_button,ui);
			let op_button = main_menu_button.clone().label("Options").w(short_l).down_from(ids.sg_button,0.0).set(ids.op_button,ui);
			for _click in ng_button{
					println!("New Game button pressed.");
					n_s_l_q_f[0] = true;
					n_s_l_q_f[6] = false;
					wo.song_to_swap = None;
			};
			for _click in sg_button{
					println!("Save Game button pressed.");
					n_s_l_q_f[6] = false;
					wo.song_to_swap = None;
					if new_game_init {
							save(&party,&p_names,spl,&p_loc,&mut comm_text,ui,ids);
							n_s_l_q_f[1] = false;
					}else{
						comm_text = "There is nothing to save- start or load a moose first.".to_owned();
						set_comm_text(&mut comm_text,ui,ids);
						n_s_l_q_f[1] = false;
					};
			};
			for _click in lg_button{
				println!("Load Game button pressed.");
				n_s_l_q_f[2] = true;
				n_s_l_q_f[6] = false;
				wo.song_to_swap = None;
			};
			for _click in op_button{
				println!("Options menu button pressed.");
				// update song list once per show of music menu (by default).
				parse_music_config(&mut gui_song_list);
				if n_s_l_q_f[6] {
					n_s_l_q_f[6] = false;
					wo.song_to_swap = None;	
				}else{
					n_s_l_q_f[6] = true;
				};
			};
			
			let mut qt_button:conrod::widget::button::TimesClicked;
			
			// If game is started and main menu active activate gm_button.
			if new_game_init {
				let gm_button = main_menu_button.clone().label("Back to Moose").w(short_l).down_from(ids.op_button,0.0).set(ids.gm_button,ui);
				qt_button = main_menu_button.clone().label("Quit").w(short_l).down_from(ids.gm_button,0.0).set(ids.qt_button,ui);
				
				for _click in gm_button{
					println!("Returning to game. Main menu be gone!.");
					n_s_l_q_f[5] = false;
					n_s_l_q_f[6] = false;
					wo.song_to_swap = None;
				};			
			}else{
				qt_button = main_menu_button.clone().label("Quit").w(short_l).down_from(ids.op_button,0.0).set(ids.qt_button,ui);
			};
			
			for _click in qt_button{
				println!("Quit game button pressed. This should quit.");
				n_s_l_q_f[3] = true;
			};
			
			if n_s_l_q_f[6] {
				//important hack to stop crashing on reload backgrounds.
				if new_game_init {
					tt_e_c_i_ll[2] = false;
					tt_e_c_i_ll[0] = true;
				};
				set_options_canvas(ui,ids,ipath,gui_song_list,
												silent_sender,
												wo,
												mon_faces,
												landscapes);
				if ipath.is_some() {
					set_music_browser(ui,ids,ipath,gui_song_list,wo);
				};
			};
			// NOT IMPLEMENTED FULLY YET.
					
		}else{
			//If game is started, activate play menu.
			let travel_button = game_menu_button.clone().label("Travel").w(short_l).h(butt_h).mid_top_of(ids.far_left_column).set(ids.travel_button,ui);		
			let fight_button = game_menu_button.clone().label("Pick a Fight").w(short_l).h(butt_h).down_from(ids.travel_button,0.0).set(ids.fight_button,ui);
			let explore_button = game_menu_button.clone().label("Explore").w(short_l).h(butt_h).down_from(ids.fight_button,0.0).set(ids.explore_button,ui);			
			let cast_button = game_menu_button.clone().label("Cast a Spell").w(short_l).h(butt_h).down_from(ids.explore_button,0.0).set(ids.cast_button,ui);
			let party_button = game_menu_button.clone().label("Inspect Party").w(short_l).h(butt_h).down_from(ids.cast_button,0.0).set(ids.party_button,ui);
			let gm_button = main_menu_button.clone().label("Main Menu").w(short_l).down_from(ids.party_button,0.0).set(ids.gm_button,ui);
			
			//Activate main menu buttons.	
			for _click in travel_button{
				println!("Travel Button Pressed");
				*tt_e_c_i_ll = if new_game_init {[true,false,false,false,false,false,tt_e_c_i_ll[6],false]}else{*tt_e_c_i_ll};
			};
			for _click in fight_button{
				println!("Pick a fight button pressed.");
				n_s_l_q_f[4] = true;
				tt_e_c_i_ll[2] = false;
				encounter_starter(party, enemies, encounter, p_loc, mons, dream_time);
				
				//might not be necessary here, but there is a leak somewhere.
				//*scenery_index = scenery_setter(landscapes,p_scape,centre_w,centre_h);
				set_comm_text(&mut "Well now you've gone and picked a fight.\nThe Great White Moose is dreaming of what this world has become...".to_owned(),ui,ids);
			};
			for _click in explore_button{
				*tt_e_c_i_ll = if new_game_init & !tt_e_c_i_ll[2] & !n_s_l_q_f[6] {
					*idungeon = dungeon_finder(p_loc,dungeons,party);
					*freeze_timer = timer;
					if (*p_scape != VOID) & (*p_scape != TIME) {*scenery_index = scenery_setter(&landscapes,*p_scape,centre_w,centre_h);};
					[false,false,true,false,false,false,tt_e_c_i_ll[6],false]
				}else if new_game_init & tt_e_c_i_ll[2] {
					[true,false,false,false,false,false,tt_e_c_i_ll[6],false]
				}else{
					*tt_e_c_i_ll
				};
				println!("Explore button pressed.");
			};
			for _click in cast_button{
				println!("Cast a spell button pressed.");
			};
			for _click in party_button{
				println!("Inspect party button pressed..");
				*tt_e_c_i_ll = if new_game_init {[false,false,false,false,true,false,tt_e_c_i_ll[6],false]}else{*tt_e_c_i_ll};
			};
			for _click in gm_button{
				println!("Returning to game. Main menu be gone!.");
				n_s_l_q_f[5] = true;
			};		
		};	
	};
	
	//if tt_e_c_i_ll[2] {println!("tecill[2], In sset_widgets D");};		
    //make the mutant menu appear if needs be.
    if mutm_box_vis & !n_s_l_q_f[4] {
		//println!("Visible!");
		let pressed:(usize,String) = if n_s_l_q_f==[true,false,false,false,false,n_s_l_q_f[5],n_s_l_q_f[6]] {
									  match stage {
										  0=> 
											  { if new_game_init {
												  comm_text = "The game has already begun. \
												  If you start again now, it will all be lost.".to_owned();
												  set_comm_text(&mut comm_text,ui,ids);
												  set_mutant_menu_bin(ui,ids,"That's fine.","Ermm...",comm_text.clone())
												}else{
												  comm_text = "What would you call yourself?".to_owned();
												  set_comm_text(&mut comm_text,ui,ids);
												  for edit in text_input {
													  *player_input = edit.clone();
													  if edit.chars().rev().nth(0)==Some('\n') {
														  let name_1 = edit.trim().to_title_case().to_owned();
														  p_names.push(name_1.clone());
														  comm_text = format!("I see, your name is {}",p_names[0]);
														  set_comm_text(&mut comm_text,ui,ids);
														  stage=1;
														  *player_input = String::new();
													  }else{
														  *player_input = edit;
													  };
												  };
												  set_mutant_menu_uni(ui,ids,"Cancel")}
											  },
										 1 => { set_mutant_menu_bin(ui,ids,"Yes, it is I!","Cancel",comm_text.clone())},
										 2 => { set_mutant_menu(ui,ids,"Warrior","Witch","Wonderer","Loser","Cancel")},
										 3 => { if &comm_text!="That's not even a number... So how many hours?"{
													comm_text = "How many hours do you spend thinking happy thoughts?".to_owned();
													set_comm_text(&mut comm_text,ui,ids);
												};
												for edit in text_input {
													*player_input = edit.clone();
													  if edit.chars().rev().nth(0)==Some('\n') {
														  let ed = edit.trim().to_owned();
														  *player_input = String::new();
														  let mut darkness = ed.parse::<isize>();
														  match darkness {
															  Ok(mut num) => {if num<0 {
																			num = 0
																		  }else if num>24 {
																			num = 24;
																		  };
																		  comm_text = format!("I see, {} hours...",num);
																		  set_comm_text(&mut comm_text,ui,ids);
																		  stage+= 1;
																		  character_dl_mod(&mut party[0].0,num-12);
																		 },
															  Err(_) => {comm_text = "That's not even a number... So how many hours?".to_owned();
																		 set_comm_text(&mut comm_text,ui,ids);
																		},
																	};
																};
													};
													set_mutant_menu_uni(ui,ids,"Cancel")
												
												},
											4 => {	comm_text = "Are you alone?".to_owned();
													set_comm_text(&mut comm_text,ui,ids);
													set_mutant_menu_tri(ui,ids,"All alone.","Never...","Cancel")},
											5 => {	set_mutant_menu(ui,ids,"A warrior..","A witch..","A wonderer..","A loser..","Cancel")},
											6 => {	let follower = if party.len()>1 {format!("a {}",party[1].0.name)}else{"no one".to_owned()};
													let light_dark = if party[0].0.Attack>party[0].0.Defence {"of darkness"}else{"of light"};
													comm_text = format!("So, {}, you are a {} of {} followed by {}...",p_names[0],party[0].0.name,light_dark,follower);
													set_comm_text(&mut comm_text,ui,ids);
													set_mutant_menu_bin(ui,ids,"Aye..","I don't want to do this.","Then let the adventure begin?".to_owned())
												 },
											_ => {set_mutant_menu_bin(ui,ids,"Into the sunset!","I don't want to do this.","A new moose has begun!".to_owned())},
											}
									 }else if n_s_l_q_f==[false,false,true,false,false,n_s_l_q_f[5],n_s_l_q_f[6]]{
										 if new_game_init {
										 comm_text = "The moose has already begun. \
										 If you load another now, it will all be lost.".to_owned();
										 set_comm_text(&mut comm_text,ui,ids);
										 set_mutant_menu_bin(ui,ids,"That's fine.","Ermm...",comm_text.clone())
										}else{
											let a = set_mutant_menu_uni(ui,ids,"Cancel");
											*to_load = loader(&mut comm_text,ui,ids,&men_wh);
											if a.0!=5 {(to_load.1,"".to_owned())}else{a}
										}
									 }else if tt_e_c_i_ll[1] {
										if world[wml-provisional_loc.0][provisional_loc.1].scape!=VOID{ 
											go_there(&mut comm_text,ui,ids,
													party,
													p_names,enemies,encounter,
													pl,
													p_loc,
													world,mons,
													coords,
													provisional_loc,
													tt_e_c_i_ll,
													&mut n_s_l_q_f,
													dream_time);
										}else{
											mutm_box_vis==false;
										};
										(0,comm_text.clone())
									}else if tt_e_c_i_ll[2]
									 & (*dungeon_pointer==0) {
										set_mutant_menu_bin(ui,ids,
															"Lets do this!",
															"I want to live.",
															format!("{}\nEnter {}?",dungeons[idungeon.unwrap()],dungeons[idungeon.unwrap()].name)
										)
									}else{(0,comm_text.clone())};
									 
									 
		if pressed.0==5 {		
					//if cancel new_game click, reset new game variables.
			if n_s_l_q_f[0] {
				n_s_l_q_f[0] = false;
				if !new_game_init {
					*party = Vec::with_capacity(5);
					*p_names = Vec::with_capacity(5);
					*p_loc = world[8][6].clone();
					*pl = (13,5);
				};
				stage = 0;
			 }else if n_s_l_q_f[1] {n_s_l_q_f[1] = false
			 }else if n_s_l_q_f[2] {n_s_l_q_f[2] = false
			 }else if tt_e_c_i_ll[2] {
				tt_e_c_i_ll[2] = false;
				tt_e_c_i_ll[0] = true;
			 }else{};
		}else if n_s_l_q_f==[true,false,false,false,false,n_s_l_q_f[5],n_s_l_q_f[6]] {
			//new game matcher.
			match stage {
				
				0 => {	if new_game_init & (pressed.0==1) {
						new_game_init = false;
						*party = Vec::with_capacity(5);
						*p_names = Vec::with_capacity(5);
						*p_loc = world[8][6].clone();
						*pl = (13,5);
					};
					},
				1 => {	if pressed.0==1{
							stage = 2;
							comm_text = format!("What would you be, {}?",&p_names[0]);
							set_comm_text(&mut comm_text,ui,ids);
						};
					},
				2 => {	match pressed.0 {
							1 => {	party.push((warrior(),0));
									stage = 3;
									comm_text = format!("I see, so you're a \'{}\'...",party[0].0.name);
									set_comm_text(&mut comm_text,ui,ids);},
							2 => {	party.push((witch(),0));
									stage = 3;
									comm_text = format!("I see, so you're a \'{}\'...",party[0].0.name);
									set_comm_text(&mut comm_text,ui,ids);},
							3 => {	party.push((wonderer(),0));
									comm_text = format!("I see, so you're a \'{}\'...",party[0].0.name);
									set_comm_text(&mut comm_text,ui,ids);
									stage = 3;},
							4 => {	party.push((loser(),0));
									stage = 3;
									comm_text = format!("I see, so you're a \'{}\'...",party[0].0.name);
									set_comm_text(&mut comm_text,ui,ids);},
							_ => {},
						};
					},
				3 => {},
				4 => {  match pressed.0 {
						1 => { stage = 6;},
						2 => {	stage = 5;
								comm_text = format!("Who follows you?");
								set_comm_text(&mut comm_text,ui,ids);},
						_ => {},
						};
					},
				5 => { match pressed.0 {
						1 => {	party.push((warrior(),0));
								sidekick_maker(&mut party, &mut p_names);
								stage+= 1;},
						2 => {	party.push((witch(),0));
								sidekick_maker(&mut party, &mut p_names);
								stage+= 1;},
						3 => {	party.push((wonderer(),0));
								sidekick_maker(&mut party, &mut p_names);
								stage+= 1;},
						4 => {	party.push((loser(),0));
								sidekick_maker(&mut party, &mut p_names);
								stage+= 1;},
						_ => {},
						};
					},
				6 => { if pressed.0==1 {stage = 7;};},
				7 => { stage = 0;
						n_s_l_q_f[0] = false;
						mutm_box_vis = false;
						comm_text = "Then let the adventure begin?".to_owned();
						set_comm_text(&mut comm_text,ui,ids);
						println!("Party debug: {:?}",party);
						println!("Party names: {:?}",p_names);
						new_game_init = true;
						tt_e_c_i_ll[0] = true;
						*dungeons = vec![malek_grove().clone(),monster_hall().clone(),citadel_of_spirit(party[0].0.clone()).clone(),elven_lake_ruins().clone(),
															 malachia_pubcrawl().clone(),lost_lighthouse().clone(),door_to_darkness(&party).clone(),
															 white_temple().clone(),stairway().clone(),witch_maze().clone(),way_down().clone(),wild_hunt().clone(),tower_of_bones().clone(),tower_of_flesh(),
															 tower_of_soul(&party).clone(),hall_of_stone(),the_path(),ice_palace(),on_the_prairie()];
					},
				_ => {},
			};
		}else if n_s_l_q_f==[false,false,true,false,false,n_s_l_q_f[5],n_s_l_q_f[6]] {
			if new_game_init & (pressed.0==1){
				new_game_init = false;
				save(&party,&p_names,spl,&p_loc,&mut comm_text,ui,ids);
				comm_text = "Backup complete... Choose a moose to load:".to_owned();
				set_comm_text(&mut comm_text,ui,ids);
			}else if !new_game_init & (pressed.0!=5){
											if to_load.0.is_some() & (pressed.0==42) {
												load(to_load.0.clone().unwrap(),
													&spl,
													world,
													mons,
													party,
													p_names,
													p_loc,
													pl,
													coords,
													&mut comm_text,ui,ids);
												n_s_l_q_f = [false,false,false,false,false,false,false];
												*to_load = (None,1);
												new_game_init = true;
												tt_e_c_i_ll[0] = true;
												*dungeons = vec![malek_grove().clone(),monster_hall().clone(),citadel_of_spirit(party[0].0.clone()).clone(),elven_lake_ruins().clone(),
															 malachia_pubcrawl().clone(),lost_lighthouse().clone(),door_to_darkness(&party).clone(),
															 white_temple().clone(),stairway().clone(),witch_maze().clone(),way_down().clone(),wild_hunt().clone(),tower_of_bones().clone(),tower_of_flesh(),
															 tower_of_soul(&party).clone(),hall_of_stone(),the_path(),ice_palace(),on_the_prairie()];
												println!("Party on! {:?}",&party);
											}else if pressed.0==0 {
												comm_text = "Could not load this moose. Try another maybe?".to_owned();
												set_comm_text(&mut comm_text,ui,ids);
											};
			}else{};
		}else if tt_e_c_i_ll[2] & (*dungeon_pointer==0) {
			match pressed.0 {
				1 => {
						*dungeon_pointer = 1;
						comm_text = format!("You take a step over the threshold separating {} from {}...",p_loc.name,dungeons[idungeon.unwrap()].name);
						set_comm_text(&mut comm_text,ui,ids);
						*freeze_timer = timer;
						mutm_box_vis = false;
					 },
				5 => {
						comm_text = format!("You turn around and head back to {}...",p_loc.name);
						set_comm_text(&mut comm_text,ui,ids);
						*freeze_timer = timer;
						mutm_box_vis = false;
						tt_e_c_i_ll[2] = false;
					 },
				_=>{},
			};
		}else{
			//General matcher, mainly works the cancel button.
			match pressed.0 {
				//if cancel:
				5 => {},
				1 => {},
				2 => {},
				3 => {},
				4 => {},
				_ => {},
			};
		};
		comm_text = if pressed.1==String::new() {comm_text}else{pressed.1};
	};
	
	//if tt_e_c_i_ll[2] {println!("tecill[2], In sset_widgets E");};	
	//set the GUI centerpiece depending on whether a new game has been selected or not.
	if !new_game_init & !n_s_l_q_f[2] {
		//set_uninit_centerpiece(ids,ui); (Maybe implement this, or not)
	}else if new_game_init & (n_s_l_q_f[2..7]==[false,false,false,false,false]) & tt_e_c_i_ll[4] {
		//Put player desc here.
		show_party_stats(party,spl,p_names,tt_e_c_i_ll,ui,ids,&mut comm_text,timer,chosen_hero);
	}else if new_game_init & !n_s_l_q_f[2] & tt_e_c_i_ll[2] {
		if *idungeon==None {
			comm_text = format!("You explore the nooks and crannies of {}, but find nothing of note.",p_loc.name);
			set_comm_text(&mut comm_text,ui,ids);
		}else{
			if (*dungeon_pointer==0) & !mutm_box_vis {
				comm_text = format!("You explore the nooks and crannies of {}, and make a discovery:\n{}",p_loc.name,dungeons[idungeon.unwrap()]);
				set_comm_text(&mut comm_text,ui,ids);
				if timer>*freeze_timer+63 {
					mutm_box_vis = true;
					n_s_l_q_f[4] = false;
				};
			}else if *dungeon_pointer==1 {
				comm_text = format!("You take a step over the threshold separating {} from {}..\nYou stand in {}.",
									p_loc.name,
									dungeons[idungeon.unwrap()].name,
									dungeons[idungeon.unwrap()].scenes[0].name);
				if timer>*freeze_timer+63 {
					*dungeon_pointer = 2;
					*dream_time = true;
					n_s_l_q_f[4] = true;
					
					dungeon_updater(&mut dungeons,&mut party,idungeon.unwrap()); 
					encounter_starter_dun(party, enemies, encounter,
								&dungeons[idungeon.unwrap()].scenes[*dungeon_pointer-2],
								&dungeons[idungeon.unwrap()].denizens,
								dream_time);
				};
			}else if (*dungeon_pointer>2)
			 & (*dungeon_pointer<dungeons[idungeon.unwrap()].scenes.len()+2)
			   & !n_s_l_q_f[4] {
				*freeze_timer = timer;
				*dream_time = true;
				n_s_l_q_f[4] = true;
				encounter_starter_dun(party, enemies, encounter,
								&dungeons[idungeon.unwrap()].scenes[*dungeon_pointer-2],
								&dungeons[idungeon.unwrap()].denizens,
								dream_time);
				comm_text = format!("Having battled your way through {} you proceed to {}",
									dungeons[idungeon.unwrap()].scenes[*dungeon_pointer-3].name,
									dungeons[idungeon.unwrap()].scenes[*dungeon_pointer-2].name);
				set_comm_text(&mut comm_text,ui,ids);
				//println!("got to where I should be!");
			}else if *dungeon_pointer+1>dungeons[idungeon.unwrap()].scenes.len() {
				let len_dun = dungeons[idungeon.unwrap()].scenes.len()-1;
				comm_text = format!("You have defeated the master of {1} and return alive from {0}",
									dungeons[idungeon.unwrap()].name,
									dungeons[idungeon.unwrap()].scenes[len_dun].name);
				set_comm_text(&mut comm_text,ui,ids);
				if (*p_scape != VOID) & (*p_scape != TIME)
				 & (!scenery_l_checker(landscapes,*p_scape)<*scenery_index) {
					*scenery_index = scenery_setter(landscapes,*p_scape,centre_w,centre_h);
				};
			};
		};
	//if tt_e_c_i_ll[2] & (*dungeon_pointer>2) {println!("Still not crashed!");};
	}else if new_game_init & !n_s_l_q_f[2] & !n_s_l_q_f[4] & tt_e_c_i_ll[0] {
		//set_init_world_map(ids,ui,
						//&mut n_s_l_q_f,
						//world,
						//diff,
						//p_names,
						//party,
						//p_loc,
						//pl,
						//coords,
						//&mut comm_text,
						//timer,
						//tt_e_c_i_ll,
						//provisional_loc);
	set_init_world_map2(ids,ui,
						&mut n_s_l_q_f,
						world,
						world_map,
						mon_faces,
						diff,
						p_names,
						party,
						p_loc,
						pl,
						coords,
						&mut comm_text,
						timer,
						tt_e_c_i_ll,
						provisional_loc);
	};
	
	//set level up alert.
	if !n_s_l_q_f[4]
	 & tt_e_c_i_ll[5]
	  & !mutm_box_vis
	   & (left_menu_l>0.0)
	    & new_game_init {
		
		//println!("X0");
		widget::Canvas::new().floating(true)
							 .color(BACKGR_COLOUR)
							 .border_color(BORDER_COLOUR)
							 .border(3.0)
							 .middle_of(ids.middle_column)
							 .wh([360.0,240.0])
							 .pad(20.0)
							 .set(ids.lvl_up_alert_canvas,ui);
				 
		text_maker_m("Someone's gained enough experience to grasp something...",color::GREEN.with_luminance(0.66), 24)
											.middle_of(ids.lvl_up_alert_canvas)
											.padded_w_of(ids.lvl_up_alert_canvas,10.0)
											.set(ids.lvl_up_alert_text,ui);
		for _click in widget::Button::new().mid_bottom_of(ids.lvl_up_alert_canvas)
										   .wh([160.0,40.0])
										   .color(color::GREEN.with_luminance(0.3))
										   .label("...")
										   .set(ids.lvl_up_alert_button,ui) {
			tt_e_c_i_ll[5] = false;
			tt_e_c_i_ll[7] = false;
		};
	};
	if *dream_time & (*p_scape != VOID) & (*p_scape != TIME) {
		//println!("line 1625");
		//set pretty background.
		*scenery_index = scenery_setter(&landscapes,*p_scape,centre_w,centre_h);
		set_battle_background(ui,ids,&landscapes,*p_scape, *scenery_index,centre_w,centre_h);
	}else if tt_e_c_i_ll[2] & !n_s_l_q_f[4] & (*p_scape != VOID) & (*p_scape != TIME) {
		//println!("X1");
		set_battle_background(ui,ids,&landscapes,*p_scape, *scenery_index,centre_w,centre_h);
	}else if tt_e_c_i_ll[2] & !n_s_l_q_f[4] & (*p_scape==TIME) {
		//println!("X2");
		set_timescape(ui,ids,timer);
	};
	
	//Try to bloody well write the afterstory.
	//if tt_e_c_i_ll[2] & (*dungeon_pointer>2) {println!("Still not crashed!X2");};
	if idungeon.is_some(){
		if *dungeon_pointer==dungeons[idungeon.unwrap()].scenes.len()+2 {
			set_afterstory(ui,ids,dungeons[idungeon.unwrap()].afterstory,dungeon_pointer);
			//println!("Afterstory should have been set now!");
		};
	};
	
	//println!("Exiting set_widgets");
	(mutm_box_vis,comm_text,new_game_init,n_s_l_q_f,battled,action,diff,stage)	
}

// Generate a unique `WidgetId` for each widget.
#[recursion_limit="1024"]
widget_ids! {
	pub struct Ids {
		master,							//Main screen
		header,							//Not used anymore
		body,							//Am I even using that now?
		marker_shape, 					//highlight battling monster
		marker_shape2, 					//highlight battling monster.
		battle_background,				//image background in battle
		battle_background_time_a,		//matrix for timescape
		battle_background_time_b,		//matrix for timescape
		battle_background_time_c,		//matrix for timescape
		battle_background_time_d,		//matrix for timescape
		dungeon_afterstory,				//text story to receive after background.
		
		
		far_left_column,				//far left column containing main and play menu.
				ng_button,				//Main menu buttons.		
				lg_button,
				sg_button,
				gm_button,
				op_button,
				qt_button,
				travel_button,			//Play menu buttons
				fight_button,
				explore_button,
				cast_button,
				party_button,
				
		map_and_word,					//Main part of screen
			mid_label_can,				//label telling whether you are.
				mid_label,
			middle_column,				//The column of everything.
				global_map,				//global map (matrix version)
				global_map_image,		//global map (image version)
				center_button,			//buttons for global map versions.
				north_button,
				east_button,
				south_button,
				west_button,
				party_stats,			//party stats canvas and ensuing stuff.
					party_stats_c1,
						party_stats_a,
					party_stats_c2,
						party_stats_b,
					char1_n,			//ids of textboxes holding character names.
					char2_n,
					char3_n,
					char4_n,
					char5_n,
				spell_list_can,			//Spell list can in party stats can
					spell_list,
					spell_list_scroll,
					spell_list_title,
				party_stats_scroll,
				load_menu,				//menu of save game file buttons
					load_menu_scroll,
				partyc_can,				//battle canvas containing party
					partyc_mtrx,
				enemyn_can,				//battle canvases containing enemies.
					enemyn_mtrx,
				enemye_can,
					enemye_mtrx,
				enemys_can,
					enemys_mtrx,
				enemyw_can,
					enemyw_mtrx,
				lvl_up_alert_canvas,	//level up canvas alert.
					lvl_up_alert_text,
					lvl_up_alert_button,
			coml_box,					//comm box for text input
				comm_link,
				coml_scroll,
			mutant_menu,				//mutant menu for options dialogs.
				mut1_box,
					mut1_but,
				mut2_box,
					mut2_but,
				mut3_box,
					mut3_but,
				mut4_box,
					mut4_but,
				mut5_box,
					mut5_but,
			comm_box,					//the box which tells you everything.
				comm,
				comm_scroll,
		fight_menu_canvas,				//battle menu.
			fight_menu_tab,				//I have no idea why I've kept this.
				fight_menu,				
					fight_menu_buttons,
							at_button,
							de_button,
							ca_button,
							wa_button,
							pa_button,
							es_button,
						spare_fight_can,
						spells_can,
							spells_can_scroll,
							spells_mtrx,
		quit_canvas,					//quit canvas and pertaining buttons.
			quit_true_can,
				quit_true_but,
			quit_false_can,
				quit_false_but,
				
		options_canvas,
			opt_can_tabs,
			opt_music,
				update_song_list_button,	//update song list (why not?)
				toggle_sound_button, 		//no idea how to implement this yet.
				song_list_can,
					songl_scroll,
					song_list,
				change_song_list,
			opt_graphics,
				opt_graphics_unimp,
				opt_interface_brightness_text,
				opt_interface_brightness_slider,
				opt_background_brightness_text,
				opt_background_brightness_slider,
				opt_reload_backgrounds_default,
				opt_reload_backgrounds,
				opt_sample_background,
				opt_sample_sprite,
			opt_antlers,
				opt_antlers_unimp,
				opt_antlers_text,
				opt_antlers_slider,
				opt_antlers_reset_but,
				
		file_browser_can, //File browser for dealing with swapping out of songs.
			fb_navi,
			fb_select_but,
			fb_back_but,
			fb_cancel_but,
			fb_standard_but,
			fb_display_current,
			
		eclair_matrix, //Set lightning.
		eclair_matrix_two, //Set the lightning ends.
		fire_matrix, //Set fire.
		//NB inferno uses fire_matrix for balls, and eclair_matrix for lines (for now)
		ice_matrix, //set ice.
		healing_matrix,
		death_matrix,
		holy_matrix,
		radiant_matrix,
		time_matrix,
				
	}
}

//function to set the options menu and thereafter certain settings.
fn set_options_dialog_marker(){}
fn set_options_canvas(ref mut ui: &mut conrod::UiCell,
					  ids:&Ids,
					  ipath:&mut Option<(usize,String)>,
					  song_list: &mut Vec<String>,
					  silent_sender: &mut SyncSender<bool>,
					  wo: &mut FlowCWin,
					  faces:&Vec<[conrod::image::Id;3]>,
					  landscapes:&Landscapes) {
	// main canvas					  
	widget::Canvas::new().floating(true)
						 .color(color::BLACK)
						 .wh_of(ids.middle_column)
						 .middle_of(ids.middle_column)
						 .set(ids.options_canvas,ui);
	
	let wh_mc = ui.wh_of(ids.middle_column).unwrap();
	let font_size = font_size_chooser(&wh_mc);
	
	// tabs for each set of options.
	widget::Tabs::new(&[(ids.opt_antlers,"Antlers"),(ids.opt_graphics,"Graphics Options"),(ids.opt_music,"Sound Options")])
				 //.color(color::BLACK)
				 .pad(BORDER)
				 .border(BORDER)
				 .border_color(BORDER_COLOUR)
				 .label_color(color::GREEN)
				 .bar_thickness(50.0)
				 .wh_of(ids.middle_column)
				 .middle_of(ids.options_canvas)
				 .set(ids.opt_can_tabs,ui);
			 
	let mut wh_muse = ui.wh_of(ids.opt_can_tabs).unwrap();
	let win_wh = ui.wh_of(ids.master).unwrap();
	wh_muse = [wh_muse[0]-BORDER*2.0,wh_muse[1]-BORDER*2.0-50.0];
	let mut xy_muse = ui.xy_of(ids.opt_antlers).unwrap();
	let mut wh_butt = ui.wh_of(ids.ng_button).unwrap();
	wh_butt[0] = wh_muse[0]/2.0-BORDER;
	
	let opt_button = widget::Button::new().color(color::DARK_RED)
									   .label_font_size(font_size_chooser_button_b(win_wh[0]));
	
	//*****SET SOUND OPTIONS*******
	// Try update song list manually, if some git messes with onfig file.
	for _click in opt_button.clone().wh(wh_butt)
									.top_left_of(ids.opt_music)
									.label("Update Playlist")
									.set(ids.update_song_list_button,ui) {
		println!("Updating song list!");
		parse_music_config(song_list);									      
	}
	let silence_label = if wo.silence {"Enable Sound"}else{"Disable Sound"};
	
	// Toggle mute button
	for _click in opt_button.clone().wh(wh_butt)
									.right_from(ids.update_song_list_button,0.0)
									.label(silence_label)
									.set(ids.toggle_sound_button,ui) {
		print!("Toggling sound. past silence = {}, ",wo.silence);
		wo.silence = !wo.silence; 
		println!("present silence = {}.",wo.silence);
		silent_sender.try_send(wo.silence);								      
	}
	
	//Canvas for matrix of song buttons
	widget::Canvas::new().wh([wh_muse[0]-BORDER*2.0,wh_muse[1]-wh_butt[1]-BORDER*2.0])
						 .down_from(ids.update_song_list_button,0.0)
						 .scroll_kids_vertically()
						 .border(BORDER)
						 .border_color(BORDER_COLOUR)
						 .set(ids.song_list_can,ui);				  
	let slcs = ui.wh_of(ids.song_list_can).unwrap();
	
	//widget::Scrollbar::y_axis(ids.song_list_can).auto_hide(true).set(ids.songl_scroll, ui);
	
	//song matrix
	let mut song_matrix = widget::Matrix::new(1,song_list.len())
										.mid_top_of(ids.song_list_can)
										.wh([slcs[0]-BORDER*2.0,50.0*song_list.len() as f64])
										.set(ids.song_list, ui);
	
	//set elements of song matrix (ie buttons for changing the themes.
	while let Some(case) = song_matrix.next(ui) {
		let i = case.row;
		let text:String = format!("Theme {}: {}",i+1,song_list[i]);
		let song_button = opt_button.clone()
									.label(&text)
									.color(color::DARK_RED);
		for _click in case.set(song_button,ui) {
			
			println!("Enter function to launch file browser to find a song. What a drag");
			*ipath = Some((i,song_list[i].clone()));
			wo.mub_path = if song_list[i]=="Standard" {
				PathBuf::from(Component::RootDir.as_os_str())
			}else{	// This may go wrong.
				PathBuf::from(PathBuf::from(&song_list[i]).parent().unwrap_or(&PathBuf::from(Component::RootDir.as_os_str())))
			};
		};
	}
	
	//*****SET GRAPHICS OPTIONS*****
	//Set interface brightness text
	
	widget::Text::new(&format!("Set interface brightness: {:.2}",wo.ifc))
									.color(color::GREEN.with_luminance(0.66))
									.top_left_of(ids.opt_graphics)
									.font_size(font_size)
									.center_justify()
									.set(ids.opt_interface_brightness_text,ui);
	//Set interface brightness adjustment slider.
	let mut var = wo.ifc;
	set_h_slider_f32 (ui,ids.options_canvas,
					  ids.opt_graphics,
					  ids.opt_interface_brightness_slider,
					  ids.opt_interface_brightness_text,
					  &mut var, 1.0,-1.0,
					  BUTTON_COLOUR);
	wo.ifc = var;					
	
	//set background brightness adjustment text.	
	  
	widget::Text::new(&format!("Background brightness: {:.2}",wo.bgc))
									.color(color::GREEN.with_luminance(0.66))
									.font_size(font_size)
									.down_from(ids.opt_interface_brightness_slider,5.0)
									.center_justify()
									.set(ids.opt_background_brightness_text,ui);
	//set background brightness adjustment slider.
	let mut var2 = wo.bgc;
	set_h_slider_f32 (ui,ids.options_canvas,
					  ids.opt_graphics,
					  ids.opt_background_brightness_slider,
					  ids.opt_background_brightness_text,
					  &mut var2, 1.0,-1.0,
					  BUTTON_COLOUR);
	wo.bgc = var2;

	// Reset Background brightness (and reload)	
	for _click in opt_button.clone()
						    .wh(wh_butt)
						    .down_from(ids.opt_background_brightness_slider,2.0)
						    .label("Reload Default")
						    .set(ids.opt_reload_backgrounds_default,ui) {
		print!("Reload backgrounds to default pressed");
		wo.bgc = 0.0;
		wo.update_bgc = true;				      
	}
	// Reload Backgrounds (with new settings).
	
	for _click in opt_button.clone()
						    .wh(wh_butt)
						    .right_from(ids.opt_reload_backgrounds_default,0.0)
						    .label("Reload")
						    .set(ids.opt_reload_backgrounds,ui) {
		print!("Reload backgrounds to new value pressed");
		wo.update_bgc = true;									      
	}
	
	// Set Sample images.
	if landscapes.grassland.get(0).is_some() & !wo.update_bgc {
		widget::Image::new(landscapes.grassland[0].0)
					  .wh([wh_muse[0]/2.0,wh_muse[1]/2.0])
					  .bottom_right_of(ids.opt_graphics)
					  .set(ids.opt_sample_background,ui);
	};
	if faces.len()>1 {
		widget::Image::new(faces[0][0])
					  .wh([wh_muse[0]/8.0,wh_muse[1]/8.0])
					  .bottom_right_with_margins_on(ids.opt_graphics,wh_muse[1]/4.0,wh_muse[0]/4.0)
					  .set(ids.opt_sample_sprite,ui);
		
	};
					  		
	//******SET AI OPTIONS*********			
	widget::Text::new(&format!("Adjust Maximum memoy for AI (currently {} MB)",wo.ai_mem/1_000_000))
				  .color(color::GREEN.with_luminance(0.66))
				  .font_size(font_size)
				  .top_left_with_margin_on(ids.opt_antlers,BORDER)
				  .left_justify()
				  .set(ids.opt_antlers_unimp,ui);
				  
	for slide in conrod::widget::Slider::new(wo.ai_mem as f64,AI_MEM_MIN,AI_MEM_MAX)
									   .w(wh_muse[0]-4.0*BORDER).h(SLIDE_H)
									   .color(BUTTON_COLOUR)
									   .border(BORDER)
									   .border_color(BORDER_COLOUR)
									   .down_from(ids.opt_antlers_unimp,BORDER)
									   .set(ids.opt_antlers_slider,ui) {
		wo.ai_mem = slide as usize;										   
	};
	
	for _click in opt_button.clone()
						    .wh(wh_butt)
						    .down_from(ids.opt_antlers_slider,3.0)
						    .label("Reset Limit")
						    .set(ids.opt_antlers_reset_but,ui) {
		print!("Reset memory pressed.");
		wo.ai_mem = AI_MEM_DEFAULT;									      
	}
	
}

//Set slider (for options brightnessess.
fn set_h_slider_f32 (ui: &mut conrod::UiCell,
					 id_template: conrod::widget::Id,
					 id_can: conrod::widget::Id,
					 id_slide: conrod::widget::Id,
					 id_anchor: conrod::widget::Id,
					 var:&mut f32,
					 max:f64,min:f64,
					 slide_col: color::Colour) {
	
	let mut wh = ui.wh_of(id_template).unwrap();
	
	wh[0] = wh[0]/2.0-BORDER*2.0;
	
	let mut slide = conrod::widget::Slider::new(*var as f64,min,max)
										   .w(wh[0]).h(SLIDE_H)
										   .color(slide_col)
										   .border(BORDER)
										   .border_color(BORDER_COLOUR)
										   .down_from(id_anchor,BORDER);
												   
	for slide in slide.set(id_slide,ui){
		*var = slide as f32;
	}; 
}


// Function for browsing files and returning them
// to config file and songlist file.
// NB this function should ONLY be triggered if ipath.is_some()
// Or the world will end.
// the browser now works
fn set_music_browser(ref mut ui: &mut conrod::UiCell, ref ids: &Ids,
					 ipath:&mut Option<(usize,String)>,
					 song_list: &mut Vec<String>,
					 wo: &mut FlowCWin) {
						 
	let (i,song) = ipath.clone().unwrap_or((0,"Standard".to_owned()));
	
	
	//set file browser canvas.
	widget::Canvas::new().wh_of(ids.options_canvas)
						 .middle_of(ids.options_canvas)
						 .border(BORDER)
						 .border_color(BORDER_COLOUR)
						 .pad(BORDER)
						 .set(ids.file_browser_can,ui);
	
	let mut sap = ui.wh_of(ids.file_browser_can).unwrap();
	
	let mut b_wh = ui.wh_of(ids.op_button).unwrap();
	
	sap = [sap[0]-BORDER*2.0,sap[1]-BORDER*2.0-b_wh[1]-50.0];
	b_wh = [sap[0]/4.0,b_wh[1]];					 
	
	//Set button for restoring default song for that theme.
	for _click in widget::Button::new().color(BUTTON_COLOUR)
									   .wh(b_wh)
									   .label("Restore Default")
									   .bottom_right_of(ids.file_browser_can)
									   .set(ids.fb_standard_but,ui) {
		*ipath = None;
		defaultise_song_in_list(song_list,i);
	};
		
	
	//set move out of folder. NB, this will crash in windows. (Not any more)
	for _click in widget::Button::new().color(BUTTON_COLOUR)
									   .wh(b_wh)
									   .label("Cancel")
									   .left_from(ids.fb_standard_but,0.0)
									   .set(ids.fb_cancel_but,ui) {
		*ipath = None;		
	};
	
	//set move out of folder.
	for _click in widget::Button::new().color(BUTTON_COLOUR)
									   .wh(b_wh)
									   .label("Descent Directory")
									   .left_from(ids.fb_cancel_but,0.0)
									   .set(ids.fb_back_but,ui) {
		match wo.mub_path.has_root() {
			false => {wo.mub_path = PathBuf::from(Component::RootDir.as_os_str())},
			true  => {wo.mub_path =
				match wo.mub_path.parent() {
					Some(dad) => dad.to_owned(),
					_		  => PathBuf::from(Component::RootDir.as_os_str()),
				}},
		};					   
		println!("Gonna climb down the tree. Wherever that it.")
	};
	
		//Make text to display current root directory.
	let path = format!("Current Path: {}",wo.mub_path.as_os_str().to_str().unwrap_or("unknown"));
	widget::Text::new(&path).color(BORDER_COLOUR)
						    .top_left_of(ids.file_browser_can)
						    .set(ids.fb_display_current,ui);
	
	//set file browser onto appropriate canvas.
	for event in widget::FileNavigator::with_extension(&wo.mub_path, &["flac","ogg","wav"])
                .color(color::BLACK)
                .font_size(16)
                .text_color(color::GREEN)
                .wh(sap)
                .down_from(ids.fb_display_current,1.0)
                .set(ids.fb_navi, ui) {
		println!("{:?}", event);
		match event {
			widget::file_navigator::Event::ChangeSelection(x) => match x.len() {
				1 => {
						if x[0].extension()==Some(OsStr::new("flac"))
						|| x[0].extension()==Some(OsStr::new("ogg"))
						|| x[0].extension()==Some(OsStr::new("wav")) {
							wo.new_selection = Some(x[0].to_str().unwrap_or("").to_owned());
						};
					 },
				_ => {},			
				},
			widget::file_navigator::Event::DoubleClick(_,x) => match x.len() {
				1 => {
						if x[0].extension()==Some(OsStr::new("flac"))
						|| x[0].extension()==Some(OsStr::new("ogg"))
						|| x[0].extension()==Some(OsStr::new("wav")) {
							wo.new_selection = Some(x[0].to_str().unwrap_or("").to_owned());
							*ipath = None;
							write_music_config(wo.new_selection.clone().unwrap_or("".to_owned()),song_list,i);
						};
					 },
				_ => {},
				},
			_  => {},			
		};
	}
	
	//set select new track button (or enter folder).
	for _click in widget::Button::new().color(BUTTON_COLOUR)
									   .wh(b_wh)
									   .label("Select")
									   .left_from(ids.fb_back_but,0.0)
									   .set(ids.fb_select_but,ui) {									   
		println!("Gonna select this shit. Whatever this it.");
		if wo.new_selection.is_some() {
			*ipath = None;
			write_music_config(wo.new_selection.clone().unwrap_or("".to_owned()),song_list,i);
		};
	};
	
	
}

//function to set a dungeon's afterstory.
fn set_afterstory(ref mut ui: &mut conrod::UiCell,
				  ids:&mut Ids,
				  text:&str,
				  mut d_p: &mut usize) {

	widget::Text::new(text)
		.color(color::YELLOW)
		.font_size(24)
		.center_justify()
		.top_left_of(ids.middle_column)
		.padded_w_of(ids.middle_column,9.0)
		.line_spacing(10.0)
		.set(ids.dungeon_afterstory, ui);
}


//standard encounter generator.
fn encounter_starter(party: &mut Vec<(Lifeform,usize)>,
					 mut enemies: &mut Vec<(Lifeform,usize)>,
					 mut encounter: &mut Vec<(Lifeform,usize,[Option<[usize;2]>;2])>,
					 p_loc: &Place,
					 mons: &Vec<Lifeform>,
					 dream_time: &mut bool) {
	*enemies = engenB(&engenA(),&p_loc,mons);
	for x in party.iter() {encounter.push((x.0.clone(),x.1,[None,None]))};
	for x in enemies.iter() {encounter.push((x.0.clone(),x.1,[None,None]))};
	for x in encounter.iter() {println!("{}: {}",x.1,x.0.name)};
	*dream_time = true;
}

//Dungeon encounter generator.
fn encounter_starter_dun(party: &mut Vec<(Lifeform,usize)>,
					 mut enemies: &mut Vec<(Lifeform,usize)>,
					 mut encounter: &mut Vec<(Lifeform,usize,[Option<[usize;2]>;2])>,
					 p_loc: &Place,
					 mons: &Vec<Lifeform>,
					 dream_time: &mut bool) {
	*enemies = engenB(&engenA_dun(p_loc),&p_loc,mons);
	for x in party.iter() {encounter.push((x.0.clone(),x.1,[None,None]))};
	for x in enemies.iter() {encounter.push((x.0.clone(),x.1,[None,None]))};
	for x in encounter.iter() {println!("{}: {}",x.1,x.0.name)};
	*dream_time = true;
}

fn character_dl_mod(mut character: &mut Lifeform, dl: isize) {
	let dlb = dl as f32;
	character.Attack-= character.Attack/25.0*dlb;
	character.Defence+= character.Defence/25.0*dlb;
	character.WM+= character.WM/25.0*dlb;
	character.BM-= character.BM/25.0*dlb;
	character.Attack_shade= character.Attack;
	character.Defence_shade= character.Defence;
	character.WM_shade= character.WM;
	character.BM_shade= character.BM;
	println!("Self Attack:{}",character.Attack);
	if dlb<0.0{
		character.Spellist = match character.name {
			"Witch"=>vec![S_CURE,S_EMBER,S_DARKNESS,S_SPARK],
			"Warrior"=>vec![S_DARKNESS],
			"Wonderer"=>vec![S_EMBER,S_DARKNESS,S_SLOW],
			_=>vec![],
		};	
	}else{
		character.Spellist = match character.name {
			"Witch"=>vec![S_CURE,S_LIGHT,S_EMBER,S_EXORCISM],
			"Warrior"=>vec![S_LIGHT],
			"Wonderer"=>vec![S_CURE,S_HASTE,S_LIGHT],
			_=>vec![],
		};
	};
}

fn sidekick_maker(mut party: &mut Vec<(Lifeform,usize)>, mut p_names: &mut Vec<String>) {
	let dlb = rand::thread_rng().gen_range(-12,13) as f32;
	party[1].0.Attack-= party[1].0.Attack/25.0*dlb;
	party[1].0.Defence+= party[1].0.Defence/25.0*dlb;
	party[1].0.WM+= party[1].0.WM/25.0*dlb;
	party[1].0.BM-= party[1].0.BM/25.0*dlb;
	party[1].0.Attack_shade= party[1].0.Attack;
	party[1].0.Defence_shade= party[1].0.Defence;
	party[1].0.WM_shade= party[1].0.WM;
	party[1].0.BM_shade= party[1].0.BM;
	println!("Self Attack:{}",party[1].0.Attack);
	if dlb<0.0{
		party[1].0.Spellist = match party[1].0.name{
			"Witch"=>vec![S_CURE,S_EMBER,S_DARKNESS,S_SPARK],
			"Warrior"=>vec![S_DARKNESS],
			"Wonderer"=>vec![S_EMBER,S_DARKNESS,S_SLOW],
			_=>vec![],
		};
	}else{
		party[1].0.Spellist = match party[1].0.name{
			"Witch"=>vec![S_CURE,S_LIGHT,S_EMBER,S_EXORCISM],
			"Warrior"=>vec![S_LIGHT],
			"Wonderer"=>vec![S_CURE,S_HASTE,S_LIGHT],
			_=>vec![],
		};
	};
	p_names.push(sidekick_namer(party));
}

//display party stats.
fn marker_of_party_stats(){}
fn show_party_stats(mut party:&mut Vec<(Lifeform,usize)>,
					spl: &Vec<Spell>,
					p_names:&Vec<String>,
					mut tt_e_c_i_ll: &mut [bool;8],
					ref mut ui: &mut conrod::UiCell,
					ids:&mut Ids,
					mut comm_text: &mut String,
					timer:usize,
					mut chosen_hero: &mut usize) {

	let p_len:usize = party.len();
	let stats:usize = 10;
			
	let mut map_size = ui.wh_of(ids.middle_column).unwrap();	
		
	//A vector for storing the strings to go in the party description.
	let mut trait_vector:Vec<String> = Vec::with_capacity(p_len*stats);
	let hat_vector = vec![format!("{} the {}",p_names[0],party[0].0.name),
												  "Health ".to_owned(),
												  "Mana ".to_owned(),
												  "Speed ".to_owned(),
												  "Attack ".to_owned(),
												  "Defence ".to_owned(),
												  "White Magic ".to_owned(),
												  "Black Magic ".to_owned(),
												  "Spell List ".to_owned()];
												  
	let mut weener_lengths:Vec<f64> = Vec::with_capacity(p_len*stats);
	
	//Extends said vector for each extra party member.
	for i in 0..p_len{
		trait_vector.push(format!("{} the {}",p_names[i],party[i].0.name));
		weener_lengths.push(0.0);
		trait_vector.extend_from_slice(&hat_vector[1..9]);
		weener_lengths.extend_from_slice(&[(party[i].0.HP/1000.0) as f64,
										   (party[i].0.MP/1000.0) as f64,
										   (party[i].0.Speed/120.0) as f64,
										   (party[i].0.Attack/250.0) as f64,
										   (party[i].0.Defence/250.0) as f64,
										   (party[i].0.WM/250.0) as f64,
										   (party[i].0.BM/250.0) as f64]);
		trait_vector.push("".to_owned());
		
		if tt_e_c_i_ll[6] & ((party[i].0.Exp-party[i].0.ExpUsed)>=10.0) {
			weener_lengths.push(1.0)
		}else{
			weener_lengths.push(0.0)
		};
		weener_lengths.push(0.0);
		
	};
	trait_vector.push("".to_owned());
	
	let matrix_height = 40.0*((p_len*stats) as f64);
	
	widget::Canvas::new().flow_right(&[
		((ids.party_stats_c1),(widget::Canvas::new().length_weight(1.0)
								.color(BACKGR_COLOUR)
		)),
		((ids.party_stats_c2),(widget::Canvas::new().length_weight(2.0)
								.color(BACKGR_COLOUR)
		)),
	]).wh([map_size[0]-BORDER*2.0,matrix_height])
	  .top_left_of(ids.middle_column)
	  .scroll_kids_vertically()
	  .set(ids.party_stats,ui);
			
	//set stat names and stats widgets into the display
	//make the two halfs.
	let mut party_matrix_a = widget::Matrix::new(1,p_len*stats)
			.wh([map_size[0]/3.0-BORDER,matrix_height])
			.top_left_of(ids.party_stats_c1)
			//.scroll_kids_vertically()
			.set(ids.party_stats_a, ui);
	let mut party_matrix_b = widget::Matrix::new(1,p_len*stats)
			.wh([map_size[0]*2.0/3.0-BORDER,matrix_height])
			.top_left_of(ids.party_stats_c2)
			//.scroll_kids_vertically()
			.set(ids.party_stats_b, ui);
	
	//initialise a guest variable for spell list setter. SHOULD REALLY USE A CONSTANT INSTEAD.
	let mut party_no_for_spl:usize = 9;		
	
	//Set character names.
	//(NB, this can be done through matrix, but looks better this way.
	//Downside; can only hold 5 players max (not currently a problem).
	//That is to say if n>5 the world will end.
	let ids_vec = vec![&ids.char1_n,&ids.char2_n,&ids.char3_n,&ids.char4_n,&ids.char5_n];
	let mut t_xy = ui.xy_of(ids.party_stats_a).unwrap();
	let mc_xy = ui.xy_of(ids.middle_column).unwrap();
	t_xy = [mc_xy[0],t_xy[1]+matrix_height/2.0-10.0];
	
	let m_w:f64 = ui.w_of(ids.master).unwrap_or(1080.0);
	for i in 0..p_len {
		widget::Text::new(&trait_vector[stats*i]).color(color::GREEN)
										 .font_size(font_size_chooser_button(m_w))
										 .center_justify()
										 .wh([map_size[0]-2.0*BORDER,40.0])
										 .xy(t_xy)
										 .set(*ids_vec[i],ui);
		t_xy[1]-= 40.0*stats as f64;
	}
	
	// Set the stat bars
	if !tt_e_c_i_ll[6] {	
		//functionalise widgets for non level up scenario.	
		while let Some(mut stat_box) = party_matrix_b.next(ui) {
			let r = stat_box.row as usize;
			if r%stats==0 {
			}else{
				stat_box.w = weener_lengths[r]*map_size[0]*2.0/3.0;
				let w = stat_box.w;
				let baseline = w/2.0 - map_size[0]/3.0;
				stat_box.rel_x = conrod::utils::map_range(0 as conrod::Scalar,
															0.0,
															(p_len*stats) as conrod::Scalar,
															(-w/2.0 + (w as conrod::Scalar/2.0))+baseline,
															(w/2.0 + (w as conrod::Scalar/2.0))+baseline);
				let weener = widget::Button::new().wh([weener_lengths[r]*map_size[0]*2.0/3.0,40.0]).color(color::RED);
				stat_box.set(weener,ui);
			}
		}
	}else{
		while let Some(mut stat_box) = party_matrix_b.next(ui) {
			let r = stat_box.row as usize;
			if r%stats==0 {
			}else{
				stat_box.w = weener_lengths[r]*map_size[0]*2.0/3.0;
				let w = stat_box.w;
				let baseline = w/2.0 - map_size[0]/3.0;
				stat_box.rel_x = conrod::utils::map_range(0 as conrod::Scalar,
															0.0,
															(p_len*stats) as conrod::Scalar,
															(-w/2.0 + (w as conrod::Scalar/2.0))+baseline,
															(w/2.0 + (w as conrod::Scalar/2.0))+baseline);
															
				
				let ween_col:Colour = if ((party[r/10].0.Exp-party[r/10].0.ExpUsed)>=10.0) & ((r+2)%10==0) {
					color::GREY.with_luminance(sync_t(timer*3/2))
				}else if (party[r/10].0.Exp-party[r/10].0.ExpUsed)>=10.0 {
					color::RED.with_luminance(sync_t(timer))
				}else{
					color::RED
				};
				if ((party[r/10].0.Exp-party[r/10].0.ExpUsed)>=10.0) & ((r+2)%10==0) {
					for _click in stat_box.set(widget::Button::new().wh([weener_lengths[r]*map_size[0]*2.0/3.0,40.0])
										 .color(ween_col)
										 .label("New Arcana")
										 .label_font_size(font_size_chooser_button_b(m_w))
										 .label_color(ween_col.invert()),
					ui) {
						//allow setting of learnable spell list.
						if !tt_e_c_i_ll[7] {
							*chosen_hero = r/10;
							tt_e_c_i_ll[7] = true;
						}else if *chosen_hero==r/10 {
							tt_e_c_i_ll[7] = false;
						}else{
							*chosen_hero = r/10;
						};
					};
				}else{
					for _click in stat_box.set(widget::Button::new().wh([weener_lengths[r]*map_size[0]*2.0/3.0,40.0])
																	.color(ween_col),ui) {
						lvl_upg(party,r,tt_e_c_i_ll);
					};
				};
			}
		}
	};
			
	//Set the labels and party names.		
	while let Some(mut stat_box) = party_matrix_a.next(ui) {
		let r = stat_box.row as usize;
		if r%stats==0 {
			//NB, party names now handled seperately.
			//let text_box = widget::TextBox::new(&trait_vector[r]).text_color(color::GREEN.with_luminance(0.66))
															//.font_size(32)
															//.left_justify()
															//.border(BORDER)
															//.border_color(BORDER_COLOUR)
															//.w(map_size[0]-2.0*BORDER);
			//stat_box.set(text_box,ui);
		}else if (r+2)%stats==0 {
			for _click in stat_box.set(
				widget::Button::new().label(&trait_vector[r])
									 .label_font_size(font_size_chooser_button_b(m_w))
									 .border_color(BORDER_COLOUR)
									 .border(BORDER)
									 .label_color(color::GREEN.with_luminance(0.66))
									 .color(BACKGR_COLOUR),
				ui) {
				//allow setting of spell list.
				if !tt_e_c_i_ll[7] {
					*chosen_hero = r/10;
					tt_e_c_i_ll[7] = true;
				}else if *chosen_hero==r/10 {
					tt_e_c_i_ll[7] = false;
				}else{
					*chosen_hero = r/10;
				};
			};
		}else{
			stat_box.set(text_maker_r( &trait_vector[r],
									   color::GREEN.with_luminance(0.66),
									   font_size_chooser_button_b(m_w)
									  ),
						 ui);
		};
	};
	
	
	//set spell list if permitted	
	if tt_e_c_i_ll[7] & tt_e_c_i_ll[6] {
		set_learnable_spell_list(ui,ids,comm_text,party,tt_e_c_i_ll,spl,p_names,*chosen_hero,m_w);
	}else if tt_e_c_i_ll[7] {
		set_spell_list(ui,ids,comm_text,party,spl,p_names,*chosen_hero,m_w);
	};
}

//set spell list into party inspector
fn set_spell_list (ref mut ui: &mut conrod::UiCell,
				   ids:& Ids,
				   mut comm_text:&mut String,
				   party: &Vec<(Lifeform,usize)>,
				   spl: &Vec<Spell>,
				   p_names:&Vec<String>,
				   i: usize,
				   w: f64) { //nb i is "chosen_hero"
	
	//set up some variables for canvas size.				   
	let mut matrix_rows:usize = 1;
	let rows:usize = if party[i].0.Spellist.len()==0 {
		1
	}else if party[i].0.Spellist.len()<10 {
		matrix_rows = party[i].0.Spellist.len()+1;
		party[i].0.Spellist.len()+1
	}else{
		matrix_rows = 9;
		party[i].0.Spellist.len()+1
	};	
	
	//calculate canvas sized based on spb length and window size.
	let mut wh_m = ui.wh_of(ids.middle_column).unwrap_or([600.0,400.0]);
	let mrf64:f64 = 50.0*(matrix_rows as f64);
	
	if wh_m[0]>400.0 {wh_m[0] = 400.0;};
	if wh_m[1]>mrf64 {wh_m[1] = mrf64;};
	
	//make canvas for spell_list.
	if party[i].0.Spellist.len()>9 {
		widget::Canvas::new()
			.scroll_kids_vertically()
			.floating(true)
			.mid_top_of(ids.map_and_word)
			.wh(wh_m)
			.pad(10.0)
			.color(BACKGR_COLOUR)
			.border(BORDER)
			.border_color(BORDER_COLOUR)
			.set(ids.spell_list_can,ui);
		widget::Scrollbar::y_axis(ids.spell_list_can).auto_hide(true).set(ids.coml_scroll, ui);
	}else{
		widget::Canvas::new()
			.floating(true)
			.mid_top_of(ids.map_and_word)
			.wh(wh_m)
			.pad(10.0)
			.color(BACKGR_COLOUR)
			.border(BORDER)
			.border_color(BORDER_COLOUR)
			.set(ids.spell_list_can,ui);
	};
	
	//make matrix containing spell list.	
	let mut spell_list = widget::Matrix::new(1,rows)
					   .w(wh_m[0]-2.0*BORDER)
					   .h(40.0*(rows as f64)-BORDER*2.0)
					   .mid_top_of(ids.spell_list_can)
					   .set(ids.spell_list,ui);
					   
	//Write spell list if character has spells. Write sorry otherwise.
	if party[i].0.Spellist.len()>0 {
		while let Some(spell) = spell_list.next(ui) {
			let snow = spell.row;
			let spell_name:String = arcana_name_from_spell_id(spl,party[i].0.Spellist[snow-1]);
			if snow==0{
				let title:String = format!("{} the {}'s Spellbook",p_names[i],party[i].0.name);
				spell.set(text_maker_m(&title,color::YELLOW,font_size_chooser_button_b(w)),ui);
			}else{
				let spell_out_spell:&Spell = &spl[arcana_index_from_spell_id(spl,party[i].0.Spellist[snow-1]).unwrap()];
				let x = widget::Button::new().label(&spell_name)
											 .label_font_size(font_size_chooser_button_b(w))
											 .color(colour_of_magic(spell_out_spell.Type));
				for _click in spell.set(x,ui){
					*comm_text = format!("{}",spell_out_spell);
					set_comm_text(&mut comm_text,ui,ids);
				};
			};
		};
	}else{
		while let Some(spell) = spell_list.next(ui) {
			let spell_button_label:String = format!("{} knows no spells...",p_names[i]);
			let x = widget::Button::new().label(&spell_button_label)
										 .label_font_size(font_size_chooser_button_b(w))
										 .color(BUTTON_COLOUR);
			spell.set(x,ui);
		};	
	};	   
}

fn set_learnable_spell_list_marker(){}
//A function to learn new spells.
fn set_learnable_spell_list (ref mut ui: &mut conrod::UiCell,
				   ids:& Ids,
				   mut comm_text:&mut String,
				   mut party: &mut Vec<(Lifeform,usize)>,
				   mut tt_e_c_i_ll: &mut [bool;8],
				   spl: &Vec<Spell>,
				   p_names:&Vec<String>,
				   i: usize,
				   w: f64) { //nb i is "chosen_hero"
					   
	//make a list of spells that the player has enough exp to learn.
	let mut learnable_spells:Vec<&str> = Vec::new();
	
	for x in spl.iter(){
		if (x.MP <= party[i].0.Exp - party[i].0.ExpUsed)
		 & !lmoose::lhas(&party[i].0.Spellist,&x.id) {learnable_spells.push(x.name);};
	 };
	
	let mut matrix_rows:usize = 1;
	let rows:usize = if learnable_spells.len()==0 {
		1
	}else if learnable_spells.len()<10 {
		matrix_rows = learnable_spells.len()+1;
		learnable_spells.len()+1
	}else{
		matrix_rows = 9;
		learnable_spells.len()+1
	};				   
	
	//make canvas for spell_list.
	let mut wh_m = ui.wh_of(ids.middle_column).unwrap_or([600.0,400.0]);
	let mrf64:f64 = 50.0*(matrix_rows as f64);
	
	if wh_m[0]>400.0 {wh_m[0] = 400.0;};
	if wh_m[1]>mrf64 {wh_m[1] = mrf64;};
	
	if learnable_spells.len()>9 {
		widget::Scrollbar::y_axis(ids.spell_list_can).auto_hide(true).set(ids.coml_scroll, ui);
		widget::Canvas::new()
		.floating(true)
		.scroll_kids_vertically()
		.mid_top_of(ids.map_and_word)
		.wh(wh_m)
		.pad(10.0)
		.color(BACKGR_COLOUR)
		.border(BORDER)
		.border_color(BORDER_COLOUR)
		.set(ids.spell_list_can,ui);
	}else{
		widget::Canvas::new()
		.floating(true)
		.mid_top_of(ids.map_and_word)
		.wh(wh_m)
		.pad(10.0)
		.color(BACKGR_COLOUR)
		.border(BORDER)
		.border_color(BORDER_COLOUR)
		.set(ids.spell_list_can,ui);
	};

	//make matrix containing spell list.	
	let mut spell_list = widget::Matrix::new(1,rows)
					   .w(wh_m[0]-BORDER*2.0)
					   .h(40.0*(rows as f64)-BORDER*2.0)
					   .mid_top_of(ids.spell_list_can)
					   .set(ids.spell_list,ui);
					   
	//Write spell list if character has spells. Write sorry otherwise.
	if learnable_spells.len()>0 {
		while let Some(spell) = spell_list.next(ui) {
			let snow = spell.row;
			if snow==0{
				let title:String = format!("{} the {} can learn:",p_names[i],party[i].0.name);
				spell.set(text_maker_m(&title,color::YELLOW,font_size_chooser_button_b(w)),ui);
			}else{
				let spell_out_spell:&Spell = &spl[arcana_index_from_spell_name(spl,learnable_spells[snow-1]).unwrap()];
				let x = widget::Button::new().label(learnable_spells[snow-1])
											 .label_font_size(font_size_chooser_button_b(w))
											 .color(colour_of_magic(spell_out_spell.Type));
				for _click in spell.set(x,ui) {
					*comm_text = format!("{} reached out for {} and made it a part of their soul...",p_names[i],spell_out_spell);
					set_comm_text(&mut comm_text,ui,ids);
					party[i].0.Spellist.push(spl[arcana_index_from_spell_name(spl,learnable_spells[snow-1]).unwrap()].id);
					party[i].0.ExpUsed+= spell_out_spell.MP;
					println!("{:?}",party[i].0.Spellist);
					tt_e_c_i_ll[7] = false;
				};
			};
		};
	}else{
		while let Some(spell) = spell_list.next(ui) {
			let spell_button_label:String = format!("There is no arcana for {} to grasp...",p_names[i]);
			let x = widget::Button::new().label(&spell_button_label)
										 .label_font_size(font_size_chooser_button_b(w))
										 .color(BUTTON_COLOUR);
			spell.set(x,ui);
		};	
	};	   
}

//set_mutant_menu_bin (ui: &mut conrod::UiCell, ids: &mut Ids,a:&str,e:&str,comm_text:String)

//function to cross the pole correctly.
fn cross_pole(strt_longitude:usize, wld: &Vec<[Place;19]>)->usize {

	if strt_longitude+wld.len()/2 < wld.len() {
		strt_longitude + wld.len()/2
	}else{
		strt_longitude - wld.len()/2
	}
}


//function to travel down the world map:
pub fn travel_down(mut pl:&mut (usize,usize),
				   mut p_loc:&mut Place,
				   world:&Vec<[Place;19]>,
				   mut coords:&mut [i32;2],
				   timer:usize,
				   mut freeze_timer: &mut usize,
				   mut comm_text: &mut String,
				   mut n_s_l_q_f: &mut [bool;7],
				   mut party:&mut Vec<(Lifeform,usize)>,
				   mut enemies:&mut Vec<(Lifeform,usize)>,
				   mut encounter:&mut Vec<(Lifeform,usize,[Option<[usize;2]>;2])>,
				   mons:&Vec<Lifeform>,
				   mut dream_time: &mut bool) {
	
	if timer > *freeze_timer+TRAVEL_DELAY {
		let temp_pl:(usize,usize) = if pl.1<17 {(pl.0,pl.1+1)}else{(cross_pole(pl.0,world),18)};
		let wml = world.len()-1-temp_pl.0;
		if world[wml][temp_pl.1].scape != VOID {
			*freeze_timer = timer;
			*pl = temp_pl;
			*p_loc = world[wml][pl.1].clone(); 
			*coords = p_loc.xy;
			*comm_text = format!("You journey south to {}...",p_loc.name);
			if rand_enc(p_loc) {
				n_s_l_q_f[4] = true;
				encounter_starter(party, enemies, encounter, p_loc, mons, dream_time);
				*comm_text = format!("{}\n...And are met with a warm welcome!",comm_text);
			};
		}else{
			*comm_text = "You cannot travel through the Void.".to_owned();
		};
	};
}

pub fn travel_up(mut pl:&mut (usize,usize),
				   mut p_loc:&mut Place,
				   world:&Vec<[Place;19]>,
				   mut coords:&mut [i32;2],
				   timer:usize,
				   mut freeze_timer: &mut usize,
				   mut comm_text: &mut String,
				   mut n_s_l_q_f: &mut [bool;7],
				   mut party:&mut Vec<(Lifeform,usize)>,
				   mut enemies:&mut Vec<(Lifeform,usize)>,
				   mut encounter:&mut Vec<(Lifeform,usize,[Option<[usize;2]>;2])>,
				   mons:&Vec<Lifeform>,
				   mut dream_time: &mut bool) {
	
	if timer > *freeze_timer+TRAVEL_DELAY {
		let temp_pl:(usize,usize) = if pl.1>0 {(pl.0,pl.1-1)}else{(cross_pole(pl.0,world),0)};
		let wml = world.len()-1-temp_pl.0;
		if world[wml][temp_pl.1].scape != VOID {
			*freeze_timer = timer;
			*pl = temp_pl;
			*p_loc = world[wml][pl.1].clone(); 
			*coords = p_loc.xy;
			*comm_text = format!("You journey north to {}...",p_loc.name);
			if rand_enc(p_loc) {
				n_s_l_q_f[4] = true;
				encounter_starter(party, enemies, encounter, p_loc, mons, dream_time);
				*comm_text = format!("{}\n...And are met with a warm welcome!",comm_text);
			};
		}else{
			*comm_text = "You cannot travel through the Void.".to_owned();
		};
	};
}

pub fn travel_left(mut pl:&mut (usize,usize),
				   mut p_loc:&mut Place,
				   world:&Vec<[Place;19]>,
				   mut coords:&mut [i32;2],
				   timer:usize,
				   mut freeze_timer: &mut usize,
				   mut comm_text: &mut String,
				   mut n_s_l_q_f: &mut [bool;7],
				   mut party:&mut Vec<(Lifeform,usize)>,
				   mut enemies:&mut Vec<(Lifeform,usize)>,
				   mut encounter:&mut Vec<(Lifeform,usize,[Option<[usize;2]>;2])>,
				   mons:&Vec<Lifeform>,
				   mut dream_time: &mut bool) {
	
	if timer > *freeze_timer+TRAVEL_DELAY {
		let temp_pl:(usize,usize) = if pl.0>0 {(pl.0-1,pl.1)}else{(world.len()-1,pl.1)};
		let wml = world.len()-1-temp_pl.0;
		if world[wml][temp_pl.1].scape != VOID {
			*freeze_timer = timer;
			*pl = temp_pl;
			*p_loc = world[wml][pl.1].clone(); 
			*coords = p_loc.xy;
			*comm_text = format!("You journey west to {}...",p_loc.name);
			if rand_enc(p_loc) {
				n_s_l_q_f[4] = true;
				encounter_starter(party, enemies, encounter, p_loc, mons, dream_time);
				*comm_text = format!("{}\n...And are met with a warm welcome!",comm_text);
			};
		}else{
			*comm_text = "You cannot travel through the Void.".to_owned();
		};
	};
}


pub fn travel_right(mut pl:&mut (usize,usize),
				   mut p_loc:&mut Place,
				   world:&Vec<[Place;19]>,
				   mut coords:&mut [i32;2],
				   timer:usize,
				   mut freeze_timer: &mut usize,
				   mut comm_text: &mut String,
				   mut n_s_l_q_f: &mut [bool;7],
				   mut party:&mut Vec<(Lifeform,usize)>,
				   mut enemies:&mut Vec<(Lifeform,usize)>,
				   mut encounter:&mut Vec<(Lifeform,usize,[Option<[usize;2]>;2])>,
				   mons:&Vec<Lifeform>,
				   mut dream_time: &mut bool) {
	
	if timer > *freeze_timer+TRAVEL_DELAY {
		let temp_pl:(usize,usize) = if pl.0<world.len()-1 {(pl.0+1,pl.1)}else{(0,pl.1)};
		let wml = world.len()-1-temp_pl.0;
		if world[wml][temp_pl.1].scape != VOID {
			*freeze_timer = timer;
			*pl = temp_pl;
			*p_loc = world[wml][pl.1].clone(); 
			*coords = p_loc.xy;
			*comm_text = format!("You journey east to {}...",p_loc.name);
			if rand_enc(p_loc) {
				n_s_l_q_f[4] = true;
				encounter_starter(party, enemies, encounter, p_loc, mons, dream_time);
				*comm_text = format!("{}\n...And are met with a warm welcome!",comm_text);
			};
		}else{
			*comm_text = "You cannot travel through the Void.".to_owned();
		};
	};
}



//Type A worldwalker function (after taking into account of voidwalking.
fn go_there(mut comm_text:&mut String, ref mut ui:&mut conrod::UiCell, ids: &mut Ids,
							 mut party:&mut Vec<(Lifeform,usize)>,
							 p_names:&Vec<String>,
							 mut enemies:&mut Vec<(Lifeform,usize)>,
							 mut encounter:&mut Vec<(Lifeform,usize,[Option<[usize;2]>;2])>,
							 pl:&mut (usize,usize),
							 p_loc:&mut Place,
							 world:&Vec<[Place;19]>,
							 mons:&Vec<Lifeform>,
							 coords:&mut [i32;2],
							 clicked:&(usize,usize),
							 mut tt_e_c_i_ll: &mut [bool;8],
							 mut n_s_l_q_f: &mut [bool;7],
							 mut dream_time: &mut bool) {
	let lon:usize = clicked.0;
	let lat:usize = clicked.1;
	
	let wml = world.len()-1-lon;
	
	if comm_text.contains("\nTravel to ") {
		*comm_text = format!("{}",comm_text);
	}else{
		*comm_text = format!("{}\nTravel to {}?",comm_text,world[wml][lat].name);
	};
	set_comm_text(comm_text,ui,ids);

	let mut_but = widget::Button::new().color(BACKGR_COLOUR).border(BORDER).border_color(BORDER_COLOUR);
	for _click in mut_but.clone().label("Go there!")
								 .label_color(color::GREEN.with_luminance(0.66))
								 .wh_of(ids.mut1_box)
								 .middle_of(ids.mut1_box)
								 .set(ids.mut1_but,ui){
		*pl = (lon,lat);
		*p_loc = world[wml][lat].clone();
		*coords = p_loc.xy;
		*comm_text = format!("You have gone to {}",world[wml][lat].name);
		set_comm_text(&mut comm_text,ui,ids);
		tt_e_c_i_ll[1] = false;
		if rand_enc(p_loc) {
			n_s_l_q_f[4] = true;
			encounter_starter(party, enemies, encounter, p_loc, mons, dream_time);
			*comm_text = format!("{}\n...Where you are clearly expected!",comm_text);
		};
	};
	for _click in mut_but.clone().label("Please don't..")
								 .label_color(color::GREEN.with_luminance(0.66))
								 .wh_of(ids.mut5_box)
								 .middle_of(ids.mut5_box)
								 .set(ids.mut5_but,ui){
		*comm_text = "Ok".to_owned();
		set_comm_text(&mut comm_text,ui,ids);
		tt_e_c_i_ll[1] = false;
	};
		
								 
								 
}
							 
//luminosity generation based on frame.
fn sync_t(timer:usize)->f32 {((timer%30) as f32)/120.0+0.25}

//relative size generation based on frame.
fn sync_s(timer:usize)->f64 {((timer%30) as f64)/60.0+0.5}

//avator positional oscillation based on timer.
fn shake_pos_b(timer:usize,shake_timer:usize,shake:bool)->conrod::Scalar {
	if !shake {
		0.0
	}else{
		match (timer-shake_timer)%4 {
			0 => 0.0,
			1 => 1.0,
			2 => 0.0,
			3 => -1.0,
			_ => 0.0,	
		}
	}
}

//avator positional oscillation based on timer.
fn shake_pos_a(timer:usize,shake_timer:usize,shake:bool)->conrod::Scalar {
	if !shake {
		0.0
	}else{
		match (timer-shake_timer)%3 {
			0 => 1.0,
			1 => -1.0,
			2 => 2.0,
			_ => -2.0,	
		}
	}
}


//state match return colour
fn sm_retc(x:&Lifeform,t:usize)->conrod::color::Colour{
	let stater:f32 = x.HP_shade/x.HP;
	let state:i32 =
		if stater>= 1.0{5}
		else if (stater<1.0) & (stater>=0.75){4}
		else if (stater<0.75) & (stater>=0.5){3}
		else if (stater<0.5) & (stater>=0.25){2}
		else if(stater<0.25) & (stater>0.0){1}
		else{0};
	match state{
		5=>color::GREEN,
		4=>color::YELLOW,
		3=>color::ORANGE,
		2=>color::RED,
		1=>color::RED.with_luminance(sync_t(t)),
		0=>color::RED.with_luminance(0.1),
		_=>color::BLACK
	}
}


pub fn correct_comm_text(mut comm_text:&mut String,pause:bool,mut n_s_l_q_f:&mut [bool;7]){
	if pause & n_s_l_q_f[4] {
		if !comm_text.contains("***Press Enter to Continue***") {
			*comm_text = format!("{}\n***Press Enter to Continue***",comm_text);
		};
	};
}

//END of GUI functions.
//END of GUI functions.
//END of GUI functions.
//END of GUI functions.
//END of GUI functions.
//END of GUI functions.
//END of GUI functions.
//END of GUI functions.
//END of GUI functions.


    //	Learn how to do this properly.
    //	Learn how to do this properly.
    //	Learn how to do this properly.
    //	Learn how to do this properly.
//Now in gmoose.
//moose will be transferred into gmoose to prevent dependency hell
//moose will be transferred into gmoose to prevent dependency hell
//moose will be transferred into gmoose to prevent dependency hell
//moose will be transferred into gmoose to prevent dependency hell
//moose will be transferred into gmoose to prevent dependency hell
//moose will be transferred into gmoose to prevent dependency hell
//moose will be transferred into gmoose to prevent dependency hell
//moose will be transferred into gmoose to prevent dependency hell
//moose will be transferred into gmoose to prevent dependency hell

//extracts encounter/party names for later use.

fn dungeon_updater (mut dungeons:&mut Vec<Dungeon>, party:&mut Vec<(Lifeform,usize)>,id:usize) {
	match dungeons[id].name {
		"the Citadel of Spirit" => {
									dungeons[id] = citadel_of_spirit(party[0].0.clone()).clone();
									println!("Updating Citadel of Spirit");
								},
		"the Black Door" => {
							 dungeons[id] = door_to_darkness(&party).clone();
							 println!("Updating the black door");
							},
		"Tower of Soul" => {
							dungeons[id] = tower_of_soul(&party).clone();
							println!("Updating tower of soul");
						},
		"the Wild Hunt" => {
							dungeons[id] = wild_hunt().clone();
							println!("Wild hunting");
						},
						_   => {println!("Entering dungeon: It needn't update.")},				
	};
}

//extracts names of players and monsters and puts them in a vector.
pub fn names_of(encounter:&Vec<(Lifeform,usize,[Option<[usize;2]>;2])>)->Vec<String>{
	let mut enc_names:Vec<String> = Vec::with_capacity(25);
	for x in encounter.iter(){enc_names.push(x.0.name.to_owned())};
	enc_names
}

//calculates relative threat level of one lifeform vs another.
fn dangerous(L:&Lifeform,S:&Lifeform)->f32{
	let sp_val_L=L.Spellist.len() as f32;
	let lf_eval_L=L.HP_shade*(L.WM_shade+L.Defence_shade)
	           +(L.MP_shade+10.0*sp_val_L)*(L.WM_shade+L.BM_shade)
	           +L.Speed_shade*(L.WM_shade+L.BM_shade+L.Attack_shade)*10.0
	           +sp_val_L;
	let sp_val_S=S.Spellist.len() as f32;
	let lf_eval_S=S.HP_shade*(S.WM_shade+S.Defence_shade)
	           +(S.MP_shade+10.0*sp_val_S)*(S.WM_shade+S.BM_shade)
	           +S.Speed_shade*(S.WM_shade+S.BM_shade+S.Attack_shade)*10.0
	           +sp_val_S;
    lf_eval_L-lf_eval_S
}

//legacy algorithm for choosing a target (ie, attack the weakest enemy).
fn dumb_choice_a_marker(){}
fn dumb_target_chooser_legacy(idm:usize,				//target id.
							  cms:usize,				//encounter len.
							  encounter:&Vec<(Lifeform,usize,[Option<[usize;2]>;2])>,	//encounter
							  battle_ifast:usize) -> usize {
								  
	let mut dngrrs: Vec<f32>=vec!(0.0;cms);
	for d in 0..cms{dngrrs[d]=dangerous(&encounter[d].0,&encounter[battle_ifast].0)};
	let mut dngm:f32=0.0;
	let mut min:f32=f32::MAX;	
	for j in 0..dngrrs.len(){
		if (min>=dngrrs[j])
		 & (encounter[battle_ifast].1!=encounter[j].1)
		 & (encounter[j].0.HP_shade>0.0) {
			 min = dngrrs[j];
		 };
	};
	dngm=min;
	vwhich(&dngrrs,dngm).unwrap_or(idm)
}

//Better target picking algorithm:
//Find most dangerous group for YOU and attack quishiest member.
//NB currently silly as does not compare BM vs WM, attack vs defence.
fn dumb_choice_b_marker(){}
fn dumb_target_chooser (idm:usize,
						cms:usize,
						encounter:&Vec<(Lifeform,usize,[Option<[usize;2]>;2])>,
						battle_ifast:usize) -> usize {
	
	//initiate vectors.
	let mut squish: Vec<f32> =  Vec::with_capacity(cms);
	let mut dngr_grs: [f32;5] = [0.0;5];
	
	//assign danger ratings and quish
	for x in encounter.iter() {
		dngr_grs[x.1]+= dangerous(&x.0,&encounter[battle_ifast].0);
		squish.push(x.0.Defence_shade+x.0.WM_shade);
	};
	
	//assign group danger ratings.
	let mut target_gr:usize = 0;
	let mut highest_danger:f32 = 0.0;
	
	for (i,x) in dngr_grs.iter().enumerate() {
		if (*x>highest_danger) & (i != encounter[battle_ifast].1) {
			highest_danger = *x;
			target_gr = i;
		};
	};
	
	//pick squishiest from dangerous group.
	let mut min:f32 = f32::MAX;	
	for (def,mon) in squish.iter().zip(encounter.iter()){
		if (min>=*def)
		 & (encounter[battle_ifast].1 != mon.1)
		 & (mon.1==target_gr)
		 & (mon.0.HP_shade>0.0) {
			 min = *def
		 };
	};
	
	vwhich(&squish,min).unwrap_or(idm)
}

//modify layout if start button is pressed.
fn start_game_pressed_a(ref mut ui: conrod::UiCell,
              ids: &mut Ids,
              mut comm_text:String) ->  (usize,bool) {
				  
	let mut ans_2:(usize,bool)=(0,false);
    //Difficulty level setting!..Give message.   

		set_comm_text(&mut "Which way would you do it?".to_owned(),ui,ids);
    //Difficulty level... Initiate buttons. NOT IMPLEMENTED I think?
    let mut_but = widget::Button::new().color(BACKGR_COLOUR).border(BORDER).border_color(BORDER_COLOUR);
    for _click in mut_but.clone().label("Rare")
								 .label_color(color::GREEN.with_luminance(0.66))
								 .middle_of(ids.mut1_box)
								 .set(ids.mut1_but,ui){
		ans_2 = (1,true);
		};
    for _click in mut_but.clone().label("Medium Rare")
								 .label_color(color::GREEN.with_luminance(0.66))
								 .middle_of(ids.mut2_box)
								 .set(ids.mut3_but,ui){
		ans_2 = (1,true);
		};	
    for _click in mut_but.clone().label("Well Done")
								 .label_color(color::GREEN.with_luminance(0.66))
								 .middle_of(ids.mut3_box)
								 .set(ids.mut3_but,ui){
		ans_2 = (1,true);
		};
    for _click in mut_but.clone().label("Scorched Earth")
								 .label_color(color::GREEN.with_luminance(0.66))
								 .middle_of(ids.mut4_box)
								 .set(ids.mut4_but,ui){
		ans_2 = (1,true);
		};
	ans_2
}


//START GAME FUNCTION OVER.



//This fucntion is for picking sidekick's name from a thematic list.
fn sidekick_namer(x:&Vec<(Lifeform,usize)>)-> String{
	let mut names:Vec<&str>=Vec::new();
	if x[1].0.name=="Witch"{
		names.extend(["Madoka","Walpurgis","Yaga","Maud","Cassandra","Circe","Morgan","Hecate","Medea","Agamede","Ariadne"].iter());
	}else if x[1].0.name=="Warrior"{
		names.extend(["Andromachos","Hector","Achilles","Bruce","Jackie","Musashi","Ilya","Alexander","Gengis","Julius"].iter());
	}else{
		names.extend(["Kino","Odysseos","Scott","Piri","Amsund","Marco","Yuri","Neil","Orpheus","Jason","Mallory","Messner","Kirk"].iter());
	};
    let picker=rand::thread_rng().gen_range(0,names.len());
    let nome=names[picker].to_string();
    println!("\nYour sidekick happens to be {} the {}.", &nome, x[1].0.name);
    //Class and name setter. OVER.
    nome
}

//engenA (generates battle variable y).
fn engenA()->Vec<usize> {
	
	println!("Entering engenA");
    let ngroups=rand::thread_rng().gen_range(0,10);
    let ngroups=[1,1,1,2,2,2,3,3,3,4][ngroups];
    let mut gsizes: Vec<usize>=vec!(0;ngroups);
    for i in 0..ngroups{
		let ggen=rand::thread_rng().gen_range(0,10);
        gsizes[i]=[1,1,1,2,2,2,3,3,4,4][ggen]
    };
	println!("Exiting engenA");
    gsizes
}

//engenA for dungeons (generates battle variable y).
fn engenA_dun(locus: &Place)->Vec<usize> {
	println!("Entering engenA_dun");
    let ngroups=rand::thread_rng().gen_range(0,10);
    let ngroups=locus.engenA[ngroups];
    let mut gsizes: Vec<usize>=vec!(0;ngroups);
    for i in 0..ngroups {
		let ggen=rand::thread_rng().gen_range(0,10);
        gsizes[i]=locus.engenG[ggen]
    };
	println!("Exiting engenA_dun");
    gsizes
}

//engenB (generates battle variable x).
fn engenB<'a,'b>(A:&'a Vec<usize>,B:&'b Place,bestiary:&Vec<Lifeform>)->Vec<(Lifeform,usize)>{
	println!("Entering engenB. Locale: {}. Populatations {}. engenA: {:?}",B.name,B.popu.len(),A);
    let mut enemies: Vec<(Lifeform,usize)> = Vec::new();
    let mut totapop = 0;
    let mut tomoty: Vec<u8> = Vec::new();
    let mut tomo: Vec<&str> = Vec::new();
    let mut mon_type:u8 = GOBLIN;
    let n_groups = A.len();
     println!("EngenB initiated");
     //generate the thread_gen max value and the type it corresponds to.
    for l in 0..B.popu.len() {
		let subpop=vec!(B.popu[l].0; B.popu[l].2 as usize);
		let subtyp=vec!(B.popu[l].1; B.popu[l].2 as usize);
        totapop +=B.popu[l].2;
        tomo.extend(subpop);
        tomoty.extend(subtyp);
    };
    println!("threadgen value obtained.tomo: {}. tomoty: {}.",tomo.len(),tomoty.len());
    //generate group type and govern inner loops.
    for i in 0..n_groups{
        //generate type for group [i]
        let i_type=rand::thread_rng().gen_range(0,totapop)  as usize;
        mon_type=tomoty[i_type];
        //generate each entity in group [i]. NEED TO:select monsters.
        println!("A: {:?}",A);
        for k in 0..A[i]{
			let mut enemy_n: &str = "";
			let mut k_name:usize = 0;
			//Make sure that all monsters are of the same type, this will get complex.
			loop{
			    k_name=rand::thread_rng().gen_range(0,totapop) as usize;
			    if tomoty[k_name]==mon_type{break}else{}
			    println!("Post cleansing tomoty length: {}",tomoty.len());	   
			};
			enemy_n = tomo[k_name];
			let enemy:Lifeform = bestiary[vvwhich_ln(&bestiary,enemy_n)[0]].clone();
			enemies.push((enemy,i+1))
		};
    };
	println!("Exiting engenB");
    enemies
}

fn marker_of_player_battle_turn(){}

//gmoose function for the player's turn in battle.
//NB do not switch n_s_l_q_f[4] off here. This is now fully regulated
//by the game_over() function.
pub fn player_battle_turn  (mut encounter: &mut Vec<(Lifeform,usize,[Option<[usize;2]>;2])>,
						ns:&Vec<String>,
						p_loc: &Place,
						y: &Vec<Spell>,
						cms: usize,
						mut battle_fast:&mut f32,
						mut battle_ifast:&mut usize,
						mut battle_ttakes:&mut Vec<usize>,
						mut battle_tturns:&mut u16,
						mut battle_orders:&mut Vec<(u8,u8)>,
						mut battle_timer:&mut Vec<f32>,
						mut in_battle_record:&mut Vec<[u8;28]>,
						timer: usize,
						mut freeze_timer:&mut usize,
						mut yt_adcwpe_bw: &mut [bool;9],
						mut n_s_l_q_f: &mut [bool;7],
						mut recl: &mut [u8;28],
						mut comm_text: &mut String,
						mut sel_targets:&mut Vec<usize>,
						mut to_cast: &mut String,
						mut to_hit: &mut Vec<(bool,bool)>,
						mut pause: &mut bool,
						mut escape: &mut bool,
						mut shaking_timer: &mut usize,
						mut shaking_dam: &mut [bool;25],
						mut sprite_boxer: &mut GraphicsBox,
						sprite_pos: &mut [[f64;2];25],
						targets:&mut Vec<usize>) {

	let mut turn_over = false;
	if *yt_adcwpe_bw==[false,false,false,false,false,false,false,false,false] {
		*sel_targets = Vec::with_capacity(25);
		*targets = Vec::with_capacity(25);
		println!("Monster number {} to go.",battle_ifast);
		//If button may not be pressed and nothing pressed, 						
		let ttturns=byteru16(*battle_tturns);
		*shaking_dam = [false;25];
		//println!("Got minus A");
		*recl = [
			ttturns[0],									//0#turns
			ttturns[1],									//1#turns
			0,											//2#action
			0,											//3#idm
			*battle_ifast as u8,								//4#ifast
			128,										//5#light
			255,										//6#x[0]
			255,										//7#x[1]
			255,										//8#x[2]
			255,										//9#x[3]									
			255,										//10#x[4]
			255,										//11#x[5]
			255,										//12#x[6]
			255,										//13#x[7]
			255,										//14#x[8]
			255,										//15#x[9]
			255,										//16#x[10]
			255,										//17#x[11]
			255,										//18#x[12]
			255,										//19#x[13]
			255,										//20#x[14]
			255,										//21#x[15]
			255,										//22#x[16]
			255,										//23#x[17]
			255,										//24#x[18]
			255,										//25#x[19]
			255,										//26#x[20]
			255,										//27#x[21]
		];
		for i in 0..cms{recl[6+i]=state_m(encounter[i].0.HP_shade,encounter[i].0.HP)};
		*battle_tturns+= 1;
		if encounter[*battle_ifast].0.HP_shade<=0.0{
			//If player is dead, initiate the next turn.
			let tvar:f32=
				(rand::thread_rng().gen_range(-10,11)-rand::thread_rng().gen_range(-10,11)) as f32;
			let time = encounter[*battle_ifast].0.Speed_shade.clone()+tvar;
			battle_timer[*battle_ifast]+= 1.0/time;
			let bt2 = battle_timer.to_owned();
			let fast = vnmin(bt2);
			*battle_ifast = vwhich(&battle_timer,fast).unwrap_or(*battle_ifast);
			in_battle_record.push(recl.clone());
			return
		}else{
			//if player is not dead, enable button mashing.
			yt_adcwpe_bw[0]=true;
		};
	}else if *yt_adcwpe_bw==[true,false,false,false,false,false,false,false,false] {
		//If player may press button but no button pressed, set comm-text to remind player.
		*comm_text = format!("{}, it is time to act!\n(You are {})",&ns[*battle_ifast],sm_rets(&encounter[*battle_ifast].0));
	}else{
		//If button may be pressed and something may be pressed, calculate the consequences and prepare next turn.
		if yt_adcwpe_bw[1] {
			if sel_targets.len()==0 {
				*comm_text = format!("Who will you attack?\n");
			}else{
				let p_t:usize = sel_targets[0];
				println!("p_t: {}",p_t);
				
				*sel_targets = Vec::with_capacity(25);
				let attack_result:(f32,usize) =  attack(encounter,
												*battle_ifast,
												p_t,
												&ns,
												comm_text);
				if attack_result.0 > 0.0 {
					encounter[attack_result.1].0.HP_shade-= attack_result.0;
					if encounter[attack_result.1].0.HP_shade>0.0 {
						shaking_dam[attack_result.1] = true;
						//*shaking_timer = timer;
					};
				};
					
				//in_battle_record.push(recl.clone());
				turn_over = true;
				if encounter[attack_result.1].0.HP_shade<=0.0 {
					for x in encounter.iter_mut() {
						if x.2[0].is_some() {
							if x.2[0].unwrap()[0]==attack_result.1 {x.2[0] = None;};
						};
					};
				};
				
				*sprite_boxer = GraphicsBox::Attack
									(SpriteBox::new
										(timer,
										&encounter[*battle_ifast],
										*battle_ifast,
										&encounter[p_t],
										p_t,
										&sprite_pos[*battle_ifast],
										&sprite_pos[p_t],
										shaking_dam[attack_result.1]
				));
				shaking_dam[attack_result.1] = false;
				
			};
		}else if yt_adcwpe_bw[2] {
			if sel_targets.len()==0 {
				*comm_text = format!("Defence? How valiant. But whom will you defend?");
			}else{
				encounter[sel_targets[0]].2[0] = Some([*battle_ifast,battle_ttakes[*battle_ifast]]);
				*comm_text = format!("You place yourself between {} and the hostile world...",ns[sel_targets[0]]); 
				//in_battle_record.push(recl.clone());
				turn_over = true;
			};
		}else if yt_adcwpe_bw[4] { //make a wait script
			//in_battle_record.push(recl.clone());
			turn_over = true;
		}else if yt_adcwpe_bw[3] {
			if *to_cast==String::new() {
				*comm_text = "Which spell will you cast?".to_owned();
				yt_adcwpe_bw[7] = false;
				yt_adcwpe_bw[8] = false;
			}else{	
				//spell casting...This can go very wrong if strings in spell list do not equal spell names.
				let to_cast_ind:usize = arcana_index_from_spell_name(y,&to_cast).unwrap();
				if untarget_boon(&y[to_cast_ind]) {
					yt_adcwpe_bw[7] = true;
				};
				if untarget_woe(&y[to_cast_ind]) {
					yt_adcwpe_bw[8] = true;
				};
				if !yt_adcwpe_bw[7] {
					*comm_text = if y[to_cast_ind].target_woe==SAME {
						"Who will your spell target?".to_owned()
					}else{
						"Who will your receive your spell?".to_owned()
					};
					if sel_targets.len() > 0 {
						sp_targets_boon(&y[to_cast_ind], &encounter, comm_text, to_hit, sel_targets);
						yt_adcwpe_bw[7] = true;
						if y[to_cast_ind].target_woe != SAME{
							*sel_targets = Vec::with_capacity(25);
						}else{
							sp_targets_woe(&y[to_cast_ind], &encounter, comm_text, to_hit, sel_targets, *battle_ifast);
							*sel_targets = Vec::with_capacity(25);						
						};
					};
				}else if !yt_adcwpe_bw[8] {
						*comm_text = "Who will your spell target?".to_owned();
					if sel_targets.len() > 0 {
						sp_targets_woe(&y[to_cast_ind], &encounter, comm_text, to_hit, sel_targets, *battle_ifast);
						yt_adcwpe_bw[8] = true;
					};
				};
				
				if yt_adcwpe_bw[7] & yt_adcwpe_bw[8] {
					println!("{:?}",to_hit);
					sp_targets(&y[to_cast_ind], &encounter, &ns, *battle_ifast,to_hit);
					println!("{:?}",to_hit);
					let shades=magic(to_hit.clone(),&encounter,&ns,&y[to_cast_ind],*battle_ifast,comm_text);
					let hpo:f32=0.0;
					for z in 0..cms{
						let hpo=encounter[z].0.HP_shade;
						encounter[z].0.MP_shade+=shades.0[z].MP_shade;
						
						encounter[z].0.HP_shade+=shades.0[z].HP_shade;
						//Damage and avatar shaking.
						if (shades.0[z].HP_shade<0.0) & (encounter[z].0.HP_shade>0.0) {
							shaking_dam[z] = true;
							*shaking_timer = timer;
						};	
						encounter[z].0.Speed_shade+= shades.0[z].S_shade;
						if encounter[z].0.Speed_shade<5.0{
							encounter[z].0.Speed_shade=5.0
						}else{};
						encounter[z].0.Attack_shade+=shades.0[z].A_shade;
						encounter[z].0.Defence_shade+=shades.0[z].D_shade;
						encounter[z].0.WM_shade+=shades.0[z].WM_shade;
						encounter[z].0.BM_shade+=shades.0[z].BM_shade;
						battle_timer[z]+=shades.0[z].Timestop;
						if shades.0[z].Death==true{
							encounter[z].0.HP_shade=0.0
						}else{};
						if shades.0[z].Teleport==true{
						*comm_text = format!("{}\n...{} is warped off the battlefield.",comm_text,encounter[z].0.name);
						battle_timer[z]=1000000.0
						}else{};
						if encounter[z].0.HP_shade<=0.0 {
							for x in encounter.iter_mut() {
								if x.2[0].is_some() {
									if x.2[0].unwrap()[0]==z {x.2[0] = None;};
								};
							};
						};
						if (encounter[z].0.HP_shade<=0.0)
						 & (hpo>0.0){
							*comm_text = format!("{}\n{} from group {} is slain by the spell.",
									 comm_text,&ns[z],encounter[z].1)
						}else{};	
					};
					println!("Effects of spellcasting over");

					//Fills the graphics box with the appropriate information,
					//to launch the sprites into tomorrow.
					if y[to_cast_ind].MP<=encounter[*battle_ifast].0.MP_shade {
						sprite_box_filler(&y[to_cast_ind],sprite_boxer,
							 &encounter[*battle_ifast],
							 *battle_ifast,
							 &to_hit,
							 targets,
							 &sprite_pos,
							 shaking_dam
						);
					};
										
					//*sel_targets = Vec::with_capacity(25);
					*to_hit = vec![(false,false);cms];	
					yt_adcwpe_bw[3] = false;
					yt_adcwpe_bw[7] = false;
					yt_adcwpe_bw[8] = false;
					*to_cast = String::new();	
					
					recl[5] = (recl[5] as i32 + shades.1) as u8;
					//in_battle_record.push(recl.clone());
					turn_over = true;
					println!("Effects of spellcasting over");
				};
			}
		}else if yt_adcwpe_bw[6] {
			yt_adcwpe_bw[6] = false;
			//write an escape script.
			let mut toy_timer:Vec<f32> = vec![0.0;cms];
			let mut a = 0.0;
			let mut b = 0.0;
			for nth in 0..cms{
				a = (rand::thread_rng().gen_range(-10,11)-rand::thread_rng().gen_range(-10,11)) as f32;
				b = encounter[nth].0.Speed_shade.clone()+a;
				toy_timer[nth]=1.0/b;
			};
			let centrifuge = vnmin(toy_timer.clone());
			let escapee:usize = vwhich(&toy_timer,centrifuge).unwrap_or(0);
			if encounter[escapee].1==encounter[*battle_ifast].1 {
				*escape = true;
				println!("You have managed to flee from battle");
				*comm_text = "You have managed to flee from battle".to_owned();
				turn_over = false;
				*freeze_timer = timer-10;
				battle_ttakes[*battle_ifast]+=1;
			}else{
				println!("You tried to escape... But alas!");
				*comm_text = "You tried to escape... But alas!".to_owned();
				turn_over = true;
				println!("cms:{}, to_hitl:{}, battle_timerl:{}, encounterl:{}.",cms,to_hit.len(),battle_timer.len(),encounter.len());
				//in_battle_record.push(recl.clone());
			};
		}else{
			//in_battle_record.push(recl.clone());
			turn_over = true;
		};
		if turn_over {
			in_battle_record.push(recl.clone());
			*freeze_timer = timer-10;
			
			//switch off specials if more than 2 turns have passed.
			if encounter[*battle_ifast].2[0]!= None {
				println!("Got to defend switch offing: battle ttakes ifast: {}",battle_ttakes[*battle_ifast]);
				if encounter[*battle_ifast].2[0].unwrap()[1]+1 < battle_ttakes[encounter[*battle_ifast].2[0].unwrap()[0]] {encounter[*battle_ifast].2[0] = None;};
			}else if encounter[*battle_ifast].2[1]!= None {
				println!("Got to special switch offing");
				if encounter[*battle_ifast].2[1].unwrap()[1]+1 < battle_ttakes[*battle_ifast] {encounter[*battle_ifast].2[1] = None;};
			};
			
			battle_ttakes[*battle_ifast]+=1;
				
			let tvar:f32=
				(rand::thread_rng().gen_range(-10,11)-rand::thread_rng().gen_range(-10,11)) as f32;
			let time = encounter[*battle_ifast].0.Speed_shade.clone()+tvar;
			battle_timer[*battle_ifast]+= 1.0/time;
			let fast = vnmin(battle_timer.clone());
			*battle_ifast = vwhich(&battle_timer,fast).unwrap_or(*battle_ifast);
			*sel_targets = Vec::with_capacity(25);
			*to_hit = vec![(false,false);cms];	
			*yt_adcwpe_bw=[false,false,false,false,false,false,false,false,false];
			*pause = true;
			*comm_text = format!("{}\n***Press Enter to Continue***",comm_text);
		};
	};	
	//println!("Got to the end of player turn, next monster: {}",battle_ifast);		
}
				

fn marker_of_ai_battle_turn(){}
//gmoose function for the AI turn in battle.
pub fn ai_battle_turn<'a,'b> ( mut encounter: &mut Vec<(Lifeform,usize,[Option<[usize;2]>;2])>,
						ns:&Vec<String>,
						p_loc: &Place,
						y: &Vec<Spell>,
						cms: usize,
						mut battle_fast:&mut f32,
						mut battle_ifast:&mut usize,
						mut battle_ttakes:&mut Vec<usize>,
						mut battle_tturns:&mut u16,
						mut battle_orders:&mut Vec<(u8,u8)>,
						mut battle_timer:&mut Vec<f32>,
						mut in_battle_record:&mut Vec<[u8;28]>,
						timer: usize,
						mut freeze_timer:&mut usize,
						mut comm_text: &mut String,
						mut pause: &mut bool,
						mut shaking_timer: &mut usize,
						mut shaking_dam: &mut [bool;25],
						mut ai_turn_started: &mut bool,
						mut ai_started_thinking: &mut bool,
						mut thought_sender_to_brain: &mut SyncSender<(usize,usize,[u8;28],i32,Vec<(u8,u8)>,bool)>,
						mut thought_receiver_to_body: &mut Receiver<(usize,usize)>,
						mut sprite_boxer: &mut GraphicsBox,
						sprite_pos: &mut [[f64;2];25],
						targets:&mut Vec<usize>
					  ){
	//println!("Monster number {} to go.",battle_ifast);
	
	let mut recl:[u8;28] = [0;28];
	if !*ai_turn_started {
		//if ai turn has not truly started, inititate recl and other important variables.
		*shaking_dam = [false;25];
		*targets = Vec::with_capacity(25);
		let ttturns=byteru16(*battle_tturns);
		//println!("Got minus A");
		recl = [
			ttturns[0],									//0#turns
			ttturns[1],									//1#turns
			0,											//2#action
			0,											//3#idm
			*battle_ifast as u8,								//4#ifast
			128,										//5#light
			255,										//6#x0
			255,										//7#x[1]
			255,										//8#x[2]
			255,										//9#x[3]									
			255,										//10#x[4]
			255,										//11#x[5]
			255,										//12#x[6]
			255,										//13#x[7]
			255,										//14#x[8]
			255,										//15#x[9]
			255,										//16#x[10]
			255,										//17#x[11]
			255,										//18#x[12]
			255,										//19#x[13]
			255,										//20#x[14]
			255,										//21#x[15]
			255,										//22#x[16]
			255,										//23#x[17]
			255,										//24#x[18]
			255,										//25#x[19]
			255,										//26#x[20]
			255,										//27#x[21]
		];
			
		for i in 0..cms{recl[6+i]=state_m(encounter[i].0.HP_shade,encounter[i].0.HP)};
		*battle_tturns+= 1;
		in_battle_record.push(recl.clone());
		//NB genocide+omnicide loop moved outside of function.
		if encounter[*battle_ifast].0.HP_shade<=0.0 {
			let tvar:f32=
				(rand::thread_rng().gen_range(-10,11)-rand::thread_rng().gen_range(-10,11)) as f32;
			let time = encounter[*battle_ifast].0.Speed_shade.clone()+tvar;
			battle_timer[*battle_ifast]+=1.0/time;
			let fast = vnmin(battle_timer.clone());
			*battle_ifast = vwhich(&battle_timer,fast).unwrap_or(*battle_ifast);
			return
		}else{
			//start ai turn if ai is not dead.
			*ai_turn_started = true;
		};
	};
			
	if *ai_turn_started & !*ai_started_thinking {
		println!("{} from group {} is about to take action!",ns[*battle_ifast],encounter[*battle_ifast].1);
		let turning = *battle_tturns as usize;
		let mut lsum:i32=0;
		let mut battle_orders:Vec<(u8,u8)> = Vec::new();
		if *battle_tturns>1{
			println!("a:pa(t+1)={},(t+2)={},ibr.len()={}",permit_a(turning+1),turning+2,in_battle_record.len());					
			for j in permit_a(turning+1)..(turning+2){battle_orders.push((in_battle_record[j][4],0))};
			//println!("b");
			for i in 3..in_battle_record.len(){lsum+=in_battle_record[i][5] as i32 -128}
			//println!("c");
		}else{
			battle_orders=vec![(0,0)]
		};

		let bif:usize = *battle_ifast;
		let trng:usize = turning.clone();
		let rcl:[u8;28] = recl.clone();
		let lsm:i32 = lsum.clone();
		let borders:Vec<(u8,u8)> = battle_orders.clone();
		if (encounter[*battle_ifast].0.Type != MINDLESS)
		& (encounter[*battle_ifast].0.SubType != MINDLESS) {
			thought_sender_to_brain.clone().send((
								bif,
								trng,
								rcl,
								lsm,
								borders,
								true
			));
			println!("AI's thoughts sent");

		};
		*ai_started_thinking = true;
		return
	};
	if *ai_turn_started & *ai_started_thinking {
		
		if timer%5 != 0 {return}; //give the computer more time to think.
		
		//try to receive the answer, if failed go to next frame and try again, else continue turn.
		//(best_act_pm,best_tar_pm)
		let mut choice:(usize,usize,bool);
		
		if (encounter[*battle_ifast].0.Type != MINDLESS)
		 & (encounter[*battle_ifast].0.SubType != MINDLESS) {
			match thought_receiver_to_body.try_recv() {
				Ok(answer) => {
					let (choice_a,choice_b) = answer;
					choice = (choice_a,choice_b,true);
				},
				Err(_) => { choice = (0,0,false)},
			};
		}else{
			choice = (255,255,true);
		};
								
		if !choice.2 {return};
		println!("Answer: {:?}",choice);						
		let mut idm=choice.1;
		let mut exI=choice.0;
		
		let libr:usize = in_battle_record.len()-1;
		
		//AI SCRIPT FAILURE COMBINATION<<<<<<<<<<<
		if (idm>=encounter.len()) | (exI>y.len()+1) {
			
			println!("AI script failed, idm={}, exI={}.",idm,exI);			
			idm = dumb_target_chooser(idm,cms,encounter,*battle_ifast);
			
			if idm>= encounter.len() {
				println!("{} Panics!\n",&ns[*battle_ifast]);
				idm = bankai(*battle_ifast,&encounter)
			}else{};
			state_match(encounter[*battle_ifast].0.HP,
						encounter[*battle_ifast].0.HP_shade,
						&ns[*battle_ifast],
						encounter[*battle_ifast].1);
			let mut dumbI:usize=99999;		
			if encounter[*battle_ifast].0.Spellist.len()>0 {
				dumbI=dumb_choice(idm,*battle_ifast,&encounter[*battle_ifast].0,&ns[*battle_ifast],&encounter,&y);
				exI=dumbI;
				println!("{}",dumbI)
			}else{
				dumbI = rand::thread_rng().gen_range(0,2);
				exI = dumbI;	
				println!("{}!",dumbI)
			};				
			in_battle_record[libr][3]=idm as u8;
			in_battle_record[libr][2]=exI as u8;
			
		}else{};	
		if exI==1 {			
			//*comm_text = format!("{} from group {} attacks {} from group {}.", &ns[*battle_ifast],encounter[*battle_ifast].1, &ns[idm], &encounter[idm].1);	
			
			let attack_result:(f32,usize) =  attack(encounter,
												*battle_ifast,
												idm,
												&ns,
												comm_text);
			//damage and shaking.									
			if attack_result.0 > 0.0 {
				encounter[attack_result.1].0.HP_shade-= attack_result.0;
				if encounter[attack_result.1].0.HP_shade>0.0 {
					shaking_dam[attack_result.1] = true;
					//*shaking_timer = timer;
				};
			};
					
			if encounter[attack_result.1].0.HP_shade<=0.0 {
				for x in encounter.iter_mut() {
					if x.2[0].is_some() {
						if x.2[0].unwrap()[0]==attack_result.1 {x.2[0] = None;};
					};
				};
			};
			
			
			*sprite_boxer = GraphicsBox::Attack
								(SpriteBox::new
									(timer,
									&encounter[*battle_ifast],
									*battle_ifast,
									&encounter[idm],
									idm,
									&sprite_pos[*battle_ifast],
									&sprite_pos[idm],
									shaking_dam[attack_result.1])
							);
			shaking_dam[attack_result.1] = true;
												
		}else if exI==0 {
			*comm_text = format!("{} from group {} takes a stance between {} from group {} and the world!", &ns[*battle_ifast],encounter[*battle_ifast].1, &ns[idm], &encounter[idm].1);			
			encounter[idm].2[0] = Some([*battle_ifast,battle_ttakes[*battle_ifast]]);
		}else if exI >1 {
			//println!("magic idm:{} dI:{}",&idm,&dumbI);
			let exj=exI-2;
			if *battle_ifast==0{
				*comm_text = format!("You prepare to cast {} at {} from group {}.", &y[exj].name, &ns[idm], &encounter[idm].1)
			}else{
				*comm_text = format!("{} prepares to cast {} at {} from group {}.",&ns[*battle_ifast], &y[exj].name, &ns[idm], &encounter[idm].1)
			};
			if y[exj].MP > encounter[*battle_ifast].0.MP_shade {
				println!("\nenemy ran out of mana\n");
				in_battle_record[libr][2]=1;
				let spare_text:String = format!("{}\n...But doesn't have enough mana!", comm_text);		
				let attack_result:(f32,usize) =  attack(encounter,
													*battle_ifast,
													idm,
													&ns,
													comm_text);
				//damage and shaking.									
				if attack_result.0 > 0.0 {
				encounter[attack_result.1].0.HP_shade-= attack_result.0;
				if encounter[attack_result.1].0.HP_shade>0.0 {
					shaking_dam[attack_result.1] = true;
					*shaking_timer = timer;
				};
			};
				if encounter[attack_result.1].0.HP_shade<=0.0 {
					for x in encounter.iter_mut() {
						if x.2[0].is_some() {
							if x.2[0].unwrap()[0]==attack_result.1 {x.2[0] = None;};
						};
					};
				};
				*comm_text = format!("{}\n{}",spare_text,comm_text);
			}else{
				let to_hit=sp_target_comp(&y[exj], &encounter, *battle_ifast, idm);
				let shades=magic(to_hit.clone(),&encounter,&ns,&y[exj],*battle_ifast,comm_text);
				for z in 0..cms{
					let hpo=encounter[z].0.HP_shade;
					encounter[z].0.MP_shade+=shades.0[z].MP_shade;
					
					encounter[z].0.HP_shade+=shades.0[z].HP_shade;
					//Damage and avatar shaking.
					if (shades.0[z].HP_shade<0.0) & (encounter[z].0.HP_shade>0.0) {
						shaking_dam[z] = true;
						*shaking_timer = timer;
					};			
					encounter[z].0.Speed_shade+= shades.0[z].S_shade;
					if encounter[z].0.Speed_shade<5.0{
							encounter[z].0.Speed_shade=5.0
					}else{};
					encounter[z].0.Attack_shade+=shades.0[z].A_shade;
					encounter[z].0.Defence_shade+=shades.0[z].D_shade;
					encounter[z].0.WM_shade+=shades.0[z].WM_shade;
					encounter[z].0.BM_shade+=shades.0[z].BM_shade;
					battle_timer[z]+=shades.0[z].Timestop;
					if shades.0[z].Death==true{
						encounter[z].0.HP_shade=0.0
					}else{};
					if shades.0[z].Teleport==true{
						*comm_text = format!("{}\n...{} is warped off the battlefield.",comm_text,encounter[z].0.name);
						battle_timer[z]=1000000.0;
					};
					if (encounter[z].0.HP_shade<=0.0) & (hpo>0.0){
						*comm_text = format!("{}\n{} from group {} is slain by the spell.",
											 comm_text,&ns[z],encounter[z].1);
					};
					if encounter[z].0.HP_shade<=0.0 {
						for x in encounter.iter_mut() {
							if x.2[0].is_some() {
								if x.2[0].unwrap()[0]==z {x.2[0] = None;};
							};
						};
					};
				};
				
				//Fills the graphics box with the appropriate information,
				//to launch the sprites into tomorrow.
				sprite_box_filler(&y[exj],sprite_boxer,
					 &encounter[*battle_ifast],
					 *battle_ifast,
					 &to_hit,
					 targets,
					 &sprite_pos,
					 shaking_dam);
				
				in_battle_record[libr][5] = (in_battle_record[libr][5] as i32 + shades.1) as u8;
			};
		}else{};
		in_battle_record[libr][2]=exI as u8;
		println!("battle record turn end length ibr.len()={},turn={}\n",in_battle_record.len(),battle_tturns);	
	
	
		if encounter[*battle_ifast].2[0].is_some() {
			println!("x[ifast].2[0].is_some(), so shouldn't crash when unwrapping");
			if encounter[*battle_ifast].2[0].unwrap()[1]+1 < battle_ttakes[encounter[*battle_ifast].2[0].unwrap()[0]] {encounter[*battle_ifast].2[0] = None;};
		};
		if encounter[*battle_ifast].2[1].is_some() {
			if encounter[*battle_ifast].2[1].unwrap()[1]+1 < battle_ttakes[*battle_ifast] {encounter[*battle_ifast].2[1] = None;};
		};
		
		//modulate special attacks
			//switch of specials if more than 2 turns have passed.
		if encounter[*battle_ifast].2[0]!= None {
			if encounter[*battle_ifast].2[0].unwrap()[1]+1 < battle_ttakes[*battle_ifast] {encounter[*battle_ifast].2[0] = None;};
		}else if encounter[*battle_ifast].2[1]!= None {
			if encounter[*battle_ifast].2[1].unwrap()[1]+1 < battle_ttakes[*battle_ifast] {encounter[*battle_ifast].2[1] = None;};
		};
		
		battle_ttakes[*battle_ifast]+= 1;
			
		let tvar:f32=
			(rand::thread_rng().gen_range(-10,11)-rand::thread_rng().gen_range(-10,11)) as f32;
		let time = encounter[*battle_ifast].0.Speed_shade.clone()+tvar;
		battle_timer[*battle_ifast]+=1.0/time;
		let bt2 = battle_timer.to_owned();
		let fast = vnmin(bt2);
		*battle_ifast = vwhich(battle_timer,fast).unwrap_or(*battle_ifast);
		*pause = true;
		*comm_text = format!("{}\n***Press Enter to Continue***",comm_text);
		*ai_turn_started = false;
		*ai_started_thinking = false;
	};
}


fn marker_of_game_over(){}
//Function for pronouncing the end of the battle.
pub fn game_over(mut encounter: &mut Vec<(Lifeform,usize,[Option<[usize;2]>;2])>,
				mut enemies:  &mut Vec<(Lifeform,usize)>,
				mut party: &mut Vec<(Lifeform,usize)>,
				dungeons: &mut Vec<Dungeon>,
				mut n_s_l_q_f:&mut [bool;7],
				mut tt_e_c_i_ll:&mut [bool;8],
				mut exp_players:&mut [f32; 5],
				mut battle_gold_pot:&mut usize,
				mut comm_text:&mut String,
				timer:usize,
				mut freeze_timer:&mut usize,
				battle_tturns:&mut u16,
				idungeon: &mut Option<usize>,
				dungeon_pointer: &mut usize,
				escape: bool) {
					
	//println!("entering game over");
	let max_group = if encounter.len()==0 {0}else{encounter[encounter.len()-1].1};
	let p_len = party.len();
	let mut alive = [false;5];
	let mut defeat = [true;5];				
	let mut omnicide = true;
	
	for x in encounter.iter() {
		if x.0.HP_shade>0.0 {
			defeat[x.1] = false;
			alive[x.1] = true;
			omnicide = false;
		};
	};
	
	let mut victory:(bool,Option<usize>)= if !omnicide | (*battle_tturns>600) | escape {who_won(&alive)}else{(false,None)};
	//println!("b2");
	if escape {
		n_s_l_q_f[4] = false;
		*freeze_timer = timer;
		dungeon_navigator_a(false,idungeon,dungeon_pointer,dungeons,tt_e_c_i_ll);
	}else if !alive[0] {
		n_s_l_q_f[4] = false;
		*freeze_timer = timer;
		*comm_text = "Your party has fallen in battle. Moose over.".to_owned();
		dungeon_navigator_a(false,idungeon,dungeon_pointer,dungeons,tt_e_c_i_ll);
		for mut x in party.iter_mut() {
			x.0.Gold = 0;
			x.0.Exp = if x.0.Exp > x.0.ExpUsed + 10.0 {
				x.0.Exp-9.0
			 }else{
				x.0.ExpUsed
			}; 	
		};
	}else if omnicide {
		n_s_l_q_f[4] = false;
		*freeze_timer = timer;
		*comm_text = "In the wake of this battle there are no survivors. Moose over.".to_owned();
		dungeon_navigator_a(false,idungeon,dungeon_pointer,dungeons,tt_e_c_i_ll);
	}else if *battle_tturns>600 {
		n_s_l_q_f[4] = false;
		*freeze_timer = timer;
		*comm_text = "The sun sets on your anger... And you live to rage another day.".to_owned();
		dungeon_navigator_a(true,idungeon,dungeon_pointer,dungeons,tt_e_c_i_ll);
	}else if victory.0 {
		n_s_l_q_f[4] = false;
		*freeze_timer = timer;
		let winner:String = match victory.1 {
			Some(x) => format!("Group {}",x),
			_ => "No one".to_owned(),
		};
		*comm_text = format!("{} has emerged victorious.",winner);
		if victory.1.unwrap()==0 {
			for (i,mut x) in party.iter_mut().enumerate() {
				x.0.Exp+= exp_players[i];
				x.0.Gold = *battle_gold_pot/p_len;
				println!("{}: has {} exp ({} unused)",x.0.name,x.0.Exp,x.0.ExpUsed); 	
			};
			dungeon_navigator_a(true,idungeon,dungeon_pointer,dungeons,tt_e_c_i_ll);
		}else{
			for mut x in party.iter_mut() {
				x.0.Gold = 0;
				x.0.Exp = if x.0.Exp > x.0.ExpUsed + 10.0 {
					x.0.Exp-9.0
				 }else{
					x.0.ExpUsed
				}; 	
			};
			dungeon_navigator_a(false,idungeon,dungeon_pointer,dungeons,tt_e_c_i_ll);
		}; 
	};	
}

//gmoose victory.
fn who_won(alive:&[bool;5])->(bool,Option<usize>){
	let mut live_count:u8 = 0;
	let out = (false,None);
	for i in 0..5 { 
		if alive[i] {live_count+= 1;};
	};
	if live_count==1 {
		for i in 0..5 {
			if alive[i] {return (true,Some(i));};
		};
		out
	}else{
		out
	}
}

fn spell_targets_to_indices(to_hit:&Vec<(bool,bool)>,targets:&mut Vec<usize>){
	*targets = Vec::with_capacity(25);
	for (i,x) in to_hit.iter().enumerate() {
		if x.1 | x.0 {targets.push(i);};
	}
}
//
//
//
fn marker_of_ai_battle_rand(){}
//Battle takes a vector of tuples of Lifeforms & their group...
//...as output by engenB
//battle_rand() is an automated random battle.
//it takes random_choice instead of dumb choice and has no player turn.
//it does not give exp and does not return new Lifeforms. This is important for implementation.
pub fn battle_rand(mut x:Vec<(Lifeform,usize,[Option<[usize;2]>;2])>,
                   y:&Vec<Spell>,
                   z:&Place,
                   ns:&Vec<String>,
                   battled:usize)->(Vec<[u8;28]>){
	//Set up recorder vector.
	let mut record:Vec<[u8;28]>= vec!([0;28],[255;28],[255;28]);
	let cms=x.len();
	for i in 0..cms{
		record[1][6+i]=x[i].0.id as u8;
		record[2][6+i]=x[i].1 as u8
	};

    let mut timer: Vec<f32>=vec!(0.0;cms);
    //let mut party=&x[0..y[0]];
    
    //create timer and initiate first turn.
    for nth in 0..cms{
		let tvar=(rand::thread_rng().gen_range(1,10)+rand::thread_rng().gen_range(1,10)) as f32;
        let time=x[nth].0.Speed_shade.clone()+tvar;
        let time=if time<1.0{1.0}else{time};
        timer[nth]=1.0/time
    };
    let mut fast=vnmin(timer.clone());
    let mut ifast:usize=vwhicht1(&timer,fast);
    //println!("timer:{:?}\nfast:{}\nifast:{}",timer,fast,ifast);
    let mut ttakes:Vec<usize> = vec![0;cms];
    
    //main battle loop.
    let mut tturns:u16=0;
    let mut omnicide:bool=false;
    while tturns<(cms*6+20) as u16{
		let ttturns=byteru16(tturns);
		//println!("Got minus A");
		let mut recl:[u8;28]=[
			ttturns[0],									//0#turns
			ttturns[1],									//1#turns
			0,											//2#action
			0,											//3#idm
			ifast as u8,								//4#ifast
			128,										//5#light
			255,										//6#x0
			255,										//7#x[1]
			255,										//8#x[2]
			255,										//9#x[3]									
			255,										//10#x[4]
			255,										//11#x[5]
			255,										//12#x[6]
			255,										//13#x[7]
			255,										//14#x[8]
			255,										//15#x[9]
			255,										//16#x[10]
			255,										//17#x[11]
			255,										//18#x[12]
			255,										//19#x[13]
			255,										//20#x[14]
			255,										//21#x[15]
			255,										//22#x[16]
			255,										//23#x[17]
			255,										//24#x[18]
			255,										//25#x[19]
			255,										//26#x[20]
			255,										//27#x[21]
		];
		for i in 0..cms{recl[6+i]=state_m(x[i].0.HP_shade,x[i].0.HP)};
		tturns+= 1;
		omnicide=all_dead(&x);
		if omnicide==true{
			record.push(recl.clone());
			break
		}else if state_m(x[ifast].0.HP,x[ifast].0.HP_shade)==0{
			let tvar:f32= (rand::thread_rng().gen_range(1,10)+rand::thread_rng().gen_range(1,10)) as f32;
			let time=x[ifast].0.Speed_shade.clone()+tvar;
			timer[ifast]+=1.0/time;
			let time=if time.is_normal()==true{time}else{5.0};
			fast=vnmin(timer.clone());
			ifast=vwhicht2(&timer,fast);
			record.push(recl);
			continue
		}else{
			let victory=genocide(&x,x[ifast].1);
			if victory==true{
				//record victory here!!
				break
			}else{
				ttakes[ifast]+= 1;
				let mut idm:usize=0;
				let mut dumbI:usize=0;			
				if ifast==0{
					idm = rand::thread_rng().gen_range(0,cms)
				}else{
					let mut liv_vec:Vec<usize> = Vec::with_capacity(25);
					for i in 0..cms{
						if state_m(x[i].0.HP,x[i].0.HP_shade)>0{liv_vec.push(i)}else{}
					};		
					let idm_pick = rand::thread_rng().gen_range(0,liv_vec.len());
					idm = liv_vec[idm_pick]
				};
				recl[3]=idm as u8;
				if x[ifast].0.Spellist.len()>0{
						dumbI=rand_choice(idm,&x[ifast].0,&ns[ifast],&x,&y,ifast);
				}else{
					dumbI = rand::thread_rng().gen_range(0,2)
				};
				
				//Record action choice (dumbI)!!
				if dumbI==1{
					//attack action	
					let attack_result:(f32,usize) =  attack_r(&mut x,
															  ifast,
															  idm);
					x[attack_result.1].0.HP_shade-= attack_result.0;
					if x[attack_result.1].0.HP_shade<=0.0 {
						for x in x.iter_mut() {
							if x.2[0].is_some() {
								if x.2[0].unwrap()[0]==attack_result.1 {x.2[0] = None;};
							};
						};
					};
                    //record target & battlefield state!!!
                }else if dumbI==0{
					//defence action
					x[ifast].2[0] = Some([idm,ttakes[ifast]]);
				}else if dumbI>1{
					//println!("magic idm:{} dI:{}",&idm,&dumbI);
					let dumbj=dumbI-2;
					if y[dumbj].MP>x[ifast].0.MP_shade {
						dumbI = 1;
						let attack_result:(f32,usize) =  attack_r(&mut x,
															  ifast,
															  idm);
						x[attack_result.1].0.HP_shade-= attack_result.0;
						if x[attack_result.1].0.HP_shade<=0.0 {
							for x in x.iter_mut() {
								if x.2[0].is_some() {
									if x.2[0].unwrap()[0]==attack_result.1 {x.2[0] = None;};
								};
							};
						};
					}else{
						let to_hit=sp_target_comp(&y[dumbj], &x, ifast, idm);
						let shades=magic_rand_battle(to_hit.clone(),&x,&ns,&y[dumbj],ifast);
						for z in 0..cms{
							let hpo=x[z].0.HP_shade;
							x[z].0.MP_shade+=shades.0[z].MP_shade;
							x[z].0.HP_shade+=shades.0[z].HP_shade;
							x[z].0.Speed_shade+= shades.0[z].S_shade;
							if x[z].0.Speed_shade<5.0{
								x[z].0.Speed_shade=5.0
							}else{};
							x[z].0.Attack_shade+=shades.0[z].A_shade;
							x[z].0.Defence_shade+=shades.0[z].D_shade;
							x[z].0.WM_shade+=shades.0[z].WM_shade;
							x[z].0.BM_shade+=shades.0[z].BM_shade;
							timer[z]+=shades.0[z].Timestop;
							if shades.0[z].Death==true{
								x[z].0.HP_shade=0.0
							}else{};
							if shades.0[z].Teleport==true{
								//record teleport!!!
								timer[z]=1000000.0
							}else{
							//record action choice & target+battlefield state!
							};
							//record target & battlefield state!!
							if x[z].0.HP_shade<=0.0 {
								for x in x.iter_mut() {
									if x.2[0].is_some() {
										if x.2[0].unwrap()[0]==z {x.2[0] = None;};
									};
								};
							};
						}
						recl[5] = (recl[5] as i32 + shades.1) as u8;
					};
				
				}else{};
				recl[2] = dumbI as u8;
				record.push(recl);
			};
		};
		
		//modulate special attacks
		if x[ifast].2[0].is_some() {
			if x[ifast].2[0].unwrap()[1]+1 < ttakes[x[ifast].2[0].unwrap()[0]] {x[ifast].2[0] = None;};
		};
		if x[ifast].2[1].is_some() {
			if x[ifast].2[1].unwrap()[1]+1 < ttakes[ifast] {x[ifast].2[1] = None;};
		};
		
		let tvar:f32= (rand::thread_rng().gen_range(1,10)+rand::thread_rng().gen_range(1,10)) as f32;
        let time=x[ifast].0.Speed_shade.clone()+tvar;
        let time=if time.is_normal()==true{time}else{5.0};
        timer[ifast]+=1.0/time;
//        for i in 0..cms{
//			if timer[i].is_normal()==true{
//				timer[i]=vnmax(timer.clone());
//			}else{};
//		};
        fast=vnmin(timer.clone());
        ifast=vwhicht3(&timer,fast);
	};	
    record
}
//
//function for converting spell name to global spell spell index:

fn arcana_index_from_spell_id(spell_list: &Vec<Spell>, id: i8) ->Option<usize> {	
	for i in 0..spell_list.len(){
		if spell_list[i].id==id {return Some(i)}
	}
	None
}
fn arcana_index_from_spell_name(spell_list: &Vec<Spell>, name: &str) ->Option<usize> {	
	for i in 0..spell_list.len(){
		if spell_list[i].name==name {return Some(i)}
	}
	None
}
fn arcana_type_from_spell_id<'a> (spell_list: &'a Vec<Spell>, id: i8) ->Option<u8> {	
	for x in spell_list{
		if x.id==id {return Some(x.Type)}
	}
	None
}

//A little unsafe
fn arcana_name_from_spell_id<'a> (spell_list: &'a Vec<Spell>, id: i8) -> String {	
	for x in spell_list{
		if x.id==id {return x.name.to_owned()}
	}
	String::new()
}

//
//
//Spellcasting shell which determined which effects affect whom...
//...and in what order.
//
//Spell targeting part A.1
fn untarget_boon(spell:&Spell)-> bool {
	match spell.target_boon {
		SELF => true,
		BOB  => true,
		ALL  => true,
		PARTY => true,
		ANY => true,
		_      => false,
	}
}

//Spell targeting part A.2
fn untarget_woe(spell:&Spell)-> bool {
	match spell.target_woe {
		SELF => true,
		SAME => true,
		BOB  => true,
		ALL => true,
		ANY  => true,
		PARTY=> true,
		_      => false,
	}
}


//transfers targets to the to_hit vector (boon).
fn sp_targets_boon(spell: &Spell,
				  encounter: &Vec<(Lifeform,usize,[Option<[usize;2]>;2])>,
				  mut comm_text: &mut String,
				  mut to_hit: &mut Vec<(bool,bool)>,
				  tl: &Vec<usize>) {
	
	match spell.target_boon {
		TARGET|GROUP|SINGLE => {to_hit[tl[0]].0 = true},
		GROUPS|ANY => 		{ let group = encounter[tl[0]].1;
							  for i in 0..encounter.len() {
								if encounter[i].1==group {
									to_hit[i].0 = true
								}; 
							  };
							},
		_ => {},
	};
}

//transfers targets to the to_hit vector (woe). MUST BE USED AFTER BOON.
fn sp_targets_woe(spell: &Spell,
				  encounter: &Vec<(Lifeform,usize,[Option<[usize;2]>;2])>,
				  mut comm_text: &mut String,
				  mut to_hit: &mut Vec<(bool,bool)>,
				  tl: &Vec<usize>,
				  ifast:usize) {
	
	match spell.target_woe {
		TARGET|SINGLE => {to_hit[tl[0]].1 = true},
		GROUP => { let group = encounter[tl[0]].1;
							  for i in 0..encounter.len() {
								if encounter[i].1==group {
									to_hit[i].1 = true
								}; 
							  };
							},
		ANY|GROUPS => {	for i in 0..encounter.len() {
						if encounter[i].1!=encounter[ifast].1 {
							to_hit[i].1 = true
						};
					};
				 },
		SAME => {	for x in to_hit.iter_mut() {x.1 = x.0};
				  },
		_ => {},
	};
}

fn sp_targets(spell:&Spell, t_l: &Vec<(Lifeform,usize,[Option<[usize;2]>;2])>,
				ns:&Vec<String>,
				ifast:usize,
				targets:&mut Vec<(bool,bool)>){
	
	//Set up variables.
	let tll=t_l.len() as usize;
	let mut target_boon:char='x';
	let mut target_woe:char='x';
	
//create target lists (WM).
//create target lists (WM).
//create target lists (WM).
//create target lists (WM).

	if  spell.target_boon==NON{
		for x in targets.iter_mut() {x.0 = false};
	}else if spell.target_boon==SELF{
		for x in targets.iter_mut() {x.0 = false};
		targets[ifast].0=true;
		println!("Your spell aids only you ourself.");
	}else if spell.target_boon==ALL{
		for i in 0..tll{
			targets[i].0=true
		};
		println!("This spell is received by all.");
	}else if spell.target_boon==PARTY{
		for x in targets.iter_mut() {x.0 = false};
		for i in 0..tll{
			if t_l[i].1==t_l[ifast].1{
			targets[i].0=true
			}else{};
		};
		println!("This spell is received by all.");
	}else{
	};
	
//create target lists (BM).
//create target lists (BM).
//create target lists (BM).
    
	if spell.target_woe==NON{
		for x in targets.iter_mut() {x.1 = false};
	}else if spell.target_woe==SAME{
		for i in 0..targets.len() {targets[i].1 = targets[i].0};
	}else if spell.target_woe==SELF{
		for x in targets.iter_mut() {x.1 = false};
		targets[ifast].1 = true;
		println!("This spell hurts only yourself");
	}else if spell.target_woe==ALL {
		for x in targets.iter_mut() {x.1 = true};
		println!("This spell targets all combatants.");
	}else if (spell.target_woe==ANY) | (spell.target_woe==GROUPS) {
		for i in 0..targets.len() {
			if t_l[i].1 != 0 {targets[i].1 = true}
		};
		println!("This spell targets all combatants.");
	}else{
	};
}

//
//
//
//Spell targeting for computer.
fn sp_target_comp(spell:&Spell, t_l: &Vec<(Lifeform,usize,[Option<[usize;2]>;2])>, ifast:usize,idm:usize)->Vec<(bool,bool)>{
	
	//Set up variables.
	let tll=t_l.len() as usize;
	let mut targets: Vec<(bool,bool)>=vec![(false,false);tll];
	
//create target lists (WM).
//create target lists (WM).
//create target lists (WM).
//create target lists (WM).
	let mut boon_chosen=false;
	while boon_chosen==false{
	if spell.target_boon==NON{
		targets=targets;
		boon_chosen=true
	}else if spell.target_boon==SELF{
		targets[ifast].0=true;
		boon_chosen=true;
	}else if spell.target_boon==ALL||(spell.target_boon==ANY){
		for i in 0..tll{
			targets[i].0=true
		};
		boon_chosen=true
	}else if spell.target_boon==PARTY{
		for i in 0..tll{
			if t_l[i].1==t_l[ifast].1{
			targets[i].0=true
			}else{};
		};
		boon_chosen=true
	}else if (spell.target_boon==SINGLE)|(spell.target_boon==TARGET){
		targets[idm].0=true;
        boon_chosen=true;
	}else if (spell.target_boon==GROUP)||(spell.target_boon==GROUPS){
		let group=t_l[ifast].1;
		for i in 0..tll{
			if t_l[i].1==group{
				targets[i].0=true
			}else{};
		};
		boon_chosen=true
	}else{boon_chosen=true};
	}
//create target lists (BM).
//create target lists (BM).
//create target lists (BM).
	let mut woe_chosen=false;	
    while woe_chosen==false{
	if spell.target_woe==NON {
		targets=targets;
		woe_chosen=true
	}else if spell.target_woe==SELF {
		targets[ifast].1=true;
		woe_chosen=true
	}else if spell.target_woe==ALL {
		for i in 0..tll{
			targets[i].1=true
		};
		woe_chosen=true
	}else if spell.target_woe==PARTY {
		for i in 0..tll{
			if t_l[i].1==t_l[ifast].1{
			targets[i].1=true
			}else{};
		};
		woe_chosen=true
	}else if (spell.target_woe==SINGLE)|(spell.target_woe==TARGET) {
		targets[idm].1=true;
        woe_chosen=true;
	}else if spell.target_woe==GROUP {
		let group=t_l[idm].1;
		for i in 0..tll{
			if t_l[i].1==group{
				targets[i].1=true
			}else{};
		};
		woe_chosen=true
	}else if (spell.target_woe==GROUPS)||(spell.target_woe==ANY){
		let group=t_l[ifast].1;
		for i in 0..tll{
			if t_l[i].1!=group{
				targets[i].1=true
			}else{};
		};
		woe_chosen=true
	}else{woe_chosen=true};
	}

targets
}

//Standard attack function.
fn marker_of_attack(){}
fn attack(encounter:&mut Vec<(Lifeform,usize,[Option<[usize;2]>;2])>,
			ifast:usize,
			p_t:usize,
			ns:&Vec<String>,
			mut comm_text:&mut String)->(f32,usize){
				
	let corpse:String = if encounter[p_t].0.HP_shade>0.0 {String::new()}else{"the corpse of ".to_owned()};
	if encounter[ifast].1==0 {
		*comm_text = format!("You prepare to attack {}{} from group {}.", corpse, &ns[p_t], &encounter[p_t].1)
	}else{
		*comm_text = format!("{} from group {} attacks {}{} from group {}.", &ns[ifast],encounter[ifast].1, corpse, &ns[p_t], &encounter[p_t].1)
	};
	let mut p_t:usize = p_t;
	
	//substitute victim for victim's defender.
	if encounter[p_t].2[0].is_some() {
		p_t = encounter[p_t].2[0].unwrap()[0];
		*comm_text = format!("\n{}...But {} from group {} stands in their path!",comm_text, &ns[p_t],encounter[p_t].1);
	};		
		
	
	//calculate if attack hits & whatnot.	  
	let aa=if encounter[ifast].0.Attack_shade <=2.0 {2} else {encounter[ifast].0.Attack_shade as isize};
	let ab=if encounter[ifast].0.Attack_shade <=0.0 {0.0} else {encounter[ifast].0.Attack_shade};
	let dd=if encounter[p_t].0.Defence_shade <=2.0 {2} else {encounter[p_t].0.Defence_shade as isize};
	let a=rand::thread_rng().gen_range(-2,aa)+rand::thread_rng().gen_range(-2,aa)+rand::thread_rng().gen_range(-2,aa);
	let b=rand::thread_rng().gen_range(-2,dd)+rand::thread_rng().gen_range(-2,dd);
	let mut HP_loss:f32 = 0.0;
	if (a<b) & (p_t != ifast) {
		*comm_text = format!("{}\n...and misses.",comm_text);
	}else{
		*comm_text = format!("{}\n...and the attack finds its mark.",comm_text);
		let att_pow:f32 = 1.5*ab*(1.0+encounter[ifast].0.Exp/1000.0);
				HP_loss = (rand::thread_rng().gen_range(-10,11)+rand::thread_rng().gen_range(-8,9)) as f32;
				HP_loss+= att_pow-encounter[p_t].0.Defence_shade/2.0;
				if p_t != ifast {HP_loss-= encounter[p_t].0.WM_shade/2.0;};
				HP_loss = if HP_loss<0.0 {
					println!("The strike is ineffective!");
					0.0
				}else{
					println!("Damage done: {}",&HP_loss);
					HP_loss
				};
	}
	dam_match(encounter[p_t].0.HP,
						  HP_loss,
						  &ns[ifast],
						  &ns[p_t],
						  encounter[p_t].1,
						  comm_text);
	if (encounter[p_t].0.HP_shade-HP_loss<=0.0) & !(encounter[p_t].0.HP_shade>0.0) {
		*comm_text = format!("{}\n{} falls to the attack.\n",comm_text, &encounter[p_t].0.name);
	};
	(HP_loss,p_t)	
}

//Automated attack function modernised.
fn attack_r(encounter:&mut Vec<(Lifeform,usize,[Option<[usize;2]>;2])>,
			ifast:usize,
			p_t: usize)->(f32,usize){
				
	let mut p_t:usize = p_t;
	
	//substitute victim for victim's defender.
	if encounter[p_t].2[0].is_some() {
			p_t = encounter[p_t].2[0].unwrap()[0];
	};		  
	
	//calculate if attack hits & whatnot.
	let aa:isize = if encounter[ifast].0.Attack_shade <=2.0 {2} else {encounter[ifast].0.Attack_shade as isize};
	let ab:f32 = if encounter[ifast].0.Attack_shade <=0.0 {0.0} else {encounter[ifast].0.Attack_shade};
	let dd:isize = if encounter[p_t].0.Defence_shade <=2.0 {2} else {encounter[p_t].0.Defence_shade as isize};
	let a:isize = rand::thread_rng().gen_range(-2,aa)+rand::thread_rng().gen_range(-2,aa)+rand::thread_rng().gen_range(-2,aa);
	let b:isize = rand::thread_rng().gen_range(-2,dd)+rand::thread_rng().gen_range(-2,dd);
	let mut HP_loss:f32 = 0.0;
	if a>=b {
		let att_pow:f32 = 1.5*ab*(1.0+encounter[ifast].0.Exp/1000.0);
				HP_loss = (rand::thread_rng().gen_range(-10,11)+rand::thread_rng().gen_range(-8,9)) as f32;
				HP_loss+= att_pow-encounter[p_t].0.Defence_shade/2.0-encounter[p_t].0.WM_shade/2.0;
				HP_loss = if HP_loss<0.0 {0.0}else{HP_loss};
	};
	(HP_loss,p_t)	
}

//function to produce battle ending bool (draw).
fn all_dead(a:&Vec<(Lifeform,usize,[Option<[usize;2]>;2])>)->bool{
	let mut omnicide:bool=true;
	for i in 0..a.len(){
		if a[i].0.HP_shade>0.0{omnicide=false
		}else{omnicide=omnicide};
	};
	omnicide
}

//function to produce battle ending bool (win).
fn genocide(a:&Vec<(Lifeform,usize,[Option<[usize;2]>;2])>,b:usize)->bool{
	let mut victory:bool=true;
	for i in 0..a.len(){
		if (a[i].0.HP_shade>0.0) & (a[i].1!=b){victory=false
		}else{victory=victory};
	};
	victory
}

//Matches damage inflicted with correct statement...
//...And will eventually output answers for machine learning.
fn state_match(HP_tot:f32, HP_now:f32, d_name:&String, group:usize){
let stater:f32=HP_now/HP_tot;
let state:i32=
    if stater>=1.0{5}
	else if (stater<1.0) & (stater>=0.75){4}
	else if (stater<0.75) & (stater>=0.5){3}
	else if (stater<0.5) & (stater>=0.25){2}
	else if(stater<0.25) & (stater>0.0){1}
	else{0};
let damned:&str= match state{
	5=>"uninjured",
	4=>"lightly wounded",
	3=>"wounded",
	2=>"gravely wounded",
	1=>"on death's door",
	0=>"dead",
	_=>"on another plane of existence"};	
println!("{} from group {} is {}.\n",
         &d_name,
         group,
         damned);
}

//state match return string.
fn sm_rets(x:&Lifeform)->String{
	let stater:f32 = x.HP_shade/x.HP;
	let state:i32 =
		if stater>= 1.0{5}
		else if (stater<1.0) & (stater>=0.75){4}
		else if (stater<0.75) & (stater>=0.5){3}
		else if (stater<0.5) & (stater>=0.25){2}
		else if(stater<0.25) & (stater>0.0){1}
		else{0};
	let damned:String = match state{
		5=>"uninjured".to_owned(),
		4=>"lightly wounded".to_owned(),
		3=>"wounded".to_owned(),
		2=>"gravely wounded".to_owned(),
		1=>"on death's door".to_owned(),
		0=>"dead".to_owned(),
		_=>"absent".to_owned()
	};
	damned
}

//machine learning version of state_match()
fn state_m(HP_tot:f32, HP_now:f32)->u8{
let stater:f32=HP_now/HP_tot;
let state:u8=
    if stater>=1.0{5}
	else if (stater<1.0) & (stater>=0.75){4}
	else if (stater<0.75) & (stater>=0.5){3}
	else if (stater<0.5) & (stater>=0.25){2}
	else if(stater<0.25) & (stater>0.0){1}
	else{0};
state
}

//Matches %HP with correct statement...
//...And will eventually output answers for machine learning.
fn dam_match(HP_tot:f32, HP_loss:f32, a_name:&String, d_name:&String, group:usize,
			 mut comm_text:&mut String){
	let stater:f32=HP_loss/HP_tot;
	let damn:i32=
		if stater>1.2{5}
		else if (stater<=1.2) & (stater>=0.6){4}
		else if (stater<0.6) & (stater>=0.3){3}
		else if (stater<0.3) & (stater>=0.1){2}
		else if(stater<0.1) & (stater>0.0){1}
		else{0};
	let damned:&str= match damn{
		5=>"physically obliterates",
		4=>"inflicts massive damage upon",
		3=>"inflicts grave wounds upon",
		2=>"lightly wounds",
		1=>"barely scratches",
		_=>"inflicts no harm upon",
	};
		
	*comm_text = format!("{}\n{} {} {} from group {}.",
		 comm_text,
         &a_name,
         damned,
         &d_name,
         group);
}

//This translate spell damage number to a string.
fn spd(being:String, damage:f32)->String{
	if damage > -100.0 {	  format!("touches {}.",being)
	}else if damage > -200.0 {format!("impacts {}.",being)
	}else if damage > -400.0 {format!("rains down upon {}.",being)
	}else if damage > -800.0 {format!("ravages {}.",being)
	}else{ 					  format!("envelops {} in utter destruction.",being)
	}
}

//Artefact of the terminal Moosequest.
fn yeno_to_bool(x: &str)-> bool {
 match x.trim().to_lowercase().as_ref() {
  "y"=>true,
  "ye"=>true,
  "yea"=>true,
  "yeh"=>true,
  "yep"=>true,
  "yes"=>true,
  "yeah"=>true,
  "true"=>true,
  "indeed"=>true,
  "aye"=>true,
  "ofcourse"=>true,
  "i do"=>true,
  _=>false,
 }
}


//A bunch of unnecessary functions from I didn't know what I was doing.
pub fn vnmin<T:PartialOrd+Copy>(a:Vec<T>)->T{
	let mut min:T=a[0];
	for i in 0..a.len(){
		if min<=a[i]{min=min}
		else{min=a[i]};
	};
	min
}

pub fn vnmax<T:PartialOrd+Copy>(a:Vec<T>)->T{
	let mut max:T=a[0];
	for i in 0..a.len(){
		if max<=a[i]{max=a[i]}
		else{max=max};
	};
	max
}
//vwhich returns index of the first value in vector a which equals b.
pub fn vwhich<T:PartialOrd>(a:&Vec<T>, b:T)->Option<usize>{
	for i in 0..a.len(){
		if a[i]==b{return Some(i)}
		else{continue}
	}
	None
}

pub fn vwhichb<T:PartialOrd>(a:&Vec<T>, b:T)->usize{
	for i in 0..a.len(){
		if a[i]==b{return i}
		else{continue}
	}
	99999
}

pub fn vwhicht1<T:PartialOrd>(a:&Vec<T>, b:T)->usize{
	for i in 0..a.len(){
		if a[i]==b{return i}
		else{continue}
	}
	11111
}
pub fn vwhicht2<T:PartialOrd>(a:&Vec<T>, b:T)->usize{
	for i in 0..a.len(){
		if a[i]==b{return i}
		else{continue}
	}
	11112
}
pub fn vwhicht3<T:PartialOrd>(a:&Vec<T>, b:T)->usize{
	for i in 0..a.len(){
		if a[i]==b{return i}
		else{continue}
	}
	11113
}
//vvwhichinv returns indexs of the values in vector a which do not equal b.
pub fn vvwhichinv<T:PartialOrd>(a:&Vec<T>, b:T)->Vec<usize>{
	let mut ivec:Vec<usize>=Vec::new();
	for i in 0..a.len(){
		if a[i]!=b{ivec.push(i)}
		else{continue}
	}
	ivec
}



//vvwhich returns indexs of the values in vector a which do not equal b.
fn vvwhich<T:PartialOrd>(a:&Vec<T>, b:T)->Vec<usize>{
	let mut ivec:Vec<usize>=Vec::new();
	for i in 0..a.len(){
		if a[i]==b{ivec.push(i)}
		else{continue}
	}
	ivec
}

//special vvwhichfor lifeform names
fn vvwhich_ln(a:&Vec<Lifeform>, b:&str)->Vec<usize>{
	let mut ivec:Vec<usize> = Vec::new();
	for i in 0..a.len(){
		if a[i].name==b{ivec.push(i)}
		else{continue}
	}
	ivec
}

//This is the magic function. It will get complicated
//It takes the target list.
fn marker_of_magic(){}
fn magic(tl:Vec<(bool,bool)>,xx:&Vec<(Lifeform,usize,[Option<[usize;2]>;2])>,ns:&Vec<String>,spell:&Spell, ifast:usize, mut comm_text:&mut String)
->(Vec<Shade>,i32){
	
	//Tell us what's happening, or chant special spell.
	if spell.name=="Inferno"{
		*comm_text = "O Pyros! O primal flame...\nWherein the world was forged...\nBurn up Everything!\n".to_owned();
	}else if spell.name=="Jovian Lightning"{
		*comm_text = "O spear of Theos! O spark of Life!\nTouch the earth!\nRevoke your gift!\n".to_owned();
	}else if spell.name=="True Crystallise"{
		*comm_text = "O star of the North!\n O cross of the south!\nUnite...\n...And pierce the world with stillness!\n".to_owned();
	}else if spell.name=="Sword of Perseus"{
		*comm_text = "O spirit of the hero,\n Come forth to soulless steel!\n...Make right the gods' mistakes!\n".to_owned();
	}else{
		*comm_text = if xx[ifast].1==0 {
			format!("You cast {}...",spell.name)
		}else{
			let a = match xx[ifast].1 {
				1 => "the north",
				2 => "the east",
				3 => "the south",
				_ => "the north",
			};
			format!("{} from {} casts {}...",xx[ifast].0.name,a,spell.name)
		};
	};
	
	//initiate output and other vectors.
	let tll=tl.len();
	let shay=Shade::new();
	let mut shades:Vec<Shade>=vec!(shay.clone();tll);
	let mut escaped_reaper:usize=0;
	let mut trapped_reaper:usize=0;
	
	//determine strength of caster effects.
	if (spell.MP>xx[ifast].0.MP_shade) & (xx[ifast].1==0){
		*comm_text = format!("{}\n...You don't have enough mana!",comm_text);
		return (shades,0)
	}else{};
	
	let caff=if (xx[ifast].0.SubType==spell.Type)
			 || (xx[ifast].0.Affinity==spell.Type){1.8
			 }else{1.3};
	
	let CM= if spell.Light==true {
		(xx[ifast].0.WM_shade+spell.Power-10.0)*caff
	}else{
		(xx[ifast].0.BM_shade+spell.Power-10.0)*caff
	};
	let BM= xx[ifast].0.BM_shade as i64;
	let BM=if BM<1 {1}else{BM};
	
	
	if spell.Death==true{
		*comm_text = "This forbidden magic summons a reaper...".to_owned();
	}else{};
	
	for i in 0..tll{
		let WMi=xx[i].0.WM as i64;
		let WMi=if WMi<1 {1} else {WMi};
		let mut extra_dam:f32=0.0;
		if spell.Health>0.0{
			extra_dam=CM-xx[i].0.WM*0.8;
			extra_dam=if extra_dam<0.0{0.0}else{extra_dam};
		}else{};
		
		//Harming script.
		if (tl[i].1==true) & ((spell.Light==false)||
		((xx[i].0.Unclean==true)||(xx[i].0.Alive==false))){
			let PW=spow(spell.Power,CM,&xx[i].0,&spell);
			let PW=if PW<=0.0{0.0}else{PW};
			//harming script.
			shades[i].MP_shade= 0.0;
			shades[i].HP_shade-= if spell.Light==false{PW*spell.Health*0.06+extra_dam
								 }else if (spell.Light==true)
								      & ((xx[i].0.Unclean==true)
								      || (xx[i].0.Alive==false)){PW*spell.Health*0.08+extra_dam
								 }else{0.0};
			shades[i].S_shade-=PW*0.005*spell.Speed;
			shades[i].A_shade-=PW*0.02*spell.Attack;
			shades[i].D_shade-=PW*0.02*spell.Defence;
			shades[i].WM_shade-=PW*0.02*spell.WM;
			shades[i].BM_shade= if xx[i].0.Unclean==true{
				                    0.0+PW*0.02*spell.BM
				                }else{0.0-PW*0.02*spell.BM};
			shades[i].Inv_shade= Vec::new();
			if spell.Death==true{
			let a=rand::thread_rng().gen_range(-1,BM)+rand::thread_rng().gen_range(-1,BM)+rand::thread_rng().gen_range(-1,BM);
			let b=rand::thread_rng().gen_range(-1,WMi)+rand::thread_rng().gen_range(-1,WMi);
			*comm_text = format!("{}\n...The summoned reaper reaches for {}!",comm_text,ns[i]);
				if (a>=b) & (xx[i].0.Type!=UNDEAD) {
					trapped_reaper+=1;
					shades[i].Death=true;
					*comm_text = format!("{}\n...And takes its soul.",comm_text)
				}else{
					escaped_reaper+=1;
					*comm_text = format!("{}\n...but {} repels it.",comm_text,ns[i])
				};
			}else{};
			shades[i].Teleport=
				if (spell.Teleport==true) &
				   (((i==ifast)||(i==xx[i].1))||PW>0.0){true
					}else{false};
			shades[i].Timestop=
				if (spell.Timestop==true) &
					((i==ifast)||(i==xx[i].1)){
					xx[ifast].0.WM_shade.powf(1.0+spell.Power/10.0)*0.001
					}else if spell.Timestop==true {PW*0.001
				}else{0.0};
			if shades[i].Timestop<0.0{
			shades[ifast].Timestop-=shades[i].Timestop;
			shades[i].Timestop=0.0}else{};
		};
		
		//Healing script.
		if (tl[i].0==true) & (((spell.Light==true) &
		(xx[i].0.Alive==true) )|| ((spell.name=="Lifestealer")
		|| (trapped_reaper>0))){
			let PW:f32=spow(spell.Power,CM,&xx[i].0,&spell);
			let LPW:f32=spowl(spell.Power,CM,&xx[i].0,&spell);
			let LPW:f32=if LPW<0.0{0.0}else{LPW};
			let PW:f32=if PW<0.0{0.0}else{PW};
			//healing script.
			shades[i].MP_shade= 0.0;
			shades[i].HP_shade+= if ((xx[i].0.HP_shade>0.0)
			                      & (xx[i].0.Alive==true)
			                      & (xx[i].0.HP>xx[i].0.HP_shade))
			                     || (spell.name=="Lifestealer"){
				spell.Health*CM*(10.0+spell.Power)*0.01
			}else if (xx[i].0.HP_shade>0.0)
			       & (xx[i].0.Alive==false)
			       & (spell.name!="Lifestealer"){
				0.0-PW*spell.Health*0.05
			}else{0.0};
			shades[i].S_shade+= PW*0.004*spell.Speed;
			if shades[i].S_shade<0.0{shades[i].S_shade=0.0}else{};
			shades[i].A_shade+= PW*0.01*spell.Attack;
			shades[i].D_shade+= PW*0.01*spell.Defence;
			shades[i].WM_shade+= PW*0.005*spell.WM;
			shades[i].BM_shade+= if xx[i].0.Unclean==true{
				                    -PW*0.005*spell.BM
				                }else{CM*0.005*spell.BM};
			
			if (spell.Death==true) & (trapped_reaper>0)
			 & (xx[i].0.HP_shade<=0.0){
				println!("The reaper returns what it took...");
				shades[i].HP_shade+=xx[i].0.HP-xx[i].0.HP_shade;
				trapped_reaper-=1}else{};
			shades[i].Teleport=
				if (spell.Teleport==true) &
				   (((i==ifast)||(i==xx[i].1))||PW>0.0){true
					}else{false};
			}else{};
			
		//Print Messages		
		if shades[i].HP_shade<0.0{
			let affected:String= if xx[i].0.HP_shade<=0.0 {
				format!("{}'s remains",ns[i])
			}else{
				ns[i].to_owned()
			};
			*comm_text = format!("{}\n{} {}",
					 comm_text,
			         convert_mag_type(spell.Type).to_title_case(),
			         spd(affected,shades[i].HP_shade));
			println!("Damage to {}: {}.",ns[i],0.0-shades[i].HP_shade);
		}else if shades[i].HP_shade>0.0{
			*comm_text = format!("{}\n{} is healed by the spell.",comm_text,ns[i]);
		}else{};
		if (shades[i].S_shade<0.0)||
		   (shades[i].A_shade<0.0)||
		   (shades[i].D_shade<0.0)||
		   (shades[i].WM_shade<0.0)||
		   (shades[i].BM_shade<0.0){
			*comm_text = format!("{}\nThe spell ails {}.",comm_text,ns[i]);
		}else if (shades[i].S_shade>0.0)||
		   (shades[i].A_shade>0.0)||
		   (shades[i].D_shade>0.0)||
		   (shades[i].WM_shade>0.0)||
		   (shades[i].BM_shade>0.0){
			*comm_text = format!("{}\nThe spell strengthens {}.",comm_text,ns[i]);
		}else{};
		shades[i].WM_shade+=(spell.Illumination as f32)*0.01*CM;
		shades[i].BM_shade-=(spell.Illumination as f32)*0.01*CM;			
		if shades[i].Timestop>0.0{
			*comm_text = format!("{}\n{}'s time stops.",comm_text,ns[i])
		}else{};
//		println!("{}: {},{},{},{},{},{},{},{},{}.",
//				 xx[i].0.name,
//				 shades[i].HP_shade,
//				 shades[i].S_shade,
//				 shades[i].A_shade,
//		 		 shades[i].D_shade,
//				 shades[i].WM_shade,
//				 shades[i].BM_shade,
//				 shades[i].Death,
//				 shades[i].Teleport,
//				 shades[i].Timestop);
				 
	};
	//loop finished.
	
	shades[ifast].MP_shade=0.0-spell.MP;
	if spell.Illumination>20{
		*comm_text = format!("{}\nA divine light descends onto the battlefield.",comm_text)
	}else if spell.Illumination>10{
		*comm_text = format!("{}\nThe spell illuminates the battlefield with a brilliant light.",comm_text)
	}else if spell.Illumination>0{
		*comm_text = format!("{}\nThe spell illuminates the battlefield.",comm_text)
	}else if spell.Illumination<0{
		*comm_text = format!("{}\nThe spell darkens the battlefield.",comm_text);
	}else if spell.Illumination<(-9){
		*comm_text = format!("{}\nThe spell casts menacing shadows on the battlefield.",comm_text);
	}else if spell.Illumination<(-19){
		*comm_text = format!("{}\nThe battlefield is plunged into Abyssal darkness.",comm_text);
    }else{};
	if escaped_reaper>0{
		*comm_text = format!("{}\nThe turned reaper turns on its master and steals its soul.",comm_text);
		shades[ifast].Death=true
	}else{};
	
	for i in 0..xx.len() {
		println!("SPF on {}: HP: {}. MP: {}. Speed: {}. Attack {}. Defence {}. WM: {}. BM {}.",
				 xx[i].0.name,shades[i].HP_shade,
							shades[i].MP_shade,
							shades[i].S_shade,
							shades[i].A_shade,
							shades[i].D_shade,
							shades[i].WM_shade,
							shades[i].BM_shade)
	};
		
	(shades,spell.Illumination)
}

fn marker_of_magic_rand(){}
fn magic_rand_battle(tl:Vec<(bool,bool)>,xx:&Vec<(Lifeform,usize,[Option<[usize;2]>;2])>,ns:&Vec<String>,spell:&Spell, ifast:usize)
->(Vec<Shade>,i32){	
	let tll=tl.len();
	let mut shay= Shade::new();
	
	let mut shades:Vec<Shade>=vec!(shay.clone();tll);
	let mut escaped_reaper:usize=0;
	let mut trapped_reaper:usize=0;
	
	if (spell.MP>xx[ifast].0.MP_shade) & (ifast==0){
		//record mana out!;
		return (shades,0)
	}else{};
	
	let caff=if (xx[ifast].0.SubType==spell.Type)
			 || (xx[ifast].0.Affinity==spell.Type){1.8
			 }else{1.3};
	
	let CM= if spell.Light==true {
		(xx[ifast].0.WM_shade+spell.Power-10.0)*caff
	}else{
		(xx[ifast].0.BM_shade+spell.Power-10.0)*caff
	};
	let BM= xx[ifast].0.BM_shade as i64;
	let BM=if BM<1 {1} else {BM};
	
	if spell.Death==true{
	}else{};
	
	for i in 0..tll{
		let WMi=xx[i].0.WM as i64;
		let WMi=if WMi<1 {1} else {WMi};
		let mut extra_dam:f32=0.0;
		if spell.Health>0.0{
			extra_dam=CM-xx[i].0.WM*0.8;
			extra_dam=if extra_dam<0.0{0.0}else{extra_dam};
		}else{};
		
		//Harming script.
		if (tl[i].1==true) & ((spell.Light==false)||
		((xx[i].0.Unclean==true)||(xx[i].0.Alive==false))){
			let PW=spow(spell.Power,CM,&xx[i].0,&spell);
			let PW=if PW<=0.0{0.0}else{PW};
			//harming script.
			shades[i].MP_shade= 0.0;
			shades[i].HP_shade-= if spell.Light==false{PW*spell.Health*0.06+extra_dam
								 }else if (spell.Light==true)
								      & ((xx[i].0.Unclean==true)
								      || (xx[i].0.Alive==false)){PW*spell.Health*0.08+extra_dam
								 }else{0.0};
			shades[i].S_shade-=PW*0.005*spell.Speed;
			shades[i].A_shade-=PW*0.02*spell.Attack;
			shades[i].D_shade-=PW*0.02*spell.Defence;
			shades[i].WM_shade-=PW*0.02*spell.WM;
			shades[i].BM_shade= if xx[i].0.Unclean==true{
				                    0.0+PW*0.02*spell.BM
				                }else{0.0-PW*0.02*spell.BM};
			shades[i].Inv_shade= Vec::new();
			if spell.Death==true{
			let a=rand::thread_rng().gen_range(-1,BM)+rand::thread_rng().gen_range(-1,BM)+rand::thread_rng().gen_range(-1,BM);
			let b=rand::thread_rng().gen_range(-1,WMi)+rand::thread_rng().gen_range(-1,WMi);
				if (a>=b) & (xx[i].0.Type!=UNDEAD) {
					trapped_reaper+=1;
					shades[i].Death=true;
				}else{
					escaped_reaper+=1;
				};
			}else{};
			shades[i].Teleport=
				if (spell.Teleport==true) &
				   (((i==ifast)||(i==xx[i].1))||PW>0.0){true
					}else{false};
			shades[i].Timestop=
				if (spell.Timestop==true) &
					((i==ifast)||(i==xx[i].1)){
					xx[ifast].0.WM_shade.powf(1.0+spell.Power/10.0)*0.001
					}else if spell.Timestop==true {PW*0.001
				}else{0.0};
			if shades[i].Timestop<0.0{
			shades[ifast].Timestop-=shades[i].Timestop;
			shades[i].Timestop=0.0}else{};
		};
		
		//Healing script.
		if (tl[i].0==true) & (((spell.Light==true) &
		(xx[i].0.Alive==true) )|| ((spell.name=="Lifestealer")
		|| (trapped_reaper>0))){
			let PW:f32=spow(spell.Power,CM,&xx[i].0,&spell);
			let LPW:f32=spowl(spell.Power,CM,&xx[i].0,&spell);
			let LPW:f32=if LPW<0.0{0.0}else{LPW};
			let PW:f32=if PW<0.0{0.0}else{PW};
			//healing script.
			shades[i].MP_shade= 0.0;
			shades[i].HP_shade+= if ((xx[i].0.HP_shade>0.0)
			                      & (xx[i].0.Alive==true)
			                      & (xx[i].0.HP>xx[i].0.HP_shade))
			                     || (spell.name=="Lifestealer"){
				spell.Health*CM*(10.0+spell.Power)*0.01
			}else if (xx[i].0.HP_shade>0.0)
			       & (xx[i].0.Alive==false)
			       & (spell.name!="Lifestealer"){
				0.0-PW*spell.Health*0.05
			}else{0.0};
			shades[i].S_shade+= PW*0.004*spell.Speed;
			if shades[i].S_shade<0.0{shades[i].S_shade=0.0}else{};
			shades[i].A_shade+= PW*0.01*spell.Attack;
			shades[i].D_shade+= PW*0.01*spell.Defence;
			shades[i].WM_shade+= PW*0.005*spell.WM;
			shades[i].BM_shade+= if xx[i].0.Unclean==true{
				                    -PW*0.005*spell.BM
				                }else{CM*0.005*spell.BM};
			
			if (spell.Death==true) & (trapped_reaper>0)
			 & (xx[i].0.HP_shade<=0.0){
				shades[i].HP_shade+=xx[i].0.HP-xx[i].0.HP_shade;
				trapped_reaper-=1}else{};
			shades[i].Teleport=
				if (spell.Teleport==true) &
				   (((i==ifast)||(i==xx[i].1))||PW>0.0){true}else{false};
		}else{};
			
		//Print Messages	
		shades[i].WM_shade+=(spell.Illumination as f32)*0.01*CM;
		shades[i].BM_shade-=(spell.Illumination as f32)*0.01*CM;	
		
//		println!("{}: {},{},{},{},{},{},{},{},{}.",
//				 xx[i].0.name,
//				 shades[i].HP_shade,
//				 shades[i].S_shade,
//				 shades[i].A_shade,
//		 		 shades[i].D_shade,
//				 shades[i].WM_shade,
//				 shades[i].BM_shade,
//				 shades[i].Death,
//				 shades[i].Teleport,
//				 shades[i].Timestop);
				 
	};
	//loop finished.
	shades[ifast].MP_shade=0.0-spell.MP;
	if escaped_reaper>0{shades[ifast].Death=true}else{};
		
	(shades,spell.Illumination)
}

//choosing an action for the computer using a "simple" algorithm.
fn dumb_choice (idm:usize,
				ifast:usize,
				caster:&Lifeform,
				c_name:&String,
				pots: &Vec<(Lifeform,usize,[Option<[usize;2]>;2])>,
				spells:&Vec<Spell>)-> usize {
					
	let mut attackq = true;
	let mut spn:Vec<i8> = Vec::new();
	let mut permitted:Vec<usize> = Vec::new();
	let mut permitted_arcana:Vec<&Spell> = Vec::new();
	let mut choice:usize=1;
	for i in 0..spells.len(){
		spn.push(spells[i].id)
	};
	//println!("Point A");
	//println!("{}",spn.len());
	for i in 0..spn.len(){
		if lmoose::lhas(&caster.Spellist,&spn[i]) & (spells[i].MP<=caster.MP_shade) {
			attackq=false;
			permitted.push(i);
			permitted_arcana.push(&spells[i]);
		};
	};
    //println!("Point B");
	if attackq==true{
		if pots[idm].1==pots[ifast].1 {0}else{1};
	}else{
		if caster.HP_shade/caster.HP<0.33 {
			for i in 0..permitted_arcana.len() {
				if permitted_arcana[i].Type==HEALING {
					return permitted[i]+2
				}
			};
		};
		
		//compare what'll do more damage, spell or attack.
		let ab:f32 = if pots[ifast].0.Attack_shade <=0.0 {0.0} else {pots[ifast].0.Attack_shade};
		let mut mean_pot_dam = 1.5*ab*(1.0+pots[ifast].0.Exp/1000.0); - pots[idm].0.Defence_shade/2.0 - pots[idm].0.WM_shade/2.0;
		
		for i in 0..permitted_arcana.len() {
			let caff=if (pots[ifast].0.SubType==permitted_arcana[i].Type)
				 || (pots[ifast].0.Affinity==permitted_arcana[i].Type){1.8
				 }else{1.3};
		
			let CM= if permitted_arcana[i].Light {
						pots[ifast].0.WM_shade*caff
					}else{
						pots[ifast].0.BM_shade*caff
					};
			let mut BM = pots[ifast].0.BM_shade as i64;
			BM = if BM<1 {1} else {BM};
		
			let mut extra_dam:f32 = CM-pots[ifast].0.WM*0.8;
			extra_dam = if extra_dam<0.0{0.0}else{extra_dam};
			
			let mut PW = spow(permitted_arcana[i].Power,CM,&pots[idm].0,&permitted_arcana[i]);
			PW = if PW<=0.0{0.0}else{PW};
			
			let spell_damage =  if !permitted_arcana[i].Light {
									PW*permitted_arcana[i].Health*0.06+extra_dam
								}else if permitted_arcana[i].Light & (pots[idm].0.Unclean | !pots[idm].0.Alive) {
									PW*permitted_arcana[i].Health*0.08+extra_dam
								}else{0.0};
								
			if spell_damage > mean_pot_dam {
				mean_pot_dam = spell_damage;
				choice = permitted[i]+2;
			};
		};

	};
	choice
}

//needs to be improved to take into account of waiting and panicking
//dumb chocie for random battle, output: 0= defend, 1 = attack. 1<magic,
//1=attack. 2...n=cast spell listed [n-2]th in ifast's spellist.
fn rand_choice(idm:usize ,caster:&Lifeform,c_name:&String, pots: &Vec<(Lifeform,usize,[Option<[usize;2]>;2])>, spells:&Vec<Spell>,ifast:usize)->usize{
	let lenny:usize=2+caster.Spellist.len();
	let mut choice:usize=0;
	if ifast==0{
		choice=rand::thread_rng().gen_range(0,lenny);
	}else{
		choice=rand::thread_rng().gen_range(0,lenny);
	};
	let spelln=if choice>1{caster.Spellist[choice-2]}else{-1};
	if choice>1{
		let mut spellcheck:bool = false;
		for i in 0..spells.len(){
			if spelln==spells[i].id{
				choice=i+2;
				spellcheck = true;
			}else{};
		};
		if !spellcheck {choice = rand::thread_rng().gen_range(0,2);};
	};
	choice
}

fn spow(power:f32, CM:f32, t:&Lifeform,spell:&Spell)->f32{ 
	
	let taff = if (t.SubType==spell.Type)
			   || (t.Affinity==spell.Type){1.5}else{1.0};
			 
	let mut pd = CM-t.WM_shade*taff;
	pd = if pd<0.0 {0.0}else{pd};
	let damage = (power+10.0)*(power+10.0)/400.0*pd;
	
	damage
}

fn spowl(power:f32, CM:f32, t:&Lifeform,spell:&Spell)->f32{ 
	
	let taff = if (t.SubType==spell.Type)
			   || (t.Affinity==spell.Type) {1.5}else{1.0};
			 
	let pd = CM*taff;
	let pd = if pd<0.0 {0.0}else{pd};
	let heal = (power+10.0)*(power+10.0)/800.0*pd;
	
	heal
}

fn bankai (ifast:usize, all:&Vec<(Lifeform,usize,[Option<[usize;2]>;2])>)->usize{
	let mut good_choice=false;
	let mut kimegao:usize=0;
	while good_choice==false{
		kimegao=rand::thread_rng().gen_range(0,all.len());
		good_choice=if all[ifast].1!=all[kimegao].1{true}else{false}
	};
	kimegao
}

//makes level up quesiton menu appear on main menu.
fn lvlq_marker(){}
pub fn lvlq(party:&Vec<(Lifeform,usize)>,
			p_names:&Vec<String>,
			mut tt_e_c_i_ll: &mut [bool;8]) {
	for q in party.iter(){			
		if (q.0.Exp-q.0.ExpUsed)>10.0{
			tt_e_c_i_ll[5] = true;
			tt_e_c_i_ll[6] = true;
			return;
		};
	};
}

//gmoose lvl up function. Does not currently cover spells.
fn lvl_upg (mut party:&mut Vec<(Lifeform,usize)>,
			r:usize,
			mut t_e_c_i_ll:&mut [bool;8]) {
	
	let i:usize = r/10;
	let mut expble:f32 = party[i].0.Exp-party[i].0.ExpUsed;
	println!("Exp to use:{}",expble);
	match r%10 {
		1 => {party[i].0.HP+= expble*2.0;
			  party[i].0.ExpUsed+= expble;},
		2 => {party[i].0.MP+= expble*2.0;;
			  party[i].0.ExpUsed+= expble;},
		3 => {party[i].0.Speed+= expble/5.0;
			  party[i].0.ExpUsed+= expble;},
		4 => {party[i].0.Attack+= expble/2.0;
			  party[i].0.ExpUsed+= expble;},
		5 => {party[i].0.Defence+= expble/2.0;
			  party[i].0.ExpUsed+= expble;},
		6 => {party[i].0.WM+= expble/2.0;
			  party[i].0.ExpUsed+= expble;},
		7 => {party[i].0.BM+= expble/2.0;
			  party[i].0.ExpUsed+= expble;},
		_ => {},
	};
	expble = 0.0;
	for x in party.iter() {
		println!("Exp left on {}:{}",x.0.name,x.0.Exp-x.0.ExpUsed);
		expble+= x.0.Exp-x.0.ExpUsed;
	};
	if expble==0.0 {t_e_c_i_ll[6] = false;};
}

//gmoose::beast_name(encounter[ifast],p_names)

pub fn beast_name<'a>(encounter:&Vec<(Lifeform,usize,[Option<[usize;2]>;2])>,ifast:usize,p_names:&'a Vec<String>)->&'a str {
	if (encounter[ifast].1==0) & (ifast<p_names.len()) {
		&p_names[ifast]
	}else{
		encounter[ifast].0.name
	}
}

//NB xx must be encounter and not party
pub fn exp_calc(xx: &Vec<(Lifeform,usize,[Option<[usize;2]>;2])>, i:usize)->f32{
	let mut expgain:f32 = 0.0;
	let kachi = xx[i].0.HP*(xx[i].0.Attack+xx[i].0.Defence)
		+xx[i].0.MP*(xx[i].0.BM+xx[i].0.WM)
		+xx[i].0.Speed*(xx[i].0.HP+xx[i].0.MP);
	
	//Baxter to the soap factory.
	for i in xx.iter(){
		let baxter = i.0.HP*(i.0.Attack+i.0.Defence)
		+i.0.MP*(i.0.BM+i.0.WM)
		+i.0.Speed*(i.0.HP+i.0.MP);
		expgain+= baxter/kachi;
	};
	expgain-= 2.0;
	expgain= if expgain<0.0 {0.0}else{expgain};
	expgain
}

//Auxillary functions for saving and loading games.
fn by_f64(x:f64)->u64{
	let y=unsafe{transmute::<f64,u64>(x)};
	y
}
fn byteri32(x:i32)->[u8;4]{
    let y= unsafe {transmute(x.to_be())};
    y
}
fn byterus(x:usize)->[u8;8]{
    let y= unsafe {transmute(x.to_be())};
    y
}
fn byter64(x:u64)->[u8;8]{
    let y= unsafe {transmute(x.to_be())};
    y
}
fn unbyteus(x:[u8;8])->usize{
    let mut out:u64=0;
    for i in 0..8{out+=(x[7-i] as u64)*256u64.pow(i as u32)};
    let out:usize=out as usize;
    out
}
fn unbyte64(x:[u8;8])->u64{
    let mut out:u64=0;
    for i in 0..8{out+=(x[7-i] as u64)*256u64.pow(i as u32)};
    let out:u64=out as u64;
    out
}
fn unbyte32(x:[u8;4])->i32{
    let mut out:u32=0;
    for i in 0..4{out+=(x[3-i] as u32)*256u32.pow(i as u32)};
    let out:i32=unsafe {transmute::<u32,i32>(out)};
    out
}
fn un64_f64(x:u64)->f64{
	let y=unsafe{transmute::<u64,f64>(x)};
	y
}
fn byteru16(x:u16)->[u8;2]{
    let y= unsafe {transmute(x.to_be())};
    y
}

// write configuration into the soundtrack config sound/
// config file has eighteen lines that look like:
// n:::/dir/song.ext
// this functions checks that the file exists and writes
// <number>:::<filename> to the appropriate line of the file.
// this is then read by the jukebox.
// filename should be checked beforehand.
// now uses song_list as template instead of re-parsing file each time.
fn write_music_config(filename:String,song_list: &mut Vec<String>, number:usize) {
	
	let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("as").unwrap();
	let note_path = assets.join("notes/notes.mqcfg");
	song_list[number] = filename;
	
	//Simplified attempt to write a new file.
	//It so happens that on windows the more complete script does not want to dance.
	let mut cfgf:File = File::create(&note_path).unwrap();
	
	let mut playlist_string:Vec<String> = Vec::with_capacity(ISEKAIN);
	
	for (i,x) in song_list.iter().enumerate() {
		if x=="Standard" {
			playlist_string.push(format!("{}:::",i));
		}else{ //NB if the file cannot be opened as a file, it is deleted from the config.
			match File::open(x) {
				Ok(_) => {playlist_string.push(format!("{}:::{}",i,x))},
				_	  => {playlist_string.push(format!("{}:::",i))},
			};
		};
	};
	
	//write everything to file.
	cfgf.write_all(playlist_string.join("\n").as_bytes());
}


// This function works like write_music_config,
// But does not check for whether the file is real,
// Instead it writes "i:::\n" for the given entry instead.
// Seems to work correctly.
fn defaultise_song_in_list(song_list: &mut Vec<String>,i_num:usize) {
	
	let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("as").unwrap();
	let note_path = assets.join("notes/notes.mqcfg");
	
	song_list[i_num] = "Standard".to_owned();
	
	//Try to open the playlist file, and if isn't there, create it.
	let mut cfgf:File = File::create(&note_path).unwrap();
	
	let mut playlist_string:Vec<String> = Vec::with_capacity(ISEKAIN);
	
	for (i,x) in song_list.iter().enumerate() {
		if x=="Standard" {
			playlist_string.push(format!("{}:::",i));
		}else{
			match File::open(x) {
				Ok(_) => {playlist_string.push(format!("{}:::{}",i,x))},
				_	  => {playlist_string.push(format!("{}:::",i))},
			};
		};
	};
	//write everything to file.
	cfgf.write_all(playlist_string.join("\n").as_bytes());
}

// Read the notes.mqcfg file and if valid files exist, they go into the vector.
// Does not do a good check for corruption of config file.
pub fn parse_music_config(songs:&mut Vec<String>) {
	
	//initialise directories.
	let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("as").unwrap();
	let note_path = assets.join("notes/notes.mqcfg");
	
		//Try to open the playlist file, and if isn't there, create it.
	let mut cfgf:File;
	match File::open(&note_path) {
		Ok(f) => {cfgf = f;},
		_	  => {return;},
	};
	
	let mut playlist_string = String::new();
	let mut song_string = String::new();
	
	cfgf.read_to_string(&mut playlist_string);
	
	for (i,mut x) in playlist_string.lines().enumerate() {
		let mut format = true;
		let mut no_of_split:usize = 0;
		for (j,y) in x.split(":::").enumerate() {
			no_of_split+= 1;
			if j==0 {
				match y.trim().parse::<usize>(){
					Ok(num) => {if num>ISEKAIN-1 {format = false;};},
					_		=> {format = false;},
				};
			}else if j==1 {
				match File::open(y) {
					//This needs to be improved to check for whether it's a music file or not.
					Ok(file) => {song_string = y.to_owned();},
					_		 => {format = false;},
				};
			};
		};
		if no_of_split != 2 {format = false;};
		if (i<ISEKAIN) & format { songs[i] = song_string.clone(); };
	};
}

//SAVE FUNCTION.
fn save(xx:&Vec<(Lifeform,usize)>,
		nx:&Vec<String>,
		spl:&Vec<Spell>,
		p:&Place,
		mut comm_text:&mut String,
		ui: &mut conrod::UiCell,
		ids: &mut Ids){
	
	let mut s_name:String = nx[0].to_owned();
	let dir=env::current_dir().unwrap().join("as/saves");
	let mut f1 = dir.join(s_name.clone()+".msqrtxt");
	let mut f2 = dir.join(s_name+".msqrb");
			
	let mut stxt=File::create(&f1).unwrap();
	let mut sfile=File::create(&f2).unwrap();

	let n_party:u8=xx.len() as u8;
	sfile.write_all(&[n_party]);
	for i in 0..xx.len(){
		let l1=xx[i].0.Spellist.len();
		let l2=xx[i].0.Inventory.len();
		let exp:[u8;8]=byter64(by_f64(xx[i].0.Exp as f64));
		let expu:[u8;8]=byter64(by_f64(xx[i].0.ExpUsed as f64));
		let mp:[u8;8]=byter64(by_f64(xx[i].0.MP as f64));
		let hp:[u8;8]=byter64(by_f64(xx[i].0.HP as f64));
		let speed:[u8;8]=byter64(by_f64(xx[i].0.Speed as f64));
		let att:[u8;8]=byter64(by_f64(xx[i].0.Attack as f64));
		let def:[u8;8]=byter64(by_f64(xx[i].0.Defence as f64));
		let wm:[u8;8]=byter64(by_f64(xx[i].0.WM as f64));
		let bm:[u8;8]=byter64(by_f64(xx[i].0.BM as f64));
		let lspl:[u8;8]=byterus(l1);
		let linv:[u8;8]=byterus(l2);
		let live:u8=if xx[i].0.Alive==true{1}else{0};
		let uncl:u8=if xx[i].0.Unclean==true{1}else{0};
		let lox:[u8;4]=byteri32(p.xy[0]);
		let loy:[u8;4]=byteri32(p.xy[1]);
		let lgp:[u8;8]=byterus(xx[i].0.Gold);
		let lid:[u8;8]=byterus(xx[i].0.id);
		
		sfile.write_all(&exp).expect("error writing sfile");
		sfile.write_all(&expu).expect("error writing sfile");
		sfile.write_all(&mp).expect("error writing sfile");
		sfile.write_all(&hp).expect("error writing sfile");
		sfile.write_all(&speed).expect("error writing sfile");
		sfile.write_all(&att).expect("error writing sfile");
		sfile.write_all(&def).expect("error writing sfile");
		sfile.write_all(&wm).expect("error writing sfile");
		sfile.write_all(&bm).expect("error writing sfile");
		sfile.write_all(&lspl).expect("error writing sfile");
		sfile.write_all(&linv).expect("error writing sfile");
		sfile.write_all(&[live]).expect("error writing sfile");
		sfile.write_all(&[uncl]).expect("error writing sfile");
		sfile.write_all(&lox).expect("error writing sfile");
		sfile.write_all(&loy).expect("error writing sfile");
		sfile.write_all(&lgp).expect("error writing sfile");
		sfile.write_all(&lid).expect("error writing sfile");
	
		stxt.write(&nx[i].as_bytes()).expect("error writing stxt");
		stxt.write("\n".as_bytes()).expect("error writing stxt");
		stxt.write(&xx[i].0.name.as_bytes()).expect("error writing stxt");
		stxt.write("\n".as_bytes()).expect("error writing stxt");
		stxt.write(&convert_mon_type(xx[i].0.Type).as_bytes()).expect("error writing stxt");
		stxt.write("\n".as_bytes()).expect("error writing stxt");
		stxt.write(&convert_affinity(xx[i].0.SubType).as_bytes()).expect("error writing stxt");
		stxt.write("\n".as_bytes()).expect("error writing stxt");
		stxt.write(&convert_affinity(xx[i].0.Affinity).as_bytes()).expect("error writing stxt");
		stxt.write("\n".as_bytes()).expect("error writing stxt");
		if l1>0{
			for j in 0..l1{
				stxt.write(arcana_name_from_spell_id(spl,xx[i].0.Spellist[j]).as_bytes()).expect("error writing stxt");
				stxt.write("\n".as_bytes()).expect("error writing stxt");
			};
		}else{};
//		if l2>0{
//			for j in 0..l2{
//				stxt.write(xx[i].0.Inventory[j].as_bytes());
//				stxt.write("\n".as_bytes());
//			};
//		}else{};
		*comm_text = format!("O holy salvation! {} was saved to disk...",nx[0]);
		set_comm_text(comm_text,ui,ids);
	};
		
}

fn marker_of_loader(){}
// Dir Selecter for loader.
// This sets the list of possible games to load.
fn loader(
		mut comm_text: &mut String,
		ui: &mut conrod::UiCell,
		ids: &mut Ids,
		men_wh: &[f64;2])->(Option<String>,usize){
			
	let mut to_load:(Option<String>,usize) = (None,0);
	let mut in_to_load = String::new();
			
	//Input initial text.
	*comm_text = "Pick a moose, any moose. And I will load it for you.".to_owned();
	set_comm_text(&mut comm_text,ui,ids);
	
	let mut fname=String::new();
	let mut saves:Vec<String>=Vec::new();
	let dir=env::current_dir().unwrap().join("as/saves");
	let sandra=fs::read_dir(dir).unwrap();
	let mut sd=std::ffi::OsString::new();
	for entry in sandra{
		sd=entry.unwrap().file_name().clone();
		let ss=sd.to_str().unwrap();
		if ss.len()>4{
			if &ss[(ss.len()-6)..ss.len()]==".msqrb"{
				saves.push(ss[0..(ss.len()-6)].to_owned().clone())
			}else{};
		}else{};
	};
	
	//if no saved games, wait five seconds and declare it to the world.
	let n_saves = saves.len();
	if n_saves==0{
			comm_text.push('\n');
		for _ in 0..10{
			comm_text.push('.');
			set_comm_text(&mut comm_text,ui,ids);
			thread::sleep(Duration::from_millis(500));
		};
		*comm_text = "I told you to pick a moose, but actually there is nothing to choose from.".to_owned();
		set_comm_text(&mut comm_text,ui,ids);
	}else{
		//run script to make menu of load game buttons and a loader for them.
		let width:f64 = ui.w_of(ids.middle_column).unwrap()-BORDER*2.0;
		let wm:f64 = ui.w_of(ids.master).unwrap();
	
		widget::Scrollbar::y_axis(ids.middle_column).auto_hide(true).set(ids.load_menu_scroll, ui);
		let mut saves_matrix = widget::Matrix::new(1,n_saves)
			.w(width)
			.h(men_wh[1]/10.0*(n_saves as f64))
			.mid_top_of(ids.middle_column)
			.set(ids.load_menu, ui);	
		
		let button = widget::Button::new().color(BUTTON_COLOUR)
										  .h(men_wh[1]/10.0)
										  .label_font_size(font_size_chooser_button_b(wm));
		
        while let Some(save) = saves_matrix.next(ui) {
            let r = save.row as usize;
            
            for _click in save.set(button.clone().label(&saves[r]),ui) {
				println!("Hey! Loading {}", saves[r]);
				in_to_load.push_str(&saves[r]);
				*comm_text = format!("Preparing to load {}", in_to_load);
				set_comm_text(comm_text,ui,ids);
			};
        };
        let pre_to_load = if in_to_load==String::new(){None}else{Some(in_to_load.to_owned().clone())};
		to_load = (pre_to_load,42);
	};
	return to_load
}	


fn marker_of_load(){}
//rewritten load function.
//NB has some "illogical" stuff here to preserve backwards compatibility
fn load<'a,'b>( file_name:String, spl:&Vec<Spell>, world:&Vec<[Place;19]>, mons:&Vec<Lifeform>,
				mut party:&mut Vec<(Lifeform,usize)>,
				mut p_names:&mut Vec<String>,
				mut p_loc:&mut Place,
				mut pl:&mut (usize,usize),
				mut coords: &mut [i32;2],
				mut comm_text:&mut String,
				ref mut ui: &mut conrod::UiCell,
				ids:&mut Ids){	
	println!("filename: {}",file_name);
	//Initiate raw data constructs.
	let mut rlb = vec![0;8000];
	let mut ltxt:Vec<String> = Vec::new();
	let mut rltxt = String::new();
	let to_open_a = env::current_dir().unwrap().join("as/saves").join(file_name.clone()+".msqrb");
	let to_open_b = env::current_dir().unwrap().join("as/saves").join(file_name+".msqrtxt");

	println!("msqrb: {:?}",to_open_a);
	println!("msqrtxt: {:?}",to_open_b);
	//open files and read into raw data constructs.
	let mut loadb= File::open(to_open_a).unwrap();
	let mut loadtxt= File::open(to_open_b).unwrap();
	loadb.read(&mut rlb);
	loadtxt.read_to_string(&mut rltxt);
	
	//reformat text file from &str to String.
	let rrltxt:Vec<&str> = rltxt.split("\n").collect();
	for i in 0..rrltxt.len(){ltxt.push(rrltxt[i].to_owned())};
		
	//reset party, party location and party names.
	*p_names = Vec::with_capacity(5);
	*party = Vec::with_capacity(5);		
	*coords = [0,0];
	
    let mut indtrack:usize=0;
//reconstitute party number u8->u64
	let p_no=(rlb.remove(0)) as usize;
	for _ in 0..p_no{		
//reconstitute Exp.
		let mut temp:[u8;8]=[0;8];
		for i in 0..8{temp[i]=rlb.remove(0)};
		let exp=un64_f64(unbyte64(temp)) as f32;
//reconstitute ExpUsed.
		let mut temp:[u8;8]=[0;8];
		for i in 0..8{temp[i]=rlb.remove(0)};
		let expu=un64_f64(unbyte64(temp)) as f32;
//reconstitute MP.
		let mut temp:[u8;8]=[0;8];
		for i in 0..8{temp[i]=rlb.remove(0)};
		let mp=un64_f64(unbyte64(temp)) as f32;		
//reconstitute HP.
		let mut temp:[u8;8]=[0;8];
		for i in 0..8{temp[i]=rlb.remove(0)};
		let hp=un64_f64(unbyte64(temp)) as f32;
//reconstitute speed.
		let mut temp:[u8;8]=[0;8];
		for i in 0..8{temp[i]=rlb.remove(0)};
		let speed=un64_f64(unbyte64(temp)) as f32;
//reconstitute attack.
		let mut temp:[u8;8]=[0;8];
		for i in 0..8{temp[i]=rlb.remove(0)};
		let attack=un64_f64(unbyte64(temp)) as f32;		
//reconstitute defence.
		let mut temp:[u8;8]=[0;8];
		for i in 0..8{temp[i]=rlb.remove(0)};
		let defence=un64_f64(unbyte64(temp)) as f32;			
//reconstitute wm.
		let mut temp:[u8;8]=[0;8];
		for i in 0..8{temp[i]=rlb.remove(0)};
		let wm=un64_f64(unbyte64(temp)) as f32;	
//reconstitute bm.
		let mut temp:[u8;8]=[0;8];
		for i in 0..8{temp[i]=rlb.remove(0)};
		let bm=un64_f64(unbyte64(temp)) as f32;	
//reconstitute spell list length.
		let mut temp:[u8;8]=[0;8];
		for i in 0..8{temp[i]=rlb.remove(0)};
		let sp_len=unbyteus(temp);
		println!("{}",sp_len);
//reconstitute inventory length.
		let mut temp:[u8;8]=[0;8];
		for i in 0..8{temp[i]=rlb.remove(0)};
		let inv_len=unbyteus(temp);		
//reconstitute Alive? Unclean?
		let temp=rlb.remove(0);
		let alive=if temp==0{false}else{true};
		let temp=rlb.remove(0);
		let unclean=if temp==0{false}else{true};
//reconstitute longitude
		let mut temp:[u8;4]=[0;4];
		for i in 0..4{temp[i]=rlb.remove(0)};
		let lox=unbyte32(temp);	
//reconstitute latitude
		let mut temp:[u8;4]=[0;4];
		for i in 0..4{temp[i]=rlb.remove(0)};
		let loy=unbyte32(temp);
//reconstitute Gold.
		let mut temp:[u8;8]=[0;8];
		for i in 0..8{temp[i]=rlb.remove(0)};
		let lgp=unbyteus(temp);
//reconstitute id.
		let mut temp:[u8;8]=[0;8];
		for i in 0..8{temp[i]=rlb.remove(0)};
		let lid=unbyteus(temp);
//reconstitute name
		let true_name= indtrack;
		indtrack+= 1;
		let mut lname="";
		let mut ltype="";
		let mut lsubtype="";
		let mut laffinity="";
		let mut lspellist:Vec<i8>=Vec::new();
		let mut linventory= Vec::new();
		for x in mons.iter(){
			if ltxt[indtrack].as_str()==x.name{
				lname=x.name
			}else{lname=lname};
			if ltxt[indtrack+1].as_str()==convert_mon_type(x.Type){
				ltype=convert_mon_type(x.Type)
			}else{ltype=ltype};
			if ltxt[indtrack+2].as_str()==convert_affinity(x.SubType){
				lsubtype=convert_affinity(x.SubType)
			}else{lsubtype=lsubtype};
			if ltxt[indtrack+3].as_str()==convert_affinity(x.Affinity){
				laffinity=convert_affinity(x.Affinity)
			}else{laffinity=laffinity};
		};
		indtrack+=4;
		for i in 0..sp_len{
			for y in spl.iter(){
				//println!("{:?}",ltxt[i+indtrack].as_str());
				if ltxt[i+indtrack].as_str()==y.name{
					lspellist.push(y.id)
				}else{};
			};
		};
		println!("{:?}",lspellist);
		indtrack+=sp_len;		
		//implement inventory later.
//export from iterator.
		
		//write final coords, names, party members and their location.
		*coords = [loy,lox];
		p_names.push(ltxt[true_name].clone());
		party.push((Lifeform{
					name: lname,
					Type: convert_affinity_rev(ltype),
					SubType: convert_affinity_rev(lsubtype),
					Affinity: convert_affinity_rev(laffinity),
					Exp: exp,
					ExpUsed: expu,
					Spellist: lspellist,
					MP: mp,
					HP: hp,
					Speed: speed,
					Attack: attack,
					Defence: defence,
					WM: wm,
					BM: bm,
					MP_shade: mp,
					HP_shade: hp,
					Speed_shade: speed,
					Attack_shade: attack,
					Defence_shade: defence,
					WM_shade: wm,
					BM_shade: bm,
					Alive: alive,
					Unclean: unclean,
					Inventory: linventory,
					Gold: lgp,
					id: lid,
					},0))	
	};
	*pl = place_loader(&world,[coords[1],coords[0]]);
	*pl = (world.len()-1-pl.0,pl.1);
	//*pl = (pl.0,pl.1);
	*p_loc = world[world.len()-1-pl.0][pl.1].clone();
	
	//Say that you've done your job and everything is loaded.
	let extra:String = if p_names.len()>1 {format!(" {} the {} is with them.",p_names[1],party[1].0.name)}else{String::new()};
	*comm_text = format!("{} the {} has been loaded.{}",p_names[0],party[0].0.name,extra);
	set_comm_text(&mut comm_text,ui,ids);
	println!("{}",&comm_text);
}

fn place_loader(w:&Vec<[Place;19]>, coords:[i32;2])->(usize,usize){
	let mut answer:(usize,usize)=(10,8);
	for i in 0..w.len(){
		for j in 0..w[i].len(){
			if w[i][j].xy==coords{
				answer=(i,j);
				return answer
			}else{};
		};
	};
	answer
}

//function that tells the computer to generate a random encounter based on the monster population of the area.
fn rand_enc(p_loc:&Place)->bool{
		let mut totapop=0;
		for i in 0..p_loc.popu.len(){totapop+= p_loc.popu[i].2};
		let limit= if totapop<30000{40000}else{totapop+10000};
		if totapop<rand::thread_rng().gen_range(0,limit){false}else{true}		
}

fn ingrids(xx:&Vec<(Lifeform,usize)>, ifast:usize)->([usize;5],Vec<usize>){
	let mut ingrids:[usize;5]=[255,255,255,255,255];
	let mut is:Vec<usize>=Vec::new();
//			if ifast==99999{
//			println!("{}",xx[ifast].0.name);
//			panic!("")}else{};
	//println!("ig A");
	//println!("{}",ifast);
	let gfast=xx[ifast].1;
	//println!("ig B");
	let mut fed:usize=0;
	for i in 0..xx.len(){
		if xx[i].1==gfast{
			ingrids[fed]=xx[i].0.id;
			is.push(i);
			fed+=1
		}else{};		
	};
	(ingrids,is)
}

fn any_key(){
	println!("[Press any key to continue]\n");
	let mut steserifu=String::new();
	io::stdin().read_line(&mut steserifu).expect("Error in any_key()");
}


//NB, final output is fn(a,b,c)->fn(party){} (aka sage function), current output is party. 
fn sage_caller(party:Vec<(Lifeform,usize)>, loc:&Place, summon:&str,spl:&Vec<Spell>)->Vec<(Lifeform,usize)>{
	let mut spell:&Spell= &spl[0];
	for x in spl.iter(){
		if summon==x.name{spell=x}else{spell=spell}
	};
	
	let die=rand::thread_rng().gen_range(0,20);
	
	if (loc.scape==DESERT) & (spell.Type==FIRE) & (die>10){
		smoose::sage_fire(party,loc,&summon,spl)
		//(mut party:Vec<(Lifeform,usize)>, loc:&Place, summon:&str,spl:&Vec<Spell>)
	}else if (loc.scape==ICE) & (spell.Type==ICE) & (die>10){
		party
		//sage_ice(party)
	}else if (loc.name=="Albion") & (spell.name=="Light") & (die>15){
		party
		//sage_albion(party)
	}else if (loc.name=="Malachia") & (spell.name=="Summon Reaper") & (die>18){
		party
		//sage_apocalypse(party)
	}else if (loc.affinity==LIGHTNING) & (spell.Type==LIGHTNING) & (die>10){
		party
		//sage_lightning(party)
	}else if (loc.name=="White Sea") & (spell.Type==HEALING) & (die>12){
		party
		//sage_life(party)
	}else if ((loc.name=="City of the Dead")||(loc.name=="Citadel of Death")) & (spell.name=="Exorcism"){
		party
		//sage_death(party)
	}else if (loc.name=="Black Obelisk") & (spell.name=="Darkness") & (die>10){
		party	
		//sage_darkness(party)
	}else{
		println!("...But nothing happens.");
		party
	}
}

fn null_sage(party:Vec<(Lifeform,usize)>)->Vec<(Lifeform,usize)>{
	println!("I did nothing with party.");
	party
}


//Housekeeping trait for splitting monster names.
#[allow(non_camel_case_types)]
trait wpl {
	fn iwpl(self)->String;
	fn x_chr_pl(self,lim:usize)->String;
	fn x_chr_pl2(self,lim:usize)->String;
}

impl<'a> wpl for &'a str {
	fn iwpl(self)->String {i_wpl(self)}	
	fn x_chr_pl(self,lim:usize)->String {xchrpl(self,lim)}
	fn x_chr_pl2(self,lim:usize)->String {xchrpl2(self,lim)}
}

//Make monster name one word per line (can go terribly wrong for long names)
fn i_wpl(text:&str)-> String {
	let mut out_string = String::new();
	for x in text.to_owned().chars(){
		if x==' ' {
			out_string.push('\n')
		}else{
			out_string.push(x);
		};
	};
	out_string
}

//Make monster name at least 'lim' letters per line. (Can still go terribly wrong).
fn xchrpl(text:&str, lim:usize)-> String {
	let mut out_string = String::new();
	for (i,x) in text.to_owned().chars().enumerate() {
		if (x==' ') & (i>lim) {
			out_string.push('\n')
		}else{
			out_string.push(x);
		};
	};
	out_string
}

//Make monster name at least 'lim' letters per line. (slightly better version).
fn xchrpl2(text:&str, lim:usize)-> String {
	let mut out_string = String::new();
	let mut len_before:usize = 0;
	let mut len_after:usize = 0;
	
	for (i,x) in text.split(' ').enumerate() {
		len_after = len_before +x.clone().chars().count() + 1;
		if len_after>lim {
			out_string.push('\n');
			len_before = 0;
		}else{
			out_string.push(' ');
			len_before+= 1;
		};
		out_string.push_str(&x);
		len_before+= x.chars().count();
	};
	out_string
}
			
fn dungeon_finder(p_loc: &mut Place, dungeons: &mut Vec<Dungeon>,party:&Vec<(Lifeform,usize)>) -> Option<usize> {
	for i in 0..dungeons.len() {
		if dungeons[i].xy==p_loc.xy {
			if dungeons[i].diff<party[0].0.ExpUsed {
				return Some(i)
			};
		};
	}
	None	
}

//dungeon navigator for advancing post battle
fn dungeon_navigator_a_marker(){}
fn dungeon_navigator_a (victory:bool,
						mut idungeon: &mut Option<usize>,
						mut dungeon_pointer: &mut usize,
						dungeons: &mut Vec<Dungeon>,
						mut tt_e_c_i_ll: &mut [bool;8]) {
	if idungeon.is_some() {
		if victory {
			if *dungeon_pointer < dungeons[idungeon.unwrap()].scenes.len()+3 {
				*dungeon_pointer+= 1;
			}else{
				*dungeon_pointer = 0;
				*idungeon = None;
			};
		}else{
			*idungeon = None;
			*dungeon_pointer = 0;
			tt_e_c_i_ll[2] = false;
			tt_e_c_i_ll[0] = true;
		};
	};
	//println!("dpoint: {}\nidun: {:?}",dungeon_pointer,idungeon);
}


//parabolic sin and cos function for quick and dirty graphical trig.
//Very dirty and inaccurate.
fn cosp(time:usize,base:usize)->f64 {
	let b4 = base/4;
	let x = time%base;
	let x4 = x%b4;
	if x<b4 {
		1.0-(x4 as f64/ b4 as f64)*(x4 as f64/ b4 as f64)
	}else if x<2*b4 {
		-(1.0-((b4-x4) as f64/ b4 as f64)*((b4-x4) as f64/ b4 as f64))
	}else if x<3*b4 {
		-(1.0-(x4 as f64/ b4 as f64)*(x4 as f64/ b4 as f64))
	}else{
		1.0-((b4-x4) as f64/ b4 as f64)*((b4-x4) as f64/ b4 as f64)
	}
}

//parabolic sin and cos function for quick and dirty graphical trig.
fn sinp(time:usize,base:usize)->f64 {
	let b4 = base/4;
	let x = time%base;
	let x4 = x%b4;
	if x<b4 {
		1.0-((b4-x4) as f64/ b4 as f64)*((b4-x4) as f64/ b4 as f64)
	}else if x<2*b4 {
		1.0-(x4 as f64/ b4 as f64)*(x4 as f64/ b4 as f64)
	}else if x<3*b4 {
		-(1.0-((b4-x4) as f64/ b4 as f64)*((b4-x4) as f64/ b4 as f64))
	}else{
		-(1.0-(x4 as f64/ b4 as f64)*(x4 as f64/ b4 as f64))
	}	
}

//Function to retrieve true/false state from a parallel universe. Ermm... Thread.
//Function assumes the existence of a parallel sender in a parallel universe. Ermm.. Thread.
//It should be easy to make a generic version for all copy types, but I don't need to.
//Message bool is the original value of the variable.
//isekai deguchi can cause problems if the returned bool is not reused in the next message.
pub fn isekai_deguchi (message:(bool,usize), postman:&mut Receiver<(bool,usize)>)->(bool,usize) {
	match postman.try_recv() {
		Ok(answer) => answer,
		Err(_) => (message),
	}
}

//Same idea as deguchi, but rather a silencing function.
pub fn isekai_urusai (silence: bool, silent_postman:&mut Receiver<bool>)->bool {
	match silent_postman.try_recv() {
		Ok(urusai) => urusai,
		Err(_)	   => silence,
	}
}

//function generates the postman inner touple's u8 value to be fed into isekai deguchi
//based on a) terrain, b) relative party strength.
//the u8 code should then correspond to the selection of battle music.
//I don't think that there will be more than 255 songs.
//TODO redo output as usize to allow direct indexing.
pub fn isekai_index (party:&Vec<(Lifeform,usize)>,
					 encounter:&Vec<(Lifeform,usize,[Option<[usize;2]>;2])>,
					 dungeons:&Vec<Dungeon>,
					 loc:&Place,
					 dp:usize,
					 idun:&Option<usize>)-> usize {
	if idun.is_some() & (dp>0) {
		match dungeons[idun.unwrap()].affinity {
			TIME | RADIANT => {1}, // 1
			UNDEAD | EVIL => {2}, // 2
			WITCH | SPIRIT => {8}, // 3
			MALACHIA | ALBION => {18},// 4
			_ => {if exp_calc(encounter,0)>10.0 {
						0 //actually 6: boss music
					}else{
						match loc.scape {
							ICE => 7, // 7
							TUNDRA => 8, // 8
							WATER => 0, // 9
							GRASSLAND => 10, // 10
							FOREST => 11, // 11
							STEPPE => 12, // 12
							DESERT => 13, // 13
							CITY => 14, // 14
							HIGHLAND => 0, // 15
							MOORLAND => 16, // 16
							VOID => 0, // 17
							RUIN => 18, // 18
							_ => 0,	
						}		
					}
			}, //5
		}
	}else{
		if exp_calc(encounter,0)>10.0 {
			0 //actually 6: boss music
		}else{
			match loc.scape {
				ICE => 7, // 7
				TUNDRA => 8, // 8
				WATER => 0, // 9
				GRASSLAND => 10, // 10
				FOREST => 11, // 11
				STEPPE => 12, // 12
				DESERT => 13, // 13
				CITY => 14, // 14
				HIGHLAND => 0, // 15
				MOORLAND => 16, // 16
				VOID => 0, // 17
				RUIN => 18, // 18
				_ => 0,	
			}		
		}
	}					 				 
}


//A try to get the colours nicer. (Borrowed function from the support module)
//Ideally need to put in the correction factor to allow redraw.
//Windows overcompensates for things sometimes...
use conrod::color::f32_to_byte;
// A needless function name must be conserved.
#[cfg(target_os = "windows")]
pub fn gamma_to_gamma(s: Color) -> [u8;4] {
	let c = s.to_rgb();
    fn component(f: f32) -> f32 {
        // Taken from https://github.com/PistonDevelopers/graphics/src/color.rs#L42
        if f <= 0.003130805 {
            f * 12.92
        } else {
           1.055*f.powf(1.0/2.4) - 0.055
        }
    }
    [f32_to_byte(component(c.0)),
     f32_to_byte(component(c.1)),
     f32_to_byte(component(c.2)), 
     f32_to_byte(c.3)]
}

// A needless function name must be conserved.
#[cfg(not(target_os = "windows"))]
pub fn gamma_to_gamma(s: Color) -> [u8;4] {
	let c = s.to_rgb();
    [f32_to_byte(c.0),
     f32_to_byte(c.1),
     f32_to_byte(c.2), 
     f32_to_byte(c.3)]
}

// color of map squares.
// NB output is image::color::rgba
// NB todo-> convert conrod colours to image colours. Should be fairly straightforwards.
pub fn map_sq_col_img(square:&Place)-> [u8; 4] {
	match square.scape {
		ICE => gamma_to_gamma(color::LIGHT_BLUE),
		TUNDRA => gamma_to_gamma(color::LIGHT_PURPLE),
		WATER => gamma_to_gamma(color::DARK_BLUE),
		GRASSLAND => gamma_to_gamma(color::DARK_GREEN.with_luminance(0.4)),
		FOREST => gamma_to_gamma(color::DARK_GREEN.with_luminance(0.2)),
		STEPPE => gamma_to_gamma(color::LIGHT_BROWN.with_luminance(0.4)),
		DESERT => gamma_to_gamma(color::DARK_YELLOW.with_luminance(0.3)),
		CITY => gamma_to_gamma(color::DARK_GREY.with_luminance(0.3)),
		HIGHLAND => gamma_to_gamma(Color::Rgba(0.12,0.15,0.3,1.0)),
		MOORLAND => gamma_to_gamma(color::DARK_GREEN.with_luminance(0.3)),
		VOID => gamma_to_gamma(color::BLACK),
		RUIN => gamma_to_gamma(color::GREY.with_luminance(0.2)),
		_ => gamma_to_gamma(color::GREY.with_luminance(0.1)),
	}
}

//Function for redrawing the map nicely.
//It might be very difficult to get this to run efficiently.
pub fn refine_map(map: &mut Vec<[u8;4]>,xs:usize,ys:usize) {
	
	//arranged image. Outer is rows. inner is columns
	let mut map_a:Vec<Vec<[u8;4]>> = vec![Vec::with_capacity(xs);ys];
	
	//transcribe map into a matrix.
	let mut col:usize = 0;
	let mut row:usize = 0;
	for x in map.iter() {
		map_a[row].push(*x);
		col+= 1;
		if col==xs {
			row+= 1;
			col = 0;
		};
	}
	println!("heightarr: {}, widtharr: {}",map_a.len(),map_a[ys-1].len());
	//Do the smoothing. (Three step)
	let mut remainder:usize = 0;
	for sm in SQUARES.iter().map(|x| *x) {
		for y in sm..(ys-sm) { //iterate through rows.
			for x in sm..(xs-sm) { //iterate through column values
				
				//Smooth corners.
				if 		 (map_a[y][x-sm]==map_a[y-sm][x]) & (map_a[y-sm][x] != map_a[y][x]) {
					map[y*xs+x] = map_a[y][x-sm]
				}else if (map_a[y][x+sm]==map_a[y-sm][x]) & (map_a[y-sm][x] != map_a[y][x]) {
					map[y*xs+x] = map_a[y][x+sm]
				}else if (map_a[y][x+sm]==map_a[y+sm][x]) & (map_a[y+sm][x] != map_a[y][x]) {
					map[y*xs+x] = map_a[y][x+sm]
				}else if (map_a[y][x-sm]==map_a[y+sm][x]) & (map_a[y+sm][x] != map_a[y][x]) {
					map[y*xs+x] = map_a[y][x-sm]
				};
				
				//wave flat horizontals. I expect this to be inefficient.
				if (map_a[y-1][x-sm]==map_a[y-1][x+sm])
				 & (map_a[y][x-sm]==map_a[y][x+sm])
				 & (map_a[y][x-sm] != map_a[y-1][x-sm]) {
					 for i in 0..saw_tooth_img(sm*2,x,1) {
						 map[(y+i)*xs+x] = map_a[y-1][x-sm];
					 }
				}
				
				//wave flat verticals. I expect this to be inefficient.
				if (map_a[y-sm][x-1]==map_a[y+sm][x-1])
				 & (map_a[y-sm][x]==map_a[y+sm][x])
				 & (map_a[y-sm][x] != map_a[y-sm][x-1]) {
					 for i in 0..saw_tooth_img(sm*2,y,2) {
						 map[y*xs+x+i] = map_a[y-sm][x-1];
					 }
				}
			}
		}
		//remap map_a
		for y in sm..(ys-sm) { //iterate through rows.
			for x in sm..(xs-sm) { //iterate through column values
				if map[y*xs+x] != map_a[y][x] {map_a[y][x] = map[y*xs+x];};
			}
		}
	}
}

//function for making saw tooth index.
fn saw_tooth_img(base:usize,x_ind:usize,attenuation:usize)->usize {
	let ini = x_ind%base;
	if ini>base-ini {
		(base-ini)/attenuation
	}else{
		ini/attenuation
	}
}


//parabolic sin and cos function for quick and dirty graphical trig.
//Very dirty and inaccurate.
fn cospt(angle:f64,base:f64)->f64 {
	let b4 = base/4.0;
	let x = angle%base;
	let x4 = x%b4;
	if x<b4 {
		1.0-(x4/b4)*(x4/b4)
	}else if x<2.0*b4 {
		-(1.0-((b4-x4)/b4)*((b4-x4)/b4))
	}else if x<3.0*b4 {
		-(1.0-(x4/b4)*(x4/b4))
	}else{
		1.0-((b4-x4)/b4)*((b4-x4)/b4)
	}
}

//parabolic sin and cos function for quick and dirty graphical trig.
fn sinpt(angle:f64,base:f64)->f64 {
	let b4 = base/4.0;
	let x = angle%base;
	let x4 = x%b4;
	if x<b4 {
		1.0-((b4-x4)/b4)*((b4-x4)/b4)
	}else if x<2.0*b4 {
		1.0-(x4/b4)*(x4/b4)
	}else if x<3.0*b4 {
		-(1.0-((b4-x4)/b4)*((b4-x4)/b4))
	}else{
		-(1.0-(x4/b4)*(x4/b4))
	}	
}

// Function to generate a series of points around a centre
// for the creation of a polygon.
// NB this is not a safe function. It will try to make a polygon
// out of ZERO,ONE or TWO points.
fn poly_round_marker(){}
fn poly_round(r:f64,n:usize,c:&[f64;2])->Vec<[f64;2]> {
	//println!("entering polygonal");
	//if less than three points 
	let mut output:Vec<[f64;2]> = Vec::with_capacity(n); 
	let mut total_angle:f64 = 0.0;
	let step_angle:f64 = 360.0/n as f64;
	for _ in 0..n {
		let x = r*(cospt(total_angle,360.0)+rand::thread_rng().gen_range(-0.3,0.3));
		let y = r*(sinpt(total_angle,360.0)+rand::thread_rng().gen_range(-0.3,0.3));
		total_angle+= step_angle;
		output.push([x+c[0],y+c[1]]);
	}
	let f = output[0].clone();
	output.push(f);
	//println!("exiting polygonal: outuput={:?}",output);
	output
}

//function to generate points of a 4 point star
fn poly_star_marker(){}
fn poly_star(r:f64,c:&[f64;2])->Vec<[f64;2]> {
	let mut out = Vec::with_capacity(9);
	out.push([c[0],c[1]+r]);out.push([c[0]+r/5.0,c[1]+r/5.0]);
	out.push([c[0]+r,c[1]]);out.push([c[0]+r/5.0,c[1]-r/5.0]);
	out.push([c[0],c[1]-r]);out.push([c[0]-r/5.0,c[1]-r/5.0]);
	out.push([c[0]-r,c[1]]);out.push([c[0]-r/5.0,c[1]+r/5.0]);
	out.push([c[0],c[1]+r]);
	out
}
