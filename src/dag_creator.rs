#![allow(non_snake_case)]
use crate::Pattern;
use crate::Relation;
use debug_print::debug_println;
use indicatif::ProgressStyle;
use std::collections::HashSet;
use std::collections::hash_map::ValuesMut;
use std::collections::HashMap;
use std::hash::Hash;
use indicatif::ProgressBar;

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

    fn firstRelationToSecondByPattern(&self, first: &Pattern, second: &Pattern) -> Relation {
        return first.selfRelationTo(second).0; // Time sink
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
        // let patterns: Vec<u32> = self.patterns_mapping.keys().map(|i| i.clone()).collect();
        let patterns: HashMap<u32, Pattern> = self.patterns_mapping.iter().clone().map(|i| (i.0.clone(), i.1.clone())).collect();
        let mut fonts: HashSet<u32> = HashSet::new();
        let mut first_level: Vec<Vec<u32>> = Vec::new();
        let bar = ProgressBar::new(patterns.len() as u64);
        bar.set_message("Checked patterns");
        bar.set_style(ProgressStyle::with_template("{msg}: {bar:40.cyan/blue} {pos:>7}/{len:7} Elapsed time: {elapsed} | Estimated time:{eta} ")
            .unwrap()
            .progress_chars("##-"));  

        // for possible_font in patterns.iter() {
        for (possible_font_id, possible_font) in patterns.iter() {
            // let first_patern: &Pattern = self.patterns_mapping.get(first_id).unwrap();
            // let possible_font: &Pattern = self.patterns_mapping.get(possible_font).unwrap();
            bar.inc(1);
            
            let mut is_font = true;
            let mut subs: Vec<u32> = Vec::new();

            // for test_pattern in patterns.iter().filter(|i| !fonts.contains(i)) {
            // for (test_pattern_id, test_pattern) in patterns.iter().filter(|i| !fonts.contains(i.0)) {
            for (test_pattern_id, test_pattern) in patterns.iter() {
                // let test_pattern: &Pattern = self.patterns_mapping.get(test_pattern).unwrap();
                // let relation = self.firstRelationToSecond(possible_font, test_pattern);
                let relation = self.firstRelationToSecondByPattern(possible_font, test_pattern);

                if relation == Relation::NotRelatable {
                    continue;
                }

                if relation == Relation::SubPattern {
                    is_font = false;
                    break;
                }

                if relation == Relation::SuperPattern {
                    subs.push(*test_pattern_id);
                }
            }

            if is_font {
                debug_println!("Found new font: {}", &possible_font_id);
                debug_println!("Rough subpatterns: {:?}", &subs);

                self.drawMultipleRelationOnDag(possible_font_id, &subs, Relation::SuperPattern);
                fonts.insert(*possible_font_id);
                first_level.push(subs);
            }
        }

        bar.finish();
        println!("{} fonts found!", &fonts.len());
        return first_level;
    }

    fn alreadyInNewGroups(&self, pattern: &u32, new_groups: &Vec<Vec<u32>>) -> bool{
        for group in new_groups{
            if group.contains(pattern){
                return true;
            }
        }

        return false;
    }

    fn refineGroup(&mut self, group: Vec<u32>) -> Vec<Vec<u32>> {
        debug_println!("\n    Refining group {:?}:", &group);
        let mut new_groups: Vec<Vec<u32>> = Vec::new();
        
        for base_pattern in group.iter() {
            let mut new_group: Vec<u32> = Vec::new();

            for test_pattern in group.iter() {
                let relation = self.firstRelationToSecond(base_pattern, test_pattern);
                // let relation = self.firstRelationToSecondByPattern(base_pattern, test_pattern);

                if relation == Relation::SuperPattern { // base is super of test
                    debug_println!("\n        Re-drawing relation of {} and {}:", &base_pattern, &test_pattern);

                    if self.alreadyInNewGroups(&test_pattern, &new_groups) || new_group.contains(&test_pattern){
                        // Behaviour for patterns with multiple supers, do not delete old relations, only add the new ones
                        debug_println!("\n        {} has been refined already, last detected supers {:?}", &test_pattern, self.pattern_supers.get(test_pattern).unwrap());
                        
                        self.drawRelationOnDag(base_pattern, test_pattern, relation);
                        continue;
                    }

                    self.eraseRelationsOnDag(test_pattern);
                    self.drawRelationOnDag(base_pattern, test_pattern, relation);
                    new_group.push(test_pattern.clone());
                }
            }

            // if !new_group.is_empty(){
            if new_group.len() > 0 {
                debug_println!("\n        NEW group added (for next iteration): {:?}", &new_group);
                new_groups.push(new_group);
            }
        }

        debug_println!("\n    ALL groups that were added (for next iteration): {:?}", &new_groups);
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
        println!("Creating level 0");
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

            println!("\n=====> Refining relations | ITERATION: {}", &self.depth);
            debug_println!("Currently on level {}: {:?}", &self.depth, &last_level);
            debug_println!("Old subs: {:?}", &self.pattern_subs);
            debug_println!("Old supers: {:?}", &self.pattern_supers);


            let mut new_level: Vec<Vec<u32>> = Vec::new();
            // let bar = ProgressBar::new(last_level.len() as u64);

            for group in last_level {
                // bar.inc(1);
                let new_groups: Vec<Vec<u32>> = self.refineGroup(group);
                new_level = self.addFlattenedGroupsTo(new_groups, new_level);
            }

            // bar.finish();

            debug_println!("\nNew subs: {:?}", &self.pattern_subs);
            debug_println!("New supers: {:?}", &self.pattern_supers);
            if new_level.is_empty() {
                // No more refinements are possible
                println!("MAXIMUM refinement reached with {} iteration(s), stopping operation\n", &self.depth);
                break;
            }
            
            last_level = new_level;
        }

        debug_println!("Pattern subs: {:?}", self.pattern_subs);
        debug_println!("Pattern supers: {:?}", self.pattern_supers);
    }

}
