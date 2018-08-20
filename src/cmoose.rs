
///GTWM: cmoose
/// A module containing flow control structures.
/// Contains basic flow control structures which are not
/// generally used (except FlowCWin)
/// Also contains the GraphicsBox enum which determines which
/// SFX is to be set. Can carry the types:
/// GraphicsBox::None,
/// CastD(SpellBoxD)
/// CastF(SpellBoxF)
/// CastH(SpellBoxH)
/// CastI(SpellBoxI)
/// CastL(SpellBoxL)
/// CastR(SpellBoxR)
/// CastS(SpellBoxS)
/// CastInferno(SpellBoxInferno)
/// AttacK(SpriteBox)
///
///
///~Alek Zholobenko


use lmoose::{Lifeform};
use gmoose;

extern crate std;

// NB: Some of these structures are currently the same internally.
// But in order to be able to change later, they're kept the same.
//Structure for drawing lightning spells.
#[derive(Debug)]
pub struct SpellBoxL {
	pub caster_indx: usize,
	pub targets: Vec<usize>,
	pub turns_to_go: usize,		//lightning will be faster for higher BM.
	pub turns_after:usize,     //useful.
	pub turns_init: f64,
	pub paths: Vec<Vec<[f64;2]>>,  //records each point on the lightning path.
	pub damage: [bool;25],
}

//Structure for drawing ice spells.
#[derive(Debug)]
pub struct SpellBoxI {
	pub caster_indx: usize,
	pub targets: Vec<usize>,
	pub turns_to_go: usize,		//lightning will be faster for higher BM.
	pub turns_after:usize,
	pub turns_init: f64,
	pub tracks: Vec<[f64;2]>,
	pub damage: [bool;25],
}

//Structure for drawing fire spells.
#[derive(Debug)]
pub struct SpellBoxF {
	pub caster_indx: usize,
	pub targets: Vec<usize>,
	pub turns_to_go: usize,		//lightning will be faster for higher BM.
	pub turns_after:usize,     //useful.
	pub turns_init: f64,
	pub tracks: Vec<[f64;2]>,  //records each point on the lightning path.
	pub damage: [bool;25],
}

//Structure for drawing a certain spell.
#[derive(Debug)]
pub struct SpellBoxInferno {
	pub caster_indx: usize,
	pub targets: Vec<usize>,
	pub turns_to_go: usize,		//lightning will be faster for higher BM.
	pub turns_after:usize,     //useful.
	pub stage_three:usize,
	pub turns_init: f64,
	pub tracks: Vec<[f64;2]>,  //records each point on the lightning path.
	pub paths: Vec<Vec<[f64;2]>>,
	pub damage: [bool;25],
}

//Structure for drawing healing spells.
#[derive(Debug)]
pub struct SpellBoxH {
	pub caster_indx: usize,
	pub targets: Vec<usize>,
	pub turns_to_go: usize,		//lightning will be faster for higher BM.
	pub turns_init: f64,
	pub damage: [bool;25],
}

//Structure for drawing sacred spells.
#[derive(Debug)]
pub struct SpellBoxS {
	pub caster_indx: usize,
	pub targets: Vec<usize>,
	pub turns_to_go: usize,		//lightning will be faster for higher BM.
	pub turns_init: f64,
	pub damage: [bool;25],
}

//Structure for drawing radiant spells.
#[derive(Debug)]
pub struct SpellBoxR {
	pub caster_indx: usize,
	pub targets: Vec<usize>,
	pub turns_to_go: usize,		//lightning will be faster for higher BM.
	pub turns_init: f64,
	pub light: bool,
	pub lightness: f32,
	pub damage: [bool;25],
	pub paths: Vec<Vec<[f64;2]>>,
	pub destinations: Vec<[f64;2]>,
}

//Structure for drawing darkness spells.
#[derive(Debug)]
pub struct SpellBoxD {
	pub caster_indx: usize,
	pub targets: Vec<usize>,
	pub turns_to_go: usize,		//lightning will be faster for higher BM.
	pub turns_init: f64,
	pub damage: [bool;25],
}

