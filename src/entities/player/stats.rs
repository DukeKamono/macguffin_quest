
pub struct Stats {
	pub lv: u32,
	pub exp: u32,
	pub hp: f32,
	pub max_hp: f32,
	pub mp: u32,
	pub max_mp: u32,
	pub atk: f32,
	pub def: f32,
	pub spd: f32,
}

impl Stats {
	pub fn new(lv: u32, exp: u32, hp: f32, mp: u32, atk: f32, def: f32, spd: f32) -> Stats {
		Stats {
			lv,
			exp,
			hp,
			max_hp: hp,
			mp,
			max_mp: mp,
			atk,
			def,
			spd,
		}
	}
	
	pub fn check_for_level_up(&mut self, exp: u32) {
		self.exp += exp;
		if self.exp >= 10 {
			self.lv += 1;
			self.level_up();
			self.exp = 0;
		}
	}
	
	// This is just a straight forward level_up, but
	// there are so many things we can do here :)
	pub fn level_up(&mut self) {
		match self.lv {
			2 => { self.atk += 0.5; self.hp += 5.0; self.max_hp += 5.0; },
			3 => { self.atk += 0.5; self.hp += 5.0; self.max_hp += 5.0; self.max_mp += 20; },
			4 => { self.atk += 0.5; self.hp += 5.0; self.max_hp += 5.0; self.def += 0.5; },
			5 => { self.atk += 1.0; self.hp += 5.0; self.max_hp += 5.0; self.max_mp += 20; },
			6 => { self.atk += 0.5; self.hp += 5.0; self.max_hp += 5.0; self.max_mp += 20; },
			7 => { self.atk += 1.0; self.hp += 5.0; self.max_hp += 5.0; self.def += 0.5; },
			8 => { self.atk += 0.5; self.hp += 5.0; self.max_hp += 5.0; self.max_mp += 20; },
			9 => { self.atk += 1.0; self.hp += 5.0; self.max_hp += 5.0; self.max_mp += 20; },
			10 => { self.atk += 1.0; self.hp += 5.0; self.max_hp += 5.0; self.def += 1.0; },
			_ => { self.hp += 5.0; self.max_hp += 5.0; },
		}

		self.mp = self.max_mp;
	}
}