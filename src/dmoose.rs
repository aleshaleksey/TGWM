#![allow(non_snake_case)]
#![allow(unused_imports)]

/// Quest for the Moose: dmoose
///
///This is the Quest for the Moose module where the dungeons are stored.
///Dungeons have a name, coordinates (to show which world map square they
///can be accessed from, difficulty (which does nothing), an affinity,
///which also does nothing at the moment, a sequence of locations
///where battles take place, a special bestiary and an afterstory for
///when the dungeon is defeated.
///
///Dungeons are stored here because they take a lot of lines and lmoose
///is getting excessively long. Since each dungeon is a function, they
///can in fact undergo all sorts of teansformation depending on various
///parameters (such as Pplayer experience).
///
///This module depends on the lmoose module.
///
///~Alek Zholobenko
///
///


//mod lmoose;
extern crate rand;
use lmoose::{Spell,Item,Lifeform,Shade,Place,Dungeon,cureL,cure,cureG,cureH,exorcism,exorcismG,exorcismH,
			 ember,fire,fireball,inferno,spark,lightning,lightningH,crystalliseL,crystallise,crystalliseH,
			 sum_reaper,teleport,teleportG,light,lightH,darkness,darknessH,slow,haste,lifestealer,curse,
			 apocalypse,timestop,world,goblin_dem,goblin_sco,goblin_witch,bandit,bandit_lord,dark_apprentice,
			 necromancer,necromancer_lord,skeleton,skeleton_kn,ghost,ghost_an,white_witch,beast_green,
			 beast_red,beast_great,fallen,titan,warrior,witch,wonderer,alien,loser,beast_serpent,sage_forsaken,
			 white_queen,shortstaff,index_arcana,tree_of_life};
use lmoose::{ADVENT,ALBION,ALIEN,ANGEL,BEAST,BONE,BRIDGE,CITY,
		     DEATH,DESERT,ELF,EVIL,FALLEN,FIRE,FOREST,GIANT,GOBLIN,GRASSLAND,
		     HEALING,HIGHLAND,HOLY,HUMAN,ICE,LIGHTNING,MALACHIA,
			 MINDLESS,MOORLAND,MOOSE,RADIANT,RUIN,STEPPE,SPIRIT,
			 TELEPORTATION,TIME,TUNDRA,UNDEAD,VOID,WATER,WITCH,WHITE,NONE,
			 S_LESSER_CURE,S_CURE,S_GREATER_CURE,S_SACRED_CURE,S_INFERNO,S_FIREBALL,S_FIRE,S_EMBER,
			 S_LESSER_CRYSTALLISE,S_CRYSTALLISE,S_TRUE_CRYSTALLISE,S_EXORCISM,S_GREATER_EXORCISM,S_SACRED_EXORCISM,
			 S_SUMMON_REAPER,S_TELEPORT,S_GREATER_TELEPORT,S_LIGHT,S_SACRED_LIGHT,S_DARKNESS,S_ABYSSAL_DARKNESS,
			 S_SLOW,S_HASTE,S_APOCALYPSE,S_GENESIS,S_SPARK,S_LIGHTNING,S_JOVIAN_LIGHTNING,S_TIMESTOP,
			 S_CURSE,S_LIFESTEALER,S_DAGGER_OF_FAWN,S_BOW_OF_TRAVELLER,S_SWORD_OF_PERSEUS};
use rand::Rng;


pub const ID_CITADEL_OF_SPIRIT:u32 = 0;
pub const ID_DOOR_TO_DARKNESS:u32 = 1;
pub const ID_ELVEN_LAKE_RUINS:u32 = 2;
pub const ID_HALL_OF_STONE:u32 = 3;
pub const ID_ICE_PALACE:u32 = 4;
pub const ID_LOST_LIGHTHOUSE:u32 = 5;
pub const ID_MALACHIA_PUBCRAWL:u32 = 6;
pub const ID_MALEK_GROVE:u32 = 7;
pub const ID_MONSTER_HALL:u32 = 8;
pub const ID_ON_THE_PRAIRIE:u32 = 9;
pub const ID_PETRIFIED_SHRINE:u32 = 10;
pub const ID_STAIRWAY:u32 = 11;
pub const ID_THE_PATH:u32 = 12;
pub const ID_TOWER_OF_FLESH:u32 = 13;
pub const ID_TOWER_OF_BONES:u32 = 14;
pub const ID_TOWER_OF_SOUL:u32 = 15;
pub const ID_WAY_DOWN:u32 = 16;
pub const ID_WHITE_TEMPLE:u32 = 17;
pub const ID_WILD_HUNT:u32 = 18;
pub const ID_WITCH_MAZE:u32 = 19;

pub fn malek_grove()-> Dungeon {
	Dungeon {
		id: ID_MALEK_GROVE,
		name: "Malek Grove",
		xy: [40,50],
		diff: 30.0,
		affinity: EVIL,
		scenes: vec![
			Place { name: "Nightshade Gate",	scape: FOREST,		xy: [40,50],		affinity: EVIL,
					engenG: [4,4,4,4,4,4,4,4,4,4],
					engenA: [1,1,1,1,1,1,1,1,1,1],
					popu:vec![("Demented Goblin",UNDEAD,100),
							  ("Goblin Witch",UNDEAD,100),
							  ("Malekian Gatekeeper",UNDEAD,400)],
			},
			Place { name: "Rhododendron Gardens",	scape: FOREST,		xy: [40,50],		affinity: EVIL,
					engenG: [2,2,2,4,4,4,4,4,4,4],
					engenA: [1,1,1,1,1,1,1,1,2,2],
					popu:vec![("Demented Goblin",UNDEAD,1000),
							  ("Goblin Witch",UNDEAD,1000),
							  ("Malekian Desciple",UNDEAD,4000),
							  ("Ghost",UNDEAD,1000)],
			},Place { name: "Melia Hall",	scape: FOREST,		xy: [40,50],		affinity: EVIL,
					engenG: [1,1,1,1,1,1,1,1,1,1],
					engenA: [1,1,1,1,1,1,1,1,1,1],
					popu:vec![("Leftenant Bones",UNDEAD,100)],
			},
			Place { name: "Yew Alley",	scape: FOREST,		xy: [40,50],		affinity: EVIL,
					engenG: [4,4,4,4,4,4,4,4,4,4],
					engenA: [1,1,1,1,1,1,1,1,1,1],
					popu:vec![("Malekian Beast",UNDEAD,100),
							  ("Ancient Ghost",UNDEAD,100),
							  ("Malekian Desciple",UNDEAD,400)],
			},
			Place { name: "Strychnos Hall",	scape: RUIN,		xy: [40,50],		affinity: EVIL,
					engenG: [1,1,1,1,1,1,1,1,1,1],
					engenA: [1,1,1,1,1,1,1,1,1,1],
					popu:vec![("Lord Malek",UNDEAD,1)],
			}
		],
		denizens: vec![
						goblin_dem().clone().attack_change(2.0),
						goblin_witch().clone().bm_change(2.0).spellist(vec![S_FIRE,S_SPARK,S_CURE]),
						ghost().clone().attack_change(1.5),
						ghost_an().clone().attack_change(1.5),
						necromancer_lord().clone().hp_change(3.0).wm_change(1.5).rename("Lord Malek").spellist(vec![S_FIREBALL,S_CURSE,S_SUMMON_REAPER,S_LIFESTEALER]),
						warrior().clone().attack_change(2.0).rename("Malekian Gatekeeper"),
						witch().clone().diff_lvl(10).rename("Malekian Desciple").spellist(vec![S_EMBER,S_CURSE,S_CURE]),
						beast_red().clone().attack_change(2.0).rename("Malekian Beast"),
						skeleton_kn().clone().diff_lvl(10).bm_change(3.0).rename("Leftenant Bones").spellist(vec![S_LESSER_CRYSTALLISE])
				  ],
		afterstory: MALEK_GROVE,
	}
}

pub fn monster_hall()-> Dungeon {

	Dungeon {
		id: ID_MONSTER_HALL,
		name: "Monster Hall",
		xy: [140,50],
		diff: 50.0,
		affinity: GOBLIN,
		scenes: vec![
			Place { name: "Goblin Sector",	scape: CITY,		xy: [140,50],		affinity: GOBLIN,
					engenG: [2,2,3,3,3,4,4,4,4,4],
					engenA: [1,1,1,1,1,2,2,2,2,3],
					popu:vec![("Goblin Guardian",GOBLIN,1000),
							  ("Obese Goblin",GOBLIN,1000),
							  ("Goblin Witch",GOBLIN,200),
							  ("Goblin Scout",GOBLIN,100),
							  ("Warrior",ADVENT,200),
							  ("Witch",ADVENT,200),
							  ("Wonderer",ADVENT,200)],
			},
			Place { name: "Orcish Sector",	scape: CITY,		xy: [140,50],		affinity: GOBLIN,
					engenG: [2,2,3,3,3,4,4,4,4,4],
					engenA: [1,1,1,1,1,2,2,2,2,3],
					popu:vec![("Goblin Witch",GOBLIN,100),
							  ("Goblin Scout",GOBLIN,100),
							  ("Orc",GOBLIN,2000),
							  ("Greater Orc",GOBLIN,500),
							  ("Warrior",ADVENT,200),
							  ("Witch",ADVENT,200),
							  ("Wonderer",ADVENT,200)],
			},
			Place { name: "Beast Sector",	scape: CITY,		xy: [140,50],		affinity: GOBLIN,
					engenG: [2,2,3,3,3,4,4,4,4,4],
					engenA: [1,1,1,1,1,2,2,2,2,3],
					popu:vec![("Goblin Witch",GOBLIN,100),
							  ("Goblin Scout",GOBLIN,100),
							  ("Red Beast",GOBLIN,2000),
							  ("Green Beast",GOBLIN,500),
							  ("Young Serpent",GOBLIN,500),
							  ("Warrior",ADVENT,200),
							  ("Witch",ADVENT,200),
							  ("Wonderer",ADVENT,200)],
			},
			Place { name: "Gobholme Walls",	scape: CITY,		xy: [140,50],		affinity: GOBLIN,
					engenG: [4,4,4,4,4,4,4,4,4,4],
					engenA: [1,1,1,1,1,1,1,1,1,1],
					popu:vec![("Goblin Guardian",GOBLIN,100),
							  ("Goblin Witch",GOBLIN,100),
							  ("Red Beast",GOBLIN,2000),
							  ("Young Serpent",GOBLIN,500)],
			},
			Place { name: "Outer Court",	scape: RUIN,		xy: [140,50],		affinity: GOBLIN,
					engenG: [4,4,4,4,4,4,4,4,4,4],
					engenA: [1,1,1,1,1,1,1,1,1,1],
					popu:vec![("Goblin Guardian",GOBLIN,1000),
							  ("Goblin Hero",GOBLIN,200),
							  ("Goblin Witch",GOBLIN,200)],
			},
			Place { name: "Inner Court",	scape: RUIN,		xy: [140,50],		affinity: GOBLIN,
					engenG: [4,4,4,4,4,4,4,4,4,4],
					engenA: [1,1,1,1,1,1,1,1,1,1],
					popu:vec![("Goblin Guardian",GOBLIN,1000),
							  ("Goblin Hero",GOBLIN,100),
							  ("Goblin Elder",GOBLIN,200)],
			},
			Place { name: "Gobholme Throne",	scape: RUIN,		xy: [140,50],		affinity: GOBLIN,
					engenG: [1,1,1,1,1,1,1,1,1,1],
					engenA: [1,1,1,1,1,1,1,1,1,1],
					popu:vec![("Queen Goblin",GOBLIN,1000)],
			},
			Place { name: "Gobholme Throne",	scape: RUIN,		xy: [140,50],		affinity: GOBLIN,
					engenG: [1,1,1,1,1,1,1,1,1,1],
					engenA: [1,1,1,1,1,1,1,1,1,1],
					popu:vec![("Elf",GOBLIN,1000)],
			}
		],
		denizens: vec![
						goblin_dem().clone().attack_change(2.0).hp_change(4.0).rename("Obese Goblin"),
						goblin_dem().clone().diff_lvl(20).rename("Goblin Guardian"),
						goblin_sco().clone().diff_lvl(10),
						goblin_dem().clone().diff_lvl(100).rename("Goblin Hero"),
						beast_red().clone().spellist(Vec::new()).speed_change(0.7).rename("Orc"),
						beast_red().clone().diff_lvl(10).speed_change(0.5).spellist(vec![S_EMBER]).rename("Greater Orc"),
						goblin_witch().clone().bm_change(2.0).spellist(vec![S_FIRE,S_SPARK,S_CURE]),
						goblin_witch().clone().diff_lvl(15).spellist(vec![S_LIGHTNING,S_GREATER_CURE,S_CURSE]).rename("Goblin Elder"),
						goblin_witch().clone().bm_change(2.0).wm_change(2.0).diff_lvl(15).spellist(vec![S_LIGHTNING,S_GREATER_CURE,S_CURSE]).rename("Queen Goblin"),
						ghost().clone().attack_change(1.5),
						beast_red().clone(),
						beast_green().clone(),
						beast_serpent().clone().speed_change(2.0).diff_lvl(-7).rename("Young Serpent"),
						warrior().clone().diff_lvl(5),
						wonderer().clone().diff_lvl(5),
						witch().clone().diff_lvl(5),
						sage_forsaken().clone().speed_change(3.0).hp_change(2.5).diff_lvl(-4).rename("Elf")
				  ],
		afterstory: MONSTER_HALL,
	}
}

