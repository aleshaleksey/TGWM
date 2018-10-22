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
pub const FIGHT_EXIT:u16 = 65530;
pub const MAX_JOIN_EXIT:u16 = 100;    //if below this or equal, monster joins.
pub const MAX_LEAVE_EXIT:u16 = 333;  //If between 100-333, monster leaves.
pub const MIN_GIVE_EXIT:u16 = 334;
pub const MAX_GIVE_EXIT:u16 = 433;
pub const MIN_TAKE_EXIT:u16 = 434;
pub const MAX_TAKE_EXIT:u16 = 533;
pub const MIN_PAY_EXIT:u16 = 534;
pub const MAX_PAY_EXIT:u16 = 633;
pub const MIN_FINE_EXIT:u16 = 634;
pub const MAX_FINE_EXIT:u16 = 733;
//Standard exit codes (keep things less confusing)
//These are special reserved numbers. Keep away.
const EXIT_1:u16 = 65535;
const EXIT_2:u16 = 65534;
const EXIT_3:u16 = 65533;
const EXIT_4:u16 = 65532;
const EXIT_5:u16 = 65531;

extern crate conrod;
use smoose::{Story,Trigger,Content};
use lmoose::*;
#[allow(unused_imports)]
use dmoose::{ID_CITADEL_OF_SPIRIT,ID_DOOR_TO_DARKNESS,ID_ELVEN_LAKE_RUINS,
			 ID_HALL_OF_STONE,ID_ICE_PALACE,ID_LOST_LIGHTHOUSE,ID_MALACHIA_PUBCRAWL,
			 ID_MALEK_GROVE,ID_MONSTER_HALL,ID_ON_THE_PRAIRIE,ID_PETRIFIED_SHRINE,
			 ID_STAIRWAY,ID_THE_PATH,ID_TOWER_OF_FLESH,ID_TOWER_OF_BONES,
			 ID_TOWER_OF_SOUL,ID_WAY_DOWN,ID_WHITE_TEMPLE,ID_WILD_HUNT,ID_WITCH_MAZE};
			 
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
		actors:vec![(&faces[16][0],ghost_an(),1)],
		tokens:Vec::new(),
		phrases_by_key:entry_map,
		entry_node: ENTRY,
		entry_description: "I had a strange encounter today. I met a ghost which wanted me to take it to a resting place.",
		exit_nodes: vec![1,2,FIGHT_EXIT],
		exit_descriptions: vec!["I agreed to take it to the Void Bridge.",
								"I agreed to take it to the Black Tower.",
								"I would have none of it, who wants to be haunted?"],
	};
	
	let mut void_dialog:BTreeMap<u16,(Vec<u16>,String)> = BTreeMap::new();
	void_dialog.insert(1,(vec![100],"\"Thank you... Traveller... May you too... Rest in peace...\"".to_owned()));
	void_dialog.insert(100,(vec![111],"...".to_owned()));
	void_dialog.insert(111,(vec![],"The spirit flows down into the void.".to_owned()));
	
	let void_bridge_content = Content {
		actors:vec![(&faces[16][0],ghost_an(),1)],
		tokens:Vec::new(),
		phrases_by_key: void_dialog,
		entry_node: 1,
		entry_description: "I arrived at the Void Bridge, ghost in tale.",
		exit_nodes: vec![111],
		exit_descriptions: vec!["It left, merging into the darkness of the Void."],
	};
	
	let mut tower_dialog:BTreeMap<u16,(Vec<u16>,String)> = BTreeMap::new();
	tower_dialog.insert(2,(vec![200],"\"Thank you... Traveller... May you too... Rest in peace...\"".to_owned()));
	tower_dialog.insert(200,(vec![222],"...".to_owned()));
	tower_dialog.insert(222,(vec![],"The spirit, and merges with the darkness of the tower.".to_owned()));
		
	let black_tower_content = Content {
		actors:vec![(&faces[16][0],ghost_an(),1)],
		tokens:Vec::new(),
		phrases_by_key: tower_dialog,
		entry_node: 2,
		entry_description: "I arrived at the Black Tower, followed by the ghost.",
		exit_nodes: vec![222],
		exit_descriptions: vec!["The ghost approached the tower, and disappeared into its darkness."],
		
	};
	
	let mut death_dialog:BTreeMap<u16,(Vec<u16>,String)> = BTreeMap::new();
	death_dialog.insert(FIGHT_EXIT,(vec![777],"The ghost fades into the night...".to_owned()));
	death_dialog.insert(777,(vec![888],"...".to_owned()));
	death_dialog.insert(888,(vec![],"Well, what's done is done.".to_owned()));
		
	let death_content = Content {
		actors:Vec::new(),
		tokens:Vec::new(),
		phrases_by_key: death_dialog,
		entry_node: FIGHT_EXIT,
		entry_description: "I did battle with the ghost.",
		exit_nodes: vec![888],
		exit_descriptions: vec!["Now it is here no more."],
		
	};
	
	let end_triggers = vec![
		(1,void_bridge_content,vec![Trigger::LocusXY([20,70])]),
		(2,black_tower_content,vec![Trigger::LocusXY([-100,-10])]),
		(FIGHT_EXIT,death_content,vec![Trigger::StartedStoryWith(666,FIGHT_EXIT)])
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
fn ghosthunt_part_1_marker(){}
pub fn ghosthunt_part_1<'a>(faces:&'a Vec<[conrod::image::Id;3]>)->Story<'a> {
	//define start_trigger
	let start_trigger = vec![Trigger::FinishedStory(666)];
	
	let mut entry_map:BTreeMap<u16,(Vec<u16>,String)> = BTreeMap::new();
	
	entry_map.insert(ENTRY,(vec![1,2,3,4,5,6],"Excuse me, did an evil spirit come this way?".to_owned()));
	entry_map.insert(1,(vec![10],"Who are you?".to_owned()));
	entry_map.insert(2,(vec![20],"No.".to_owned()));
	entry_map.insert(3,(vec![30],"Yes.".to_owned()));
	entry_map.insert(4,(vec![20],"Yes, and got the better of me.".to_owned()));
	entry_map.insert(5,(vec![20],"Yes, but I managed to fight it off.".to_owned()));
	entry_map.insert(6,(vec![60],"Oh, I annihilated it, and I'll do the same to you!".to_owned()));
	
	entry_map.insert(60,(vec![61,62,63],"We have no quarrel with the living.".to_owned()));
	entry_map.insert(61,(vec![70],"Sorry, got caught up in the heat of the moment.".to_owned()));
	entry_map.insert(62,(vec![EXIT_1],"Then get lost!".to_owned()));
	entry_map.insert(63,(vec![FIGHT_EXIT],"Oh, but I do... En-garde!".to_owned()));
	
	entry_map.insert(30,(vec![72,31,32,33,6],"Would you tell us where it went?".to_owned()));
	entry_map.insert(31,(vec![20],"I don't know.".to_owned()));
	entry_map.insert(32,(vec![35],"I destroyed it.".to_owned()));
	entry_map.insert(33,(vec![40],"I laid it to rest in darkness.".to_owned()));
	
	entry_map.insert(35,(vec![36,37,72,81,82],"Is that so? Well met! Maybe you can help us with something...".to_owned()));
	entry_map.insert(36,(vec![110],"Can I do something to aid you?".to_owned()));
	entry_map.insert(37,(vec![70],"I'd rather not?".to_owned()));
	
	entry_map.insert(200,(vec![72,81,201,202,203],"Do you understand what you have done? All of its sins will go unpunished..".to_owned()));
	entry_map.insert(201,(vec![10],"Who are you to babble on about sins..".to_owned()));
	entry_map.insert(202,(vec![60],"If you don't like it, you can try to punish me instead..".to_owned()));
	entry_map.insert(203,(vec![210],"Yes. And I'll do it all again...".to_owned()));
	
	entry_map.insert(210,(vec![211,212,213,214,215],"Then we regret you that you must be terminated.".to_owned())); //Well, that escalated quickly.
	entry_map.insert(211,(vec![220],"Wait! Who are you?".to_owned()));
	entry_map.insert(212,(vec![221],"I'm sorry! I'm sorry! Spare me!".to_owned()));
	entry_map.insert(213,(vec![221],"Can't we resolve this peacefully?".to_owned()));
	entry_map.insert(214,(vec![222],"O goody. This is exactly what I was hoping for!".to_owned()));
	entry_map.insert(215,(vec![223],"Fools! It is you who shall meet your maker!".to_owned()));
	
	entry_map.insert(220,(vec![230],"No more question!".to_owned()));
	entry_map.insert(221,(vec![230],"There is no forgiveness for necromancers and their ilk!".to_owned()));
	entry_map.insert(222,(vec![230],"...".to_owned()));
	entry_map.insert(223,(vec![230],"We will see about that.".to_owned()));
	
	entry_map.insert(230,(vec![FIGHT_EXIT],"...".to_owned()));
	
	entry_map.insert(20,(vec![21,73],"Is that so? In that case, may the light be with you".to_owned()));
	entry_map.insert(21,(vec![EXIT_3],"Farewell".to_owned()));
	
	entry_map.insert(70,(vec![71,72,73],"We understand. In that case, may the light be with you".to_owned()));
	entry_map.insert(71,(vec![EXIT_2],"Farewell".to_owned()));
	entry_map.insert(72,(vec![60],"Actually I think I do want to check the colour of your blood after all".to_owned()));
	entry_map.insert(73,(vec![80],"Wait!".to_owned()));
	
	entry_map.insert(80,(vec![72,81,82,83,84],"Yes?".to_owned()));
	entry_map.insert(81,(vec![90],"Who are you?".to_owned()));
	entry_map.insert(82,(vec![100],"What are you doing here?".to_owned()));
	entry_map.insert(83,(vec![EXIT_3],"Thank you for your concern! All the best!".to_owned()));
	entry_map.insert(84,(vec![110],"Can I do something to aid you?".to_owned()));
	
	entry_map.insert(90,(vec![72,84,62,84,82],"We are the exorcists of the White Temple.".to_owned()));
	entry_map.insert(10,(vec![84,71,82,11],"We are the exorcists of the White Temple. We hunt the creatures of darkness.".to_owned()));
	entry_map.insert(11,(vec![40],"Nice to meet you.".to_owned()));
	
	entry_map.insert(40,(vec![81,82,83,84],"Well met...".to_owned()));
	entry_map.insert(100,(vec![81,84,72,113,6,114],"We are hunting an evil spirit that is trying to escape its eternal damnation".to_owned()));
	entry_map.insert(110,(vec![81,82,72,111,112],"Have you heard the rumours of a necromantic cult at work in Malachia?".to_owned()));
	entry_map.insert(111,(vec![120],"Yes".to_owned()));
	entry_map.insert(112,(vec![130],"No".to_owned()));
	entry_map.insert(113,(vec![30],"Oh, I think I've met this spirit before.".to_owned()));
	entry_map.insert(114,(vec![20],"Haven't seen one...".to_owned()));
	
	entry_map.insert(120,(vec![121,122,123,62,72,124],"Could you look into it for us and cleanse it for us?".to_owned()));
	entry_map.insert(130,(vec![121,122,123,62,72,124],"Well, there seems to be a necromantic cult in downtown Malachia, could you clense it for us?".to_owned()));
	entry_map.insert(121,(vec![EXIT_4],"Yes".to_owned()));
	entry_map.insert(122,(vec![20],"No".to_owned()));
	entry_map.insert(123,(vec![160],"What's in it for me?".to_owned()));
	entry_map.insert(124,(vec![150],"What if I am one of thes necromancers?".to_owned()));
	
	
	entry_map.insert(150,(vec![151,152],"What? Is that so!".to_owned()));
	entry_map.insert(151,(vec![70],"Haha! Sorry no, that was a joke.".to_owned()));
	entry_map.insert(152,(vec![221],"Aye! In the flesh!".to_owned()));
	
	entry_map.insert(160,(vec![161,162],"A place in heaven and favour with the priesthood of the White Temple".to_owned()));
	entry_map.insert(161,(vec![EXIT_4],"Excellent!".to_owned()));
	entry_map.insert(162,(vec![70],"Uugh..".to_owned()));
	
	entry_map.insert(EXIT_4,(vec![],"Excellent. Please let out headquarters at the White Temple know once you're done.".to_owned()));
	entry_map.insert(FIGHT_EXIT,(vec![],"So be it..".to_owned())); //Fight exit.
	entry_map.insert(EXIT_1,(vec![],"May the light be with you..".to_owned())); //Refuse quest exit. 
	entry_map.insert(EXIT_2,(vec![],"...".to_owned())); //Bad terms exit.
	entry_map.insert(EXIT_3,(vec![],"Thank you. May the light be with you..".to_owned())); //Good terms exit.
	
	//Create entry content.
	let entry_content = Content {
		actors:vec![(&faces[21][0],witch().rename("The Exorcist").wm_change(2.0).mp_change(2.0).hp_change(2.0).spellist(vec![S_EXORCISM,S_GREATER_EXORCISM,S_GREATER_CURE,S_SPARK]),1),
					(&faces[20][0],warrior().rename("Paladin").diff_lvl(10).speed_change(0.7),1),
					(&faces[22][0],witch().rename("Escort").diff_lvl(10).speed_change(0.7),1)],
		tokens:Vec::new(),
		phrases_by_key: entry_map,
		entry_node: ENTRY,
		entry_description: "After the incident with the ghost, I was approached by an odd group of fellows calling themselves \"Exorcists\".",
		exit_nodes: vec![EXIT_1,EXIT_2,EXIT_3,EXIT_4,FIGHT_EXIT],
		exit_descriptions: vec!["I spoke with the exorcists. Intolerable! I hope never to see them again!",
								"I spoke with the exorcists, and I did not like what I heard.",
								"We had a pleasant conversations.",
								"I agreed to help them to rid Malachia of a necromantic cult.",
								"I spoke with the exorcists. Horrific folk! I entered battle with them."],
	};
	
	
	//FIGHT EXIT conclusion. (You fought the exorcists).
	//FIGHT EXIT dialog tree.
	let mut death_dialog:BTreeMap<u16,(Vec<u16>,String)> = BTreeMap::new();
	death_dialog.insert(FIGHT_EXIT,(vec![777],"The battle with the exorcists is lost and won...".to_owned()));
	death_dialog.insert(777,(vec![EXIT_1],"...".to_owned()));
	death_dialog.insert(EXIT_1,(vec![],"Well, what's done is done, but you have made some enemies today...".to_owned()));
		
	//FIGHT EXIT conclusion content.
	let death_content = Content {
		actors:Vec::new(),
		tokens:Vec::new(),
		phrases_by_key: death_dialog,
		entry_node: FIGHT_EXIT,
		entry_description: "I did battle with the exorcists.",
		exit_nodes: vec![EXIT_1],
		exit_descriptions: vec!["I wonder what the White Temple will do next?"],
		
	};
	
	//EXIT_4 conclusion. (You abused the exorcists and they went away).
	let mut exit4_dialog:BTreeMap<u16,(Vec<u16>,String)> = BTreeMap::new();
	exit4_dialog.insert(EXIT_4,(vec![1],"You stand among the ruins of glory.".to_owned()));
	exit4_dialog.insert(1,(vec![EXIT_1],"...".to_owned()));
	exit4_dialog.insert(EXIT_1,(vec![],"So you have come...".to_owned()));
	
	//EXIT_4 conclusion. (You ended up helping the exorcists).
	//And of course when it's finished there will be a fight.
	let ex4_content = Content {
		actors:vec![(&faces[20][0],warrior().rename("Paladin").diff_lvl(10).speed_change(0.7).spellist(vec![S_FIRE,S_EXORCISM]),1),
					(&faces[21][0],witch().rename("The Exorcist").wm_change(2.0).mp_change(2.0).hp_change(2.0).spellist(vec![S_EXORCISM,S_GREATER_EXORCISM,S_GREATER_CURE,S_SPARK]),1),
					(&faces[20][0],warrior().rename("Paladin").diff_lvl(10).speed_change(0.7).spellist(vec![S_FIRE,S_EXORCISM]),1),
					(&faces[22][0],witch().rename("Escort").diff_lvl(10).speed_change(0.7),1)],
		tokens:Vec::new(),
		phrases_by_key: exit4_dialog,
		entry_node: EXIT_4,
		entry_description: "I annihilated the cult in downtown Malachia.",
		exit_nodes: vec![EXIT_1],
		exit_descriptions: vec!["..And came to the white temple."],
		
	};
	
	//EXIT_1 conclusion. (You abused the exorcists and they went away).
	let mut exit1_dialog:BTreeMap<u16,(Vec<u16>,String)> = BTreeMap::new();
	exit1_dialog.insert(EXIT_1,(vec![1],"The exorcists left in a huff.".to_owned()));
	exit1_dialog.insert(1,(vec![EXIT_2],"...".to_owned()));
	exit1_dialog.insert(EXIT_2,(vec![],"But will they be back?..".to_owned()));
	
	//EXIT_1 conclusion. (You abused the exorcists and they went away).
	//Just a commentary.
	let ex1_content = Content {
		actors:Vec::new(),
		tokens:Vec::new(),
		phrases_by_key: exit1_dialog,
		entry_node: EXIT_1, 
		entry_description: "",
		exit_nodes: vec![EXIT_2],
		exit_descriptions: vec!["..Perhaps I should watch my back."],
		
	};
	
	//EXIT_2 conclusion. (You left these guys with a badish relation.).
	let mut exit2_dialog:BTreeMap<u16,(Vec<u16>,String)> = BTreeMap::new();
	exit2_dialog.insert(EXIT_2,(vec![1],"The exorcists didn't seem too happy.".to_owned()));
	exit2_dialog.insert(1,(vec![EXIT_1],"...".to_owned()));
	exit2_dialog.insert(EXIT_1,(vec![],"But will they be back?..".to_owned()));
	
	//EXIT_2 conclusion. (You abused the exorcists and they went away).
	//Just a commentary.
	let ex2_content = Content {
		actors:Vec::new(),
		tokens:Vec::new(),
		phrases_by_key: exit2_dialog,
		entry_node: EXIT_2, 
		entry_description: "",
		exit_nodes: vec![EXIT_1],
		exit_descriptions: vec!["..Perhaps I should watch my back."],
		
	};
	
	//EXIT_3 conclusion. (You left these guys with a badish relation.).
	let mut exit3_dialog:BTreeMap<u16,(Vec<u16>,String)> = BTreeMap::new();
	exit3_dialog.insert(EXIT_3,(vec![1],"Well, those folk seemed amiable.".to_owned()));
	exit3_dialog.insert(1,(vec![EXIT_1],"...".to_owned()));
	exit3_dialog.insert(EXIT_1,(vec![],"Perchance you shall meet again?..".to_owned()));
	
	//EXIT_3 conclusion. (You abused the exorcists and they went away).
	//Just a commentary.
	let ex3_content = Content {
		actors:Vec::new(),
		tokens:Vec::new(),
		phrases_by_key: exit3_dialog,
		entry_node: EXIT_3, 
		entry_description: "",
		exit_nodes: vec![EXIT_1],
		exit_descriptions: vec!["..Perhaps I might visit the White Temple at some point..."],
		
	};
	
	//Create end vector -NB this will crash hard the way it is. I think.
	let ends = vec![
		(EXIT_1,
		 ex1_content,
		 vec![Trigger::StartedStoryWith(667,EXIT_1)]
		),
		(EXIT_2,
		 ex2_content,
		 vec![Trigger::StartedStoryWith(667,EXIT_2)]
		),
		(EXIT_3,
		 ex3_content,
		 vec![Trigger::StartedStoryWith(667,EXIT_3)]
		),
		(EXIT_4,
		 ex4_content,
		 vec![
			Trigger::StartedStoryWith(667,EXIT_4),
			Trigger::FinishedDungeon(ID_MALACHIA_PUBCRAWL),
			Trigger::LocusXY([-160,20])]
		),
		(FIGHT_EXIT,death_content,vec![Trigger::StartedStoryWith(667,FIGHT_EXIT)])
	];
	
	//The actual story.
	Story {
		name: "Ghosthunt: Chapter 1",
		trigger: start_trigger,	
		completion: ends,
		content: entry_content,
		id:667,
	}	
}


