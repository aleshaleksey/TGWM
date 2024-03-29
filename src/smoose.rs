#[allow(unreachable_code)]
#[allow(unused_mut)]
///TGWM: module smoose
///
///The smoose module handles NPCs.
///For now it a clone of the functions
///from the terminal version.
///A placeholder.
///
///About Sage structures:
///
///In the original Moosequest, sages were functions which sold spells
///and provided tidbits of world related information. Here and now
///sages will perform the same function, but as a structure which is queried
///by a function. The advantage is that this allows easy looping in combination
///with set mutant menu. Thus all the information is in the sage, ad the query function
///simply chooses which information to display based on player interaction.
///The basic Sage archetype can be adapted for any NPC.
///
///About Story structures:
///
///Story consists of several parts. Importantly it has triggers for start,
///and triggers for end. Some content and a concluson. While the triggers 
///can be quite complex, the content is linear, thus a branching quest will
///consist of several stories chained together with each branch having
///slightly different triggers. Importantly, it has an ids field for saving
///game plot-line progress in a simple manner- thus a story can be not-started,
///started or finished. There can be no degree of completion. Actual plotlines
///with several stories can be build only by chaining Stories together.
///NB: Stories themselves are to be stored elsewhere. Or this module will be too big.
///
///About MyStories.
///
///This is a structure with a single field- a vector of (u32,u16,u16), ie, story
///ids and start dialog exit node and end dialog exit node. 0 implies not done. For each stage.
///
///About MyDungeons.
///
///Analogous to MyStories.
///This is a structure with a single field- a vector of ([i32,i32],u32,u32),
///the coords of each dungeon and the number of times tried and completed.
///sensibly seperate from MyStories for modularity.
///
///About Triggers.
///
///Triggers is an enum which runs to check for certain criteria. If it is met,
///A Story or Sage can be triggered or a Story can be marked as concluded.
///
///About Content.
///
///Content is the fluff of a story. It is the combination of dialog and graphic
///scene that accompanies it. I have not yet decided how this should be handled.
///But most likely will have a series of Questions and answers, possibly in a tree structure,
///and a list of image graphics.
///
///Aleksey Zholobenko.

//mod lmoose;

extern crate conrod;

use std::collections::BTreeMap;
use std::collections::btree_map;
use std::slice;
use std::option;

#[allow(unused_imports)] use lmoose::{Spell,Item,Lifeform,Shade,Place,Dungeon,cureL,cure,cureG,cureH,exorcism,exorcismG,exorcismH,
			 ember,fire,fireball,inferno,spark,lightning,lightningH,crystalliseL,crystallise,crystalliseH,
			 sum_reaper,teleport,teleportG,light,lightH,darkness,darknessH,slow,haste,lifestealer,curse,
			 apocalypse,timestop,world,goblin_dem,goblin_sco,goblin_witch,bandit,bandit_lord,dark_apprentice,
			 necromancer,necromancer_lord,skeleton,skeleton_kn,ghost,ghost_an,white_witch,beast_green,
			 beast_red,beast_great,fallen,titan,warrior,witch,wonderer,alien,loser,beast_serpent,sage_forsaken,
			 white_queen,shortstaff};
			 
#[allow(unused_imports)] use lmoose::{ADVENT,ALBION,ALIEN,ANGEL,BEAST,BONE,BRIDGE,CITY,
		     DEATH,DESERT,ELF,EVIL,FALLEN,FIRE,FOREST,GIANT,GOBLIN,GRASSLAND,
		     HEALING,HIGHLAND,HOLY,HUMAN,ICE,LIGHTNING,MALACHIA,
			 MINDLESS,MOORLAND,MOOSE,RADIANT,RUIN,STEPPE,SPIRIT,
			 TELEPORTATION,TIME,TUNDRA,UNDEAD,VOID,WATER,WITCH,WHITE,NONE,
			 
			 S_LESSER_CURE,S_CURE,S_GREATER_CURE,S_SACRED_CURE,S_INFERNO,S_FIREBALL,S_FIRE,S_EMBER,
			 S_LESSER_CRYSTALLISE,S_CRYSTALLISE,S_TRUE_CRYSTALLISE,S_EXORCISM,S_GREATER_EXORCISM,S_SACRED_EXORCISM,
			 S_SUMMON_REAPER,S_TELEPORT,S_GREATER_TELEPORT,S_LIGHT,S_SACRED_LIGHT,S_DARKNESS,S_ABYSSAL_DARKNESS,
			 S_SLOW,S_HASTE,S_APOCALYPSE,S_GENESIS,S_SPARK,S_LIGHTNING,S_JOVIAN_LIGHTNING,S_TIMESTOP,
			 S_CURSE,S_LIFESTEALER,S_DAGGER_OF_FAWN,S_BOW_OF_TRAVELLER,S_SWORD_OF_PERSEUS};


//Sage dialog stage constants.
pub const GREETING1:u8 = 93;
pub const GREETING2:u8 = 90;
pub const MAGIC:u8 = 87;
pub const SAGES:u8 = 84;
pub const WORLD:u8 = 81;
pub const TERRAIN:u8 = 78;
pub const GOODBYE:u8 = 75;
pub const SPELL1:u8 = 72;
pub const SPELL2:u8 = 69;
pub const SPELL3:u8 = 66;

//A structure to contain spells, prices therefore
//and fixed answers to questions.
//this structure will change with time		
#[derive(Clone,Debug,PartialEq)] 
pub struct Sage<'a> {
	pub name: String,
	pub exp_min: f32,
	pub face: &'a conrod::image::Id,
	pub trigger: Vec<Trigger>,
	pub spells: Vec<i8>,
	pub dialog_greeting:[String;2],
	pub dialog_magic:[String;2],
	pub dialog_sages:[String;2],
	pub dialog_world:[String;2],
	pub dialog_terrain:[String;2],
	pub dialog_goodbye:[String;2],
}