pub fn citadel_of_spirit(player:Lifeform)-> Dungeon {
	
	let player = player.clone();

	Dungeon {
		id: ID_CITADEL_OF_SPIRIT,
		name: "the Citadel of Spirit",
		xy: [0,50],
		diff: 108.0,
		affinity: SPIRIT,
		scenes: vec![
			Place { name: "Witches' Moor",	scape: MOORLAND,		xy: [0,50],		affinity: WITCH,
					engenG: [2,3,3,3,4,4,4,4,4,4],
					engenA: [1,1,1,1,1,2,2,2,2,3],
					popu:vec![("Witch",WITCH,1000),
							  ("Dark Apprentice",WITCH,1000),
							  ("Goblin Witch",WITCH,1000),
							  ("White Witch",WITCH,1000)],
			},
			Place { name: "Reedy Moat",	scape: MOORLAND,		xy: [0,50],		affinity: UNDEAD,
					engenG: [2,3,3,3,4,4,4,4,4,4],
					engenA: [1,1,1,1,1,2,2,2,4,3],
					popu:vec![("Hero's Bones",UNDEAD,1000),
							  ("Ghost",UNDEAD,1000)],
			},
			Place { name: "the Vieled Doorway",	scape: RUIN,		xy: [0,50],		affinity: SPIRIT,
					engenG: [4,4,4,4,4,4,4,4,4,4],
					engenA: [1,1,1,1,1,1,1,1,1,1],
					popu:vec![("Shade",SPIRIT,1000),
							  ("Gatekeeper",SPIRIT,2000)],
			},
			Place { name: "the Borderlands",	scape: FOREST,		xy: [0,50],		affinity: SPIRIT,
					engenG: [4,4,4,4,4,4,4,4,4,4],
					engenA: [1,1,1,1,1,1,1,1,2,2],
					popu:vec![("Shade",SPIRIT,3000),
							  ("Great Beast",SPIRIT,500),
							  ("Ghost",HUMAN,500),
							  ("Lost Witch",HUMAN,150)],
			},
			Place { name: "the Cold River",	scape: WATER,		xy: [0,50],		affinity: SPIRIT,
					engenG: [1,1,1,1,1,1,1,1,1,1],
					engenA: [1,1,1,1,1,1,1,1,1,1],
					popu:vec![("Michael",SPIRIT,3000)],
			},
			Place { name: "the Distant Shore",	scape: GRASSLAND,		xy: [0,50],		affinity: SPIRIT,
					engenG: [1,1,1,1,1,1,1,1,1,1],
					engenA: [1,1,1,1,1,2,2,2,3,3],
					popu:vec![("Shade",SPIRIT,3000),
							  ("Ancient Spirit",SPIRIT,3000),
							  ("Hero's Ghost",HUMAN,200),
							  ("Villain's Ghost",UNDEAD,200)],
			},
			Place { name: "Yester-year",	scape: RUIN,		xy: [0,50],		affinity: SPIRIT,
					engenG: [1,1,1,1,1,2,2,2,2,3],
					engenA: [1,1,1,1,1,1,1,2,2,3],
					popu:vec![("Elf",ELF,1000),
							  ("Ancient Spirit",SPIRIT,1000),
							  ("Forsaken Sage",HUMAN,200)],
			},
			Place { name: "the Brilliant Road",	scape: VOID,		xy: [0,50],		affinity: SPIRIT,
					engenG: [1,1,1,1,1,1,1,1,1,1],
					engenA: [1,1,1,1,1,1,1,1,1,1],
					popu:vec![("Your Better Self",SPIRIT,1000)],
			}
		],
		denizens: vec![
						witch().clone().wm_change(1.3).bm_change(1.3),
						white_witch().clone().wm_change(1.3).bm_change(1.3),
						goblin_witch().clone().wm_change(1.3).bm_change(1.3),
						dark_apprentice().clone().wm_change(1.3).bm_change(1.3),
						ghost().clone().wm_change(1.3).bm_change(1.3),
						beast_great().clone().wm_change(1.3).bm_change(1.3),
						skeleton().clone().diff_lvl(50).wm_change(1.3).speed_change(0.5).bm_change(1.3).rename("Hero's Bones"),
						white_witch().clone().wm_change(4.0).bm_change(1.0).spellist(vec![S_EXORCISM,S_GREATER_EXORCISM,S_CURSE]).rename("Gatekeeper"),
						ghost().clone().wm_change(3.0).re_type(SPIRIT).spellist(vec![S_SLOW,S_ABYSSAL_DARKNESS]).rename("Shade"),
						witch().clone().wm_change(1.3).bm_change(1.3).rename("Lost Witch"),
						fallen().clone().wm_change(1.5).diff_lvl(10).spellist(vec![S_JOVIAN_LIGHTNING,S_INFERNO,S_SACRED_LIGHT,S_GREATER_CURE]).rename("Michael"),
						ghost_an().clone().wm_change(1.3).re_type(SPIRIT).spellist(vec![S_TIMESTOP,S_CURSE,S_ABYSSAL_DARKNESS]).rename("Ancient Spirit"),
						ghost().clone().hp_change(1.3).attack_change(1.3).wm_change(3.0).re_type(SPIRIT).spellist(vec![S_CURE,S_LIGHT]).rename("Hero's Ghost"),
						ghost().clone().hp_change(1.3).attack_change(4.0).wm_change(1.3).re_type(SPIRIT).spellist(vec![S_FIRE,S_DARKNESS]).rename("Villain's Ghost"),
						sage_forsaken().clone().speed_change(3.0).hp_change(2.5).diff_lvl(-4).rename("Elf"),
						sage_forsaken().clone(),
						player.diff_lvl(10).rename("Your Better Self") //player should be cloned initially.
				  ],
		afterstory: CITADEL_OF_SPIRIT,
	}
}

pub fn elven_lake_ruins()-> Dungeon {

	Dungeon {
		id: ID_ELVEN_LAKE_RUINS,
		name: "Elven Ruins",
		xy:[40,-70],
		diff: 30.0,
		affinity: RADIANT,
		scenes: vec![
			Place { name: "Mangrove Harbour",	scape: WATER,		xy: [40,-70],		affinity: RADIANT,
					engenG: [2,3,3,3,4,4,4,4,4,4],
					engenA: [1,1,1,1,1,2,2,2,2,3],
					popu:vec![("Green Beast",BEAST,1000),
							  ("Elven Ghost",UNDEAD,1000),
							  ("Elven Dead",UNDEAD,1000),
							  ("Lesser Guardian Serpent",ELF,200)],
			},
			Place { name: "Garden of Larch",	scape: FOREST,		xy: [40,-70],		affinity: RADIANT,
					engenG: [2,3,3,3,4,4,4,4,4,4],
					engenA: [1,1,1,1,1,2,2,2,2,3],
					popu:vec![("Green Beast",BEAST,1000),
							  ("Elven Ghost",UNDEAD,1000),
							  ("Elven Dead",UNDEAD,1000)],
			},
			Place { name: "Moracea Village",	scape: RUIN,		xy: [40,-70],		affinity: RADIANT,
					engenG: [2,3,3,3,4,4,4,4,4,4],
					engenA: [1,1,1,1,1,2,2,2,2,3],
					popu:vec![("Green Beast",BEAST,1000),
							  ("Elven Ghost",UNDEAD,1000),
							  ("Elven Dead",UNDEAD,1000),
							  ("Guardian Spirit",ELF,1000)],
			},
			Place { name: "Moracea Tower",	scape: RUIN,		xy: [40,-70],	affinity: RADIANT,
					engenG: [1,1,1,1,1,1,1,1,1,1],
					engenA: [1,1,1,1,1,1,1,1,1,1],
					popu:vec![("Princess Mora",UNDEAD,1000)],
			}
		],
		denizens: vec![
						beast_green().clone().wm_change(1.3).bm_change(1.3).spellist(vec![S_CURE,S_EXORCISM,S_SPARK,S_HASTE]),
						ghost().clone().wm_change(2.0).bm_change(1.3).spellist(vec![S_CURE,S_CURSE]).rename("Elven Ghost"),
						skeleton().clone().speed_change(4.0).wm_change(2.0).bm_change(1.3).spellist(vec![S_HASTE,S_CURE]).rename("Elven Dead"),
						beast_serpent().clone().wm_change(2.0).bm_change(0.7).spellist(vec![S_LIGHT,S_EXORCISM]).re_type(SPIRIT).rename("Lesser Guardian Serpent"),
						ghost_an().clone().wm_change(2.0).re_type(SPIRIT).spellist(vec![S_SLOW,S_LIGHT,S_LIGHTNING,S_EXORCISM]).rename("Guardian Spirit"),
						warrior().clone().diff_lvl(20).wm_change(3.0).mp_change(5.0).bm_change(3.3).spellist(vec![S_HASTE,S_CURE,S_SWORD_OF_PERSEUS]).re_type(UNDEAD).rename("Princess Mora")
				  ],
		afterstory: ELVEN_LAKE_RUINS,
	}
}