//Fight exit from chapter 1.
//You get an invitation to a shady meeting at a shady place.
fn ghosthunt_part_2a_marker(){}
pub fn ghosthunt_part_2a<'a>(faces:&'a Vec<[conrod::image::Id;3]>)->Story<'a>  {
	let name = "Ghosthunt: Chapter 2 - A letter from the other side.";
	let start_trigger = vec![Trigger::Exp(20.0),Trigger::FinishedStoryNotWith(666,888)];
	
	//Make content.
	let mut entry_map:BTreeMap<u16,(Vec<u16>,String)> = BTreeMap::new();
	
	entry_map.insert(ENTRY,(vec![111,222],"You are approached by a lone skeleton. Outstretched hand holds an elegant envelope in your direction.".to_owned()));
	entry_map.insert(111,(vec![10],"Begone! I want nothing with you and nothing from you!".to_owned()));
	entry_map.insert(222,(vec![334],"O! I'll take that...".to_owned()));
	
	entry_map.insert(10,(vec![11,12],"The skeleton does not heed your words, but holds the letter in outstretched hands".to_owned()));
	entry_map.insert(11,(vec![FIGHT_EXIT],"I said I will have nothing with this! If you do not leave, then I shall grind your bones to dust!".to_owned()));
	entry_map.insert(12,(vec![1],"I said I will have nothing with this! If you do not leave then I will.".to_owned()));
	
	entry_map.insert(1,(vec![],"You turn around and walk away...".to_owned()));
	entry_map.insert(FIGHT_EXIT,(vec![],"You take a breath and prepare to make good on your promise...".to_owned()));
	entry_map.insert(334,(vec![],"You take the mysterious letter...".to_owned()));
	
	let entry_content = Content {
		actors:vec![(&faces[13][0],skeleton().diff_lvl(20),1)],
		tokens:Vec::new(),
		phrases_by_key:entry_map,
		entry_node: ENTRY,
		entry_description: "I had a another strange encounter today. A skeleton came to give me a letter.",
		exit_nodes: vec![1,334,FIGHT_EXIT],
		exit_descriptions: vec!["I wouldn't take the letter.\n",
								"I took its letter and read it.",
								"I would have none of it, why do these monsters keep coming to me?"],
	};
	
	//Make good exit (took the letter).
	let mut exit1_map:BTreeMap<u16,(Vec<u16>,String)> = BTreeMap::new();
	exit1_map.insert(1,(vec![2],"You hear footsteps behind you. And turn around.".to_owned()));
	exit1_map.insert(2,(vec![EXIT_1],"...".to_owned()));
	exit1_map.insert(EXIT_1,(vec![],"At a respectable distance, the skeleton kneels before you. Clearly it seems to have no intent to leave you alone.".to_owned()));
	
	let exit1_content = Content {
		actors:vec![(&faces[13][0],skeleton().diff_lvl(20),0)],
		tokens:Vec::new(),
		phrases_by_key:exit1_map,
		entry_node: 1,
		entry_description: "Now this strange messenger won't leave me alone.",
		exit_nodes: vec![EXIT_1],
		exit_descriptions: vec!["I wonder how long this will continue?"],
	};
	
	//Make ambivalent exit (the skeleton follows you forever).
	let letter = "Greetings!
    It has come to my attention that you have assisted an unfortunate soul. \