impl <'a>Sage<'a> {
	pub fn get_first_q1(&self)->[&str;6] {
		[&self.dialog_magic[0],
		 &self.dialog_sages[0],
		 &self.dialog_world[0],
		 &self.dialog_terrain[0],
		 &self.dialog_goodbye[0],
		 &self.dialog_greeting[0]]
	 }
	
	pub fn get_first_q2(&self)->[&str;6] {
		[&self.dialog_magic[0],
		 &self.dialog_sages[0],
		 &self.dialog_world[0],
		 &self.dialog_terrain[0],
		 &self.dialog_goodbye[0],
		 &self.dialog_greeting[1]]
	}
	
	pub fn get_post_magic(&self)->[&str;6] {
		["Maybe not..",
		 "",
		 "",
		 "",
		 "",
		 &self.dialog_magic[1]]
	}
	
	pub fn get_post_sage(&self)->[&str;6] {
		["I want to ask something else.",
		 "",
		 "",
		 "",
		 "...",
		 &self.dialog_sages[1]]
	}
	
	pub fn get_post_world(&self)->[&str;6] {
		["I want to ask something else.",
		 "",
		 "",
		 "",
		 "...",
		 &self.dialog_world[1]]
	}
	
	pub fn get_post_terrain(&self)->[&str;6] {
		["I want to ask something else.",
		 "",
		 "",
		 "",
		 "...",
		 &self.dialog_terrain[1]]
	}
	
	pub fn get_post_goodbye(&self)->[&str;6] {
		["I want to ask something else.",
		 "",
		 "",
		 "",
		 "...",
		 &self.dialog_goodbye[1]]
	}
	
	pub fn get_post_spell(&self)->[&str;6] {
		["Hmm... Maybe not...",
		 "",
		 "",
		 "",
		 "...",
		 &self.dialog_greeting[1]]
	}
}

// A structure to record how many monsters you have slain.
#[derive(Debug)]
pub struct KillList {
	kills:Vec<(String,u64)>,
}

impl KillList {
	pub fn new() -> KillList {
		KillList {
			kills:Vec::with_capacity(50),
		}
	}
	
	pub fn len(&self)->usize {
		self.kills.len()
	}
	
	pub fn push(&mut self,name:&str,kills:usize) {
		self.kills.push((name.to_owned(),kills as u64))
	}
	
	pub fn take_kills(&self)-> Vec<(String,u64)> {
		self.kills.clone()
	}
	
	pub fn replace_kills(&mut self,kills:Vec<(String,u64)>) {
		self.kills = kills; 
	}
	
	//returns true if monster is in kill list.
	pub fn has(&self,name:&str)->bool {
		for x in self.kills.iter() {
			if x.0==name {return true;};
		};
		false
	}
	
	//returns number of kills of said monster.
	pub fn poll(&self,name:&str)->usize {
		for x in self.kills.iter() {
			if x.0==name {return x.1 as usize;};
		};
		0
	}
	
	//incremenrs number of kills of said monster or adds it in at 1.
	pub fn increment_or(&mut self,name:&str) {
		for x in self.kills.iter_mut() {
			if x.0==name {
				x.1+= 1;
				return;
			};
		};
		self.push(name,1);
	}
}

// A structure for storing dungeons.
// Contains ids, tries, successes, and last stage.
#[derive(Debug)]
pub struct MyDungeons {
	ids:BTreeMap<u32,[u32;3]>,
}

impl MyDungeons {
	pub fn new()-> MyDungeons {
		MyDungeons{
			ids: BTreeMap::new(),
		}
	}
	
	pub fn len(&self)->usize {
		self.ids.len()
	}
	
	pub fn iter(&self)->btree_map::Iter<u32,[u32;3]> {
		self.ids.iter()
	}
	
	//Tries to add a new dungeon.
	pub fn try_push(&mut self,id:u32,tries:u32,done:u32,last:u32) {
		self.ids.insert(id,[tries,done,last]);
	}
	
	//Tries to add a new dungeon.
	pub fn try_get(&mut self,id:u32)->option::Option<&[u32;3]> {
		self.ids.get(&id)
	}
	
	//A function for extracting the numbers. Used when saving.
	pub fn get_ids(&self)->Vec<[u32;4]> {
		let mut output_vec = Vec::with_capacity(self.ids.len());
		for (x,[a,b,c]) in self.ids.iter() {
			output_vec.push([*x,*a,*b,*c])
		}
		output_vec
	}
	
	//A function for placing the numbers. Used when loading.
	pub fn replace_ids(&mut self,ids:Vec<[u32;4]>) {
		for [x,a,b,c] in ids.iter() {
			self.ids.insert(*x,[*a,*b,*c]);
		}; 
	}
	
	//Check if you've tried this dungeon.
	pub fn has(&self,id:u32)->bool {
		if self.ids.get(&id).is_some() {true}else{false}
	}
	
	//check if you've finished this dungeon.
	pub fn has_done(&self,id:u32)->bool {
		match self.ids.get(&id) {
			Some([_,a,_]) => if *a>0 {true}else{false},
			_			  => {false},
		}
	}
	
	//Update the status of a dungeon entry.
	//Should only be called when entering the dungeon
	//Or changing scene. Or the world will end.
	pub fn update_status(&mut self,dungeon:&Dungeon,pointer:usize) {
		if self.ids.get(&dungeon.id).is_some() {
			//get entry and current scene.
			let mut entry = self.ids.get_mut(&dungeon.id).unwrap();
			let scene = if pointer>1 {pointer-2}else{0};
			//decide what to do with it. NB entry==[attempts,successes,deepest_stage]
			if scene==0 {
				entry[0]+= 1;
			}else if (scene<dungeon.scenes.len()) & (scene as u32>entry[2]) {
				entry[2] = scene as u32;
			}else{
				entry[1]+= 1;
			};
		}else{
			//no need to check anything here, as this
			//should only be called if there is no entry,
			//and hence no progress to start off with.
			self.ids.insert(dungeon.id,[1,0,0]);
		};
	}
}