impl SpellBoxL { //NB, positions from the position structure will be used.
	pub fn new(caster: &(Lifeform,usize,[Option<[usize;2]>;2]),
			   a_i: usize,
			   targets: &Vec<usize>,
			   positions: &[[f64;2];25],
			   damage: [bool;25])->SpellBoxL {
		
		let mut fin_targets:Vec<usize> = Vec::with_capacity(25);
		let mut paths:Vec<Vec<[f64;2]>> = Vec::with_capacity(25);
		for x in targets.iter() {
			fin_targets.push(*x);
			paths.push(vec![positions[a_i]]);
		}
			
		SpellBoxL {
			caster_indx: a_i,
			targets: fin_targets,
			turns_to_go: (gmoose::FPS as f64*2.0*100.0/caster.0.BM_shade as f64) as usize,
			turns_after: 0,
			turns_init: (gmoose::FPS as f64*2.0*100.0/caster.0.BM_shade as f64),
			paths: paths,
			damage: damage.clone(),
		}
	}
}

impl SpellBoxI {
	
	pub fn new(caster: &(Lifeform,usize,[Option<[usize;2]>;2]),
			   a_i: usize,
			   targets: &Vec<usize>,
			   positions: &[[f64;2];25],
			   damage: [bool;25])->SpellBoxI{
		
		let mut fin_targets:Vec<usize> = Vec::with_capacity(25);
		let mut paths:Vec<[f64;2]> = Vec::with_capacity(25);
		for x in targets.iter() {
			fin_targets.push(*x);
			paths.push(positions[a_i]);
		}
		
		SpellBoxI {
			caster_indx: a_i,
			targets: fin_targets,
			turns_to_go: (gmoose::FPS as f64*2.0*100.0/caster.0.BM_shade as f64) as usize,
			turns_after: 0,
			turns_init: gmoose::FPS as f64*2.0*100.0/caster.0.BM_shade as f64,
			tracks: paths,
			damage: damage.clone(),
		}
	}
}

impl SpellBoxF {
	
	pub fn new(caster: &(Lifeform,usize,[Option<[usize;2]>;2]),
			   a_i: usize,
			   targets: &Vec<usize>,
			   positions: &[[f64;2];25],
			   damage: [bool;25])->SpellBoxF {
		
		let mut fin_targets:Vec<usize> = Vec::with_capacity(25);
		let mut paths:Vec<[f64;2]> = Vec::with_capacity(25);
		for x in targets.iter() {
			fin_targets.push(*x);
			paths.push(positions[a_i]);
		}
			
		SpellBoxF {
			caster_indx: a_i,
			targets: fin_targets,
			turns_to_go: (gmoose::FPS as f64*1.0*100.0/caster.0.BM_shade as f64) as usize,
			turns_after: 0,
			turns_init: (gmoose::FPS as f64*1.0*100.0/caster.0.BM_shade as f64),
			tracks: paths,
			damage: damage.clone(),
		}
	}
}

impl SpellBoxH {
	
	pub fn new(caster: &(Lifeform,usize,[Option<[usize;2]>;2]),
			   a_i: usize,
			   targets: &Vec<usize>,
			   positions: &[[f64;2];25],
			   damage: [bool;25])->SpellBoxH{
		
		let mut fin_targets:Vec<usize> = Vec::with_capacity(25);
		for x in targets.iter() {fin_targets.push(*x);}
		
		SpellBoxH {
			caster_indx: a_i,
			targets: fin_targets,
			turns_to_go: (gmoose::FPS as f64*2.0*100.0/caster.0.WM_shade as f64) as usize,
			turns_init: gmoose::FPS as f64*2.0*100.0/caster.0.WM_shade as f64,
			damage: damage.clone(),
		}
	}
}

impl SpellBoxD {
	
