///The storage space for the plot, sideplots and all the other stories.
///As a reminder, here are what the structures look like:
/// pub struct Story<'a> {
/// 	pub trigger: Vec<Trigger>,	
/// 	pub completion: Vec<(u16,Content<'a>,Vec<Trigger>)>,
/// 	pub content: Content<'a>,
/// 	pub id:u32,
/// }
/// 
/// pub struct Content<'a> {
/// 	pub actors: Vec<(&'a conrod::image::Id,String)>,
/// 	pub phrases_by_key: BTreeMap<u16,(Vec<u16>,String)>,
/// 	pub entry_node: u16,
/// 	pub exit_nodes: Vec<u16>,
/// }
/// 
/// pub enum Trigger {
/// 	HasSpell(i8),
/// 	CastSpell(i8),
/// 	CastSpellType(u8),
/// 	HasItem(usize),
/// 	UseItem(usize),
/// 	LFType(u8),
/// 	LFSubType(u8),
/// 	Exp(f32),
/// 	StartedStory(u32),
/// 	FinishedStory(u32),
/// 	FinishedStoryWith(u32,u16),
/// 	FinishedDungeon(usize), 
/// 	Other(usize), 
/// 	Locus(Place),
/// 	LocusType(u8),
/// 	LocusXY([i32;2]),
/// }
///NB, an exit node code of 666 will start a battle with the story's actors.
//Standard entry code.
const ENTRY:u16 = 0;
//Special exit codes.
const FIGHT_EXIT:u16 = 666;
const JOIN_EXIT:u16 = 333;
//Standard exit codes (keep things less confusing)
//These are special reserved numbers. Keep away.
const EXIT_1:u16 = 65535;
const EXIT_2:u16 = 65534;
const EXIT_3:u16 = 65533;
const EXIT_4:u16 = 65532;
const EXIT_5:u16 = 65531;

extern crate conrod;
use smoose::{Story,Trigger,Content};
use std::collections::BTreeMap;

//The test quest, carry yourself to either of these locations.
fn void_bridge_or_black_tower_marker(){}
pub fn void_bridge_or_black_tower<'a>(faces:&'a Vec<[conrod::image::Id;3]>)->Story<'a> {
	
	let start_trigger = vec![Trigger::Exp(10.0)];
	
	let mut entry_map:BTreeMap<u16,(Vec<u16>,String)> = BTreeMap::new();
	entry_map.insert(ENTRY,(vec![999],"Take me... To the Void Bridge...".to_owned()));
	entry_map.insert(999,(vec![998],"...".to_owned()));
	entry_map.insert(998,(vec![997,996,800,555],"Please... Take me... To the Void Bridge...".to_owned()));
	
	entry_map.insert(996,(vec![1],"Sure".to_owned()));
	entry_map.insert(1,(vec![],"Then... I will stay... In your shadow...".to_owned()));
	
	entry_map.insert(991,(vec![2],"Sure".to_owned()));
	entry_map.insert(2,(vec![],"Then... I will stay... In your shadow...".to_owned()));
	
	entry_map.insert(800,(vec![3],"Why?".to_owned()));
	entry_map.insert(700,(vec![4],"Why?".to_owned()));
	
	entry_map.insert(997,(vec![994],"No".to_owned()));
	entry_map.insert(994,(vec![991,993,700,555],"Then... At the least... take me to the Black Tower".to_owned()));
	
	entry_map.insert(993,(vec![900],"No".to_owned()));
	entry_map.insert(900,(vec![996,997,800,555],"Then... At the least... take me to the Void Bridge".to_owned()));
	
	entry_map.insert(3,(vec![997,996,555],"I wish... To rest in darkness... For all eternity..\n \
...And the darkest darkness... is.. the Void".to_owned()));	
	entry_map.insert(4,(vec![993,991,555],"I wish... To rest in darkness... For all eternity..\n \
...And the Black Tower is almost... as dark.. as the Void...".to_owned()));

	entry_map.insert(555,(vec![FIGHT_EXIT],"Never! I will never help you! Begone foul spirit!".to_owned()));
	entry_map.insert(FIGHT_EXIT,(vec![],"So be it...".to_owned()));
	
	let entry_content = Content {
		actors:vec![(&faces[16][0],16,1)],
		phrases_by_key:entry_map,
		entry_node: ENTRY,
		exit_nodes: vec![1,2,FIGHT_EXIT],
	};
	
	let mut void_dialog:BTreeMap<u16,(Vec<u16>,String)> = BTreeMap::new();
	void_dialog.insert(1,(vec![],"\"Thank you... Traveller... May you too... Rest in peace...\" \
\nThe spirit, and flows down into the Void...".to_owned()));
	
	let void_bridge_content = Content {
		actors:vec![(&faces[16][0],16,1)],
		phrases_by_key: void_dialog,
		entry_node: 1,
		exit_nodes: vec![1],
	};
	
	let mut tower_dialog:BTreeMap<u16,(Vec<u16>,String)> = BTreeMap::new();
	tower_dialog.insert(2,(vec![],"\"Thank you... Traveller... May you too... Rest in peace...\" \
\nThe spirit, and merges with the darkness of the tower.".to_owned()));
		
	let black_tower_content = Content {
		actors:vec![(&faces[16][0],16,1)],
		phrases_by_key: tower_dialog,
		entry_node: 2,
		exit_nodes: vec![2],
		
	};
	
	let mut death_dialog:BTreeMap<u16,(Vec<u16>,String)> = BTreeMap::new();
	death_dialog.insert(FIGHT_EXIT,(vec![777],"The ghost fades into the night...".to_owned()));
	death_dialog.insert(777,(vec![888],"...".to_owned()));
	death_dialog.insert(888,(vec![],"Well, what's done is done.".to_owned()));
		
	let death_content = Content {
		actors:Vec::new(),
		phrases_by_key: death_dialog,
		entry_node: FIGHT_EXIT,
		exit_nodes: vec![888],
		
	};
	
	let end_triggers = vec![
		(1,void_bridge_content,vec![Trigger::LocusXY([20,70])]),
		(2,black_tower_content,vec![Trigger::LocusXY([-100,-10])]),
		(FIGHT_EXIT,death_content,vec![Trigger::StartedStoryWith(666,666)])
	];
	
	Story {
		name: "Rest in Darkness",
		trigger: start_trigger,	
		completion: end_triggers,
		content: entry_content,
		id:666,
	}
}