//A structure that stores a vector of story ids that have been
//started by the party and their status (finished or not).
//x.0 is the unique story id.
//x.1 is the exit node for the entry content.
//x.2 is the exit node for the completion sequence.
#[derive(Debug)]
pub struct MyStories {
	ids:Vec<(u32,u16,u16)>, 
}

impl MyStories {
	pub fn new()-> MyStories {
		MyStories {ids: Vec::with_capacity(20)}
	}
	pub fn make_ids(&mut self,ids:Vec<(u32,u16,u16)>) {
		self.ids = ids;
	}
	
	pub fn get_ids(&self) -> Vec<(u32,u16,u16)> {
		self.ids.clone()
	}
	 
	pub fn push(&mut self,entry:(u32,u16,u16)) {
		self.ids.push(entry);
	} 
	
	pub fn len(&self)->usize {
		self.ids.len()
	}
	
	pub fn iter(&self)-> slice::Iter<(u32,u16,u16)> {
		self.ids.iter()	
	}
	
	//poll by completed and incompleted.
	//false means incomplete, true means complete
	fn poll_ids(&self,id:u32,done:bool)-> bool {
		if done {
			for x in self.ids.iter() {
				if (x.0==id) & (x.2==0) { return true;};
			}
		}else{
			for x in self.ids.iter() {
				if (x.0==id) & (x.1!=0) { return true;};
			}
		}
		false
	}
	
	//Polls only for this story if started.
	fn poll_started(&self,id:u32)-> bool {
		for x in self.ids.iter() {
			if (x.0==id) & (x.1!=0) { return true;};
		}
		false
	}
	
	//Inserts exit code from content stage. NB, this can go horrifically wrong.
	// IE: You can insert a non existent exit code or fail to found a story.
	pub fn insert_exit_code(&mut self,id:u32,code:u16,conclusion:bool) {
		for x in self.ids.iter_mut() {
			if x.0==id {
				if !conclusion {x.1 = code}else{x.2 = code};
				return;
			};
		}
	}
	
	//polls inly for this story if finished.
	fn poll_finished(&self,id:u32)-> bool {
		for x in self.ids.iter() {
			if (x.0==id) & (x.2!=0) { return true;};
		}
		false
	}
	//polls only for this story if finished with a particular end.
	fn poll_finished_with(&self,id:u32,finish_code:u16)-> bool {
		for x in self.ids.iter() {
			if (x.0==id) & (x.2==finish_code) { return true;};
		}
		false
	}
	//polls only for this story if finished with any end but.
	fn poll_finished_not_with(&self,id:u32,finish_code:u16)-> bool {
		for x in self.ids.iter() {
			if (x.0==id) & (x.2!=finish_code) { return true;};
		}
		false
	}
	
	
	//polls only for this story if finished with a particular end.
	fn poll_started_with(&self,id:u32,starting_code:u16)-> bool {
		for x in self.ids.iter() {
			if (x.0==id) & (x.1==starting_code) { return true;};
		}
		false
	}
	
	//polls only for this story if started with an end other than starting code.
	fn poll_started_not_with(&self,id:u32,starting_code:u16)-> bool {
		for x in self.ids.iter() {
			if (x.0==id) & (x.1!=starting_code) { return true;};
		}
		false
	}
	
	fn get_stage_by_id(&self,id:u32)-> Option<u16> {
		for x in self.ids.iter() {
			if x.0==id {return Some(x.1);};
		}
		None
	}
	
	pub fn get_by_id(&self,id:u32)-> Option<&(u32,u16,u16)> {
		for x in self.ids.iter() {
			if x.0==id {return Some(x);};
		}
		None
	}
	
	pub fn poll_ids_only(&self,id:u32)-> bool {
		for x in self.ids.iter() {
			if x.0==id { return true;};
		}
		false
	}
}

// A structure to keep the whole of a plot-section.
// This includes the triggers for its start and end,
// conclusion sentence, id and "content".
// NB id is u32 to allow easy shifting between platforms (broken elsewhere).
// So the flow is, when a trigger is initially triggered,
// you enter at node 0 (ENTRY node). Go through nodes in content until you
// exit at an exit node. The exit node is saved in my_stories. When the
// completion trigger for this exit node is tripped, you enter the completion
// dialog. At the moment my_stories does not record exit triggers of conclusion. 
#[derive(Debug,Clone,PartialEq)]
pub struct Story<'a> {
	pub name: &'a str,
	pub trigger: Vec<Trigger>,	
	//if your story has more than 255 branches, you have a problem.
	//Likewise it MUST have a completion, else we have a problem.
	pub completion: Vec<(u16,Content<'a>,Vec<Trigger>)>,
	pub content: Content<'a>,
	pub id:u32,
}

impl <'a>Story<'a> {
	pub fn try_get_completion(&self,node:u16)->&Vec<Trigger> {
		for i in 0..self.completion.len() {
			if self.completion[i].0==node {return &self.completion[i].2};
		};
		&self.trigger
	}
	
	pub fn try_get_completion_cont(&self,node:u16)->&Content {
		for i in 0..self.completion.len() {
			if self.completion[i].0==node {return &self.completion[i].1};
		};
		&self.content
	}

}

