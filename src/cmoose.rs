
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
/// CastT(SpellBoxT)
/// CastInferno(SpellBoxInferno)
/// AttacK(SpriteBox)

///
/// GUIBox: Now implemented but not still somewhat experimental.
/// Substates (ie mutm_box states) should be covered under
/// MainNew and MainLoad
/// GUIBox<'a> {
///		Uninitiated,
///		Main(bool),
///		MainNew((usize,bool)),
///		MainLoad((usize,bool)),
///		MainOptions(bool),
///		MainQuit(bool),
///		GameTravel,
///		GameTravelTeleport,
///		GameFight(bool),
///		GameExplore,
///		GameCastPre,
///		GameCastCast(Spell),
///		GameCastSage(Sage<'a>,u8),
///		GameStory(Story<'a>,u16,u16),
///		GameInspectParty(bool),
///		GameInspectInventory(Option<usize>),
///		GameInspectQuests(Option<u32>),
///		GameInspectDungeons(Option<u32>),
/// }
///
/// NB flow control structures for story elements are kept in smoose
///
///~Alek Zholobenko


extern crate conrod;
extern crate std;

//use std::collections::BTreeMap;

use lmoose::{Lifeform,Spell,Place,VOID,TIME};
use smoose::{Sage,MyStories,MyDungeons,KillList,Story,story_poller};
//use moose_button::MooseButton;
//use moose_matrix::MooseElement;
use gmoose;

//A vector-like structure for carrying image ids for landscape features.
pub struct Landscapes {
	pub city: Vec<(conrod::image::Id,[conrod::Scalar;2])>,
	pub desert: Vec<(conrod::image::Id,[conrod::Scalar;2])>,
	pub forest: Vec<(conrod::image::Id,[conrod::Scalar;2])>,
	pub grassland: Vec<(conrod::image::Id,[conrod::Scalar;2])>,
	pub highland: Vec<(conrod::image::Id,[conrod::Scalar;2])>,
	pub moorland: Vec<(conrod::image::Id,[conrod::Scalar;2])>,
	pub ruin: Vec<(conrod::image::Id,[conrod::Scalar;2])>,
	pub steppe: Vec<(conrod::image::Id,[conrod::Scalar;2])>,
	pub tundra: Vec<(conrod::image::Id,[conrod::Scalar;2])>,
	pub void: Vec<(conrod::image::Id,[conrod::Scalar;2])>,
	pub water: Vec<(conrod::image::Id,[conrod::Scalar;2])>,
	pub ice: Vec<(conrod::image::Id,[conrod::Scalar;2])>,
}

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
	pub turns_after2:usize,
	pub stage_four:usize,
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

//Structure for drawing time spells.
#[derive(Debug)]
pub struct SpellBoxT {
	pub caster_indx: usize,
	pub targets: Vec<usize>,
	pub turns_to_go: usize,
	pub light: bool,
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

//Structure to cycle through a certain predefined subset of widgets.
#[derive(Debug)]
pub struct Widgetcycler<'a> {
	pub widgets: Vec<conrod::widget::Id>,
	pub current_index: usize, //NB, should never be less than zero.s
	pub guibox_state: GUIBox<'a>,
}


//Structure to cycle through all widgets, and keeptrack of whether...
//...They are set, or have multiple components.
#[derive(Debug,Clone)]
pub struct AdvWidgetCycler {
	//Map of widgets (get by index,set?)
	//Polymorphic widget marking is to be handled by a different process.
	pub widgets:std::collections::btree_map::BTreeMap<conrod::widget::Id,(bool,MooseWidgetType)>,
	//NB, we need to keep track of whether the current widget is set or not,
	//To determine whether it needs removed or not.
	pub current_widget: Option<(bool,conrod::widget::Id,MooseWidgetType)>,
}

#[derive(Debug,Clone,Copy,PartialEq)]
pub enum MooseWidgetType {
	MooseButton,
	MooseElement,
	Other,
}

impl MooseWidgetType {
	pub fn is_relevant(self) -> bool {
		self != MooseWidgetType::Other
	}
}

impl AdvWidgetCycler {
	//Create new widget cycler
	pub fn new()->AdvWidgetCycler {
		AdvWidgetCycler {
			widgets: std::collections::btree_map::BTreeMap::new(),
			current_widget: None,
		}
	}