A commendable action which not everyone would take kindly to, or indeed take. \
Perhaps you may be who we have been looking for all along. Would you not come \
to the black sands on a moonlit night for a touch of sophistry?

Yours Truly,
Anthracene.
	";
	
	let mut exit334_map:BTreeMap<u16,(Vec<u16>,String)> = BTreeMap::new();
	exit334_map.insert(334,(vec![2],letter.to_owned()));
	exit334_map.insert(2,(vec![EXIT_2],"...".to_owned()));
	exit334_map.insert(EXIT_2,(vec![],"By the time you have read it, the skeleton has crumbled to dust.".to_owned()));
	
	let exit334_content = Content {
		actors:vec![(&faces[13][2],skeleton().diff_lvl(20),0)],
		tokens:Vec::new(),
		phrases_by_key:exit334_map,
		entry_node: 1,
		entry_description: "Then the skeleton cumbled to dust. By the way, the letter read:\n",
		exit_nodes: vec![EXIT_2],
		exit_descriptions: vec![letter],
	};
	
	//Make fight exit.
	let mut exitf_map:BTreeMap<u16,(Vec<u16>,String)> = BTreeMap::new();
	exitf_map.insert(334,(vec![2],letter.to_owned()));
	exitf_map.insert(2,(vec![EXIT_2],"...".to_owned()));
	exitf_map.insert(EXIT_2,(vec![],"By the time you have read it, the skeleton has crumbled to dust.".to_owned()));
	
	let exitf_content = Content {
		actors:vec![(&faces[13][0],skeleton().diff_lvl(20),0)],
		tokens:Vec::new(),
		phrases_by_key:exitf_map,
		entry_node: FIGHT_EXIT,
		entry_description: "I fought the creepy messenger. I want nothing to do with necromancers!\n",
		exit_nodes: vec![EXIT_3],
		exit_descriptions: vec![""],
	};
	
	let ends = vec![(1,exit1_content,vec![Trigger::StartedStoryWith(668,1)]),
					(334,exit334_content,vec![Trigger::StartedStoryWith(668,334)]),
					(FIGHT_EXIT,exitf_content,vec![Trigger::StartedStoryWith(668,FIGHT_EXIT)])];
					
	//The actual story.
	Story {
		name: name,
		trigger: start_trigger,
		completion: ends,
		content: entry_content,
		id: 668,
	}
}