//I have not decided what content should be stored as.
//Some care must be taken to avoid out of index content.
//Give it thought.
#[derive(Debug,Clone,PartialEq)]
pub struct Content<'a> {
	//Monster's picture, Monster's id, monster's side if battle is started.
	pub actors: Vec<(&'a conrod::image::Id,Lifeform,usize)>,
	pub tokens: Vec<Item>,
	pub phrases_by_key: BTreeMap<u16,(Vec<u16>,String)>, //There must be at least one answer.
	pub entry_node: u16,
	pub entry_description: &'a str,
	pub exit_nodes: Vec<u16>,
	pub exit_descriptions: Vec<&'a str>,
}

impl <'a>Content<'a> {
	
	//A function that gets references to descriptions of a quest, based on exit node.
	pub fn get_descriptions(&self,exit_node:u16)->Vec<&str> {
		let mut output = Vec::new();
		
		//Put entry description into the output.
		output.push(self.entry_description);
		
		//Poll exit nodes and place the description corresponding to
		//the exit node selected here.
		for (i,x) in self.exit_nodes.iter().enumerate() {
			if exit_node==*x {output.push(self.exit_descriptions[i]);};
		};
		
		output
	}
	
	//Function to insert guest monsters into a party.
	//Plan stories carefully so that you can then remove the inserted lifeforms afterwards.
	//Be really careful with this. NB each inserted lifeform must be "unique" no twins.
	pub fn insert_party_guest(&self,party:&mut Vec<(Lifeform,usize)>,p_names:&mut Vec<String>) {
		for x in self.actors.iter() {
			let mut inserted = false;
			for i in 0..party.len() {
				if (x.1.id==party[i].0.id) & (x.1.name==&p_names[i]) {
					inserted = true;
				};
			};
			
			if !inserted {
				p_names.push(self.actors[0].1.name.to_owned());
				party.push((self.actors[0].1.clone(),0));
			}
		};
	}
	
	//Function to remove guest monsters from a party.
	//NB plan stories carefully so you don't end with NULL parties.
	pub fn remove_party_guest(&self,party:&mut Vec<(Lifeform,usize)>,p_names:&mut Vec<String>) {
		for x in self.actors.iter() {
			for i in 0..party.len() {
				if (x.1.id==party[i].0.id) & (x.1.name==&p_names[i]) {
					party.remove(i);
					p_names.remove(i);
				};
			};
		};
	}
	
	//function to give an item to the party.
	//NB, at the moment only the party lead has an inventory.
	pub fn item_to_party(&self,party:&mut Vec<(Lifeform,usize)>){
		for x in self.tokens.iter() {
			let mut insert = true;
			for y in party[0].0.Inventory.iter() {
				if *y==x.id {insert = false;};
			}
			if insert { party[0].0.Inventory.push(x.id);};
		}
	}
	
	//function to get item from party.
	pub fn item_from_party(&self,party:&mut Vec<(Lifeform,usize)>){
		for x in self.tokens.iter() {
			for i in 0..party[0].0.Inventory.len() {
				if x.id==party[0].0.Inventory[i] {
					party[0].0.Inventory.remove(i);
				};
			}
		}
	}
	
	//TODO::GP quest endings...Ermm but I'd rather not.
}

//Trigger for the start of  story dialog can be any of the below,
//or a combination thereof. Not sure combinations will work yet.
#[derive(Debug,Clone,PartialEq)]
pub enum Trigger {
	HasSpell(i8),
	CastSpell(i8),
	CastSpellType(u8),
	HasItem(usize),
	UseItem(usize),
	LFType(u8),
	LFSubType(u8),
	Exp(f32),
	StartedStory(u32),
	StartedStoryWith(u32,u16),
	StartedStoryNotWith(u32,u16),
	FinishedStory(u32),
	FinishedStoryWith(u32,u16),
	FinishedStoryNotWith(u32,u16),
	FinishedDungeon(u32),
	Other(usize), //This is a placeholder.
	Locus(Place),
	LocusAffinity(u8),
	LocusScape(u8),
	LocusXY([i32;2]),
	HasKills(String,u64),
}



fn sage_prices<'a>(list:&'a Vec<Spell>,typ:u8,special:Vec<&str>)->Vec<(&'a Spell,usize)>{
	let mut shopping:Vec<(&Spell,usize)>=Vec::new();
	if special.len()==0{
		for x in list.iter(){
			if x.Type==typ{
				shopping.push((x,(x.MP*x.MP) as usize))
			}	
		}
	}else{
		for x in special.into_iter(){
			for y in list.iter(){
				if x==y.name{
					shopping.push((y,(y.MP*y.MP) as usize))
				}
			}
		}
	};
	shopping
}

