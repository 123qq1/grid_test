use bevy::prelude::*;
use std::collections::HashMap;

const GRID_HASH: [(i32,(i32,i32)); 54] = [
    (45,(0,5)), (46,(1,5)), (47,(2,5)), (48,(3,5)), (49,(4,5)), (50,(5,5)), (51,(6,5)), (52,(7,5)), (53,(8,5)),
    (36,(0,4)), (37,(1,4)), (38,(2,4)), (39,(3,4)), (40,(4,4)), (41,(5,4)), (42,(6,4)), (43,(7,4)), (44,(8,4)),
    (27,(0,3)), (28,(1,3)), (29,(2,3)), (30,(3,3)), (31,(4,3)), (32,(5,3)), (33,(6,3)), (34,(7,3)), (35,(8,3)),
    (18,(0,2)), (19,(1,2)), (20,(2,2)), (21,(3,2)), (22,(4,2)), (23,(5,2)), (24,(6,2)), (25,(7,2)), (26,(8,2)),
    (9,(0,1)),  (10,(1,1)), (11,(2,1)), (12,(3,1)), (13,(4,1)), (14,(5,1)), (15,(6,1)), (16,(7,1)), (17,(8,1)),
    (0,(0,0)),  (1,(1,0)),  (2,(2,0)),  (3,(3,0)),  (4,(4,0)),  (5,(5,0)),  (6,(6,0)),  (7,(7,0)),  (8,(8,0))
];

const RANGE_HASH_EVEN: [(i32,(i32,i32)); 19] = [
    (18,(0,2)),
    (7,(-2,1)),(8,(-1,1)),(9,(0,1)),(10,(1,1)),(11,(2,1)),
    (2,(0,2)),(1,(0,1)),(0,(0,0)),(-1,(0,-1)),(-2,(0,-2)),
    (-11,(-2,-1)),(-10,(-2,-1)),(-9,(0,-1)),(-8,(1,-1)),(-7,(2,-1)),
    (-17,(-1,-2)),(-18,(0,-2)),(-19,(1,-2))
];

const RANGE_HASH_ODD: [(i32,(i32,i32)); 19] = [
             (17,(-1,2)),(18,(0,2)),(19,(1,2)),
    (7,(-2,1)),(8,(-1,1)),(9,(0,1)),(10,(1,1)),(11,(2,1)),
      (2,(0,2)),(1,(0,1)),(0,(0,0)),(-1,(0,-1)),(-2,(0,-2)),
(-11,(-2,-1)),(-10,(-2,-1)),(-9,(0,-1)),(-8,(1,-1)),(-7,(2,-1)),
                        (-18,(0,-2))
];

pub struct ChipLib{
    lib: HashMap<i32,ChipData>,
    grid_hash: HashMap<i32,(i32,i32)>,
    range_hash_even: HashMap<i32,(i32,i32)>,
    range_hash_odd: HashMap<i32,(i32,i32)>,
}

impl ChipLib {
    fn new() -> ChipLib{
        ChipLib{
            lib: HashMap::new(),
            grid_hash: HashMap::from(GRID_HASH),
            range_hash_even: HashMap::from(RANGE_HASH_EVEN),
            range_hash_odd: HashMap::from(RANGE_HASH_ODD),
        }
    }
    fn add(&mut self,index: i32, data: ChipData){
        self.lib.insert(index,data);
    }
    pub fn get(&self, index: i32) -> &ChipData{
        self.lib.get(&index).expect("Chip at index does not exist")
    }

    fn index_to_coords(&self, index: i32) -> &(i32,i32){
        self.grid_hash.get(&index).expect("Index outside of grid")
    }

    pub fn range_check(&self, origin: i32, target: &i32) -> bool{
        let (x,y) = self.index_to_coords(origin);
        let t_index = origin + target;
        if t_index < 0 || t_index > 53 { return false}

        let (t_x,t_y) = self.index_to_coords(origin + target);

        let map;
        if x%2 == 0 { map = &self.range_hash_even; }
        else { map = &self.range_hash_odd; }
        let (r_x, r_y) = map.get(&target).expect("Invalid Range");

        (r_x + x) == *t_x && (r_y + y) == *t_y
    }
}

#[derive(Debug, Clone, Copy,PartialEq)]
pub enum ChipType {
    Empty,
    Defence,
    Offensive,
}

#[derive(Debug, Clone)]
pub struct ChipData{
    pub name: String,
    pub value: i32,
    pub c_type: ChipType,
    pub increase: f32,
    pub more: f32,
    pub add: f32,
    pub eff: f32,
    pub targets: Option<Vec<i32>>,
}

pub fn chip_setup(
    mut commands: Commands,
){
    let mut chip_lib = ChipLib::new();

    chip_lib.add(0,ChipData{
        name: "Empty".to_string(),
        value: 0,
        c_type: ChipType::Empty,
        more: 0.0,
        increase: 0.0,
        add: 0.0,
        eff: 1.0,
        targets: None
    });
    chip_lib.add(1,ChipData{
        name: "Defend".to_string(),
        value: 1,
        c_type: ChipType::Defence,
        more: 0.0,
        increase: 0.0,
        add: 1.0,
        eff: 1.0,
        targets: None
    });
    chip_lib.add(2,ChipData{
        name: "Strike".to_string(),
        value: 1,
        c_type: ChipType::Offensive,
        more: 0.0,
        increase: 0.0,
        add: 0.0,
        eff: 1.0,
        targets: None
    });

    commands
        .spawn()
        .insert(chip_lib);
}