pub fn malachia_pubcrawl()-> Dungeon {

	Dungeon {
		id: ID_MALACHIA_PUBCRAWL,
		name: "Downtown Malachia",
		xy:[60,30],
		diff: 10.0,
		affinity: MALACHIA,
		scenes: vec![
			Place { name: "The Shiv",	scape: CITY,		xy: [60,30],		affinity: MALACHIA,
					engenG: [2,3,3,3,4,4,4,4,4,4],
					engenA: [1,1,1,1,1,2,2,2,2,3],
					popu:vec![("Bandit",HUMAN,1000),
							  ("Bandit Lord",HUMAN,50)],
			},
			Place { name: "The Shiv",	scape: CITY,		xy: [60,30],		affinity: MALACHIA,
					engenG: [1,1,1,1,1,1,1,1,1,1],
					engenA: [1,1,1,1,1,1,1,1,1,1],
					popu:vec![("One Eyed Bill",HUMAN,1000)],
			},
			Place { name: "The Outer Circle",	scape: CITY,		xy: [60,30],		affinity: MALACHIA,
					engenG: [2,3,3,3,4,4,4,4,4,4],
					engenA: [1,1,1,1,1,2,2,2,2,3],
					popu:vec![("Dark Apprentice",HUMAN,1000),
							  ("Necromancer",HUMAN,50)],
			},
			Place { name: "The Outer Circle",	scape: CITY,		xy: [60,30],		affinity: MALACHIA,
					engenG: [1,1,1,1,1,1,1,1,1,1],
					engenA: [1,1,1,1,1,1,1,1,1,1],
					popu:vec![("Necromancer Lord",HUMAN,1000)],
			},
			Place { name: "The Yellow Cauldron",	scape: CITY,		xy: [60,30],		affinity: MALACHIA,
					engenG: [2,3,3,3,4,4,4,4,4,4],
					engenA: [1,1,1,1,1,1,1,1,1,1],
					popu:vec![("Necromancer",HUMAN,333),
							  ("Skeleton",HUMAN,666)],
			},
			Place { name: "The Falling Keg",	scape: CITY,		xy: [60,30],		affinity: MALACHIA,
					engenG: [2,3,3,3,4,4,4,4,4,4],
					engenA: [1,1,1,1,1,1,1,1,1,1],
					popu:vec![("Witch",HUMAN,333),
							  ("Warrior",HUMAN,666),
							  ("Wonderer",HUMAN,666)],
			},
			Place { name: "The Falling Keg",	scape: CITY,		xy: [60,30],		affinity: MALACHIA,
					engenG: [1,1,1,1,1,1,1,1,1,1],
					engenA: [1,1,1,1,1,1,1,1,1,1],
					popu:vec![("Fallen One",HUMAN,333)],
			},
		],
		denizens: vec![
						bandit().clone(),
						bandit_lord().clone(),
						bandit_lord().clone().diff_lvl(10).speed_change(0.7).spellist(vec![S_FIRE,S_SLOW]).rename("One Eyed Bill"),
						dark_apprentice().clone(),
						necromancer().clone(),
						skeleton().clone(),
						necromancer_lord().clone(),
						witch().clone().re_type(EVIL),
						warrior().clone().re_type(EVIL),
						wonderer().clone().re_type(EVIL),
						fallen().clone()
				  ],
		afterstory: MALACHIA_PUBCRAWL,
	}
}

pub fn lost_lighthouse()-> Dungeon {

	Dungeon {
		id: ID_LOST_LIGHTHOUSE,
		name: "the Lost Lighthouse",
		xy: [-100,-60],
		diff: 40.0,
		affinity: RADIANT,
		scenes: vec![
			Place { name: "the Courtyard",	scape: DESERT,		xy: [-100,-60],		affinity: RADIANT,
					engenG: [4,4,4,4,4,4,4,4,4,4],
					engenA: [4,4,4,4,4,4,4,4,4,4],
					popu:vec![("Skeletal Beast",UNDEAD,1000)],
			},
			Place { name: "the Lower Level",	scape: RUIN,		xy: [-100,-60],		affinity: RADIANT,
					engenG: [1,1,1,1,1,1,1,1,1,1],
					engenA: [4,4,4,4,4,4,4,4,4,4],
					popu:vec![("Necromancer Lord",UNDEAD,1000)],
			},
			Place { name: "the Upper Level",	scape: RUIN,		xy: [-100,-60],		affinity: RADIANT,
					engenG: [4,4,4,4,4,4,4,4,4,4],
					engenA: [1,1,1,1,1,1,1,1,1,1],
					popu:vec![("Fallen One",ANGEL,1000),
							  ("Void Lord",ANGEL,200)],
			},
			Place { name: "the Beacon",	scape: RUIN,		xy: [-100,-60],		affinity: RADIANT,
					engenG: [1,1,1,1,1,1,1,1,1,1],
					engenA: [1,1,1,1,1,1,1,1,1,1],
					popu:vec![("The Last Angel",ANGEL,1000)],
			}
		],
		denizens: vec![
						skeleton_kn().clone().hp_change(3.0).spellist(Vec::new()).rename("Skeletal Beast"),
						necromancer_lord().clone(),
						fallen().clone(),
						fallen().clone().hp_change(3.0).bm_change(1.5).wm_change(1.5).rename("Void Lord"),
						fallen().clone().re_type(ANGEL).wm_change(5.0).defence_change(5.0)
								.spellist(vec![S_SACRED_LIGHT,S_GREATER_CURE,S_SACRED_EXORCISM,S_SWORD_OF_PERSEUS,S_INFERNO]).rename("The Last Angel")
				  ],
		afterstory: LOST_LIGHTHOUSE,
	}
}


//NB: Black door dungeon must take a non-zero length vector. Which should be updated upon start of battle using player's party.
pub fn door_to_darkness(party:&Vec<(Lifeform,usize)>)-> Dungeon {

	let doppel_a:Lifeform = party[0].0.clone();
	let doppel_b:Lifeform = if party.len()>1 {party[0].0.clone()}else{alien().clone()};
	Dungeon {
		id: ID_DOOR_TO_DARKNESS,
		name: "the Black Door",
		xy: [-100,-10],
		diff: 80.0,
		affinity: EVIL,
		scenes: vec![
			Place { name: "the Black Doorway",	scape: RUIN,		xy: [-100,-10],		affinity: EVIL,
					engenG: [4,4,4,4,4,4,4,4,4,4],
					engenA: [4,4,4,4,4,4,4,4,4,4],
					popu:vec![("Familiar Shadow",UNDEAD,1000)],
			},
			Place { name: "the Hall of Echoes",	scape: RUIN,		xy: [-100,-10],		affinity: EVIL,
					engenG: [4,4,4,4,4,4,4,4,4,4],
					engenA: [4,4,4,4,4,4,4,4,4,4],
					popu:vec![("Ghost",UNDEAD,1000),
							  ("Ancient Ghost",UNDEAD,300),
							  ("Echo",UNDEAD,1000)],
			},
			Place { name: "the Hall of Shadows",	scape: RUIN,		xy: [-100,-10],		affinity: EVIL,
					engenG: [4,4,4,4,4,4,4,4,4,4],
					engenA: [4,4,4,4,4,4,4,4,4,4],
					popu:vec![("Ghost",UNDEAD,1000),
							  ("Ancient Ghost",UNDEAD,300),
							  ("Shadow",UNDEAD,1000)],
			},
			Place { name: "the Hall of Dreams",	scape: RUIN,		xy: [-100,-10],		affinity: EVIL,
					engenG: [4,4,4,4,4,4,4,4,4,4],
					engenA: [4,4,4,4,4,4,4,4,4,4],
					popu:vec![("Echo",UNDEAD,1000),
							  ("Nightmare",UNDEAD,300),
							  ("Shadow",UNDEAD,1000)],
			},
			Place { name: "the Hall of Dreams",	scape: RUIN,		xy: [-100,-10],		affinity: EVIL,
					engenG: [4,4,4,4,4,4,4,4,4,4],
					engenA: [4,4,4,4,4,4,4,4,4,4],
					popu:vec![("Echo",UNDEAD,1000),
							  ("Nightmare",UNDEAD,300),
							  ("Shadow",UNDEAD,1000)],
			},
			Place { name: "the Empty Hall",	scape: RUIN,		xy: [-100,-10],		affinity: EVIL,
					engenG: [1,1,1,1,1,1,1,1,1,1],
					engenA: [1,1,1,1,1,1,1,1,1,1],
					popu:vec![("Anthracene",UNDEAD,1000)],
			},
			Place { name: "Anthracene's Mirror",	scape: TIME,		xy: [-100,-10],		affinity: DEATH,
					engenG: [1,1,1,1,1,1,1,1,1,1],
					engenA: [2,2,2,2,2,2,2,2,2,2],
					popu:vec![("True Reflection",UNDEAD,1000),
							  ("False Reflection",UNDEAD,1000)],
			}
		],
		denizens: vec![
						doppel_a.clone().diff_lvl(-2).re_type(EVIL).rename("Familiar Shadow"),
						ghost().re_type(EVIL).clone(),
						ghost_an().re_type(EVIL).clone(),
						doppel_a.clone().speed_change(2.0).re_type(EVIL).spellist(vec![S_EXORCISM,S_CURSE]).rename("Echo"),
						doppel_a.clone().attack_change(3.0).re_type(EVIL).spellist(vec![S_ABYSSAL_DARKNESS,S_LIFESTEALER]).rename("Shadow"),
						titan().clone().hp_change(2.0).bm_change(3.0).re_type(EVIL).spellist(vec![S_ABYSSAL_DARKNESS,S_LIFESTEALER]).rename("Nightmare"),
						white_queen().clone().wm_change(1.5).bm_change(3.0).hp_change(2.0).re_type(EVIL).spellist(vec![S_LIFESTEALER,S_SUMMON_REAPER,S_TIMESTOP]).rename("Anthracene"),
						doppel_a.clone().diff_lvl(10).re_type(EVIL).rename("True Reflection"),
						doppel_b.clone().diff_lvl(10).re_type(EVIL).rename("False Reflection"),
				  ],
		afterstory: DOOR_TO_DARKNESS,
	}
}