// A function to poll stories vs start and finish triggers.
// If conditions are met a story index is given into the story box.
pub fn story_poller (stories:&Vec<Story>,my_stories:&mut MyStories,my_dungeons:&mut MyDungeons,kill_list:&KillList, p_loc:&Place,party:&Vec<(Lifeform,usize)>)->Option<(usize,u16)> {
	//Initiate story.
	//iterate over stories.
	for (i,x) in stories.iter().enumerate() {
		
		let mut get = true;
		let stage:Option<&(u32,u16,u16)> = my_stories.get_by_id(x.id);
		match stage {
			None => {
			//if my stories does not include this story,
			//then poll its triggers.
				for y in x.trigger.iter() {
					match y {
						Trigger::LocusXY(tr) => {
							println!("xy = {:?}",tr);
							if p_loc.xy != *tr {get = false;};
						},
						Trigger::LocusScape(tr) => {
							if p_loc.scape != *tr {get = false;};
						},
						Trigger::LocusAffinity(tr) => {
							if p_loc.affinity != *tr {get = false;};
						},
						Trigger::Locus(tr) => {
							if p_loc != tr {get = false;};
						},
						Trigger::HasSpell(tr) => {
							for z in party[0].0.Spellist.iter() {
								if z != tr {get = false;};
							};
						},
						Trigger::HasItem(tr) => {
							for z in party[0].0.Inventory.iter() {
								if z != tr {get = false;};
							};
						},
						Trigger::LFType(tr) => {
							if party[0].0.Type != *tr {get = false;};
						},
						Trigger::LFSubType(tr) => {
							if party[0].0.SubType != *tr {get = false;};
						},
						Trigger::Exp(tr) => {
							if party[0].0.ExpUsed<*tr {get = false;};
						},
						Trigger::StartedStory(x) => {
							if !my_stories.poll_started(*x) {get = false;};
						},
						Trigger::FinishedStory(x) => {
							if !my_stories.poll_finished(*x) {get = false;};
						},
						Trigger::FinishedStoryWith(x,y) => {
							if !my_stories.poll_finished_with(*x,*y) {get = false;};
						},
						Trigger::FinishedStoryNotWith(x,y) => {
							if !my_stories.poll_finished_not_with(*x,*y) {get = false;};
						},
						Trigger::FinishedDungeon(x) => {
							if !my_dungeons.has_done(*x) {get = false;};
						},
						Trigger::HasKills(n,k) => {
							if kill_list.poll(n)<*k as usize {get = false;};
						},
						_				   => {},
					};
				};
				if get {return Some((i,0))}; //here the zero has the opposite meaning to normal AKA not started.
			},
			Some((_,z,z2)) => {
				if *z2 != 0 {
					get = false;
				}else if *z==0 {
					get = false;
				}else{
				//Poll triggers for completion of story. If not returned.
					for y in x.try_get_completion(*z).iter() {
						match y {
							Trigger::LocusXY(tr) => {
								if p_loc.xy != *tr {get = false;};
							},
							Trigger::LocusScape(tr) => {
								if p_loc.scape != *tr {get = false;};
							},
							Trigger::LocusAffinity(tr) => {
								if p_loc.affinity != *tr {get = false;};
							},
							Trigger::Locus(tr) => {
								if p_loc != tr {get = false;};
							},
							Trigger::HasSpell(tr) => {
								for z in party[0].0.Spellist.iter() {
									if z != tr {get = false;};
								};
							},
							Trigger::HasItem(tr) => {
								for z in party[0].0.Inventory.iter() {
									if z != tr {get = false;};
								};
							},
							Trigger::LFType(tr) => {
								if party[0].0.Type != *tr {get = false;};
							},
							Trigger::LFSubType(tr) => {
								if party[0].0.SubType != *tr {get = false;};
							},
							Trigger::Exp(tr) => {
								if party[0].0.ExpUsed>=*tr {get = false;};
							},
							Trigger::StartedStory(x) => {
								get = my_stories.poll_started(*x);
							},
							Trigger::FinishedStory(x) => {
								get = my_stories.poll_finished(*x);
							},
							Trigger::FinishedStoryWith(x,y) => {
								get = my_stories.poll_finished_with(*x,*y);
							},
							Trigger::FinishedStoryNotWith(x,y) => {
								get = my_stories.poll_finished_not_with(*x,*y);
							},
							_				   => {},
						};
					};
					if get {return Some((i,*z));};
				};
			},
		};
	};
	None
}

//A function to poll sages vs conditions. If triggers are met, sage is summoned.
pub fn sage_poller<'a,'b,'c,'d,'e>(sages: &'e Vec<Sage<'a>>,p_loc:&'b Place,spell:&'c Spell,party:&'d Vec<(Lifeform,usize)>)->Option<usize> {
	//Initiate summoned sage.
	let mut summoned_sage:Option<usize> = None;
	//iterate over the sages.
	for (i,x) in sages.iter().enumerate() {
		
		let mut summon = true;
		for y in x.trigger.iter() {
			
			//NB, not all triggers are relevent for sages.
			//The main ones are locus and spell cast.
			match y {
				Trigger::LocusXY(tr) => {
					if p_loc.xy != *tr {summon = false;};
				},
				Trigger::LocusAffinity(tr) => {
					if p_loc.affinity != *tr {summon = false;};
				},
				Trigger::CastSpell(tr) => {
					if spell.id != *tr {summon = false;};
				},
				Trigger::HasSpell(tr) => {
					for z in party[0].0.Spellist.iter() {
						if z != tr {summon = false;};
					};
				},
				Trigger::CastSpellType(tr) => {
					if spell.Type != *tr {summon = false;};
				},
				Trigger::HasItem(tr) => {
					for z in party[0].0.Inventory.iter() {
						if z != tr {summon = false;};
					};
				},
				Trigger::LFType(tr) => {
					if party[0].0.Type != *tr {summon = false;};
				},
				Trigger::LFSubType(tr) => {
					if party[0].0.SubType != *tr {summon = false;};
				},
				Trigger::Exp(tr) => {
					if party[0].0.ExpUsed>=*tr {summon = false;};
				},
				Trigger::Locus(tr) => {
					if p_loc != tr {summon = false;};
				},
				_				   => {},
			};
		};
		if summon {summoned_sage = Some(i);};
	}
	
	//last check to check if summon is valid.
	if summoned_sage.is_some() {
		if sages[summoned_sage.unwrap()].exp_min>party[0].0.ExpUsed {
			summoned_sage = None
		};
	};
	
	summoned_sage
}

pub fn sage_dialog(){}

pub fn sage_generator<'a>(mon_faces:&'a Vec<[conrod::image::Id;3]>,p_names:&Vec<String>)->Vec<Sage<'a>> {
	
	vec![
		sage_fire(mon_faces,p_names),
		sage_ice(mon_faces,p_names),
		sage_lightning(&mon_faces,&p_names),
		sage_darkness(&mon_faces,&p_names),
		sage_light(&mon_faces,&p_names),
		sage_death(&mon_faces,&p_names),
		sage_life(&mon_faces,&p_names),
		sage_albion(&mon_faces,&p_names),
		sage_malachia(&mon_faces,&p_names)
	]
}

