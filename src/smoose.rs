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
///
///About MyStories.
///
///This is a structure with a single field- a vector of (u32,bool), ie, story
///ids and completion status. Presence of an entry signals a story has been started,
/// "true" for .1 indicates it has been concluded. It is possible that this needs to be made
///more complex.
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

use std::io;
use shared_moose::*;
#[allow(unused_imports)] use lmoose::{Spell,Item,Lifeform,Shade,Place,cureL,cure,cureG,cureH,exorcism,exorcismG,exorcismH,
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

//A structure to contain spells, prices therefore
//and fixed answers to questions.
//this structure will change with time		
#[derive(Clone,Debug)] 
pub struct Sage<'a> {
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


//A structure that stores a vector of story ids that have been
//started by the party and their status (finished or not).
#[derive(Debug)]
pub struct MyStories {
	ids:Vec<(u32,bool)>,
}

impl MyStories {
	pub fn new()-> MyStories {
		MyStories {ids: Vec::with_capacity(20)}
	}
	pub fn make_ids(&mut self,ids:Vec<(u32,bool)>) {
		self.ids = ids;
	}
	
	pub fn get_ids(&self) -> Vec<(u32,bool)> {
		self.ids.clone()
	}
	 
	pub fn len(&self)->usize {
		self.ids.len()
	}
}

// A structure to keep the whole of a plot-section.
// This includes the triggers for its start and end,
// conclusion sentence, id and "content".
// NB id is u32 to allow easy shifting between platforms (broken elsewhere).
#[derive(Debug)]
pub struct Story {
	pub trigger: Vec<Trigger>,
	pub completion: Vec<Trigger>,
	pub content: Content,
	pub conclusion: Content,
	pub id:u32,
}

//I have not decided what content should be stored as.
#[derive(Debug)]
pub struct Content {
	
}