pub fn white_temple()-> Dungeon {

	Dungeon {
		id: ID_WHITE_TEMPLE,
		name: "the White Temple",
		xy: [-160,20],
		diff: 60.0,
		affinity: TIME,
		scenes: vec![
			Place { name: "the White Gate",	scape: TIME,		xy: [-160,20],		affinity: TIME,
					engenG: [4,4,4,4,4,4,4,4,4,4],
					engenA: [1,1,1,1,1,1,1,1,1,1],
					popu:vec![("Marble Knight",WHITE,1000)],
			},
			Place { name: "Chapel of Yesterday",	scape: TIME,		xy: [-160,20],		affinity: TIME,
					engenG: [4,4,4,4,4,4,4,4,4,4],
					engenA: [1,1,1,1,1,1,1,1,1,1],
					popu:vec![("Ancient Ghost",WHITE,1000)],
			},
			Place { name: "Chapel of Today",	scape: TIME,		xy: [-160,20],		affinity: TIME,
					engenG: [4,4,4,4,4,4,4,4,4,4],
					engenA: [1,1,1,1,1,1,2,2,2,2],
					popu:vec![("Wonderer",WHITE,1000),
							  ("Bandit Lord",HUMAN,1000)],
			},
			Place { name: "Chapel of Tomorrow",	scape: TIME,		xy: [-160,20],		affinity: TIME,
					engenG: [4,4,4,4,4,4,4,4,4,4],
					engenA: [1,1,1,1,1,1,2,2,2,2],
					popu:vec![("Wonderer",WHITE,1000),
							  ("Bandit Lord",HUMAN,1000)],
			},
			Place { name: "The Sage's Alter",	scape: TIME,		xy: [-160,20],		affinity: TIME,
					engenG: [2,2,2,2,2,2,2,2,2,2],
					engenA: [1,1,1,1,1,1,1,1,1,1],
					popu:vec![("Fallen One",ANGEL,1000),
							  ("Falling One",ANGEL,1000)],
			},
			Place { name: "Albion Clocktower",	scape: TIME,		xy: [-160,20],		affinity: TIME,
					engenG: [1,1,1,1,1,1,1,1,1,1],
					engenA: [1,1,1,1,1,1,1,1,1,1],
					popu:vec![("Sage's Afterimage",ANGEL,1000)],
			},
		],		
		denizens: vec![
						warrior().clone().hp_change(2.0).wm_change(2.0).defence_change(2.0).spellist(vec![S_HASTE]).rename("Marble Knight"),
						ghost_an().clone().diff_lvl(5),
						wonderer().clone().diff_lvl(10).spellist(vec![S_HASTE,S_SLOW,S_FIRE]),
						bandit_lord().clone(),
						fallen().clone(),
						fallen().clone().wm_change(1.5).spellist(vec![S_TIMESTOP,S_LIGHTNING,S_LIFESTEALER,S_CURSE,S_SUMMON_REAPER]).rename("Falling One"),
						sage_forsaken().clone().diff_lvl(4).rename("Sage's Afterimage"),
				  ],
		afterstory: WHITE_TEMPLE,
	}			
}


//Place{name: "Heaven's Stairway",scape: RUIN,		xy: [80,-70],		affinity: ANGEL,
	//engenG: [1,1,2,2,2,3,3,3,4,4],
	//engenA: [1,1,1,2,2,2,3,3,3,4],
    //popu:vec![("Fallen One",ANGEL,5000),
			  //("Ancient Ghost",UNDEAD,1000),
			  //("Serpent",BEAST,500)],},
pub fn stairway()-> Dungeon {

	Dungeon {
		id: ID_STAIRWAY,
		name: "the Stairway",
		xy: [80,-70],
		diff: 120.0,
		affinity: TIME,
		scenes: vec![
			Place { name: "the Atrium of Dreams",	scape: RUIN,		xy: [80,-70],		affinity: HOLY,
					engenG: [1,2,3,4,4,4,4,4,4,4],
					engenA: [1,1,1,1,1,1,2,2,3,4],
					popu:vec![("Serpent",WHITE,1000)],
			},
			Place { name: "the Landing of Sweat",	scape: RUIN,		xy: [80,-70],		affinity: BEAST,
					engenG: [1,2,3,4,4,4,4,4,4,4],
					engenA: [1,1,1,1,1,1,2,2,3,4],
					popu:vec![("Serpent",WHITE,1000),
							  ("Fallen One",FALLEN,500)],
			},
			Place { name: "the Landing of Blood",	scape: RUIN,		xy: [80,-70],		affinity: FIRE,
					engenG: [1,2,3,4,4,4,4,4,4,4],
					engenA: [1,1,1,1,1,1,2,2,3,4],
					popu:vec![("Serpent",WHITE,500),
							  ("Fallen One",FALLEN,1000),
							  ("Forsaken Sage",SPIRIT,250)],
			},
			Place { name: "the Landing of Tears",	scape: RUIN,		xy: [80,-70],		affinity: EVIL,
					engenG: [1,2,3,4,4,4,4,4,4,4],
					engenA: [1,1,1,1,1,1,2,2,3,4],
					popu:vec![("Serpent",WHITE,250),
							  ("Fallen One",FALLEN,1000),
							  ("Falling One",ANGEL,500),
							  ("Saint",ANGEL,500),
							  ("Forsaken Sage",SPIRIT,250)],
			},
			Place { name: "the Broken Gates",	scape: RUIN,		xy: [80,-70],		affinity: ANGEL,
					engenG: [1,1,1,1,1,1,1,1,1,1],
					engenA: [4,4,4,4,4,4,4,4,4,4],
					popu:vec![("Heaven's Sword",ANGEL,1000),
							  ("Heaven's Shield",ANGEL,1000),
							  ("Heaven's Mercy",ANGEL,1000),
							  ("Heaven's Wrath",ANGEL,1000)],
			},
			Place { name: "the Barren Heaven",	scape: VOID,		xy: [80,-70],		affinity: ANGEL,
					engenG: [1,1,1,1,1,1,1,1,1,1],
					engenA: [1,1,1,1,1,1,1,1,1,1],
					popu:vec![("the Star of Madness",ANGEL,1000)],
			}
			
		],		
		denizens: vec![
						beast_serpent().clone(),
						fallen().clone(),
						fallen().clone().wm_change(1.5).rename("Falling One")
								.spellist(vec![S_TIMESTOP,S_LIGHTNING,S_LIFESTEALER,S_CURSE,S_SUMMON_REAPER]),
						sage_forsaken().clone(),
						warrior().clone().diff_lvl(40).speed_change(0.2).re_type(ANGEL)
								 .spellist(vec![S_GREATER_CURE,S_GREATER_EXORCISM,S_SACRED_LIGHT]).rename("Saint"),	 
						warrior().clone().hp_change(5.0).attack_change(10.0).speed_change(2.0)
								 .spellist(Vec::new()).re_type(ANGEL).rename("Heaven's Sword"),		 
						warrior().clone().hp_change(10.0).attack_change(2.0).defence_change(10.0)
								 .spellist(Vec::new()).re_type(ANGEL).rename("Heaven's Shield"),
						beast_great().clone().hp_change(2.0).wm_change(2.0).bm_change(5.0)
								 .spellist(vec![S_JOVIAN_LIGHTNING,S_INFERNO]).re_type(ANGEL).rename("Heaven's Wrath"),
						beast_great().clone().hp_change(2.0).mp_change(4.0).wm_change(5.0).bm_change(2.0)
								 .spellist(vec![S_SACRED_EXORCISM,S_GREATER_CURE,S_SUMMON_REAPER]).mp_change(4.0).re_type(ANGEL).rename("Heaven's Mercy"),
						ghost_an().clone().hp_change(10.0).mp_change(10.0).wm_change(4.0).bm_change(4.0).re_type(ANGEL).rename("the Star of Madness")
								  .spellist(vec![S_EMBER,S_FIRE,S_FIREBALL,S_INFERNO,S_LIFESTEALER,S_TIMESTOP,S_SACRED_EXORCISM])
						],
		afterstory: STAIRWAY,
	}			
}


pub fn witch_maze()-> Dungeon {

	Dungeon {
		id: ID_WITCH_MAZE,
		name: "the Witch's Maze",
		xy: [-140,10],
		diff: 40.0,
		affinity: WITCH,
		scenes: vec![
			Place { name: "Witch's Garden",	scape: GRASSLAND,		xy: [-140,10],		affinity: WITCH,
					engenG: [4,4,4,4,4,4,4,4,4,4],
					engenA: [1,1,1,1,1,1,1,1,1,1],
					popu:vec![("Green Beast",WITCH,1000),
							  ("Goblin Witch",WITCH,1000)],
			},
			Place { name: "Green Gazebo",	scape: GRASSLAND,		xy: [-140,10],		affinity: WITCH,
					engenG: [1,1,1,1,1,1,1,1,1,1],
					engenA: [1,1,1,1,1,1,1,1,1,1],
					popu:vec![("Green Witch",WITCH,1000)],
			},
			Place { name: "Witch's Woods",	scape: FOREST,		xy: [-140,10],		affinity: WITCH,
					engenG: [4,4,4,4,4,3,3,3,3,3],
					engenA: [1,1,1,1,1,1,1,2,2,3],
					popu:vec![("Green Beast",WITCH,1000),
							  ("Goblin Witch",WITCH,1000),
							  ("Ghost",WITCH,1000),
							  ("Ancient Ghost",WITCH,200)],
			},
			Place { name: "Witch's Woods",	scape: FOREST,		xy: [-140,10],		affinity: WITCH,
					engenG: [4,4,4,4,4,3,3,3,3,3],
					engenA: [1,1,1,1,1,1,1,2,2,3],
					popu:vec![("Green Beast",WITCH,1000),
							  ("Goblin Witch",WITCH,1000),
							  ("Ghost",WITCH,1000),
							  ("Ancient Ghost",WITCH,200)],
			},
			Place { name: "the Lemon Tree",	scape: FOREST,		xy: [-140,10],		affinity: WITCH,
					engenG: [1,1,1,1,1,1,1,1,1,1],
					engenA: [1,1,1,1,1,1,1,1,1,1],
					popu:vec![("Sour Witch",WITCH,1000)],
			},
			Place { name: "the Crystal Grove",	scape: ICE,		xy: [-140,10],		affinity: WITCH,
					engenG: [4,4,4,4,4,4,4,4,4,4],
					engenA: [1,1,1,1,1,1,1,2,2,3],
					popu:vec![("White Witch",WITCH,1000),
							  ("Ghost",UNDEAD,1000),
							  ("Ghost",UNDEAD,1000),
							  ("Great Beast",WITCH,1000),],
			},
			Place { name: "the Crystal Palace",	scape: ICE,		xy: [-140,10],		affinity: WITCH,
					engenG: [4,4,4,4,4,4,4,4,4,4],
					engenA: [1,1,1,1,1,1,1,1,1,1],
					popu:vec![("Guardian of the Crystal",WITCH,1000)],
			},
			Place { name: "the Crystal Ball",	scape: RUIN,		xy: [-140,10],		affinity: WITCH,
					engenG: [1,1,1,1,1,1,1,1,1,1],
					engenA: [1,1,1,1,1,1,1,1,1,1],
					popu:vec![("the Lost Witch",WITCH,1000)],
			},
			
		],		
		denizens: vec![
						beast_green().clone(),
						ghost().clone(),
						ghost_an().clone(),
						titan().clone().wm_change(2.0).rename("Guardian of the Crystal"),
						goblin_witch().clone().spellist(vec![S_SLOW,S_SPARK,S_LIFESTEALER]),
						goblin_witch().clone().wm_change(2.0).bm_change(2.0).spellist(vec![S_SLOW,S_LIGHTNING,S_LIFESTEALER]).rename("Green Witch"),
						white_witch().clone().wm_change(2.0).bm_change(2.0).spellist(vec![S_EMBER,S_CURSE,S_LIFESTEALER]).rename("Sour Witch"),
						witch().clone().speed_change(0.2).diff_lvl(40).spellist(vec![S_EMBER,S_CURE,S_SPARK,S_INFERNO,S_TIMESTOP]).rename("the Lost Witch"),
						],
		afterstory: WITCH_MAZE,
	}			
}