pub fn sage_fire<'a>(mon_faces:&'a Vec<[conrod::image::Id;3]>,p_names:&Vec<String>)->Sage<'a> {
	Sage {
		name: "Sage of Fire".to_owned(),
		exp_min: 10.0,
		face: &mon_faces[22][1],
		trigger: vec![Trigger::CastSpell(S_EMBER),Trigger::LocusAffinity(FIRE)],
		spells: vec![S_EMBER,S_FIRE,S_FIREBALL,S_INFERNO],
		dialog_greeting:["You light a fire in the desert night and realise \
that you are not alone. The form of a nomad sits by the fire, adoring it \
like a dear child...\n...\"I am the Sage of Fire\" it states.\n".to_owned(),
format!("\nWhy did you call me, {}? Or did you just want to stay warm?",p_names[0])
],
		dialog_magic:["Tell me about fire spells.".to_owned(),
"Fire magic is the light born of destruction that burns up everything. \
Fire magic is the most destructive of magics, but it also carries the seed of rebirth. \
Inferno is the greatest fire magic. It is the fire that burns up everything.\n".to_owned()],
		dialog_sages:["Tell me why you sages are always so fickle.".to_owned(),
"Fickle? It is simply that without an arcane wisdom equaling our own, \
you cannot understand the logic that binds us.\n".to_owned()],
		dialog_world:["Tell me when the world will end.".to_owned(),
"The world will end when Bremnor extinguishes the last fire...\n".to_owned()],
		dialog_terrain:["Tell me about the Desert.".to_owned(),
"The desert is where the unnecessary aspects of self are burned away, and the soul is reborn in its true form.
The ashes of that which no longer belongs are mixed into the sands...
...Nations...
...Cultures...
...Gods.\n".to_owned()],
		dialog_goodbye:["It's cold here".to_owned(),"I see. Don't burn out\n...\nThe sage is gone and \
you are left alone in the desert again.\n".to_owned()],	
	}
}


pub fn sage_lightning<'a>(mon_faces:&'a Vec<[conrod::image::Id;3]>,p_names:&Vec<String>)->Sage<'a> {
	Sage {
		name: "Sage of Lightning".to_owned(),
		exp_min: 50.0,
		face: &mon_faces[19][0],
		trigger: vec![Trigger::CastSpell(S_SPARK),Trigger::LocusAffinity(LIGHTNING)],
		spells: vec![S_SPARK,S_LIGHTNING,S_JOVIAN_LIGHTNING],
		dialog_greeting:["A single spark in the stony labyrinth...\n...Calls down lightning from a clear sky. \
The flash momentarily blinds you and you realise that this lightning is something else. \
\n...\"I am the Sage of Lighning\" it states.\n".to_owned(),
format!("\nWhy did you call me, {}?",p_names[0])
],
		dialog_magic:["Tell me about lightning spells.".to_owned(),
"Lightning magic is the anger of the gods. \
It destroys and disempowers. Only the truly righteous can withstand it. \
Jovian Lightning is the greatest lightning magic. It is the spear of burning gold borrowed from the heavens.\n".to_owned()],
		dialog_sages:["Tell me what you wished for when you became a sage.".to_owned(),
"Each of us had their own wish. But they could only be fulfilled by learning that which is forbidden to mortals.\n".to_owned()],
		dialog_world:["Tell me when the world will end.".to_owned(),
"The world will end when Bremnor punishes the gods for their arrogance...\n".to_owned()],
		dialog_terrain:["Tell me about the Stone Maze.".to_owned(),
"This maze is all that remains of the nation that challenged the gods...
...Flora...
...Fauna...
...Lands...
...Waters...
...Sky...
...All purified by Jove's lightning.\n".to_owned()],
		dialog_goodbye:["...".to_owned(),
"...\n...\nThe lightning fades and the Sage of Lightning is gone as if they never were...".to_owned()],
	}
}

pub fn sage_ice<'a>(mon_faces:&'a Vec<[conrod::image::Id;3]>,p_names:&Vec<String>)->Sage<'a> {
	Sage {
		name: "Sage of Ice".to_owned(),
		exp_min: 50.0,
		face: &mon_faces[25][0],
		trigger: vec![Trigger::CastSpell(S_LESSER_CRYSTALLISE),Trigger::LocusAffinity(ICE)],
		spells: vec![S_LESSER_CRYSTALLISE,S_CRYSTALLISE,S_TRUE_CRYSTALLISE],
		dialog_greeting:["You try to make the artic wastes a little colder...\n...And the air around you begins to coalesce. \
Before you stands the White Queen. \
\n...\"I am the Sage of Ice\" it states.\n".to_owned(),
format!("\nWhy did you call me, {}?",p_names[0])
],
		dialog_magic:["Tell me about ice spells.".to_owned(),
"Ice magic is the destruction of fire. \
It is the stillness that waits at the end of the world. \
There is a spell that brings peace and stillness: True Crystalise is that gift.\n".to_owned()
],
		dialog_sages:["Tell me what you sages are anyway.".to_owned(),
"We are what happens to mere mortals when you surpass the limits of mortal knowledge. \
We have become inseperate from the sacred laws that have created the world.\n".to_owned()
],
		dialog_world:["Tell me when the world will end.".to_owned(),
"The world will end when Bremnor's heart crystalises...\n".to_owned()
],
		dialog_terrain:["Tell me about the Frozen Wastes.".to_owned(),
"These icy plains and peaks are exactly what they seem.\n".to_owned()
],
		dialog_goodbye:["...".to_owned(),
"The Sage of Ice was just another jagged block of ice.....".to_owned()
],
	}
}