//This is a continuation of id:666 assuming the ghost is slain.
//NB: NOT FINISHED. Will crash on take off.
pub fn ghosthunt_part_1<'a>(faces:&'a Vec<[conrod::image::Id;3]>)->Story<'a> {
	//define start_trigger
	let start_trigger = vec![Trigger::FinishedStoryWith(666,888)];
	
	let mut entry_map:BTreeMap<u16,(Vec<u16>,String)> = BTreeMap::new();
	
	entry_map.insert(ENTRY,(vec![1,2,3,4,5],"Excuse me, did an evil spirit come this way?".to_owned()));
	entry_map.insert(1,(vec![10],"Who are you?".to_owned()));
	entry_map.insert(2,(vec![20],"No.".to_owned()));
	entry_map.insert(3,(vec![30],"Yes.".to_owned()));
	entry_map.insert(4,(vec![20],"Yes, and got the better of me.".to_owned()));
	entry_map.insert(5,(vec![30],"Yes, but I managed to fight it off.".to_owned()));
	entry_map.insert(6,(vec![60],"Oh, I annihilated it, and I'll do the same to you!".to_owned()));
	
	entry_map.insert(60,(vec![61,62,63],"We have no quarrel with the living.".to_owned()));
	entry_map.insert(61,(vec![70],"Sorry, got caught up in the heat of the moment.".to_owned()));
	entry_map.insert(62,(vec![EXIT_1],"Then get lost!".to_owned()));
	entry_map.insert(63,(vec![FIGHT_EXIT],"Oh, but I do... En-garde!".to_owned()));
	
	
	entry_map.insert(20,(vec![21,80],"Is that so? In that case, may the light be with you".to_owned()));
	entry_map.insert(21,(vec![EXIT_3],"Farewell".to_owned()));
	
	entry_map.insert(70,(vec![71,72,73],"We understand. In that case, may the light be with you".to_owned()));
	entry_map.insert(71,(vec![EXIT_2],"Farewell".to_owned()));
	entry_map.insert(72,(vec![60],"Actually I think I do want to check the colour of your blood after all".to_owned()));
	entry_map.insert(73,(vec![80],"Wait!".to_owned()));
	
	entry_map.insert(80,(vec![72,81,82,83,84,85],"Yes?".to_owned()));
	entry_map.insert(81,(vec![90],"Who are you?".to_owned()));
	entry_map.insert(82,(vec![100],"What are you doing here?".to_owned()));
	entry_map.insert(83,(vec![EXIT_3],"Thank you for your concern! All the best!".to_owned()));
	entry_map.insert(84,(vec![110],"Can I do something to aid you?".to_owned()));
	
	entry_map.insert(FIGHT_EXIT,(vec![],"So be it..".to_owned())); //Fight exit.
	entry_map.insert(EXIT_1,(vec![],"May the light be with you..".to_owned())); //Refuse quest exit. 
	entry_map.insert(EXIT_2,(vec![],"...".to_owned())); //Bad terms exit.
	entry_map.insert(EXIT_3,(vec![],"Thank you. May the light be with you..".to_owned())); //Good terms exit.
	
	let entry_content = Content {
		actors:vec![(&faces[21][0],21,1),(&faces[21][0],22,1),(&faces[21][0],22,1)],
		phrases_by_key: entry_map,
		entry_node: ENTRY,
		exit_nodes: vec![EXIT_1,EXIT_2,EXIT_3,FIGHT_EXIT],
	};
		
		
	//Create end vector -NB this will crash hard the way it is. I think.
	let ends = Vec::new();
	
	Story {
		name: "Ghosthunt: Chapter 1",
		trigger: start_trigger,	
		completion: ends,
		content: entry_content,
		id:667,
	}	
}