pub fn way_down()-> Dungeon {

	Dungeon {
		id: ID_WAY_DOWN,
		name: "the Way Down",
		xy: [20,70],
		diff: 60.0,
		affinity: TIME,
		scenes: vec![
			Place { name: "The Great Well",					scape: RUIN,		xy: [20,70],		affinity: GIANT,
					engenG: [4,4,4,4,4,4,4,4,4,4],
					engenA: [1,1,1,1,1,1,1,1,1,1],
					popu:vec![("Guardian Titan",GIANT,1000)],
			},
			Place { name: "The Slippery Slope",				scape: HIGHLAND,		xy: [20,70],		affinity: EVIL,
					engenG: [2,3,4,4,4,4,4,4,4,4],
					engenA: [1,1,1,1,1,1,2,2,3,3],
					popu:vec![("Fallen One",ANGEL,1000),
							  ("Lost Soul",UNDEAD,1000),
							  ("Titan",GIANT,1000)],
			},
			Place { name: "The Road of Good Intentions",	scape: RUIN,		xy: [20,70],		affinity: EVIL,
					engenG: [2,3,4,4,4,4,4,4,4,4],
					engenA: [1,1,1,1,1,1,1,1,2,3],
					popu:vec![("Guardian Titan",GIANT,1000),
							  ("Lost Soul",GIANT,1000),],
			},
			Place { name: "Rubicon",						scape: RUIN,		xy: [20,70],		affinity: EVIL,
					engenG: [2,3,4,4,4,4,4,4,4,4],
					engenA: [1,1,1,1,1,1,2,2,3,3],
					popu:vec![("Fallen One",ANGEL,1000),
							  ("Ancient Ghost",UNDEAD,1000),
							  ("Guardian Spirit",GIANT,1000)],
			},
			Place { name: "Point of no Return",				scape: VOID,		xy: [20,70],		affinity: EVIL,
					engenG: [1,1,1,1,1,1,1,1,1,1],
					engenA: [1,1,1,1,1,1,1,1,1,1],
					popu:vec![("the Emptiness",ANGEL,1000)],
			},
			
		],		
		denizens: vec![
						titan().clone().defence_change(1.5).wm_change(1.5).spellist(vec![S_EXORCISM]).rename("Guardian Titan"),
						ghost().clone().hp_change(3.0).rename("Lost Soul"),
						titan().clone(),
						fallen().clone(),
						ghost_an().clone().hp_change(2.0),
						ghost_an().clone().hp_change(3.0).wm_change(2.0).re_type(SPIRIT).spellist(vec![S_LIGHT,S_LIGHTNING,S_GREATER_EXORCISM]).rename("Guardian Spirit"),
						ghost_an().clone().hp_change(2.0).diff_lvl(10).re_type(EVIL).rename("the Emptiness"),						
					  ],
		afterstory: WAY_DOWN,
	}
}

pub fn wild_hunt()-> Dungeon {
	let hunting:usize = rand::thread_rng().gen_range(1,15);
	let mut dungeon = Dungeon {
		id: ID_WILD_HUNT,
		name: "the Wild Hunt",
		xy: [-100,60],
		diff: 100.0,
		affinity: TIME,
		scenes: vec![
			Place { name: "the Chase",					scape: FOREST,		xy: [-100,60],		affinity: SPIRIT,
					engenG: [2,3,3,4,4,4,4,4,4,4],
					engenA: [1,1,1,1,1,1,1,2,2,3],
					popu:vec![("Moonhound",SPIRIT,1000),
							  ("Bloodhound",SPIRIT,1000),
							  ("Wild Tracker",SPIRIT,1000),
							  ("Green Beast",SPIRIT,1000),
							  ("Great Beast",SPIRIT,1000),
							  ("Wild Herald",SPIRIT,1000),
							  ("Wildman",SPIRIT,1000)],
			};hunting],		
		denizens: vec![
						goblin_dem().clone().diff_lvl(40).speed_change(0.7).re_type(SPIRIT).rename("Wildman"),
						goblin_dem().clone().diff_lvl(30).speed_change(0.7).attack_change(4.0).re_type(SPIRIT).rename("Bloodhound"),
						ghost().clone().diff_lvl(30).speed_change(0.7).attack_change(3.0).spellist(vec![]).re_type(SPIRIT).rename("Moonhound"),
						bandit_lord().clone().diff_lvl(20).re_type(SPIRIT).rename("Wild Tracker"),
						bandit().clone().diff_lvl(40).spellist(vec![S_SLOW]).re_type(SPIRIT).rename("Wild Herald"),
						beast_green().clone().diff_lvl(20).spellist(vec![S_EXORCISM]).re_type(SPIRIT),
						beast_great().clone().diff_lvl(20).spellist(vec![S_CURE,S_LIGHTNING,S_EXORCISM]).re_type(SPIRIT),		
						wonderer().clone().diff_lvl(50).spellist(vec![S_HASTE,S_CRYSTALLISE,S_GREATER_EXORCISM,S_BOW_OF_TRAVELLER]).re_type(SPIRIT).rename("Artemis"),				
					  ],
		afterstory: WILD_HUNT,
	};
	dungeon.scenes.push(Place { name: "the Chase",					scape: FOREST,		xy: [-100,60],		affinity: SPIRIT,
					engenG: [1,1,1,1,1,1,1,1,1,1],
					engenA: [1,1,1,1,1,1,1,1,1,1],
					popu:vec![("Artemis",SPIRIT,1000)],
			});
	dungeon
}

pub fn tower_of_bones()-> Dungeon {
	
	Dungeon {
		id: ID_TOWER_OF_BONES,
		name: "Tower of Bones",
		xy: [-40,-10],
		diff: 40.0,
		affinity: TIME,
		scenes: vec![
			Place { name: "Ground Level",					scape: CITY,		xy: [-40,-10],		affinity: UNDEAD,
					engenG: [2,3,3,4,4,4,4,4,4,4],
					engenA: [4,4,4,4,4,4,4,4,4,4],
					popu:vec![("Serpent's Bones",BONE,1000),
							  ("Skeletal Knight Lord",UNDEAD,1000),
							  ("Giant's Bones",GIANT,1000)],
			},	
			Place { name: "First Level",					scape: CITY,		xy: [-40,-10],		affinity: UNDEAD,
					engenG: [2,3,3,4,4,4,4,4,4,4],
					engenA: [4,4,4,4,4,4,4,4,4,4],
					popu:vec![("Scholar's Bones",UNDEAD,1000),
							  ("Skeletal Knight Lord",UNDEAD,1000)],
			},	
			Place { name: "Second Level",					scape: CITY,		xy: [-40,-10],		affinity: UNDEAD,
					engenG: [2,3,3,4,4,4,4,4,4,4],
					engenA: [2,2,2,2,2,2,2,2,2,2],
					popu:vec![("Hero's Bones",UNDEAD,1000),
							  ("Villain's Bones",UNDEAD,1000)],
			},	
			Place { name: "Third Level",					scape: CITY,		xy: [-40,-10],		affinity: UNDEAD,
					engenG: [2,3,3,4,4,4,4,4,4,4],
					engenA: [2,2,2,2,2,2,2,2,2,2],
					popu:vec![("Hero's Bones",UNDEAD,1000),
							  ("Villain's Bones",UNDEAD,1000)],
			},	
			Place { name: "Fourth Level",					scape: CITY,		xy: [-40,-10],		affinity: UNDEAD,
					engenG: [4,4,4,4,4,4,4,4,4,4],
					engenA: [4,4,4,4,4,4,4,4,4,4],
					popu:vec![("Royal Skeleton",UNDEAD,1000),
							  ("Slave's Bones",UNDEAD,1000)],
			},	
			Place { name: "The Top",					scape: CITY,		xy: [-40,-10],		affinity: UNDEAD,
					engenG: [1,1,1,1,1,1,1,1,1,1],
					engenA: [1,1,1,1,1,1,1,1,1,1],
					popu:vec![("Bones of the Tower",UNDEAD,1000)],
			},
		],	
		denizens: vec![
						skeleton_kn().clone().diff_lvl(10).hp_change(5.0).rename("Serpent's Bones"),
						skeleton_kn().clone().diff_lvl(5).attack_change(2.0).spellist(vec![S_ABYSSAL_DARKNESS]).rename("Skeletal Knight Lord"),
						skeleton_kn().clone().defence_change(3.0).hp_change(10.0).rename("Giant's Bones"),
						wonderer().clone().re_type(UNDEAD).bm_change(5.0).re_type(UNDEAD).spellist(vec![S_EXORCISM,S_FIREBALL]).rename("Scholar's Bones"),
						skeleton().clone().diff_lvl(50).defence_change(2.0).speed_change(0.5).rename("Hero's Bones"),
						skeleton().clone().diff_lvl(50).attack_change(2.0).speed_change(0.5).rename("Villain's Bones"),
						skeleton().clone().rename("Slave's Bones"),
						skeleton().clone().hp_change(10.0).rename("Royal Skeleton"),
						titan().clone().re_type(UNDEAD).hp_change(20.0).attack_change(2.0).rename("Bones of the Tower")
					  ],
		afterstory: TOWER_OF_BONES,
	}
}

