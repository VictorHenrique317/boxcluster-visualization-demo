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
        // let mut patterns_mapping: HashMap<u32, Pattern> = HashMap::new();
        // for pattern in patterns {
        //     patterns_mapping.insert(pattern.identifier, pattern);
        // }

        // return DagCreator {
        //     pattterns_mapping: patterns_mapping,
        // };   
        return DagCreator{pattern_subs: HashMap::new(), pattern_supers: HashMap::new()};
    }

    // fn getSuperPatternsOf(identifier: u32) -> Vec<u32>{
    //     return Vec::new();
    // }

    fn setRelationToMultiple(&mut self, pattern: &Pattern, interested_patterns:Vec<&Pattern>){
        let mut super_patterns: Vec<&Pattern> = Vec::new();

        for interested_pattern in interested_patterns {
            // Filter the relationed patterns
            let current_relation = interested_pattern.selfRelationTo(&pattern);

            if current_relation.0 == Relation::SuperPattern {
                super_patterns.push(interested_pattern);
                continue;
            }

            if current_relation.0 == Relation::SubPattern {
                // let wrongly_connected_patterns = interested_pattern.super_patterns;
                // let wrongly_connected_patterns = DagCreator::getSuperPatternsOf(interested_pattern.identifier);
                let wrongly_connected_patterns = self.pattern_supers.get(&interested_pattern.identifier).unwrap();

                for wrongly_connected_pattern in wrongly_connected_patterns {

                    let wrong_subpatterns = self.pattern_subs.get_mut(wrongly_connected_pattern).unwrap();
                    wrong_subpatterns.retain(|identifier| *identifier != interested_pattern.identifier);
                }

                // DagCreator::getSuperPatternsOf(interested_pattern.identifier);
                // interested_pattern.super_patterns = vec![pattern];
                let interested_pattern_supers = self.pattern_supers.get_mut(&interested_pattern.identifier).unwrap();
                *interested_pattern_supers = vec![pattern.identifier];
            }
        }

        if super_patterns.len() == 0 {
            // Pattern is a font
            return;
        }

        for super_pattern in super_patterns {
            // Recursive call to set relations of possible subpatterns from each super pattern
            // if super_pattern.sub_patterns.contains(&pattern.identifier) {
            let sub_patterns = self.pattern_subs.get_mut(&super_pattern.identifier).unwrap();
            if sub_patterns.contains(&pattern.identifier) {
                continue;
            }

            if sub_patterns.len() == 0 {
                sub_patterns.push(pattern.identifier); // One end of DAG tree
                self.pattern_supers.get_mut(&pattern.identifier).unwrap().push(super_pattern.identifier);
                continue;
            }

            // let subpatterns: Vec<Pattern> = super_pattern
            //     .sub_patterns
            //     .iter()
            //     .map(|i| self.pattterns_mapping.get(i).unwrap())
            //     .collect();

            // result = Self::setRelationToMultiple(result, super_pattern, subpatterns);
        }


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

        for pattern in patterns.iter(){
            let current_pattern = pattern;
            let other_patterns: Vec<&Pattern> = patterns.iter().filter(|i| *i != current_pattern).collect();
            self.setRelationToMultiple(current_pattern, other_patterns);
        }

        dbg!(&self.pattern_subs);
        dbg!(&self.pattern_supers);

    }
}