//Trigger for the start of  story dialog can be any of the below,
//or a combination thereof. Not sure combinations will work yet.
#[derive(Debug,Clone)]
pub enum Trigger {
	HasSpell(i8),
	CastSpell(i8),
	CastSpellType(u8),
	HasItem(u8),
	UseItem(u8),
	LFType(u8),
	LFSubType(u8),
	Exp(f32),
	StartedStory(u32),
	FinishedStory(u32),
	FinishedDungeon(usize),
	Other(usize),
	Locus(Place),
	LocusType(u8),
	LocusXY([i32;2]),
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


pub fn sage_fire<'a>(mon_faces:&'a Vec<[conrod::image::Id;3]>)->Sage {
	Sage {
		exp_min: 10.0,
		face: &mon_faces[22][1],
		trigger: vec![Trigger::CastSpell(S_EMBER),Trigger::LocusType(DESERT)],
		spells: vec![S_EMBER,S_FIRE,S_FIREBALL,S_INFERNO],
		dialog_greeting:["You light a fire in the desert night and realise \
that you are not alone. The form of a nomad sits by the fire, adoring it \
like a dear child...\n...\"I am the Sage of Fire\" it states.\n".to_owned(),
"\nWhy did you call me, PLAYER? Or did you just want to stay warm?".to_owned()],
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


pub fn sage_lightning<'a>(mon_faces:&'a Vec<[conrod::image::Id;3]>)->Sage {
	Sage {
		exp_min: 50.0,
		face: &mon_faces[19][0],
		trigger: vec![Trigger::CastSpell(S_SPARK),Trigger::LocusType(DESERT)],
		spells: vec![S_SPARK,S_LIGHTNING,S_JOVIAN_LIGHTNING],
		dialog_greeting:["A single spark in the stony labyrinth...\n...Calls down lightning from a clear sky. \
The flash momentarily blinds you and you realise that this lightning is something else. \
\n...\"I am the Sage of Lighning\" it states.\n".to_owned(),
"\nWhy did you call me, PLAYER?".to_owned()],
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

pub fn sage_ice<'a>(mon_faces:&'a Vec<[conrod::image::Id;3]>)->Sage {
	Sage {
		exp_min: 50.0,
		face: &mon_faces[25][0],
		trigger: vec![Trigger::CastSpell(S_LESSER_CRYSTALLISE),Trigger::LocusType(ICE)],
		spells: vec![S_LESSER_CRYSTALLISE,S_CRYSTALLISE,S_TRUE_CRYSTALLISE],
		dialog_greeting:["You try to make the artic wastes a little colder...\n...And the air around you begins to coalesce. \
Before you stands the White Queen. \
\n...\"I am the Sage of Ice\" it states.\n".to_owned(),
"\nWhy did you call me, PLAYER?".to_owned()],
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

pub fn sage_light<'a>(mon_faces:&'a Vec<[conrod::image::Id;3]>)->Sage {
	Sage {
		exp_min: 100.0,
		face: &mon_faces[26][0],
		trigger: vec![Trigger::CastSpell(S_LIGHT),Trigger::LocusType(HOLY),Trigger::LocusType(RADIANT)],
		spells: vec![S_LIGHT,S_SACRED_LIGHT,S_EXORCISM,S_GREATER_EXORCISM,S_SACRED_EXORCISM],
		dialog_greeting:["As you cast your spell, a shining figure approaches you from the mists. \
Serene, it walks over the water. As it approaches your ship, it says: \
\n...\"I am the Sage of Light\" it states.\n".to_owned(), 
"\nWhy did you call me, PLAYER?".to_owned()],
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



pub fn sage_darkness<'a>(mon_faces:&'a Vec<[conrod::image::Id;3]>)->Sage {
	Sage {
		exp_min: 100.0,
		face: &mon_faces[26][0],
		trigger: vec![Trigger::CastSpell(S_DARKNESS),Trigger::LocusXY([-100,-10])],
		spells: vec![S_DARKNESS,S_ABYSSAL_DARKNESS,S_CURSE],
		dialog_greeting:["As darkness descends upon the Black Obelisk, you feel a presence besides you. \
Standing at your side, darker than black, it utters: \
\n...\"I am the Sage of Darkness\".\n".to_owned(),
"\nWhy did you call me, PLAYER?".to_owned(),
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

pub fn sage_life<'a>(mon_faces:&'a Vec<[conrod::image::Id;3]>)->Sage {
	Sage {
		exp_min: 100.0,
		face: &mon_faces[26][0],
		trigger: vec![Trigger::CastSpell(S_LESSER_CURE),Trigger::LocusXY([-100,60])],
		spells: vec![S_LESSER_CURE,S_CURE,S_GREATER_CURE,S_SACRED_CURE],
		dialog_greeting:["You cast your spell amongst the harsh winds of the White Island. \
Someone stands before you. \
\n...\"I am the Sage of Light\" it utters.\n".to_owned(),
"\nWhy did you call me, PLAYER?".to_owned(),
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

pub fn sage_death<'a>(mon_faces:&'a Vec<[conrod::image::Id;3]>)->Sage {
	Sage {
		exp_min: 200.0,
		face: &mon_faces[13][2],
		trigger: vec![Trigger::CastSpell(S_EXORCISM),Trigger::LocusXY([-40,-30])],
		spells: vec![S_DARKNESS,S_ABYSSAL_DARKNESS,S_CURSE,S_SUMMON_REAPER,S_SWORD_OF_PERSEUS,S_LIFESTEALER],
		dialog_greeting:["You finish exorcising the crypts and the dead lie still as they should. Then they rise again. \
The corpses bow as one and speak as a raspy chorus: \
\n...\"I am the Sage of Death\"\n".to_owned(),
"\nWhy did you call me, PLAYER?".to_owned(),
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


pub fn sage_albion<'a>(mon_faces:&'a Vec<[conrod::image::Id;3]>)->Sage {
	Sage {
		exp_min: 200.0,
		face: &mon_faces[26][0],
		trigger: vec![Trigger::CastSpell(S_LIGHT),Trigger::LocusXY([-160,20])],
		spells: vec![S_HASTE,S_SLOW,S_TIMESTOP,S_TELEPORT,S_GREATER_TELEPORT],
		dialog_greeting:["O Albion! Is this your former splendour?..
You illuminate the empty temples with your spell and see a young man stride towards you.
\n...\"I am the Sage of Albion\" he states.\n".to_owned(),
"\nWelcome to the Albion of my youth, PLAYER. My apologies for this poor hospitality.".to_owned()
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

pub fn sage_malachia<'a>(mon_faces:&'a Vec<[conrod::image::Id;3]>)->Sage {
	Sage {
		exp_min: 1000.0,
		face: &mon_faces[20][1],
		trigger: vec![Trigger::CastSpell(S_SUMMON_REAPER),Trigger::LocusXY([60,30])],
		spells: vec![S_APOCALYPSE],
		dialog_greeting:["You summon death incarnate at the square of Malachia...
The reaper stands motionless, observing the sky, then it turns to face you.
\n...\"I am the Sage of Malachia\" it states.\n".to_owned(),
"\nHas the time finally come, PLAYER?".to_owned()
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


