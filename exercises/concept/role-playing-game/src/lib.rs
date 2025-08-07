pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health == 0 {
            let mana = if self.level >= 10 { Some(100) } else { None };
            Some(Player {
                health: 100,
                mana,
                level: self.level,
            })
        } else {
            None
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(mana) => {
                if mana_cost > mana {
                    return 0;
                }

                let mana_remaining = mana - mana_cost;
                self.mana = if mana_remaining > 0 {
                    Some(mana_remaining)
                } else {
                    None
                };
                mana_cost * 2
            }
            None => {
                if self.health < mana_cost {
                    self.health = 0;
                } else {
                    self.health -= mana_cost;
                }
                0
            }
        }
    }
}
