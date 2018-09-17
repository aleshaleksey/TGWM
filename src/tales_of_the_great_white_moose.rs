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
extern crate conrod;
use smoose::{Story,Trigger,Content};
use std::collections::BTreeMap;

//The test quest, carry yourself to either of these locations.
fn void_bridge_or_black_tower_marker(){}
pub fn void_bridge_or_black_tower<'a>(faces:&'a Vec<[conrod::image::Id;3]>)->Story<'a> {
	
	let start_trigger = vec![Trigger::Exp(10.0)];
	
	let mut entry_map:BTreeMap<u16,(Vec<u16>,String)> = BTreeMap::new();
	entry_map.insert(0,(vec![999],"Take me... To the Void Bridge...".to_owned()));
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

	entry_map.insert(555,(vec![666],"Never! I will never help you! Begone foul spirit!".to_owned()));
	entry_map.insert(666,(vec![],"So be it...".to_owned()));
	
	let entry_content = Content {
		actors:vec![(&faces[16][0],16,1)],
		phrases_by_key:entry_map,
		entry_node: 0,
		exit_nodes: vec![1,2,666],
	};
	
	let mut void_dialog:BTreeMap<u16,(Vec<u16>,String)> = BTreeMap::new();
	void_dialog.insert(1,(vec![],"\"Thank you... Traveller... May you too... Rest in peace...\" \
Says the spirit, and flies down into the Void...".to_owned()));
	
	let void_bridge_content = Content {
		actors:vec![(&faces[16][0],16,1)],
		phrases_by_key: void_dialog,
		entry_node: 1,
		exit_nodes: vec![1],
	};
	
	let mut tower_dialog:BTreeMap<u16,(Vec<u16>,String)> = BTreeMap::new();
	tower_dialog.insert(2,(vec![],"\"Thank you... Traveller... May you too... Rest in peace...\" \
Says the spirit, and merges with the darkness of the tower.".to_owned()));
		
	let black_tower_content = Content {
		actors:vec![(&faces[16][0],16,1)],
		phrases_by_key: tower_dialog,
		entry_node: 2,
		exit_nodes: vec![2],
		
	};
	
	let mut death_dialog:BTreeMap<u16,(Vec<u16>,String)> = BTreeMap::new();
	death_dialog.insert(666,(vec![777],"You fought the evil spirit... But was this truly for the best?".to_owned()));
	death_dialog.insert(777,(vec![888],"...".to_owned()));
	death_dialog.insert(888,(vec![],"Well, what's done is done.".to_owned()));
		
	let death_content = Content {
		actors:Vec::new(),
		phrases_by_key: death_dialog,
		entry_node: 666,
		exit_nodes: vec![888],
		
	};
	
	let end_triggers = vec![
		(1,void_bridge_content,vec![Trigger::LocusXY([20,70])]),
		(2,black_tower_content,vec![Trigger::LocusXY([-100,-10])]),
		(666,death_content,vec![Trigger::StartedStoryWith(666,666)])
	];
	
	Story {
		trigger: start_trigger,	
		completion: end_triggers,
		content: entry_content,
		id:666,
	}
}
