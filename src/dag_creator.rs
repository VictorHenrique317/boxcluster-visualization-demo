#![allow(non_snake_case)]
use crate::Pattern;
use crate::Relation;
use debug_print::debug_println;
use std::collections::hash_map::ValuesMut;
use std::collections::HashMap;
use std::hash::Hash;

pub struct DagCreator {
    depth: u32,
    patterns_mapping: HashMap<u32, Pattern>,
    pub pattern_subs: HashMap<u32, Vec<u32>>,
    pub pattern_supers: HashMap<u32, Vec<u32>>,
}

impl DagCreator {
    pub fn new() -> Self {
        return DagCreator {
            depth: 0,
            patterns_mapping: HashMap::new(),
            pattern_subs: HashMap::new(),
            pattern_supers: HashMap::new(),
        };
    }

    fn initializeDag(&mut self){
        self.depth = 0;
        for identifier in self.patterns_mapping.keys(){
            self.pattern_subs.insert(identifier.clone(), Vec::new());
            self.pattern_supers.insert(identifier.clone(), Vec::new());
        }
    }

    fn createPatternsMapping(&mut self, patterns: Vec<Pattern>) {
        for pattern in patterns {
            self.patterns_mapping.insert(pattern.identifier, pattern);
        }
    }

    fn firstRelationToSecond(&self, first_id: &u32, second_id: &u32) -> Relation {
        let first_patern: &Pattern = self.patterns_mapping.get(first_id).unwrap();
        let second_patern: &Pattern = self.patterns_mapping.get(second_id).unwrap();
        return first_patern.selfRelationTo(second_patern).0;
    }

    fn drawRelationOnDag(&mut self, first_pattern: &u32, second_pattern: &u32, relation:Relation){
        if relation == Relation::SuperPattern{
            self.pattern_subs.get_mut(first_pattern).unwrap().push(second_pattern.clone());
            self.pattern_supers.get_mut(second_pattern).unwrap().push(first_pattern.clone());
        }

        else if relation == Relation::SubPattern{
            self.pattern_subs.get_mut(second_pattern).unwrap().push(first_pattern.clone());
            self.pattern_supers.get_mut(first_pattern).unwrap().push(second_pattern.clone());
        }

        else{
            panic!("Incorrect use of method");
        }
    }

    fn drawMultipleRelationOnDag(&mut self, pattern: &u32, others: &Vec<u32>, relation_to_others: Relation){
        for other in others{
            self.drawRelationOnDag(pattern, other, relation_to_others.clone());
        }
    }

    fn createFirstLevel(&mut self) -> Vec<Vec<u32>> {
        let patterns: Vec<u32> = self.patterns_mapping.keys().map(|i| i.clone()).collect();
        let mut first_level: Vec<Vec<u32>> = Vec::new();

        for possible_font in patterns.iter() {
            let mut is_font = true;
            let mut subs: Vec<u32> = Vec::new();

            for test_pattern in patterns.iter() {
                let relation = self.firstRelationToSecond(possible_font, test_pattern);

                if relation == Relation::SubPattern {
                    is_font = false;
                    break;
                }

                if relation == Relation::SuperPattern {
                    subs.push(test_pattern.clone());
                }
            }

            if is_font {
                self.drawMultipleRelationOnDag(possible_font, &subs, Relation::SuperPattern);
                first_level.push(subs);
            }
        }

        return first_level;
    }

    fn refineGroup(&self, group: Vec<u32>) -> Vec<Vec<u32>>{
        let mut new_groups: Vec<Vec<u32>> = Vec::new();
        
        for base_pattern in group.iter(){
            let mut new_group: Vec<u32> = Vec::new();

            for test_pattern in group.iter(){
                let relation = self.firstRelationToSecond(base_pattern, test_pattern);

                if relation == Relation::SuperPattern{ // base is super of test
                    new_group.push(test_pattern.clone());
                }
            }

            if !new_group.is_empty(){
                new_groups.push(new_group);
            }
        }

        return new_groups;
    }

    fn addFlattenedGroupsTo(&self, groups: Vec<Vec<u32>>, level: Vec<Vec<u32>>) -> Vec<Vec<u32>>{
        let mut level = level;
        for group in groups{
            level.push(group);
        }

        return level;
    }

    pub fn calculate(&mut self, patterns: Vec<Pattern>, depth_limit: Option<u32>) {
        self.createPatternsMapping(patterns);
        self.initializeDag();
        let mut last_level: Vec<Vec<u32>> = self.createFirstLevel();

        let depth_limit = match depth_limit{
            None => 0,
            Some(i) => i,
        };

        while true{
            self.depth += 1;
            
            if self.depth >= depth_limit{
                break
            }

            let mut new_level: Vec<Vec<u32>> = Vec::new();
            for group in last_level{
                let new_groups: Vec<Vec<u32>> = self.refineGroup(group);
                new_level = self.addFlattenedGroupsTo(new_groups, new_level);
            }

            if new_level.is_empty(){ // No more refinements are possible
                break;
            }

            last_level = new_level;
        }

        debug_println!("Pattern subs: {:?}", self.pattern_subs);
        debug_println!("Pattern supers: {:?}", self.pattern_supers);
    }
}
