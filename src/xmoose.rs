


/// "xmoose.rs" module contains "special effects and graphics".
/// Anything where a widget is used
/// to store anything wherre theer are moving parts.

extern crate find_folder;
extern crate num_cpus;
extern crate inflector;
extern crate num;
extern crate rand;
extern crate time;
extern crate conrod;

use lmoose::*;
use cmoose::*;
use cmoose::GraphicsBox::*;
use gmoose::*;
use shared_moose::*;

use std;
use rand::Rng;
use conrod::{color, widget,Colorable, Positionable, Sizeable, Widget};
use conrod::color::Color;

const HOLY_COLOUR:color::Colour = Color::Rgba(255.0/255.0,222.2/255.0,222.2/255.0,200.0/255.0);

//function to make a sprite attack another.
//moves the attacker by the appropriate amount.
//needs to be inside a "match sprite_boxer {" clause.
fn sprite_approach_marker(){}
pub fn sprite_approach (sprite_box:&SpriteBox)-> [f64;2] {
	
	let factor = (sprite_box.turns_init-sprite_box.turns_to_go as f64)/sprite_box.turns_init;			 
	let mut dx:f64 = (sprite_box.def_coord[0] - sprite_box.att_coord[0])*factor;
	let mut dy:f64 = (sprite_box.def_coord[1] - sprite_box.att_coord[1])*factor;
	
	[sprite_box.att_coord[0]+dx,sprite_box.att_coord[1]+dy]
}

//it is imperative to decrement this at the end of each turn and make it disappear.
//and to set the sprite to vibrate if the attack actually hits.
fn sprite_box_dec_marker(){}
pub fn sprite_box_decrement (sprite_boxer:&mut GraphicsBox,
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
pub fn sprite_box_filler(magic:&Spell,
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
			println!("filling lightning box with {}",magic.name);
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
			println!("filling lightning box with {}",magic.name);
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

//Time scape related functions.
pub fn border_crawler_a(centre:conrod::Point, wh:conrod::Dimensions, 
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

pub fn border_crawler_b(centre:conrod::Point, wh:conrod::Dimensions, 
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

pub fn border_crawler_c(centre:conrod::Point, wh:conrod::Dimensions, 
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

pub fn border_crawler_d(centre:conrod::Point, wh:conrod::Dimensions, 
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

// Function to generate a series of points around a centre
// for the creation of a polygon.
// NB this is not a safe function. It will try to make a polygon
// out of ZERO,ONE or TWO points.
fn poly_round_marker(){}
pub fn poly_round(r:f64,n:usize,c:&[f64;2])->Vec<[f64;2]> {
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
pub fn poly_star(r:f64,c:&[f64;2])->Vec<[f64;2]> {
	let mut out = Vec::with_capacity(9);
	out.push([c[0],c[1]+r]);out.push([c[0]+r/5.0,c[1]+r/5.0]);
	out.push([c[0]+r,c[1]]);out.push([c[0]+r/5.0,c[1]-r/5.0]);
	out.push([c[0],c[1]-r]);out.push([c[0]-r/5.0,c[1]-r/5.0]);
	out.push([c[0]-r,c[1]]);out.push([c[0]-r/5.0,c[1]+r/5.0]);
	out.push([c[0],c[1]+r]);
	out
}

//Sets the lightning strike upon casting a lightning spell.
fn set_lightning_marker(){}
pub fn set_lightning(ids: &mut Ids, ref mut ui: &mut conrod::UiCell,
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
pub fn set_fire(ids: &mut Ids, ref mut ui: &mut conrod::UiCell,
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
pub fn set_inferno(ids: &mut Ids, ref mut ui: &mut conrod::UiCell,
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
				
				thing.rel_x = sbi.tracks[i][0]+(i%4-sbi.turns_after%5) as f64;
				thing.rel_y = sbi.tracks[i][1]-(i%3+sbi.turns_after%5) as f64;
				let size = 10.0+(sbi.turns_after%5+i%7) as f64;
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
				
				thing.rel_x = sbi.tracks[i][0]+(i%4-sbi.turns_after2%5) as f64;
				thing.rel_y = sbi.tracks[i][1]-(i%3+sbi.turns_after2%5) as f64;
				let size = 10.0+(sbi.turns_after2%5+i%8) as f64;
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
				
				thing.rel_x = sprite_pos[sbi.targets[i]][0]+(i%4-sbi.stage_four%5) as f64;
				thing.rel_y = sprite_pos[sbi.targets[i]][1]-(i%3+sbi.stage_four%5) as f64;
				let size = 40.0+5.0*((sbi.stage_four%FPSU+i%4) as f64);
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
pub fn set_ice(ids: &mut Ids, ref mut ui: &mut conrod::UiCell,
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
pub fn set_death(ids: &mut Ids, ref mut ui: &mut conrod::UiCell,
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
pub fn set_heal(ids: &mut Ids, ref mut ui: &mut conrod::UiCell,
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
pub fn set_time(ids: &mut Ids, ref mut ui: &mut conrod::UiCell,
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
pub fn set_holy(ids: &mut Ids, ref mut ui: &mut conrod::UiCell,
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
pub fn set_radiant(ids:&mut Ids, ref mut ui:&mut conrod::UiCell,
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
pub fn spell_setter(ids: &mut Ids, ref mut ui: &mut conrod::UiCell,
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

///Basic functions.
//luminosity generation based on frame.
pub fn sync_t(timer:usize)->f32 {((timer%30) as f32)/120.0+0.25}

//relative size generation based on frame.
pub fn sync_s(timer:usize)->f64 {((timer%30) as f64)/60.0+0.5}

//avator positional oscillation based on timer.
pub fn shake_pos_b(timer:usize,shake_timer:usize,shake:bool)->conrod::Scalar {
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
pub fn shake_pos_a(timer:usize,shake_timer:usize,shake:bool)->conrod::Scalar {
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


//parabolic sin and cos function for quick and dirty graphical trig.
//Very dirty and inaccurate.
pub fn cosp(time:usize,base:usize)->f64 {
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
pub fn sinp(time:usize,base:usize)->f64 {
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



//parabolic sin and cos function for quick and dirty graphical trig.
//Very dirty and inaccurate.
pub fn cospt(angle:f64,base:f64)->f64 {
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
pub fn sinpt(angle:f64,base:f64)->f64 {
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
