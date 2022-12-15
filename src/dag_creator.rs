#![allow(non_snake_case)]
use std::collections::HashMap;

use crate::{Dag, Pattern, Relation};
use debug_print::{debug_println, debug_print};
use indicatif::{ProgressBar, ProgressStyle};

#[derive(PartialEq, Debug, Clone, Copy)]
enum InsertionPlace{
    Bellow,
    Above,
}
pub struct DagCreator{
    actual_node: u32,
    insertion_points: HashMap<u32, InsertionPlace>,
    assigned_belonging_level: bool,
    // relation_matrix: HashMap<Vec<u32>, Relation>,
    pub dag: Dag,
}

impl DagCreator{
    pub fn new(patterns:Vec<Pattern>) -> Self{
        // return DagCreator { actual_node: 0, insertion_points: Vec::new(), relation_matrix: HashMap::new(), dag: Dag::new(patterns) };
        return DagCreator { actual_node: 0, insertion_points: HashMap::new(), assigned_belonging_level: false, dag: Dag::new(patterns) };
    }

    // fn putRelationInMemory(&mut self, node1: &u32, node2: &u32, relation: &Relation){
    //     let mut key: Vec<u32> = vec![*node1, *node2];

    //     self.relation_matrix.insert(key, *relation);
    // }
    
    // fn getRelationFromMemory(&self, node1: &u32, node2: &u32) -> Relation{
    //     let mut key: Vec<u32> = vec![*node1, *node2];

    //     return *self.relation_matrix.get(&key).unwrap();
    // }

    fn changePosition(&mut self, new_position: u32) -> &Vec<u32> {
        self.actual_node = new_position;
        return self.dag.traverse(&self.actual_node);
    }

    fn firstRelationToSecond(&self, first_node: &u32, second_node: &u32) -> Relation {
        let first_patern: &Pattern = self.dag.getPattern(first_node);
        let second_patern: &Pattern = self.dag.getPattern(second_node);
        return first_patern.selfRelationTo(second_patern);
    }

    fn actualRelationTo(&self, node: &u32) -> Relation {
        let first_patern: &Pattern = self.dag.getPattern(&self.actual_node);
        let second_patern: &Pattern = self.dag.getPattern(node);
        return first_patern.selfRelationTo(second_patern);
    }

    fn isNodeSubOfAny(&self, node_to_compare: &u32, nodes: &Vec<u32>) -> bool {
        for node in nodes.iter(){
            if self.firstRelationToSecond(node_to_compare, node) == Relation::SubPattern{
                return true;
            }
        }
        return false;
    }

    fn traverseTree(&mut self, subtree_font: &u32, node_to_compare: &u32){
        debug_print!("\n    => Traversing subtree of {}, ", subtree_font);
        let current_level_nodes: Vec<u32> = self.changePosition(*subtree_font).clone();
        let mut next_level_fonts: Vec<u32> = Vec::new();
        debug_println!(" current_level size: {}", current_level_nodes.len());

        let mut belongs_to_this_level = false;
        let relation = self.firstRelationToSecond(node_to_compare, &subtree_font);
        if relation == Relation::SubPattern{
            belongs_to_this_level = true; 
        }
        
        if relation == Relation::SuperPattern{
            // A pattern from a upper branch is super of the font of this branch
            // Set the super relation without interfering on the traversing
            debug_println!("    {} located in an upper branch is super of {}, CONNECTING them", &node_to_compare, &subtree_font);
            self.dag.addBellow(subtree_font, node_to_compare);
        }
        for current_level_node in current_level_nodes.iter(){
            next_level_fonts.push(*current_level_node);
            
            // let relation = self.firstRelationToSecond(node_to_compare, &current_level_node);
            
        }

        for next_level_font in next_level_fonts{
            self.traverseTree(&next_level_font, node_to_compare, );
        }

        // if current_level_nodes.len() == 0 && belongs_to_this_level{ // Empty level
        if belongs_to_this_level && !self.assigned_belonging_level{
            self.insertion_points.insert(*subtree_font, InsertionPlace::Bellow);
            self.assigned_belonging_level = true; // Previous branches cannot change insertion point now
            debug_println!("\n    Setting insertion point to bellow {}", subtree_font);
        }

    }