	pub fn new(caster: &(Lifeform,usize,[Option<[usize;2]>;2]),
			   a_i: usize,
			   targets: &Vec<usize>,
			   positions: &[[f64;2];25],
			   damage: [bool;25])->SpellBoxD{
		
		let mut fin_targets:Vec<usize> = Vec::with_capacity(25);
		for x in targets.iter() {fin_targets.push(*x);}
		
		SpellBoxD {
			caster_indx: a_i,
			targets: fin_targets,
			turns_to_go: (gmoose::FPS as f64*3.0*100.0/caster.0.BM_shade as f64) as usize,
			turns_init: gmoose::FPS as f64*3.0*100.0/caster.0.BM_shade as f64,
			damage: damage.clone(),
		}
	}
}

impl SpellBoxS {
	
	pub fn new(caster: &(Lifeform,usize,[Option<[usize;2]>;2]),
			   a_i: usize,
			   targets: &Vec<usize>,
			   positions: &[[f64;2];25],
			   damage: [bool;25])->SpellBoxS{
		
		let mut fin_targets:Vec<usize> = Vec::with_capacity(25);
		for x in targets.iter() {fin_targets.push(*x);}
		
		SpellBoxS {
			caster_indx: a_i,
			targets: fin_targets,
			turns_to_go: (gmoose::FPS as f64*3.0*100.0/caster.0.BM_shade as f64) as usize,
			turns_init: gmoose::FPS as f64*3.0*100.0/caster.0.BM_shade as f64,
			damage: damage.clone(),
		}
	}	
}

impl SpellBoxR {
	//need to change a few things here.
	pub fn new(caster: &(Lifeform,usize,[Option<[usize;2]>;2]),
			   a_i: usize,
			   targets: &Vec<usize>,
			   positions: &[[f64;2];25],
			   light:bool,
			   l_change:i32,
			   damage: [bool;25])->SpellBoxR{
		
		let mut destinations:Vec<[f64;2]> = Vec::with_capacity(125);
		let mut paths:Vec<Vec<[f64;2]>> = Vec::with_capacity(125);
		let mut fin_targets:Vec<usize> = Vec::with_capacity(25);
		
		//need to change this.
		for x in positions.iter() {
			if *x != [0.0;2] {
				for _ in 0..5 {paths.push(vec![positions[a_i]]);}
				destinations.push([x[0],x[1]]);
				destinations.push([x[0]*0.95,x[1]*0.95]);
				destinations.push([x[0]*1.05,x[1]*1.05]);
				destinations.push([x[0]*1.05,x[1]*0.95]);
				destinations.push([x[0]*0.95,x[1]*1.05]);
			};
		}
		for x in targets.iter() {fin_targets.push(*x);}
		
		SpellBoxR {
			caster_indx: a_i,
			targets: fin_targets,
			turns_to_go: (gmoose::FPS*3.0) as usize,
			turns_init: (gmoose::FPS*3.0) as f64,
			light: light,
			lightness: l_change as f32/255.0,
			damage: damage.clone(),
			paths: paths,
			destinations: destinations,
		}
	}
	
}

impl SpellBoxInferno {
	
	pub fn new(caster: &(Lifeform,usize,[Option<[usize;2]>;2]),
			   a_i: usize,
			   targets: &Vec<usize>,
			   positions: &[[f64;2];25],
			   damage: [bool;25])->SpellBoxInferno {
		
		let mut fin_targets:Vec<usize> = Vec::with_capacity(25);
		let mut tracks:Vec<[f64;2]> = Vec::with_capacity(25);
		let mut paths:Vec<Vec<[f64;2]>> = Vec::with_capacity(25);
		for x in targets.iter() {
			fin_targets.push(*x);
			tracks.push(positions[a_i]);
			paths.push(vec![positions[*x]]);
		}
		
		let timing:f64 = gmoose::FPS as f64*1.0*100.0/caster.0.BM_shade as f64;
			
		SpellBoxInferno {
			caster_indx: a_i,
			targets: fin_targets,
			turns_to_go: timing as usize,
			turns_after: timing as usize,
			stage_three: 0,
			turns_init: (gmoose::FPS as f64*1.0*100.0/caster.0.BM_shade as f64),
			tracks: tracks,
			paths:paths,
			damage: damage.clone(),
		}	
	}
}

