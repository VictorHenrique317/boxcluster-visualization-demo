#![allow(non_snake_case)]
use crate::Pattern;
use crate::Relation;
use std::collections::HashMap;
use std::collections::hash_map::ValuesMut;
use std::hash::Hash;
use std::mem::replace;

pub struct DagCreator {
    // pattterns_mapping: HashMap<u32, Pattern>,
    pattern_subs: HashMap<u32, Vec<u32>>,
    pattern_supers: HashMap<u32, Vec<u32>>,
}

impl DagCreator {
    pub fn new() -> Self {
        return DagCreator{pattern_subs: HashMap::new(), pattern_supers: HashMap::new()};
    }

    fn setRelationToMultiple(&mut self, pattern: &Pattern, interested_patterns:&Vec<&Pattern>, all_patterns: &Vec<&Pattern>) -> bool{
        let mut super_patterns: Vec<&Pattern> = Vec::new();

        for interested_pattern in interested_patterns { // Filter the relationed patterns
            let current_relation = interested_pattern.selfRelationTo(&pattern);
            println!("{} relation to {}: {:?}", &interested_pattern.identifier, &pattern.identifier, &current_relation);

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
                    
                    println!("   {} is wrongly connected to {}, switching connections\n", &interested_pattern.identifier, &pattern.identifier);
                    // Here only the relations of nodes != pattern will be changed (expected)

                    let wrong_subpatterns = self.pattern_subs.get_mut(wrongly_connected_pattern).unwrap();
                    wrong_subpatterns.retain(|identifier| *identifier != interested_pattern.identifier);
                }

                let interested_pattern_supers = self.pattern_supers.get_mut(&interested_pattern.identifier).unwrap();
                *interested_pattern_supers = vec![pattern.identifier];
            }
        }

        if super_patterns.len() == 0 {
            // Pattern is a font
            println!("No super patterns found for {}", &pattern.identifier);
            return false;
        }

        println!("Super patterns found for {}: {:?}", &pattern.identifier, &super_patterns.iter().map(|i| i.identifier).collect::<Vec<u32>>());
        for super_pattern in super_patterns {
            // Recursive call to set relations of possible subpatterns from each super pattern
            // if super_pattern.sub_patterns.contains(&pattern.identifier) {
            let sub_patterns = self.pattern_subs.get_mut(&super_pattern.identifier).unwrap();

            if sub_patterns.contains(&pattern.identifier) && sub_patterns.len() == 1{ // maybe remove length check?
                return true;
            }

            if sub_patterns.contains(&pattern.identifier) { // possible bug?
                continue;
            }

            if sub_patterns.len() == 0 {
                println!("\nHit one end of DAG tree");
                println!("Adding subpattern {} to {}", &pattern.identifier, &super_pattern.identifier);

                sub_patterns.push(pattern.identifier); // One end of DAG tree

                if !self.pattern_supers.get_mut(&pattern.identifier).unwrap().contains(&super_pattern.identifier){
                    // Blocks double adition in case block current_relation.0 == Relation::SubPattern is executed
                    
                    println!("Adding superpattern {} to {}", &super_pattern.identifier, &pattern.identifier);
                    self.pattern_supers.get_mut(&pattern.identifier).unwrap().push(super_pattern.identifier)
                }

                continue;
            }

            let super_subs_identifiers: &Vec<u32> = &self.pattern_subs.get(&super_pattern.identifier).unwrap().clone();
            // let mut super_subs: Vec<&Pattern> = Vec::new();

            for identifier in super_subs_identifiers{
            // for identifier in sub_patterns.iter(){
                for possible_super_sub in all_patterns.iter(){
                    if possible_super_sub.identifier == *identifier{
                        // super_subs.push(possible_super_sub);

                        println!("\n=====> RECURSIVE call to see {}", &possible_super_sub.identifier);
                        let mut interested_patterns = interested_patterns.clone();
                        interested_patterns.retain(|i| i.identifier != possible_super_sub.identifier);

                        let is_dag_end = self.setRelationToMultiple(possible_super_sub, &interested_patterns, all_patterns);
                        // if is_dag_end{
                        //     println!("\nHit one end of DAG tree");
                        //     println!("Adding subpattern {} to {}", &pattern.identifier, &super_pattern.identifier);
                        //     sub_patterns.push(pattern.identifier);
                        // }
                        break;
                    }
                }
            }


            
        }

        return false;
    }

    pub fn calculate(&mut self, patterns: Vec<Pattern>) {
        // let mut fonts: Vec<&Pattern> = Vec::new();
        // let mut found_heritage: Vec<&Pattern> = Vec::new();
        // let mut calculated_patterns: Vec<Pattern> = self.patterns.clone();

        // for outer_pattern in self.patterns{
        //     let mut is_font = true;

        //     for inner_pattern in self.patterns{
        //         if outer_pattern.selfRelationTo(inner_pattern) == Relation::SubPattern{
        //             is_font = false;
        //             break;
        //         }
        //     }

        //     if is_font{
        //         fonts.insert(outer_pattern);
        //     
        // }

        // let mut patterns_to_change = patterns.clone();
        
        // for pattern in patterns{
        //     let current_pattern = pattern;
        //     let other_patterns: Vec<Pattern> = Vec::new();

        //     for pattern2 in patterns{
        //         if pattern != pattern2{
        //             other_patterns.push(pattern);
        //         } 
        //     }

        //     patterns_to_change = DagCreator::setRelationToMultiple(patterns_to_change, current_pattern, other_patterns);
        // }


        for pattern in patterns.iter(){
            self.pattern_subs.insert(pattern.identifier, Vec::new());
            self.pattern_supers.insert(pattern.identifier, Vec::new());
        }

        let all_patterns: Vec<&Pattern> = patterns.iter().collect();
        for pattern in patterns.iter(){
            let current_pattern = pattern;
            let other_patterns: Vec<&Pattern> = patterns.iter().filter(|i| *i != current_pattern).collect();
            
            println!("\n=====> NORMAL call from {}", &pattern.identifier);
            self.setRelationToMultiple(current_pattern, &other_patterns, &all_patterns);
        }

        dbg!(&self.pattern_subs);
        dbg!(&self.pattern_supers);

    }
}
