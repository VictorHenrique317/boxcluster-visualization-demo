#![allow(non_snake_case)]
use debug_print::{debug_print, debug_println, debug_eprint, debug_eprintln};
use crate::Pattern;
use crate::Relation;
use std::collections::HashMap;
use std::collections::hash_map::ValuesMut;
use std::hash::Hash;
use std::mem::replace;

pub struct DagCreator {
    // pattterns_mapping: HashMap<u32, Pattern>,
    pub pattern_subs: HashMap<u32, Vec<u32>>,
    pub pattern_supers: HashMap<u32, Vec<u32>>,
}

impl DagCreator {
    pub fn new() -> Self {
        return DagCreator{pattern_subs: HashMap::new(), pattern_supers: HashMap::new()};
    }

    fn setRelationToMultiple(&mut self, pattern: &Pattern, interested_patterns:&Vec<&Pattern>, all_patterns: &Vec<&Pattern>) -> bool{
        let mut super_patterns: Vec<&Pattern> = Vec::new();
        let mut was_wrongly_connected = false;

        for interested_pattern in interested_patterns { // Filter the relationed patterns
            let current_relation = interested_pattern.selfRelationTo(&pattern);
            debug_println!("{} relation to {}: {:?}", &interested_pattern.identifier, &pattern.identifier, &current_relation);

            if current_relation.0 == Relation::SuperPattern {
                super_patterns.push(interested_pattern);
                continue;
            }

            if current_relation.0 == Relation::SubPattern {
                let wrongly_connected_patterns = self.pattern_supers.get(&interested_pattern.identifier).unwrap();

                for wrongly_connected_pattern in wrongly_connected_patterns {
                    if wrongly_connected_pattern == &pattern.identifier{  // Prevents a node from altering it's own relations
                        continue;
                    }
                    was_wrongly_connected = true;
                    debug_println!("    {} is wrongly connected to {}, switching connections", &interested_pattern.identifier, &pattern.identifier);
                    // Here only the relations of nodes != pattern will be changed (expected)
                    debug_println!("    Disconnecting {} from {}", &interested_pattern.identifier, &wrongly_connected_pattern);
                    let wrong_subpatterns = self.pattern_subs.get_mut(wrongly_connected_pattern).unwrap();
                    wrong_subpatterns.retain(|identifier| *identifier != interested_pattern.identifier);
                }

                debug_println!("    Changing supers of {} to {:?}\n", &interested_pattern.identifier, vec![pattern.identifier]);

                let interested_pattern_supers = self.pattern_supers.get_mut(&interested_pattern.identifier).unwrap();
                *interested_pattern_supers = vec![pattern.identifier];
            }
        }

        if super_patterns.len() == 0 { // Pattern is a font or is and end
            debug_println!("No super patterns found for {}", &pattern.identifier);
            return true;
        }

        debug_println!("Super patterns found for {}: {:?}:", &pattern.identifier, &super_patterns.iter().map(|i| i.identifier).collect::<Vec<u32>>());
        let had_multiple_supers = super_patterns.len() > 1;

        for super_pattern in super_patterns { // Recursive call to set relations of possible subpatterns from each super pattern
            debug_println!("");
            let sub_patterns = self.pattern_subs.get_mut(&super_pattern.identifier).unwrap();
            debug_println!("Sub patterns found for {}: {:?}", &super_pattern.identifier, &sub_patterns);
            
            if sub_patterns.contains(&pattern.identifier) && sub_patterns.len() == 1{ // maybe remove length check?
            // if sub_patterns.contains(&pattern.identifier){ // maybe remove length check?
                debug_println!("\n{} is the only subpattern of {}, can add previous pattern now", &pattern.identifier, &super_pattern.identifier);
                return true;
            }

            if sub_patterns.contains(&pattern.identifier) { // possible bug?
                // continue;
            }

            if sub_patterns.len() == 0 && !was_wrongly_connected{ // possible bug
                debug_println!("\nHit one end of DAG tree");
                debug_println!("Adding subpattern {} to {}", &pattern.identifier, &super_pattern.identifier);

                sub_patterns.push(pattern.identifier); // One end of DAG tree

                if !self.pattern_supers.get_mut(&pattern.identifier).unwrap().contains(&super_pattern.identifier){
                    // Blocks double adition in case block current_relation.0 == Relation::SubPattern is executed
                    
                    debug_println!("Adding superpattern {} to {}", &super_pattern.identifier, &pattern.identifier);
                    self.pattern_supers.get_mut(&pattern.identifier).unwrap().push(super_pattern.identifier)
                }

                continue;
            }

            let super_subs_identifiers: &Vec<u32> = &self.pattern_subs.get(&super_pattern.identifier).unwrap().clone();
            let mut no_super_subs_has_subs = true;
            for identifier in super_subs_identifiers{
                for super_sub in all_patterns.iter(){
                    if super_sub.identifier == *identifier{
                        let mut interested_patterns = interested_patterns.clone();
                        interested_patterns.retain(|i| i.identifier != super_sub.identifier);
                        interested_patterns.retain(|i| i.identifier != super_pattern.identifier);

                        if interested_patterns.len() == 0{
                            break;
                        }

                        debug_println!("\n=====> RECURSIVE call to see subs of {}", &super_sub.identifier);
                        debug_println!("Interested patterns {:?}", &interested_patterns.iter().map(|i| i.identifier).collect::<Vec<u32>>());
                        let is_dag_end = self.setRelationToMultiple(super_sub, &interested_patterns, all_patterns);
                        if is_dag_end == false{
                            no_super_subs_has_subs = false;
                        }
                        // if is_dag_end{
                        //     debug_println!("Adding subpattern {} to {}", &pattern.identifier, &super_pattern.identifier);

                        //     self.pattern_subs.get_mut(&super_pattern.identifier).unwrap().push(pattern.identifier); // Borrowing again
                        // }
                        break;
                    }
                }
            }

            if no_super_subs_has_subs && !had_multiple_supers{
                debug_println!("Adding subpattern {} to {}", &pattern.identifier, &super_pattern.identifier);

                self.pattern_subs.get_mut(&super_pattern.identifier).unwrap().push(pattern.identifier); // Borrowing again
            }
        }

        return false;
    }

    pub fn calculate(&mut self, patterns: Vec<Pattern>) {
        for pattern in patterns.iter(){
            self.pattern_subs.insert(pattern.identifier, Vec::new());
            self.pattern_supers.insert(pattern.identifier, Vec::new());
        }

        let all_patterns: Vec<&Pattern> = patterns.iter().collect();
        for pattern in patterns.iter(){
            let current_pattern = pattern;
            let other_patterns: Vec<&Pattern> = patterns.iter().filter(|i| *i != current_pattern).collect();
            
            debug_println!("\n=====> NORMAL call from {}", &pattern.identifier);
            self.setRelationToMultiple(current_pattern, &other_patterns, &all_patterns);
        }

        dbg!(&self.pattern_subs);
        dbg!(&self.pattern_supers);

    }
}
