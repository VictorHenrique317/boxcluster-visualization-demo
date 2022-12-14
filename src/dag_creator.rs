#![allow(non_snake_case)]
use crate::{Dag, Pattern, Relation};
use debug_print::debug_println;
use indicatif::{ProgressBar, ProgressStyle};

pub struct DagCreator{
    actual_node: u32,
    insertion_points: Vec<u32>,
    pub dag: Dag,
}

impl DagCreator{
    pub fn new(patterns:Vec<Pattern>) -> Self{
        return DagCreator { actual_node: 0, insertion_points: Vec::new(), dag: Dag::new(patterns) };
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

    fn actualRelationTo(&self, node: &u32) -> Relation {
        let first_patern: &Pattern = self.dag.getPattern(&self.actual_node);
        let second_patern: &Pattern = self.dag.getPattern(node);
        return first_patern.selfRelationTo(second_patern);
    }

    fn isActualSubOfAny(&self, nodes: &Vec<u32>) -> bool {
        for node in nodes.iter(){
            if self.actualRelationTo(node) == Relation::SubPattern{
                return true;
            }
        }
        return false;
    }

    // fn traverseFontTree(&mut self, font: &u32){
    //     self.insertion_points.clear();
        
    //     let mut current_level_nodes: Vec<u32> = self.changePosition(*font).clone();
    //     let mut next_level_nodes: Vec<u32> = Vec::new();

    //     for current_level_node in current_level_nodes{
    //         if self.actualRelationTo(&current_level_node) == Relation::SubPattern{
    //             next_level_nodes.push(current_level_node);
    //         }
    //     }

    //     if next_level_nodes.len() == 0 { // No next node, the only insertion point is the font
    //         self.insertion_points.push(*font);
    //         return;
    //     }

    //     if !self.isActualSubOfAny(&next_level_nodes){ // Actual is not sub of any node in next level, the only insertion point is the font
    //         self.insertion_points.push(*font);
    //         return;
    //     }

    //     // Here actual node is sub of one or more nodes on the next level
    //     for next_level_node in next_level_nodes{
    //         self.traverseTree(&next_level_node);
    //     }



    // }

    fn traverseTree(&mut self, subtree_font: &u32){
        let current_level_nodes: Vec<u32> = self.changePosition(*subtree_font).clone();
        let mut next_level_nodes: Vec<u32> = Vec::new();

        for current_level_node in current_level_nodes{
            let relation = self.actualRelationTo(&current_level_node);

            if relation == Relation::SubPattern{
                next_level_nodes.push(current_level_node);
                continue;
            }
        }

        if next_level_nodes.len() == 0 { // No next node, insertion point is the font of this subtree
            self.insertion_points.push(*subtree_font);
            return;
        }

        if !self.isActualSubOfAny(&next_level_nodes){ // Actual is not sub of any node in next level, insertion point is the font of this subtree
            self.insertion_points.push(*subtree_font);
            return;
        }

        // Here actual node is sub of one or more nodes on the next level
        for next_level_node in next_level_nodes{
            self.traverseTree(&next_level_node);
        }

    }

    fn getRelationedFonts(&self, node: &u32) -> Vec<u32> {
        let fonts: &Vec<u32> = self.dag.getFontNodes();
        let mut relationed_fonts: Vec<u32> = Vec::new();

        for font in fonts{
            if self.firstRelationToSecond(node, &font) == Relation:: SubPattern{
                relationed_fonts.push(*font);
            }
        }
        return relationed_fonts;
    }

    fn setInsertionPoints(&mut self, node: &u32){
        self.insertion_points.clear();
        let relationed_fonts = self.getRelationedFonts(node);

        if relationed_fonts.len() == 0{
            // This node is a new font
            self.dag.addFont(node);
            return;
        }

        for relationed_font in relationed_fonts {
            // Finds the insertion points on each font subtree
            self.traverseTree(&relationed_font);
        }
    }

    fn insertNodeOnDag(&mut self, node: &u32){
        for insertion_point in self.insertion_points.iter(){
            let subs =  self.dag.traverse(insertion_point).clone();

            if subs.len() == 0{
                // No subpatterns on insertion point, just add the new node directly
                self.dag.addBellow(node, insertion_point);
                continue;
            }

            // Here the insertion point has one or more children
            for sub in subs{
                if self.firstRelationToSecond(node, &sub) == Relation::SuperPattern{
                    // If the node is super of someone rearrange dag
                    self.dag.moveSubtreeBellow(&sub, node);
                }
            }
            
        }
    }

    pub fn create(&mut self){
        let unorganized_nodes: Vec<u32> = self.dag.getNodes();
        
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
    }

    
}