pub fn tower_of_flesh()-> Dungeon {
	
	Dungeon {
		id: ID_TOWER_OF_FLESH,
		name: "Tower of Flesh",
		xy: [-40,-30],
		diff: 20.0,
		affinity: TIME,
		scenes: vec![
			Place { name: "The Blue Gate",						scape: CITY,		xy: [-40,-30],		affinity: UNDEAD,
					engenG: [1,1,1,1,1,1,1,1,1,1],
					engenA: [1,1,1,1,1,1,1,1,1,1],
					popu:vec![("Beast of Flesh",UNDEAD,1000)],
			},	
			Place { name: "First Atrium",						scape: CITY,		xy: [-40,-30],		affinity: UNDEAD,
					engenG: [4,4,4,4,4,4,4,4,4,4],
					engenA: [4,4,4,4,4,4,4,4,4,4],
					popu:vec![("Walking Dead",UNDEAD,1000)],
			},
			Place { name: "First Ventricle",					scape: CITY,		xy: [-40,-30],		affinity: UNDEAD,
					engenG: [4,4,4,4,4,4,4,4,4,4],
					engenA: [4,4,4,4,4,4,4,4,4,4],
					popu:vec![("Walking Dead",UNDEAD,1000)],
			},
			Place { name: "Second Atrium",						scape: CITY,		xy: [-40,-30],		affinity: UNDEAD,
					engenG: [4,4,4,4,4,4,4,4,4,4],
					engenA: [4,4,4,4,4,4,4,4,4,4],
					popu:vec![("Mage's Corpse",UNDEAD,1000)],
			},
			Place { name: "Second Ventricle",					scape: CITY,		xy: [-40,-30],		affinity: UNDEAD,
					engenG: [2,3,4,4,4,4,4,4,4,4],
					engenA: [3,4,4,4,4,4,4,4,4,4],
					popu:vec![("Crawling Horror",UNDEAD,1000)],
			},
			Place { name: "The Red Gate",						scape: CITY,		xy: [-40,-30],		affinity: UNDEAD,
					engenG: [1,1,1,1,1,1,1,1,1,1],
					engenA: [1,1,1,1,1,1,1,1,1,1],
					popu:vec![("The Tower's Flesh",UNDEAD,1000)],
			},
		],	
		denizens: vec![
						beast_great().clone().hp_change(5.0).re_type(UNDEAD).spellist(vec![S_LIFESTEALER]).rename("Beast of Flesh"),
						goblin_dem().clone().hp_change(5.0).re_type(UNDEAD).rename("Walking Dead"),
						dark_apprentice().clone().bm_change(3.0).hp_change(15.0).re_type(UNDEAD).spellist(vec![S_FIRE,S_LIFESTEALER,S_SLOW]).rename("Mage's Corpse"),
						beast_serpent().clone().hp_change(2.0).re_type(UNDEAD).spellist(vec![]).rename("Crawling Horror"),
						alien().clone().hp_change(50.0).re_type(UNDEAD).rename("The Tower's Flesh"),
					  ],
		afterstory: TOWER_OF_FLESH,
	}
}

pub fn tower_of_soul(party:&Vec<(Lifeform,usize)>) -> Dungeon {

	Dungeon {
		id: ID_TOWER_OF_SOUL,
		name: "Tower of Soul",
		xy: [-60,-20],
		diff: 40.0,
		affinity: UNDEAD,
		scenes: vec![
			Place { name: "The Empty Window",							scape: VOID,		xy: [-60,-20],		affinity: UNDEAD,
					engenG: [1,1,1,1,1,1,1,1,1,1],
					engenA: [4,4,4,4,4,4,4,4,4,4],
					popu:vec![("Shadow of Thyself",UNDEAD,1000)],
			},
			Place { name: "The Ephemeral Hall",							scape: VOID,		xy: [-60,-20],		affinity: UNDEAD,
					engenG: [4,4,4,4,4,4,4,4,4,4],
					engenA: [4,4,4,4,4,4,4,4,4,4],
					popu:vec![("Phantasm",UNDEAD,1000),
							  ("Dark Shadow",UNDEAD,1000)],
			},
			Place { name: "The Hall of Yesterday's Ghosts",				scape: TIME,		xy: [-60,-20],		affinity: UNDEAD,
					engenG: [2,2,2,2,3,3,3,3,4,4],
					engenA: [4,4,4,4,4,4,4,4,4,4],
					popu:vec![("Ancient Ghost",UNDEAD,1000)],
			},
			Place { name: "The Staircase of Lost Dreams",				scape: TIME,		xy: [-60,-20],		affinity: UNDEAD,
					engenG: [1,1,1,1,1,1,1,1,1,1],
					engenA: [1,1,1,1,1,1,1,1,1,1],
					popu:vec![("Chronicler of Dreams",UNDEAD,1000)],
			},
			Place { name: "The Hall of Sins",							scape: VOID,		xy: [-60,-20],		affinity: UNDEAD,
					engenG: [4,4,4,4,4,4,4,4,4,4],
					engenA: [4,4,4,4,4,4,4,4,4,4],
					popu:vec![("Sinner's Ghost",UNDEAD,1000),
							  ("Angel's Ghost",SPIRIT,1000)],
			},	
			Place { name: "The Inescapable Circle",						scape: TIME,		xy: [-60,-20],		affinity: UNDEAD,
					engenG: [1,1,1,1,1,1,1,1,1,1],
					engenA: [1,1,1,1,1,1,1,1,1,1],
					popu:vec![("Tower of Soul",UNDEAD,1000)],
			},				
		],	
		denizens: vec![
						loser().clone().mirror(party[0].0.clone()).diff_lvl(-5).re_type(UNDEAD).rename("Shadow of Thyself"),
						ghost().clone().diff_lvl(10).rename("Phantasm"),
						ghost_an().clone().rename("Dark Shadow"),
						ghost_an().clone(),
						necromancer_lord().clone().wm_change(2.0).hp_change(2.0).spellist(vec![S_EXORCISM,S_GREATER_CURE,S_FIRE,S_JOVIAN_LIGHTNING]).rename("Chronicler of Dreams"),
						ghost().clone().attack_change(3.0).rename("Sinner's Ghost"),
						beast_red().clone().diff_lvl(10).speed_change(0.7).spellist(vec![S_EXORCISM,S_FIRE,S_LIGHTNING]).rename("Angel's Ghost"),
						sage_forsaken().clone().hp_change(10.0).diff_lvl(5).speed_change(0.3).re_type(UNDEAD).rename("Tower of Soul")
											   .spellist(vec![S_TIMESTOP,S_CURSE,S_GREATER_CURE])
					  ],
		afterstory: TOWER_OF_SOUL,
	}
}


pub fn hall_of_stone() -> Dungeon {

	Dungeon {
		id: ID_HALL_OF_STONE,
		name: "Halls of Stone",
		xy: [180,-60],
		diff: 70.0,
		affinity: RUIN,
		scenes: vec![
			Place { name: "Outer Wards",							scape: DESERT,		xy: [180,-60],		affinity: GOBLIN,
					engenG: [1,1,2,2,2,2,3,3,3,4],
					engenA: [1,1,1,2,2,2,2,3,3,4],
					popu:vec![("Goblin Scout",GOBLIN,2000),
							  ("Goblin Witch",GOBLIN,1000),
							  ("Ghost",UNDEAD,200),
							  ("Skeletal Knight",UNDEAD,200)
					],
			},
			Place { name: "Inner Wards",							scape: HIGHLAND,		xy: [180,-60],		affinity: GOBLIN,
					engenG: [1,1,2,2,2,2,3,3,3,4],
					engenA: [1,2,2,2,2,3,3,3,4,4],
					popu:vec![("Goblin Scout",GOBLIN,500),
							  ("Goblin Witch",GOBLIN,1000),
							  ("Green Beret",GOBLIN,1000),
							  ("Goblin Oracle",GOBLIN,200),
							  ("Ghost",UNDEAD,200),
							  ("Skeletal Knight",UNDEAD,200),
							  ("Necromancer",UNDEAD,200),
					],
			},
			Place { name: "Hall of the Ancients",					scape: RUIN,		xy: [180,-60],		affinity: BONE,
					engenG: [1,1,2,2,2,2,3,3,3,4],
					engenA: [2,2,2,2,2,2,2,2,2,2],
					popu:vec![("Green Beret",GOBLIN,500),
							  ("Goblin Oracle",GOBLIN,500),
							  ("Ancient Bones",UNDEAD,200),
							  ("Skeletal Knight",UNDEAD,200),
							  ("Serpent",BEAST,100),
					],
			},
			Place { name: "Hall of the Giants",						scape: RUIN,		xy: [180,-60],		affinity: GIANT,
					engenG: [1,1,2,2,2,2,3,3,3,4],
					engenA: [2,2,2,2,2,3,3,3,4,4],
					popu:vec![("Green Beret",GOBLIN,200),
							  ("Goblin Oracle",GOBLIN,200),
							  ("Ancient Bones",UNDEAD,400),
							  ("Serpent",GIANT,400),
							  ("Titan",GIANT,400),
							  ("Titan of Soul",GIANT,400),
					],
			},
			Place { name: "Hall of the King",						scape: RUIN,		xy: [180,-60],		affinity: SPIRIT,
					engenG: [1,1,1,1,1,1,1,1,1,1],
					engenA: [1,1,1,1,1,1,1,1,1,1],
					popu:vec![("King of Stone",SPIRIT,100),
					],
			},
			Place { name: "Hall of the Records",						scape: RUIN,		xy: [180,-60],		affinity: SPIRIT,
					engenG: [1,1,1,1,1,1,1,1,1,1],
					engenA: [1,1,1,1,2,3,2,3,1,2],
					popu:vec![("Witch",ADVENT,500),
							  ("Necromancer",ADVENT,500),
							  ("Sage's Afterimage",SPIRIT,100),
					],
			},
			Place { name: "Hall of the Golden Land",					scape: RUIN,		xy: [180,-60],		affinity: ANGEL,
					engenG: [1,1,1,1,1,1,1,1,1,1],
					engenA: [1,1,1,1,1,1,1,1,1,1],
					popu:vec![("King of Ancients",SPIRIT,500)
					],
			},
			
		],	
		denizens: vec![
						goblin_sco().diff_lvl(10).spellist(vec![S_HASTE,S_EMBER]),
						goblin_sco().rename("Green Beret").diff_lvl(20).spellist(vec![S_HASTE,S_SPARK,S_DAGGER_OF_FAWN]),
						goblin_witch().diff_lvl(10),
						goblin_witch().rename("Goblin Oracle").diff_lvl(15)
									  .bm_change(1.5)
									  .spellist(vec![S_LIGHTNING,S_SPARK,S_GREATER_CURE,S_SWORD_OF_PERSEUS]),
						ghost(),
						ghost_an().rename("King of Ancients").defence_change(2.0).wm_change(2.0).attack_change(3.0)
								  .spellist(vec![S_SWORD_OF_PERSEUS]),
						skeleton_kn().defence_change(2.0).hp_change(1.5),
						skeleton().rename("Ancient Bones").magic_up(10.0).bm_change(2.0).diff_lvl(40)
								  .spellist(vec![S_LESSER_CRYSTALLISE]),
						necromancer().bm_change(3.0).mp_change(2.0),
						beast_serpent(),
						titan(),
						sage_forsaken().rename("Titan of Soul").hp_change(2.0).defence_change(1.5),
						sage_forsaken().rename("Sage's Afterimage").diff_lvl(4).rename("Sage's Afterimage"),
						warrior().rename("King of Stone")
								 .defence_change(4.0).attack_change(2.0).wm_change(4.0).bm_change(4.0).hp_change(5.0)
								 .spellist(vec![S_SWORD_OF_PERSEUS]).re_type(SPIRIT),
						witch().diff_lvl(20).spellist(vec![S_CRYSTALLISE,S_FIRE,S_CURE,S_SLOW]),
					  ],
		afterstory: HALL_OF_STONE,
	}
}