    fn getRelationedFonts(&self, node: &u32) -> HashMap<u32, Relation> {
        let fonts: &Vec<u32> = self.dag.getFontNodes();
        debug_println!("    Current fonts {:?}", fonts);
        let mut relationed_fonts: HashMap<u32, Relation> = HashMap::new();

        for font in fonts{
            let relation = self.firstRelationToSecond(node, &font);
            if relation == Relation::NotRelatable{
                continue;
            }

            relationed_fonts.insert(*font, relation);
        }
        return relationed_fonts;
    }

    fn setInsertionPoints(&mut self, node: &u32){
        debug_println!("\n=> SETTING insertion points for node {}", node);
        self.insertion_points.clear();
        let relationed_fonts: HashMap<u32, Relation> = self.getRelationedFonts(node);

        if relationed_fonts.len() == 0{
            // This node is a new font
            debug_println!("    Node does not have any relationed font, setting it to be a new font");
            return;
        }

        debug_println!("    Found relationed fonts: {:?}", &relationed_fonts);

        for relationed_font in relationed_fonts {
            // Finds the insertion points on each font subtree
            if relationed_font.1 == Relation::SuperPattern{
                // Node is super of relationed_font, consequently node is the new font
                debug_println!("    {} is super of {}, setting insertion point to be above {}", node, &relationed_font.0, &relationed_font.0);
                self.insertion_points.insert(relationed_font.0, InsertionPlace::Above);
                continue;
            }

            self.assigned_belonging_level = false;
            self.traverseTree(&relationed_font.0, node);
        }
    }

    fn insertNodeAbove(&mut self, node: &u32, insertion_point: &u32){
        debug_println!("    Inserting {} above {}", node, insertion_point);
        self.dag.moveSubtreeBellow(insertion_point, node);
    }

    fn insertNodeBellow(&mut self, node: &u32, insertion_point: &u32){
        let subs = self.dag.traverse(insertion_point).clone();

        debug_println!("    Inserting {} bellow {}", node, insertion_point);
        self.dag.addBellow(node, insertion_point);

        if subs.is_empty(){
            return;
        }

        debug_println!("    Insertion point has subs {:?}", &subs);
        for sub in subs{
            if self.firstRelationToSecond(node, &sub) == Relation::SuperPattern{
                // If the node is super of someone rearrange dag
                debug_println!("    {} is super of {}, putting {} subtree bellow {}", node, &sub, &sub, node);
                self.dag.moveSubtreeBellow(&sub, node);
            }
        }
        
    }

    fn insertNodeOnDag(&mut self, node: &u32){
        debug_println!("\n==> INSERTING node {} on DAG", node);
        debug_println!("    Insertion points: {:?} (empty if is new font)", &self.insertion_points);

        if self.insertion_points.is_empty(){
            self.dag.addFont(node);
        }

        for insertion_point in self.insertion_points.clone().iter(){
            debug_println!();
            let insertion_place = insertion_point.1;
            let insertion_point = insertion_point.0;

            if *insertion_place == InsertionPlace::Above{
                // This should only trigger if the dag has draw a pseudo-font
                self.dag.removeFont(insertion_point);
                self.dag.addFont(node);
                
                self.insertNodeAbove(node, insertion_point);
                continue;
            }

            if *insertion_place == InsertionPlace::Bellow{
                self.insertNodeBellow(node, insertion_point);
                continue;
            }

            
        }
    }

    pub fn create(&mut self){
        let unorganized_nodes: Vec<u32> = self.dag.getNodes();
        debug_println!("Unorganized nodes: {:?}", &unorganized_nodes);

        let bar = ProgressBar::new(unorganized_nodes.len() as u64);
        bar.set_message("Checked patterns");
        bar.set_style(ProgressStyle::with_template("{msg}: {bar:40.cyan/blue} {pos:>7}/{len:7} Elapsed time: {elapsed} | Estimated time:{eta} ")
            .unwrap()
            .progress_chars("##-"));  
        
        for unorganized_node in unorganized_nodes{
            self.setInsertionPoints(&unorganized_node);
            self.insertNodeOnDag(&unorganized_node);
            // bar.inc(1);
        }

        bar.finish();

        debug_println!("Subs: {:?}", self.dag.getFlattenedSubs());
        debug_println!("Supers: {:?}", self.dag.getFlattenedSupers());
    }

    
}