//The long talk with Anthracene. Takes your type and
fn ghosthunt_part_2_1a_marker(){}
pub fn ghosthunt_part_2_1a<'a>(faces:&'a Vec<[conrod::image::Id;3]>,p:&Vec<(Lifeform,usize)>,pn:&Vec<String>)->Story<'a> {
	let name = "Ghosthunt: Chapter 2 - A letter from the other side II.";
	let start_trigger = vec![Trigger::LocusScape(DESERT),Trigger::LocusAffinity(DEATH),Trigger::FinishedStory(668)];
	
	//Make content.
	let mut entry_map:BTreeMap<u16,(Vec<u16>,String)> = BTreeMap::new();
	
	entry_map.insert(ENTRY,(vec![1],"As night falls over the desert of fine, black grains, you notice a shadow walking with you.".to_owned()));
	entry_map.insert(1,(vec![10],"...".to_owned()));
	entry_map.insert(10,(vec![11],format!("A pleasant evening to you, {}. ",pn[0])));
	
	let entry_content = Content {
		actors:vec![(&faces[13][0],skeleton().diff_lvl(20),1)],
		tokens:Vec::new(),
		phrases_by_key:entry_map,
		entry_node: ENTRY,
		entry_description: "When I entered the Black Sands, I had another interesting encounter. I met the sorceress Anthracene, or better put, her shadow.",
		exit_nodes: vec![1,334,FIGHT_EXIT],
		exit_descriptions: vec!["I wouldn't take the letter.\n",
								"I took its letter and read it.",
								"I would have none of it, why do these monsters keep coming to me?"],
	};
	
	let ends = vec![];
	
	//The actual story.
	Story {
		name: name,
		trigger: start_trigger,
		completion: ends,
		content: entry_content,
		id: 669,
	}
}