pub fn the_path() -> Dungeon {

	Dungeon {
		id: ID_THE_PATH,
		name: "The Path to the Holy Peak",
		xy: [-20,70],
		diff: 20.0,
		affinity: HOLY,
		scenes: vec![
			Place { name: "The Valley Floor",							scape: STEPPE,		xy: [-20,70],		affinity: HOLY,
					engenG: [1,1,2,2,2,2,3,3,3,4],
					engenA: [1,1,1,2,2,2,2,3,3,4],
					popu:vec![("Green Beast",BEAST,1000),
							  ("Dark Apprentice",EVIL,2000),
							  ("Lost Soul",SPIRIT,2000),
					],
			},
			Place { name: "The Valley Slope",							scape: FOREST,		xy: [-20,70],		affinity: HOLY,
					engenG: [1,1,2,2,2,2,3,3,3,4],
					engenA: [1,1,1,2,2,2,2,3,3,4],
					popu:vec![("Green Beast",BEAST,1000),
							  ("Wonderer",EVIL,2000),
							  ("Goblin Scout",EVIL,2000),
							  ("Lost Soul",EVIL,2000),
					],
			},
			Place { name: "The Subglacial Cliffs",						scape: HIGHLAND,		xy: [-20,70],		affinity: HOLY,
					engenG: [1,1,2,2,2,2,3,3,3,4],
					engenA: [1,1,1,2,2,2,2,3,3,4],
					popu:vec![("Red Beast",BEAST,1000),
							  ("Bandit Lord",EVIL,2000),
							  ("Warrior",ADVENT,2000),
							  ("Lost Soul",EVIL,2000),
					],
			},
			Place { name: "The Glacial Zone",							scape: ICE,		xy: [-20,70],		affinity: HOLY,
					engenG: [1,1,2,2,2,2,3,3,3,4],
					engenA: [1,1,1,2,2,2,2,3,3,4],
					popu:vec![("White Witch",WITCH,1000),
							  ("Great Beast",EVIL,2000),
							  ("Lost Soul",EVIL,2000),
					],
			},
			Place { name: "The Lonely Summit",							scape: ICE,		xy: [-20,70],		affinity: HOLY,
					engenG: [4,4,4,4,4,4,4,4,4,4],
					engenA: [4,4,4,4,4,4,4,4,4,4],
					popu:vec![("Lost Soul",EVIL,2000),
					],
			},
		],	
		denizens: vec![
						dark_apprentice().diff_lvl(10).speed_change(0.6),
						white_witch(),
						warrior().diff_lvl(10),
						wonderer().diff_lvl(10),
						bandit_lord().diff_lvl(10),
						beast_green().attack_change(2.0),
						beast_red().attack_change(2.0),
						beast_great().attack_change(2.0),
						goblin_sco().diff_lvl(10).speed_change(0.6),
						loser().rename("Lost Soul").hp_change(20.0).attack_change(5.0).re_type(UNDEAD),
		],
						
		afterstory: THE_PATH,
	}
}

pub fn on_the_prairie() -> Dungeon {

	Dungeon {
		id: ID_ON_THE_PRAIRIE,
		name: "A House on the Steppe",
		xy: [-80,10],
		diff: 20.0,
		affinity: UNDEAD,
		scenes: vec![
			Place { name: "The Farmstead",								scape: STEPPE,		xy: [-80,10],		affinity: UNDEAD,
					engenG: [4,4,4,4,4,4,4,4,4,4],
					engenA: [4,4,4,4,4,4,4,4,4,4],
					popu:vec![("Livestock",UNDEAD,1000),
					],
			},
			Place { name: "The Porch",									scape: RUIN,		xy: [-80,10],		affinity: UNDEAD,
					engenG: [1,1,2,2,2,2,3,3,3,4],
					engenA: [1,1,1,1,1,1,1,1,1,1],
					popu:vec![("Dog",BEAST,1000),
					],
			},
			Place { name: "The Homestead",							scape: RUIN,		xy: [-80,10],		affinity: UNDEAD,
					engenG: [1,2,2,2,2,2,2,2,2,2],
					engenA: [1,1,1,1,1,1,1,1,1,1],
					popu:vec![("Settler",UNDEAD,1000),
					],
			},
		],	
		denizens: vec![
						skeleton_kn().hp_change(4.0).rename("Livestock"),
						beast_great().hp_change(1.5).rename("Dog"),
						ghost().rename("Settler").hp_change(4.0).spellist(vec![S_FIRE,S_CURSE,S_LIGHT,S_DARKNESS]),
		],
						
		afterstory: ON_THE_PRAIRIE,
	}
}


pub fn ice_palace() -> Dungeon {

	Dungeon {
		id: ID_ICE_PALACE,
		name: "Palace of Ice",
		xy: [-160,90],
		diff: 20.0,
		affinity: ICE,
		scenes: vec![
			Place { name: "The Frozen Gates",							scape: TUNDRA,		xy: [-160,90],		affinity: ICE,
					engenG: [1,1,1,1,1,1,1,1,1,1],
					engenA: [4,4,4,4,4,4,4,4,4,4],
					popu:vec![("Southern Guard",SPIRIT,1000),
							  ("Northern Guard",SPIRIT,1000),
							  ("Eastern Guard",SPIRIT,1000),
							  ("Western Guard",SPIRIT,1000),
					],
			},
			Place { name: "The Garden of Rime",							scape: ICE,		xy: [-160,90],		affinity: SPIRIT,
					engenG: [1,1,2,2,2,2,3,3,3,4],
					engenA: [3,3,3,3,3,3,3,3,3,3],
					popu:vec![("Sculptor of the North Star",SPIRIT,200),
							  ("Sculpture of Hero",GIANT,400),
							  ("Sculpture of Villain",HUMAN,400),
					],
			},
			Place { name: "Ballroom of Snowflakes",						scape: RUIN,		xy: [-160,90],		affinity: ICE,
					engenG: [1,2,2,2,2,2,2,3,3,4],
					engenA: [4,4,4,4,4,4,4,4,4,4],
					popu:vec![("Guest of the North",SPIRIT,10000),
							  ("White Witch",SPIRIT,1000),
							  ("Snowflake",SPIRIT,2000),
					],
			},
			Place { name:"Dream of Summer",								scape: TIME,		xy: [-160,90],		affinity: ICE,
					engenG: [1,2,2,2,2,2,2,3,3,4],
					engenA: [4,4,4,4,4,4,4,4,4,4],
					popu:vec![("Green Beast",SPIRIT,10000),
							  ("Red Beast",BEAST,10000),
							  ("Great Beast",WITCH,10000),
							  ("Serpent",WITCH,10000),
					],
			},
			Place { name:"Dream of Winter",								scape: TIME,		xy: [-160,90],		affinity: ICE,
					engenG: [1,2,2,2,2,2,2,3,3,4],
					engenA: [1,1,1,2,2,2,3,3,4,4],
					popu:vec![("Snowflake",SPIRIT,10000),
							  ("Ghost",SPIRIT,10000),
							  ("White Witch",SPIRIT,10000),
					],
			},
			Place { name:"The Winter Itself",							scape: ICE,		xy: [-160,90],		affinity: ICE,
					engenG: [1,1,1,1,1,1,1,1,1,1],
					engenA: [1,1,1,1,1,1,1,1,1,1],
					popu:vec![("Aurora",SPIRIT,1000),
					],
			},
		],	
		denizens: vec![
						ghost().rename("Southern Guard").re_type(SPIRIT).diff_lvl(20).speed_change(0.4).spellist(vec![S_LESSER_CRYSTALLISE,S_SLOW]),
						ghost().rename("Northern Guard").re_type(SPIRIT).diff_lvl(20).speed_change(0.4).spellist(vec![S_CRYSTALLISE,S_HASTE]),
						ghost().rename("Eastern Guard").re_type(SPIRIT).diff_lvl(20).speed_change(0.4).spellist(vec![S_SPARK,S_DARKNESS]),
						ghost().rename("Western Guard").re_type(SPIRIT).diff_lvl(20).speed_change(0.4).spellist(vec![S_SPARK,S_LIGHT]),
						
						ghost().rename("Sculptor of the North Star").re_type(SPIRIT).diff_lvl(20).speed_change(0.4)
							 .spellist(vec![S_CRYSTALLISE,S_SACRED_LIGHT,S_ABYSSAL_DARKNESS]),
						warrior().rename("Sculpture of Hero").re_type(SPIRIT).diff_lvl(20).speed_change(0.4).spellist(vec![S_LESSER_CRYSTALLISE,S_SLOW]),
						titan().rename("Sculpture of Villain").re_type(SPIRIT).diff_lvl(10).speed_change(0.4).spellist(vec![S_LESSER_CRYSTALLISE,S_SLOW]),
						
						ghost().rename("Snowflake").re_type(SPIRIT).diff_lvl(5).spellist(vec![S_LESSER_CRYSTALLISE]),
						wonderer().rename("Guest of the North").diff_lvl(20).speed_change(0.5).spellist(vec![S_CURE,S_SPARK,S_FIRE]),
						white_witch().diff_lvl(5),
						
						beast_green().diff_lvl(10),
						beast_red(),
						beast_great(),
						beast_serpent(),
						ghost().diff_lvl(10),
						white_queen().rename("Aurora").hp_change(2.0).defence_change(2.0).diff_lvl(4),
		],
						
		afterstory: ICE_PALACE,
	}
}


