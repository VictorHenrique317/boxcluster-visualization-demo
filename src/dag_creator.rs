use crate::Pattern;
use crate::Relation;

pub struct DagCreator<'a>{
    patterns: &'a Vec<Pattern>,
}

impl<'a> DagCreator<'a>{
    pub fn new(patterns: &'a Vec<Pattern>) -> Self {
        return DagCreator{
            patterns: patterns,
        };
    }

    // fn setRelationToMultiple(pattern:&Pattern, interested_patterns:Vec<Pattern>){
    //     let mut super_patterns: Vec<&'a Pattern<'a>> = Vec::new();

    //     for interested_pattern in &mut interested_patterns{ // Filter the relationed patterns
    //         let current_relation = interested_pattern.selfRelationTo(pattern);
            
    //         if current_relation.0 == Relation::SuperPattern{
    //             super_patterns.push(interested_pattern);
    //             continue;
    //         }
            
    //         if current_relation.0 == Relation::SubPattern{
    //             let wrongly_connected_patterns = &interested_pattern.super_patterns;

    //             // for wrongly_connected_pattern in &mut interested_pattern.super_patterns.iter(){
    //             for wrongly_connected_pattern in wrongly_connected_patterns{
    //                 // wrongly_connected_pattern.sub_patterns.retain(|i| *i != interested_pattern);
    //                 let mut right_sub_patterns = wrongly_connected_pattern.sub_patterns.clone();
    //                 right_sub_patterns.retain(|i| i != interested_pattern);

    //                 wrongly_connected_pattern.sub_patterns = right_sub_patterns;
    //             }
    //             interested_pattern.super_patterns = vec![pattern];
    //         }
    //     }

    //     if super_patterns.len() == 0{ // Pattern is a font
    //         return;
    //     }

    //     for super_pattern in super_patterns{ // Recursive call to set relations of possible subpatterns from each super pattern
    //         if super_pattern.sub_patterns.contains(&pattern){
    //             continue;
    //         }

    //         if super_pattern.sub_patterns.len() == 0{
    //             super_pattern.sub_patterns.push(pattern); // One end of DAG tree
    //             pattern.super_patterns.push(super_pattern);
    //             continue;

    //         } 

    //         Self::setRelationToMultiple(super_pattern, super_pattern.sub_patterns);
    //     }
    // }

    pub fn calculate(&self){
        // let mut fonts: Vec<&Pattern> = Vec::new();
        // let mut found_heritage: Vec<&Pattern> = Vec::new();
        let mut calculated_patterns: Vec<Pattern> = self.patterns.clone();
        

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
        //     }
        // }

        // for pattern in self.patterns.iter(){
        //     Self::setRelationToMultiple(pattern, self.patterns);
        // }

    }
    
}

