use crate::Pattern;
use crate::Relatoion
use std::collections::HashSet;

pub struct DagCreator {
    patterns: HashSet<Pattern>,
    // patterns_identifiers: Vec<u32>,
}

impl DagCreator {
    pub fn new(patterns: &Vec<Pattern>) -> Self {
        return DagCreator {
            pattern: HashSet::from_iter(patterns.iter().cloned()),
        };
    }

    fn setRelationToMultiple(pattern:Pattern, interested_patterns:HashSet<Pattern>){
        let relationed_patterns: HashSet<Pattern> = HashSet::new();

        for interested_pattern in interested_patterns{ // Filter the relationed patterns
            current_relation = pattern.selfRelationTo(interested_pattern);
            
            if current_relation.0 == Relation::SubPattern{
                relationed_patterns.insert(interested_pattern);
            }
            
            if current_relation.0 == Relation::SuperPattern{
                panic!("Super patterns relations should not reach this point");
            }
        }


        for relationed_pattern in relationed_patterns{ // Recursive call to set relations of possible subpatterns from each relationed pattern

        }
    }

    pub fn calculate(&self) {
        let mut fonts: HashSet<Pattern> = HashSet::new();
        let mut found_heritage: HashSet<Pattern> = HashSet::new();
        

        for outer_pattern in self.patterns{
            let mut is_font = true;

            for inner_pattern in self.patterns{
                if outer_pattern.selfRelationTo(inner_pattern) == Relation::SubPattern{
                    is_font = false;
                    break;
                }
            }   

            if is_font{
                fonts.insert(outer_pattern);
            }
        }

        for pattern in self.patterns{
            self.setRelationToMultiple(pattern, fonts);
        }
    }
    
}