pub fn petrified_shrine() -> Dungeon {
	
	Dungeon {
		id: ID_PETRIFIED_SHRINE,
		name: "Petrified Shrine",
		xy: [0,-60],
		diff: 1000.0,
		affinity: RADIANT,
		scenes: vec![
			Place { name: "Forgotten Alley",							scape: DESERT,		xy: [0,-60],		affinity: RADIANT,
					engenG: [4,4,4,4,4,4,4,4,4,4],
					engenA: [1,1,1,1,1,1,1,1,1,1],
					popu:vec![("Forgotten Beast",SPIRIT,1000),
					],
			},
			Place { name: "Ancient Meadow",								scape: RUIN,		xy: [0,-60],		affinity: RADIANT,
					engenG: [3,3,3,3,3,4,4,4,4,4],
					engenA: [1,1,1,1,1,2,2,2,2,2],
					popu:vec![("Petrified Fairy",SPIRIT,1000),
							  ("Wisp of Fire",BEAST,1000),
					],
			},
			Place { name: "The Eldest Hollow",							scape: FOREST,		xy: [0,-60],		affinity: RADIANT,
					engenG: [1,1,1,1,1,1,1,1,1,1],
					engenA: [2,2,2,2,2,2,2,2,2,1],
					popu:vec![("Petrified Elf",SPIRIT,1000),
							  ("Elven Spirit",BEAST,1000),
					],
			},
			Place { name: "Cave of the Shrine",							scape: RUIN,		xy: [0,-60],		affinity: RADIANT,
					engenG: [1,1,1,1,1,1,1,1,1,1],
					engenA: [4,4,4,4,4,4,4,4,4,4],
					popu:vec![("Primal Green Beast",BEAST,1000),
							  ("Primal Red Beast",BEAST,1000),
							  ("Primal Great Beast",BEAST,1000),
							  ("Primal Serpent",BEAST,1000),
					],
			},
			Place { name: "Shrine of Elvenkind",						scape: TIME,		xy: [0,-60],		affinity: RADIANT,
					engenG: [1,1,1,1,1,1,1,1,1,1],
					engenA: [3,3,3,3,3,3,3,3,3,3],
					popu:vec![("First Elf",SPIRIT,1000),
							  ("True Elf",BEAST,1000),
							  ("Last Elf",UNDEAD,1000),
					],
			},
			
		],	
		denizens: vec![
					alien().rename("Forgotten Beast").re_type(BEAST)
						   .magic_up(100.0).hp_change(2.0).bm_change(1.5).spellist(vec![S_SPARK,S_SLOW,S_DAGGER_OF_FAWN]),
						   
					white_queen().rename("Petrified Fairy").spellist(vec![S_LESSER_CRYSTALLISE,S_SLOW]),
					beast_red().rename("Wisp of Fire").bm_change(2.0).wm_change(2.0).re_type(SPIRIT).spellist(vec![S_FIREBALL,S_FIRE]),
					
					warrior().rename("Petrified Elf").re_type(GIANT).speed_change(0.2).diff_lvl(40).spellist(vec![]),
					sage_forsaken().rename("Elven Spirit").re_type(SPIRIT).speed_change(3.0).hp_change(2.5),
					
					beast_great().rename("Primal Great Beast").speed_change(0.3).diff_lvl(30).spellist(vec![S_JOVIAN_LIGHTNING,S_GREATER_EXORCISM,S_SPARK]),
					beast_red().rename("Primal Red Beast").speed_change(0.2).diff_lvl(40).spellist(vec![S_INFERNO,S_FIREBALL,S_EMBER,S_SACRED_EXORCISM]),
					beast_green().rename("Primal Green Beast").speed_change(0.2).diff_lvl(50).spellist(vec![S_SACRED_CURE,S_TIMESTOP,S_CURSE]),
					beast_serpent().rename("Primal Serpent").speed_change(0.4).diff_lvl(20).spellist(vec![S_LIGHTNING,S_LIFESTEALER]),
					
					sage_forsaken().rename("First Elf").re_type(SPIRIT).speed_change(1.0).hp_change(5.5).bm_change(2.0).wm_change(4.0).mp_change(2.0)
								   .spellist(vec![S_GREATER_TELEPORT,S_TIMESTOP,S_SACRED_CURE,S_SACRED_EXORCISM,S_EMBER]),
					sage_forsaken().rename("True Elf").re_type(SPIRIT).speed_change(2.0).hp_change(3.5).bm_change(3.0).wm_change(3.0).mp_change(2.0)
								   .spellist(vec![S_GREATER_TELEPORT,S_TIMESTOP,S_GREATER_CURE,S_JOVIAN_LIGHTNING,S_EMBER]),
					sage_forsaken().rename("Last Elf").re_type(SPIRIT).speed_change(3.0).hp_change(1.5).bm_change(4.0).wm_change(2.0).mp_change(2.0)
								   .spellist(vec![S_GREATER_TELEPORT,S_TIMESTOP,S_CURE,S_SUMMON_REAPER,S_EMBER]),
		],
						
		afterstory: PETRIFIED,
	}
	
}


const CITADEL_OF_SPIRIT:&str = "...It is said that those who wonder up onto the moors with the wish, fixed in their heart, \
to find another world, may see, before themselves, the gates of the Citadel of Spirit.\


...It is said that the citadel straddles this world and the next, the world without and the world within. \
That those who go through hell and can walk the brilliant road to its end, can become their better selves...\


...Did you become your better self? \
";
const DOOR_TO_DARKNESS:&str = "...In the dryest corner of the desert dwelt the reclusive sorceress Anthracene \
who was said to know the secrets of the soul, and the light and darkness that dwelt within. \
Pilgrims of all shapes and walks of life came to learn from the sorceress. \


...Eventually, tired of the pilgrims, Anthracene created a mirror and all who gazed into the mirror would spawn two reflections. \
The sorceress bid all visitors distinguish between the true reflection and the false, \
those who could not, would be sent away. But no one could tell the reflections apart. \


...One day Anthracene looked into her own mirror, but not even she could tell apart the reflections. \
Haunted by her own reflections, she summoned the sages, imploring for a solution to her conundrum. \


...It was the Sage of Darkness who stepped forth. The Sage bound Anthracene and her reflections \
In the lightless microcosm of the Black Tower. There she has dwelt ever since... \


...Who did you behold in Anthracene's mirror? \
";
const ELVEN_LAKE_RUINS:&str = "...After that which was left of the southern forests had become stone, \
the island of Moracea, in the middle of the southern ocean, \
became the Elves' last refuge. \
Those few elves who remained, flocked to the Islands of Moracea, \
Surrounded by the deepest ocean, \
where under the care of Princess Mora and her guardian beasts they had at least some degree of protection \
against the encroaching darkness. \


...But the darkness was ever encroaching. Thus as the years went on, goblins and undead grew bolder, \
the seas receded and the rains failed. \
And while the Princess, renowned as the most skilled of warriors, could protect her people from monsters, \
there was nothing she could do to stop her lands from dying. \
And with time, Princess Mora, despaired. \


...Summoning the Sages, she implored for a way to save her people. One of the sages took pity on the elves and steppd forth. \
The Sage offered the princess a magical gem. With it, the sage assured, her people would need fear neither hunger nor thirst, \
and nor would she have to worry fear death at the claws of the endless hordes that assailed Moraecea... \
Princess Mora took the offered gem and called upon its power. \


...And the gem's power took the breath and heartbeat of Moracea's elves, \
trapping them and their princess in undeath for the rest of time.\


...And Princess Mora had all eternity to curse her decision to trust \
an offer from the Sage of Death.\
";
const LOST_LIGHTHOUSE:&str = "...Once upon a time, before the world ended, one would occasionally see an angel above, \
but since time immemoral, an exeption to the rule, one such being dwelt on the eastern coast of the golden lands. \


...And the light of its holy being were a guide to those who sailed those treacherous rocky shores \
throughout the ages. Through ages of prosperity and ages of woe, through times of peace and times of war, \
the angel continued to dwell, unchanging on the same coast.


...And as the gods led the golden lands to ruin and as the sages brought the world down around them, \
the angel continued to shine. \
Only now, by your hand, does the last angel shine no more...\
";
const MALACHIA_PUBCRAWL:&str = "...You have survived a night out in Malachia, that in and of itself is an achievement.\
";
const MALEK_GROVE:&str = "...The archdruid Malek used to watched over the forests surrounding Malachia. \
Be it by the light of the sun or of the moon, Malek, his disciples and guardian beasts \
watched over Malachia.


...Then the world ended...


...And with it, so did Malek's benevolence. Now, withdrawn from the world, \
he presides over his grove ignorant of the ruin beyond...\
";
const MONSTER_HALL:&str = "...To a human, they are all monsters, but to them, there is truly only one monster...\


...You...

...And in the age of man they came together and founded a stronghold where they would be safe from you. \
Now, that the world has ended, the stronghold served to protect them from the dead and malicious spirits that wonder the world.\
";
const STAIRWAY:&str = "...If there was a way of reaching the heavens in life, would you too not emigrate? \
";
const TOWER_OF_BONES:&str = "...One of the three foundations of the city of the dead:

The Tower of Soul";
const TOWER_OF_FLESH:&str = "...One of the three foundations of the city of the dead:

The tower of Flesh";
const TOWER_OF_SOUL:&str = "...One of the three foundations of the city of the dead:

The Tower of Soul";
const WAY_DOWN:&str = "...When the world broke and the void appeared, the titans built a bridge connecting its two halves. \
And half way accross they placed a pillar anchoring the bridge against the nothingness...


...It is said that by descending this pillar, it is possible to descend down into the void \
and behold the heart of this abbyss, and still return to the world above. \
One more way to go to hell and back...\
";
const WHITE_TEMPLE:&str = "If Albion was the white city of hope, then the White Temple was the crucible where \
those hopes were forged. It is the heart of Albion, with the city built around it. \
It is here that the Sage of Albion uncovered the secrets of time...


...It is said that its walls are permeated with the hopes of the ages, \
that by touching its walls it is possible to touch those hopes and see the shades of those to whom they belong...


...Was this what the Sage of Albion was hoping for, or is this what the sage will hope for?\
";
const WILD_HUNT:&str = "...When the world ended and even the gods gave up hope, there was but one who stayed. \
\"The world may have died, but all who dwell here are yet to die. \
My forests and beasts will still be here till the corpse they inhabit has wilted away\" \
She reasoned, \"Thus the hunt must goes on.\"...


...Therefore, when you enter the wilds, do not be surprised if the wild hunt finds you...\
";
const WITCH_MAZE:&str = "The archdruid Malek had three sisters. All witches. All outstanding in their field. \
For the longest of ages they travelled the forests of Malachia, guarding it with their brother...\


...Then, when Malek withdrew into his grove, he cut his links with his sisters and they too went their seperate way, \
withdrawing into a far corner of the woods. There between them they cast a spell to make that would make all \
within those woods lose their way...


...And those three witches lost themselves in their spell.\
";
const HALL_OF_STONE:&str = "...In the heart of the Stone Maze lies the City of Stone. In the heart of the City of Stone \
lie the Halls of Stone. At the heart of the Halls of Stone lies the last of the treasures of the Golden Land. \
Left standing by the gods as a reminder that it once existed. That it once dared to challenge them...


...Now the only ones in this place are those who seek its treasures, those who seek its secrets, and those who cannot let go \
of what it used to be...\
";
const THE_PATH:&str = "In south, it is said that there is a stairway that physically \
connects this world to the world above. On the opposite side of the world there is another \
stairway. Deep in the northern highlands, a land of rock and ice, \
there is a particularly high and lonely range. Since time immemorial, this range \
has been a holy place. In the heart of this range, there stands the Holy Peak...


...There is nothing higher than this summit. Nothing holier.
It stands above and beyond its peers. Isolated. Indomitable.
Many seek to stand on its summit, but few can walk the path...


...All the way to the summit. And for those lost souls who make it there \
all that awaits is the empty summit.
";
const ON_THE_PRAIRIE:&str = "...Can you feel the wind of the steppes? \
It carried the scent of far away, new life, new seasons and new peoples. \
It brought with it droughts, rains, snows...


...But what when the wind is empty?
";
const ICE_PALACE:&str = "...What lies at the end of the world? A desert of snow and ice, \
 blanketed by a wall of unending storms...
 
 
 ...It is the winter itself that builds the walls surrounding its palace and weaves the dreams of summer. \
 ";
 
const PETRIFIED:&str = "...It was here that elvenkind willed itself into existence...
...It was here that elvenkind lost its will...
";