	//Initialise the advanced cycler with widgets to insert.
	//I could not find how Graph/Dag knows the widget type,
	//So this will handle some aspect of this.
	//NB, this to some extent reduces flexibility as you are declaring,
	//Some aspect of widget type in advance (not that conrod allows otherwise anyway).
	pub fn initialise(&mut self, ids:Vec<(conrod::widget::Id,MooseWidgetType)>) {
		for (x,y) in ids.into_iter() {
			self.widgets.insert(x,(false,y));
		};

		'finder: for (i,(x,ty)) in self.widgets.iter() {
			if *x {
				self.current_widget = Some((*x,*i,*ty));
				break 'finder;
			};
		};
	}

	//sets current_widget to the next set widget.
	pub fn advance(&mut self) {
		let len = self.widgets.len();
		let mut it = self.widgets.iter().cycle();
		let (mut not_at_current,mut got_next) = (true,false);

		if self.current_widget.is_none() {
			while let Some((i,(x,ty))) = it.next() {
				if *x & ty.is_relevant() {
					self.current_widget = Some((*x,*i,*ty));
					return;
				};
			};
		}else{
			let mut c = 0;
			let (bool,index,ty) =self.current_widget.expect("It's nonnullic.");
			while (it.next()!=Some((&index,&(bool,ty)))) & (c<len) {};
			println!("Current widget is {:?} upon advancement.",self.current_widget);

			c = 0;
			while len>=c {
				c+= 1;
				let i;
				let x;
				let ty;
				if let Some((j,(y,tz))) = it.next() {
					i = j;
					x = y;
					ty = tz;
				}else{
					continue;
				};
				if let Some((bool,index,_)) = self.current_widget  {
					not_at_current = index!=*i;
				};

				if !not_at_current {
					continue;
				}else if not_at_current & *x & ty.is_relevant() {
					self.current_widget = Some((true,*i,*ty));
					return;
				};
			};
			self.current_widget = None;
			println!("Resetting to spare upon advancement.");
		};
	}

	//sets current_widget to the previous set widget.
	pub fn regress(&mut self) {
		let len = self.widgets.len();
		let mut it = self.widgets.iter().rev().cycle();

		if self.current_widget.is_none() {
			println!("Current widget is None upon regression.");
			while let Some((i,(x,ty))) = it.next() {
				if *x & ty.is_relevant() {
					self.current_widget = Some((*x,*i,*ty));
					println!("Returning with: {:?}",self.current_widget);
					return;
				};
			};
		}else{
			let mut c = 0;
			let (bool,index,ty) =self.current_widget.expect("It's nonnullic.");
			while (it.next()!=Some((&index,&(bool,ty)))) & (c<len) {};
			println!("Current widget is {:?} upon regression.",self.current_widget);

			c = 0;
			while len>=c {
				c+= 1;
				let i;
				let x;
				let ty;
				if let Some((j,(y,tz))) =it.next() {
					println!("next is: {:?}",Some((j,(y,tz))));
					i = j;
					x = y;
					ty = tz;
				}else{
					println!("Next is none.");
					continue;
				};

				if index == *i {
					continue;
				}else if *x & ty.is_relevant() {
					self.current_widget = Some((true,*i,*ty));
					println!("Returning None upon regression.");
					return;
				};
			};
			println!("Returning spare upon regression.");
			self.current_widget = None;
		};
	}

	//Gets the current widget index or failing that, the next set widget.
	//If there are no set widgets, return a nun.
	pub fn current_or(&mut self)-> Option<conrod::widget::Id> {

		if let Some((true,wid,_)) = self.current_widget {
			//If there is an current widget, get it.
			return Some(wid)
		}else{
			//If not, get the first active widget.
			for (i,x) in self.widgets.iter() {
				if x.0 {
					self.current_widget = Some((true,*i,x.1));
					return Some(*i)
				};
			};
		};
		None
	}

	//A faster version of current_or() if we do not care about other widgets.
	pub fn current(&mut self)->Option<conrod::widget::Id> {
		if let Some((true,wid,_)) = self.current_widget {
			//If there is an current widget, get it.
			return Some(wid)
		}else{
			None
		}
	}

	//Function to mark a widget as set. Not efficient.
	pub fn mark_as_set(&mut self,widget: conrod::widget::Id, r#type: MooseWidgetType) {

		if let Some((ref mut x, ref mut i,_)) = self.current_widget {
			if *i==widget {
				*x = true;
			};
		};
		if let Some(mut x) = self.widgets.get_mut(&widget) {
			x.0 = true;
			return;
		};
		self.widgets.insert(widget,(true,r#type));
	}

	//Function to mark all widgets as not set.
	pub fn mark_all_as_unset(&mut self) {
		if let Some((true,wid,ty)) = self.current_widget.clone() {
			self.current_widget = Some((false,wid,ty));
		};
		for (_,x) in self.widgets.iter_mut() {
			x.0 = false;
		};
	}

	//Function to mark all widgets as not set in a violent manner.
	pub fn hard_unset(&mut self) {
		if let Some((true,wid,ty)) = self.current_widget.clone() {
			self.current_widget = Some((false,wid,ty));
		};
		self.widgets.clear();
	}

	//Removes all widgets.
	pub fn remove_all(&mut self) {
		self.widgets.clear();
		self.current_widget = None;
	}
	//NB the set outer function is best done from
	// moose_button.Go there to see it done.

}

impl <'a>Widgetcycler<'a> {

	//get length of widget vecotr in the widget cycler.
	fn len(&self)-> usize { self.widgets.len() }

	//Advance to the next widget in the cycle.
	pub fn advance(&mut self) {
		let l = self.len();

		if l==0 {return;}; //futile, I know.

		if self.current_index+1<l {
			self.current_index+= 1;
		}else{
			self.current_index = 0;
		};
	}

	//Advance to the next widget in the cycle.
	pub fn regress(&mut self) {
		let l = self.len();

		if l==0 {return;};

		if self.current_index==0 {
			self.current_index = l;
		}else{
			self.current_index-= 1;
		};
	}

	//Get the next widget in the cycle.
	pub fn next(&mut self)-> Option<&conrod::widget::Id> {
		let l = self.len();

		if l==0 {return None};

		if self.current_index+1<l {
			self.current_index+= 1;
		}else{
			self.current_index = 0;
		};

		Some(&self.widgets[self.current_index])
	}

	//Get the previous widget in the cycle.
	pub fn previous(&mut self)-> Option<&conrod::widget::Id> {
		let l = self.len();
		if l==0 {return None};
		if self.current_index==0 {
			self.current_index = l;
		}else{
			self.current_index-= 1;
		};

		Some(&self.widgets[self.current_index])
	}

	//Get the current widget in the cycle.
	pub fn current(&self)-> Option<&conrod::widget::Id> {
		if (self.len()>0) & (self.current_index<self.len()) {
			Some(&self.widgets[self.current_index])
		}else{
			None
		}
	}

	//Get a widget at a certain index.
	pub fn get(&mut self,which:usize)-> Option<&conrod::widget::Id> {

		if which>=self.len() {
			//Bounds check.
			None
		}else{
			//If bounds check is ok, get the widget.
			self.current_index = which;
			Some(&self.widgets[self.current_index])
		}
	}

	//function to tell you whether the widgetcycler contains a certain widget,
	//...and what its index is.
	pub fn contains(&mut self,widget:&conrod::widget::Id)-> Option<usize> {

		for (i,x) in self.widgets.iter().enumerate() {
			if x==widget {return Some(i)};
		}
		None
	}


	//function to generate list of cyclable widgets and to sanity check them.
	//Realistically will have this
	//NB, does not check if widgets are active. Use with care.
	pub fn new()-> Widgetcycler<'a> {

		Widgetcycler {
			widgets: Vec::with_capacity(100),
			current_index:0,
			guibox_state:GUIBox::Uninitiated,
		}
	}

	//Function to update the widget cycler when guibox state chanes or widget number changes.
	pub fn update_wc(&mut self,widgets_to_cycle:Vec<conrod::widget::Id>,gbs:&GUIBox<'a>) {

		if (*gbs != self.guibox_state) | (self.widgets.len()!=widgets_to_cycle.len()) {
			self.current_index = 0;
		};

		self.widgets = widgets_to_cycle;
		self.guibox_state = gbs.clone();
	}
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
			turns_to_go: (gmoose::FPS*3.0) as usize,
			turns_init: (gmoose::FPS*3.0) as f64,
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

		let turns_to_last = if caster.0.BM_shade/gmoose::FPS>2.0 {gmoose::FPS*2.0}else{caster.0.BM_shade};

		SpellBoxD {
			caster_indx: a_i,
			targets: fin_targets,
			turns_to_go: turns_to_last as usize,
			turns_init: turns_to_last as f64,
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
			turns_to_go: (gmoose::FPS as f64*3.0*caster.0.BM_shade as f64/100.0 as f64) as usize,
			turns_init: gmoose::FPS as f64*3.0*caster.0.BM_shade as f64/100.0 as f64,
			damage: damage.clone(),
		}
	}
}