pub fn sage_light<'a>(mon_faces:&'a Vec<[conrod::image::Id;3]>,p_names:&Vec<String>)->Sage<'a> {
	Sage {
		name: "Sage of Light".to_owned(),
		exp_min: 100.0,
		face: &mon_faces[26][0],
		trigger: vec![Trigger::CastSpell(S_LIGHT),Trigger::LocusAffinity(HOLY)],
		spells: vec![S_LIGHT,S_SACRED_LIGHT,S_EXORCISM,S_GREATER_EXORCISM,S_SACRED_EXORCISM],
		dialog_greeting:["As you cast your spell, a shining figure approaches you from the mists. \
Serene, it walks over the water. As it approaches your ship, it says: \
\n...\"I am the Sage of Light\" it states.\n".to_owned(), 
format!("\nWhy did you call me, {}?",p_names[0])
],
		dialog_magic:["Tell me about light spells.".to_owned(),
"Light spells bring light to dark places, be they physical or metaphysical.
White magic is strengthened and black magic is weakened in the light.
The more potent light spells banish unclean creatures directly.
The Sacred spells are the most potent spells available to a white mage.\n".to_owned()
],
		dialog_sages:["Tell me about white magic.".to_owned(),
"White magic is born of goodwill. Even without using spells, it protects against both physical harm and dark sorceries.
When projected as a spell, white magic preserves and restores things to the way they ought to be.\n".to_owned()
],
		dialog_world:["Tell me when the world will end.".to_owned(),
"The world will end when Bremnor can no longer see the light...\n".to_owned()
],
		dialog_terrain:["Tell me about the White Sea.".to_owned(),
"The White Sea... With the light fading, it is no longer the sacred place it used to be. But I still believe...\n".to_owned()],
		dialog_goodbye:["I see.".to_owned(),
"If there is something you need from me, I here.\n...\nThe Sage of Light turns and melts into the mists...".to_owned()
],
	}
}



pub fn sage_darkness<'a>(mon_faces:&'a Vec<[conrod::image::Id;3]>,p_names:&Vec<String>)->Sage<'a> {
	Sage {
		name: "Sage of Darkness".to_owned(),
		exp_min: 100.0,
		face: &mon_faces[26][0],
		trigger: vec![Trigger::CastSpell(S_DARKNESS),Trigger::LocusXY([-100,-10])],
		spells: vec![S_DARKNESS,S_ABYSSAL_DARKNESS,S_CURSE],
		dialog_greeting:["As darkness descends upon the Black Obelisk, you feel a presence besides you. \
Standing at your side, darker than black, it utters: \
\n...\"I am the Sage of Darkness\".\n".to_owned(),
format!("\nWhy did you call me, {}?",p_names[0]),
],
		dialog_magic:["Tell me about darkness spells.".to_owned(),
"My magic gives birth to darkness, extinguishing the light that gives life its meaning. \
Just as the rain extingushes a small flame, the darkness extinguishes the hope that fuels white magic. \
The more potent spells of darkness transcend the unphysical and weaken order upon which physical matter is based. \
Abbysal spells, despite their potence, are not the most powerful dark magics in this world.\n".to_owned()	
],
		dialog_sages:["Tell me about black magic.".to_owned(),
"Black magic is born of despair. Your despair will not help you unless you project it as a spell. \
When projected as a spell, black magic will to restore all things to the nothingness they were before the world began.".to_owned()		
],
		dialog_world:["Tell me when the world will end.".to_owned(),
"The world will end when Bremnor curses all the lands, sea and sky, and the void engulfs all things in its blackness.\n".to_owned()		
],
		dialog_terrain:["Tell me about the Black Obelisk.".to_owned(),
"The Black Obelisk is my masterpiece: It is the crystalisation of the cumulative negative energy of all living things. \
You can see all that is wrong with yourself if you look for long enough into this stone.\n".to_owned()		
],
		dialog_goodbye:["...".to_owned(),
"\"Then may you see only darkness on your quest.\"\n...\nYour spell lifts and you are alone in the land of the Black Obelisk...".to_owned()
],
	}
}

pub fn sage_life<'a>(mon_faces:&'a Vec<[conrod::image::Id;3]>,p_names:&Vec<String>)->Sage<'a> {
	Sage {
		name: "Sage of Life".to_owned(),
		exp_min: 100.0,
		face: &mon_faces[26][0],
		trigger: vec![Trigger::CastSpell(S_LESSER_CURE),Trigger::LocusXY([-100,60])],
		spells: vec![S_LESSER_CURE,S_CURE,S_GREATER_CURE,S_SACRED_CURE],
		dialog_greeting:["You cast your spell amongst the harsh winds of the White Island. \
Someone stands before you. \
\n...\"I am the Sage of Light\" it utters.\n".to_owned(),
format!("\nWhy did you call me, {}?",p_names[0]),
],
		dialog_magic:["Tell me about healing spells.".to_owned(),
"Healing spells return all things and beings to their true and rightful form. \
The rightful form of the living is alive, and the rightful form of that which is dead \
is to return to the circle...".to_owned()		
],
		dialog_sages:["Tell me why I keep coming back to life.".to_owned(),
"You keep coming back to life because I want you to.\n".to_owned()		
],
		dialog_world:["Tell me when the world will end.".to_owned(),
"The world will end when Bremnor leaves the White Island for the last time...\n".to_owned()		
],
		dialog_terrain:["Tell me about the White Island.".to_owned(),
"The White Island is a haunted place.
It is the last place where those who have overstayed their welcome \
upon this world are permitted to exist before they go on their way.\n".to_owned()		
],
		dialog_goodbye:["...".to_owned(),
"\"Then stay alive till we next meet\".\n...\nThe wind picks up, and you can no longer \
remember whether an old man or a young child stood before you...".to_owned()
],
	}
}

