#![allow(non_snake_case)]
use debug_print::debug_println;

use crate::Pattern;
use std::collections::HashMap;

pub struct Dag {
    fonts: Vec<u32>,
    mapping: HashMap<u32, Pattern>,
}

impl Dag {
    pub fn new(all_nodes: Vec<Pattern>) -> Self {
        return Dag { fonts: Vec::new(), mapping: Dag::createMapping(all_nodes)};
    }

    fn createMapping(patterns: Vec<Pattern>) -> HashMap<u32, Pattern>{
        let mut mapping: HashMap<u32, Pattern> = HashMap::new();
        for pattern in patterns{
            mapping.insert(pattern.identifier, pattern);
        }

        return mapping;
    }

    pub fn moveSubtreeBellow(&mut self, moving_node: &u32, new_parent: &u32) {
        let mut moving_node_p = self.mapping.get_mut(&moving_node).unwrap();
        let old_parents: Vec<u32> = moving_node_p.supers.clone();
        moving_node_p.supers = vec![*new_parent]; // Removes old parents and adds new super of moving node

        for old_parent in old_parents{ // Deletes moving node from its old parents
            let old_parent_p = self.mapping.get_mut(&old_parent).unwrap();
            old_parent_p.subs.retain(|p| p != moving_node);
        }

        let new_parent = self.mapping.get_mut(&new_parent).unwrap();
        new_parent.subs.push(*moving_node); // Adds moving node to its new super
    }

    pub fn addBellow(&mut self, adding_node: &u32, parent: &u32){
        let adding_node_p = self.mapping.get_mut(&adding_node).unwrap();
        adding_node_p.supers.push(*parent);

        let parent_p = self.mapping.get_mut(&parent).unwrap();
        parent_p.subs.push(*adding_node);
    }

    // pub fn moveChildrenTo(&mut self, old_parent: &u32, new_parent: &u32){
    //     let old_parent_p = self.mapping.get_mut(&old_parent).unwrap();

    //     let mut children_to_move: Vec<u32> = old_parent_p.subs.clone();
    //     children_to_move.retain(|p| p != new_parent);
    //     for children in children_to_move{
    //         self.moveSubtreeBellow(&children, new_parent);
    //     }
    // }

    pub fn addFont(&mut self, new_font: &u32){
        debug_println!("    {} is now a font", new_font);
        if !self.fonts.contains(new_font){
            self.fonts.push(*new_font);
        }
    }

    pub fn removeFont(&mut self, old_font: &u32){
        debug_println!("    {} is not a font anymore", old_font);
        self.fonts.retain(|f| f != old_font);
    }

    pub fn traverse(&self, to_pattern: &u32) -> &Vec<u32>{
        return &self.mapping.get(to_pattern).unwrap().subs;
    }

    pub fn getPattern(&self, pattern: &u32) -> &Pattern{
        return self.mapping.get(pattern).unwrap();
    }

    pub fn getNodes(&self) -> Vec<u32> {
        return self.mapping.keys().map(|n| *n).collect();
    }

    // pub fn getFontNodes(&self) -> Vec<u32> {
    //     let mut fonts: Vec<u32> = Vec::new();
    //     for node in self.mapping.values(){
    //         if node.supers.len() == 0 {
    //             fonts.push(node.identifier);
    //         }
    //     }
    //     return fonts;
    // }

    pub fn getFontNodes(&self) -> &Vec<u32> {
        return &self.fonts;
    }

    pub fn isEdge(&self, pattern: &u32) -> bool {
        let pattern_p = self.mapping.get(pattern).unwrap();
        return pattern_p.subs.len() == 0;
    }

    pub fn isFont(&self, pattern: &u32) -> bool {
        return self.mapping.get(pattern).unwrap().supers.len() == 0;
    }

    pub fn hasSubs(&self, pattern: &u32) -> bool {
        return self.mapping.get(pattern).unwrap().subs.len() != 0;
    }

    pub fn getFlattenedSubs(&self) -> HashMap<u32, Vec<u32>>{
        let mut flattened_subs: HashMap<u32, Vec<u32>> = HashMap::new();
        for pattern in self.mapping.values(){
            flattened_subs.insert(pattern.identifier, pattern.subs.clone());
        }        

        return flattened_subs;
    }

    pub fn getFlattenedSupers(&self) -> HashMap<u32, Vec<u32>>{
        let mut flattened_supers: HashMap<u32, Vec<u32>> = HashMap::new();
        for pattern in self.mapping.values(){
            flattened_supers.insert(pattern.identifier, pattern.supers.clone());
        }        

        return flattened_supers;
    }

}