impl SpellBoxT {
	pub fn new (caster: &(Lifeform,usize,[Option<[usize;2]>;2]),
			   a_i: usize,
			   targets: &Vec<usize>,
			   positions: &[[f64;2];25],
			   damage: [bool;25],
			   spell_light:bool)->SpellBoxT {

		let mut fin_targets:Vec<usize> = Vec::with_capacity(25);
		for x in targets.iter() {fin_targets.push(*x);}
		SpellBoxT {
			caster_indx: a_i,
			targets: fin_targets,
			turns_to_go: (gmoose::FPS*3.0) as usize,
			turns_init: (gmoose::FPS*3.0) as f64,
			light: spell_light,
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
			tracks.push(positions[*x]);
			paths.push(vec![positions[a_i]]);
		}

		let timing:usize = (gmoose::FPS as f32*1.5) as usize;

		SpellBoxInferno {
			caster_indx: a_i,
			targets: fin_targets,
			turns_to_go: timing,
			turns_after: timing,
			turns_after2: timing,
			stage_four: 0,
			turns_init: gmoose::FPS as f64*1.5,
			tracks: tracks, //for balls
			paths:paths,	//for lines
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
	CastT(SpellBoxT),
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

#[derive(Clone,Debug,PartialEq)]
pub enum GUIBox<'a> {
	Uninitiated,
	ShowControls,
	Main(bool),
	MainNew((usize,bool)),
	MainLoad((usize,bool)),
	MainOptions(bool),
	MainQuit(bool),
	GameTravel,
	GameTravelTeleport,
	GameFight(bool),
	GameExplore,
	GameCastPre,
	GameCastCast(Spell),
	GameCastSage(Sage<'a>,u8),
	 //u16 first is entry node.u16 2nd is exit node of first part.
	 //GS.1 can change, GS.2 should stay constant.
	GameStory(Story<'a>,u16,u16),
	GameInspectParty(bool),
	GameInspectInventory(Option<usize>), 	//Opens party inventory
	GameInspectQuests(Option<u32>),			//Opens quest records.
	GameInspectDungeons(Option<u32>),		//Opens dungeons records.
}

impl <'a>GUIBox<'a> {
	pub fn is_fight(&self)->bool {
		match self {
			GUIBox::GameFight(x) => true,
			_				     => false,
		}
	}

	pub fn is_travel(&self)->bool {
		match self {
			GUIBox::GameTravel	 => true,
			_				     => false,
		}
	}

	pub fn is_sage_sage(&self)->bool {
		match self {
			GUIBox::GameCastSage(_,_) => true,
			_				    	=> false,
		}
	}

	//A heavy function to poll stories if travelling.
	pub fn check_for_story(&mut self,stories:&Vec<Story<'a>>,
									 my_stories:&mut MyStories,
									 my_dungeons:&mut MyDungeons,
									 my_kills:&mut KillList,
									 landscapes: &Landscapes,
									 p_loc:&Place,
									 party:&Vec<(Lifeform,usize)>,
									 centre_w:&mut f64,
									 centre_h:&mut f64,
									 scenery_index:&mut usize,
									 timer:usize) {

		if self.is_travel() & (timer%20==0) {
			//println!("Polling stories");
			//Poll stories for whether triggers for start/end dialog are tipped.
			let maybe_story:Option<(usize,u16)> = story_poller(stories,my_stories,my_dungeons,my_kills,p_loc,party);

			if maybe_story.is_some() {
					println!("We have a story");
					println!("We have a story.ID={}",stories[maybe_story.unwrap().0].id);
				//if trigger is tipped load story into gui box.
				*self = GUIBox::GameStory(
					stories[maybe_story.unwrap().0].clone(),
					maybe_story.unwrap().1, //This could go wrong on the way into the function.
					maybe_story.unwrap().1 //This should stay constant.
				);

				//if my stories does not contain it, add entry to my stories.
				if !my_stories.poll_ids_only(stories[maybe_story.unwrap().0].id) {
					println!("Adding story to my story");
					my_stories.push((stories[maybe_story.unwrap().0].id,0,0));
				};

				//Set scenery if needed.
				if (p_loc.scape != VOID) & (p_loc.scape != TIME) {
					*scenery_index = gmoose::scenery_setter(&landscapes,p_loc.scape,centre_w,centre_h);
				};
			};
		};
	}

}
