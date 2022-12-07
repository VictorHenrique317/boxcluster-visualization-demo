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

    fn initializeDag(&mut self) {
        self.depth = 0;
        for identifier in self.patterns_mapping.keys() {
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

    fn drawRelationOnDag(&mut self, first_pattern: &u32, second_pattern: &u32, relation: Relation) {
        if relation == Relation::SuperPattern {
            debug_println!("        ==> Setting {} to be a sub of {}", &second_pattern, &first_pattern);
            self.pattern_subs
                .get_mut(first_pattern)
                .unwrap()
                .push(second_pattern.clone());

            debug_println!("        ==> Setting {} to be a super of {}", &first_pattern, &second_pattern);
            self.pattern_supers
                .get_mut(second_pattern)
                .unwrap()
                .push(first_pattern.clone());

        } else if relation == Relation::SubPattern {
            debug_println!("        ==> Setting {} to be a sub of {}", &first_pattern, &second_pattern);
            self.pattern_subs
                .get_mut(second_pattern)
                .unwrap()
                .push(first_pattern.clone());

            debug_println!("        ==> Setting {} to be a super of {}", &second_pattern, &first_pattern);
            self.pattern_supers
                .get_mut(first_pattern)
                .unwrap()
                .push(second_pattern.clone());
        } else {
            panic!("Incorrect use of method");
        }
    }

    fn eraseRelationsOnDag(&mut self, pattern_to_erase: &u32) {
        debug_println!("\n        Erasing relations of {}:", &pattern_to_erase);
        debug_println!("        ==> Erasing supers of of {}", &pattern_to_erase);
        *self.pattern_supers.get_mut(pattern_to_erase).unwrap() = vec![];

        for (pattern, subs) in self.pattern_subs.clone(){
            if subs.contains(pattern_to_erase){ // pattern_to_erase is a sub of this pattern
                self.pattern_subs.get_mut(&pattern).unwrap().retain(|i| *i != *pattern_to_erase);
                debug_println!("        ==> {} was a sub of {}, new subs of {}: {:?}", &pattern_to_erase, &pattern, &pattern, self.pattern_subs.get(&pattern).unwrap());
            }
        }

        for (pattern, supers) in self.pattern_supers.clone(){
            if supers.contains(pattern_to_erase){ // pattern_to_erase is a super of this pattern
                self.pattern_supers.get_mut(&pattern).unwrap().retain(|i| *i != *pattern_to_erase);
                debug_println!("        ==> {} was a super of {}, new supers of {}: {:?}", &pattern_to_erase, &pattern, &pattern, self.pattern_supers.get(&pattern).unwrap());
            }
        }
    }

    fn drawMultipleRelationOnDag(
        &mut self,
        pattern: &u32,
        others: &Vec<u32>,
        relation_to_others: Relation,
    ) {
        for other in others {
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
                debug_println!("Found new font: {}", &possible_font);
                debug_println!("Rough subpatterns: {:?}", &subs);

                self.drawMultipleRelationOnDag(possible_font, &subs, Relation::SuperPattern);
                first_level.push(subs);
            }
        }

        return first_level;
    }

    fn refineGroup(&mut self, group: Vec<u32>) -> Vec<Vec<u32>> {
        debug_println!("\n    Refining group {:?}:", &group);
        let mut new_groups: Vec<Vec<u32>> = Vec::new();

        for base_pattern in group.iter() {
            let mut new_group: Vec<u32> = Vec::new();

            for test_pattern in group.iter() {
                let relation = self.firstRelationToSecond(base_pattern, test_pattern);

                if relation == Relation::SuperPattern {
                    // base is super of test
                    self.eraseRelationsOnDag(test_pattern);
                    // self.eraseRelationsOnDag(base_pattern);
                    debug_println!("\n        Re-drawing relation of {} and {}:", &base_pattern, &test_pattern);
                    self.drawRelationOnDag(base_pattern, test_pattern, relation);
                    new_group.push(test_pattern.clone());
                }
            }

            // if !new_group.is_empty(){
            if new_group.len() > 1 {
                new_groups.push(new_group);
            }
        }

        debug_println!("    ==> Done!");
        return new_groups;
    }

    fn addFlattenedGroupsTo(&self, groups: Vec<Vec<u32>>, level: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
        let mut level = level;
        for group in groups {
            level.push(group);
        }

        return level;
    }

    pub fn calculate(&mut self, patterns: Vec<Pattern>, depth_limit: Option<u32>) {
        self.createPatternsMapping(patterns);
        self.initializeDag();
        debug_println!("Creating level 0");
        let mut last_level: Vec<Vec<u32>> = self.createFirstLevel();
        debug_println!("Created first level: {:?}", &last_level);

        let depth_limit = match depth_limit {
            None => 0, // No depth limits
            Some(i) => i,
        };

        while true {
            self.depth += 1;

            if self.depth >= depth_limit && depth_limit != 0{
                debug_println!("\nMAXIMUM depth limit reached, stopping operation\n");
                break;
            }

            debug_println!("\n=====> Refining relations | ITERATION: {}", &self.depth);
            debug_println!("Currently on level {}: {:?}", &self.depth, &last_level);
            debug_println!("Old subs: {:?}", &self.pattern_subs);
            debug_println!("Old supers: {:?}", &self.pattern_supers);


            let mut new_level: Vec<Vec<u32>> = Vec::new();
            for group in last_level {
                let new_groups: Vec<Vec<u32>> = self.refineGroup(group);
                new_level = self.addFlattenedGroupsTo(new_groups, new_level);
            }

            debug_println!("\nNew subs: {:?}", &self.pattern_subs);
            debug_println!("New supers: {:?}", &self.pattern_supers);
            if new_level.is_empty() {
                // No more refinements are possible
                debug_println!("MAXIMUM refinement reached with {} iteration(s), stopping operation\n", &self.depth);
                break;
            }
            
            last_level = new_level;
        }

        debug_println!("Pattern subs: {:?}", self.pattern_subs);
        debug_println!("Pattern supers: {:?}", self.pattern_supers);
    }
}