//Structure for instructions for moving sprites
//following attacks.
#[derive(Debug)] 
pub struct SpriteBox {
	pub att_index: usize,
	pub def_index: usize,
	pub turns_to_go: usize,			// Sprite must reach destination
									// thus this must be dx/speed.
	pub turns_init: f64,
	pub def_coord: [f64;2],			// these are initial coordinates
	pub att_coord: [f64;2],			// initial.			
	pub damage: bool,				// did the attack to damage? Do sprites need to vibrate?
}

impl SpriteBox {
	
	//put a new SpriteBox into the Option<SpriteBox>
	//NB it still needs to be initialised with the coordinates of the sprites.
	pub fn new( timer:usize,
				attacker:&(Lifeform,usize,[Option<[usize;2]>;2]),
				a_i:usize,
				defender:&(Lifeform,usize,[Option<[usize;2]>;2]),
				d_i:usize,
				attacker_pos:&[f64;2],
				defender_pos:&[f64;2],
				damage:bool)->SpriteBox {
		
		SpriteBox {
			att_index: a_i,
			def_index: d_i,
			//by default 0.5 seconds (40 frames) approach at speed of 50.0
			turns_to_go: (gmoose::FPS as f64*0.5*50.0/attacker.0.Speed_shade as f64) as usize,
			turns_init: (gmoose::FPS as f64*0.5*50.0/attacker.0.Speed_shade as f64),
			def_coord: [defender_pos[0],defender_pos[1]],
			att_coord: [attacker_pos[0],attacker_pos[1]],
			damage: damage,
		}
	}
}
#[derive(Debug)]
pub enum GraphicsBox {
	Attack(SpriteBox),
	CastL(SpellBoxL),
	CastI(SpellBoxI),
	CastF(SpellBoxF),
	CastH(SpellBoxH),
	CastD(SpellBoxD),
	CastS(SpellBoxS),
	CastR(SpellBoxR),
	CastInferno(SpellBoxInferno),
	None,
}

//Act like an option!
impl GraphicsBox {
	pub fn is_some(&self)->bool {
		match self {
			&GraphicsBox::None => false,
			_	 => true,
		}
	}
	pub fn is_none(&self)->bool {
		match self {
			&GraphicsBox::None => true,
			_	 => false,
		}
	}
}
// Flow control structures. (TODO - also will be reworked several times).
// Flow control structure for options
// (eg Brightness variables, playlist, mute).	
#[derive(Debug)] 
pub struct FlowCWin {
	pub update_bgc: bool,
	pub silence: bool,
	pub ifc:f32,
	pub bgc:f32,
	pub ai_mem:usize,
	pub song_to_swap: Option<String>,
	pub new_selection: Option<String>,
	pub mub_path: std::path::PathBuf,	
}

impl FlowCWin {
	pub fn new()->FlowCWin {
		FlowCWin {
			update_bgc: false,
			silence: false,
			ifc:0.0,
			bgc:0.0,
			ai_mem:500_000_000,
			song_to_swap: None,
			new_selection: None,
			mub_path: std::path::PathBuf::new(),		
		}
	}
}

// Global flow control options
// (n_s_l_q_f, tt_e_c_i_ll, hero_chosen (etc) and various timers).
#[derive(Debug)]
pub struct FlowCGlo {
	pub mut_mb_vis: bool,
	pub started: bool,
	pub dream_time: bool,
	pub nslqf: [bool;7],
	pub ttecill: [bool;8],
	pub chosen_h: usize,
	pub stage: usize,
	pub timer: usize,
	pub frz_timer: usize,
	
}

// Story related flow control
// (Note decided <- will need to rework loader and saver)
#[derive(Debug)]
pub struct FlowCSto {}

// (yt_adcwpe_bw) (for now holding off on this).
#[derive(Debug)]
pub struct FlowCBat {
	pub your_turn: bool,
	pub attack: bool,
	pub defend: bool,
	pub cast: bool,
	pub wait: bool,
	pub panic: bool,
	pub escape: bool,
	pub black: bool,
	pub white: bool,
}
		 