pub fn sage_death<'a>(mon_faces:&'a Vec<[conrod::image::Id;3]>,p_names:&Vec<String>)->Sage<'a> {
	Sage {
		name: "Sage of Death".to_owned(),
		exp_min: 200.0,
		face: &mon_faces[13][2],
		trigger: vec![Trigger::CastSpell(S_EXORCISM),Trigger::LocusXY([-40,-30])],
		spells: vec![S_DARKNESS,S_ABYSSAL_DARKNESS,S_CURSE,S_SUMMON_REAPER,S_SWORD_OF_PERSEUS,S_LIFESTEALER],
		dialog_greeting:["You finish exorcising the crypts and the dead lie still as they should. Then they rise again. \
The corpses bow as one and speak as a raspy chorus: \
\n...\"I am the Sage of Death\"\n".to_owned(),
format!("\nWhy did you call me, {}?",p_names[0]),
],
		dialog_magic:["Tell me about death spells.".to_owned(),
"My magic gives birth to darkness, extinguishing the light that gives life its meaning. \
Just as the rain extingushes a small flame, the darkness extinguishes the hope that fuels white magic. \
The more potent spells of darkness transcend the unphysical and weaken order upon which physical matter is based. \
Abbysal spells, despite their potence, are not the most powerful dark magics in this world.\n".to_owned()		
],
		dialog_sages:["Tell me why I keep coming back to life.".to_owned(),
"You keep coming back to life because the world has not yet died.\n".to_owned()		
],
		dialog_world:["Tell me when the world will end.".to_owned(),
"The world will end when Bremnor can rest no longer...\n".to_owned()		
],
		dialog_terrain:["Tell me about the City of the Dead.".to_owned(),
"The City of the Dead is a place of rest. Just as the desert is the grave of oceans... \
This is the grave of civilisations. \
Now that the world itelf is tired, it has become crowded in my beloved city.".to_owned()		
],
		dialog_goodbye:["...".to_owned(),
"\"Then stay close to the grave\".\n...\nThe corpses crumple like puppets, \
just as they should have to start of with...".to_owned()		
],
	}
}


pub fn sage_albion<'a>(mon_faces:&'a Vec<[conrod::image::Id;3]>,p_names:&Vec<String>)->Sage<'a> {
	Sage {
		name: "Sage of Albion".to_owned(),
		exp_min: 200.0,
		face: &mon_faces[26][0],
		trigger: vec![Trigger::CastSpell(S_LIGHT),Trigger::LocusXY([-160,20])],
		spells: vec![S_HASTE,S_SLOW,S_TIMESTOP,S_TELEPORT,S_GREATER_TELEPORT],
		dialog_greeting:["O Albion! Is this your former splendour?..
You illuminate the empty temples with your spell and see a young man stride towards you.
\n...\"I am the Sage of Albion\" he states.\n".to_owned(),
format!("\nWelcome to the Albion of my youth, {}. My apologies for this poor hospitality.",p_names[0])
],
		dialog_magic:["Tell me about time magic.".to_owned(),
"Chronomancy is the power to manipulate your vector in a non-spacial dimension.
It can change the course of history...
...Timestop is the most powerful time magic. Alas, there is no way to truly turn back the clocks...\n".to_owned()		
],
		dialog_sages:["Tell me, were your wishes granted.".to_owned(),
"Our wishes were granted, but it would have been better had they been left unfulfilled...\n".to_owned()		
],
		dialog_world:["Tell me when the world will end.".to_owned(),
"The world has already ended, Bremnor just hasn't realised it yet...\n".to_owned()		
],
		dialog_terrain:["Tell me about Albion.".to_owned(),
"Albion is but a ruin now. Once upon a time a city of light by the name of Albion stood here...
...It embodied, peace...
...Fairness...
...Prosperity...
...The future that our wishes devoured.\n".to_owned()		
],
		dialog_goodbye:["Oops".to_owned(),
"The sage freezes motionless and flickers out of existence...".to_owned()
],
	}
}

pub fn sage_malachia<'a>(mon_faces:&'a Vec<[conrod::image::Id;3]>,p_names:&Vec<String>)->Sage<'a> {
	Sage {
		name: "Sage of Malachia".to_owned(),
		exp_min: 1000.0,
		face: &mon_faces[20][1],
		trigger: vec![Trigger::CastSpell(S_SUMMON_REAPER),Trigger::LocusXY([60,30])],
		spells: vec![S_APOCALYPSE],
		dialog_greeting:["You summon death incarnate at the square of Malachia...
The reaper stands motionless, observing the sky, then it turns to face you.
\n...\"I am the Sage of Malachia\" it states.\n".to_owned(),
format!("\nHas the time finally come, {}?",p_names[0])
],
		dialog_magic:["Tell me about Apocaplypse.".to_owned(),
"Apocalypse is a magic that can wrong all that is wrong..
...And the breathing corpse that is this world, will be no more upon its casting...
...And a living world, which has yet to give up, can be born anew.\n".to_owned()		
],
		dialog_sages:["Tell me, what did your wishes do to this world?".to_owned(),
"We became sages, part of the logic of this world..
...and all our wishes were granted, and all our hopes were fulfilled.
But our hopes had become this world's hopes, and with our hopes fulfilled...
...The world had nothing left to hope for.
So this hopeless world died.\n".to_owned()		
],
		dialog_world:["Tell me when the world will end.".to_owned(),
"That... That is up to you.\n".to_owned()		
],
		dialog_terrain:["Tell me about Malachia.".to_owned(),
"Malachia is the green city. More precisely that is what it used to be. \
Perhaps next time it will not end like this.\n".to_owned()		
],
		dialog_goodbye:["Oops".to_owned(),
"\"We all reaped as we sowed\".\n...\n\"You too will not reap but what you sowed\"
...And the reaper fades away...".to_owned()		
],
	}
}

