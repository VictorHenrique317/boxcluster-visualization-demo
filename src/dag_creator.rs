#![allow(non_snake_case)]
use std::collections::HashMap;
use colored::Colorize;

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
    belonging_level: u32,
    belonging_branch: u32,

    pub dag: Dag,
}

impl DagCreator{
    pub fn new(patterns:Vec<Pattern>) -> Self{
        // return DagCreator { actual_node: 0, insertion_points: Vec::new(), relation_matrix: HashMap::new(), dag: Dag::new(patterns) };
        return DagCreator { actual_node: 0, insertion_points: HashMap::new(), assigned_belonging_level: false, belonging_level:0, belonging_branch:0, dag: Dag::new(patterns) };
    }

    fn changePosition(&mut self, new_position: u32) -> &Vec<u32> {
        self.actual_node = new_position;
        return self.dag.traverse(&self.actual_node);
    }

    fn firstRelationToSecond(&self, first_node: &u32, second_node: &u32) -> Relation {
        let first_patern: &Pattern = self.dag.getPattern(first_node);
        let second_patern: &Pattern = self.dag.getPattern(second_node);
        return first_patern.selfRelationTo(second_patern);
    }

    fn traverseTree(&mut self, subtree_font: &u32, node_to_compare: &u32, current_branch: u32, current_level: u32){
        debug_print!("\n    => Traversing subtree of {}, ", subtree_font);
        let current_level_nodes: Vec<u32> = self.changePosition(*subtree_font).clone();
        let mut next_level_fonts: Vec<u32> = Vec::new();
        debug_println!("level: {}, level size: {}, branch: {}, belonging_branch: {}, belonging_level: {}", &current_level, current_level_nodes.len(), &current_branch, &self.belonging_branch, &self.belonging_level);

        let mut belongs_to_this_level: bool = false;
        let relation = self.firstRelationToSecond(node_to_compare, &subtree_font);
        if relation == Relation::SubPattern{
            belongs_to_this_level = true;
        }
        
        for current_level_node in current_level_nodes.iter(){
            if relation == Relation::SuperPattern{ // Node to compare is super of subtree_font, does not need to traverse this branch
                continue;
            }

            if relation == Relation::NotRelatable{ // Node to compare does not have 'physical' contact with subtree_font, does not need to traverse this branch
                continue;
            }
            next_level_fonts.push(*current_level_node);
        }

        for (i, next_level_font) in next_level_fonts.iter().enumerate(){ // RECURSIVE
            let next_branch = current_branch + i as u32;
            self.traverseTree(&next_level_font, node_to_compare, next_branch, current_level + 1);
        }
        // Recursion returnal bellow

        // Insertion operation
        if belongs_to_this_level && !self.assigned_belonging_level{ // Makes sure to insert on the deepest possible
            debug_println!("    Setting insertion point to bellow {}", subtree_font);
            
            self.insertion_points.insert(*subtree_font, InsertionPlace::Bellow);
            self.assigned_belonging_level = true; // Previous levels cannot change insertion point now
            self.belonging_level = current_level;
            self.belonging_branch = current_branch;
            debug_println!("    Belonging branch is now {}", &self.belonging_branch);
        }

        // Connects node_to_compare as super of this subtree font
        if relation == Relation::SuperPattern{
            // A pattern (node_to_compare) from an upper branch is super of the font of this branch
            // Sets the super relation on the recursion returnal
            debug_println!("    {} {} {}{}", format!("{}", &node_to_compare).yellow(), "located in a different upper branch is super of".yellow(), format!("{}", &subtree_font).yellow(), ", CONNECTING them".yellow());
            self.dag.addBellow(subtree_font, node_to_compare);
        }
    
        // Connects node_to_compare as sub of different branches
        if relation == Relation::SubPattern && current_branch != self.belonging_branch{ // Makes sure to connect ONLY to different branches
            // A pattern (node_to_compare) from a DIFFERENT branch is sub of the font of this branch
            // Sets the sub relation on the recursion returnal

            if current_level < self.belonging_level{ // Avoids the connection of patterns that are above the insertion point
                return;
            }

            debug_println!("    {} {} {}{}", format!("{}", node_to_compare).yellow(), "located in a different below branch is sub of".yellow(), format!("{}", &subtree_font).yellow(), ", CONNECTING them".yellow());
            self.dag.addBellow(node_to_compare, subtree_font);
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
        debug_println!("{} {}", "\n=> SETTING insertion points for node".green(), format!("{}", node).green());
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
            self.belonging_branch = 0;
            self.belonging_level = 0;
            self.traverseTree(&relationed_font.0, node, 1, 2);
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
        debug_println!("{} {} {}", "\n==> INSERTING node".yellow(), format!("{}", node).yellow(), "on DAG".yellow());
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
        // let unorganized_nodes: Vec<u32> = vec![2, 5, 1, 4, 3]; // FOR TESTING ONLY

        debug_println!("Unorganized nodes: {:?}", &unorganized_nodes);

        let bar = ProgressBar::new(unorganized_nodes.len() as u64);
        bar.set_message("Checked patterns");
        bar.set_style(ProgressStyle::with_template("{msg}: {bar:40.cyan/blue} {pos:>7}/{len:7} Elapsed time: {elapsed} | Estimated time:{eta} ")
            .unwrap()
            .progress_chars("##-"));  
        
        for unorganized_node in unorganized_nodes{
            self.setInsertionPoints(&unorganized_node);
            self.insertNodeOnDag(&unorganized_node);
            bar.inc(1);
        }
        bar.finish();

        debug_println!("Subs: {:?}", self.dag.getFlattenedSubs());
        debug_println!("Supers: {:?}", self.dag.getFlattenedSupers());

        println!("Nb of found fonts: {}", self.dag.getFontNodes().len());
    }

    
}