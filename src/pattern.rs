#![allow(non_snake_case)]
use itertools::iproduct;

pub struct Cell {
    coordinates: Vec<u32>,
}

#[derive(PartialEq, Debug)]
pub enum Relation {
    NotRelatable,
    SuperPattern,
    SubPattern,
}

#[derive(Clone, Debug)]
pub struct Pattern {
    pub identifier: u32,
    pub dims_values: Vec<Vec<u32>>, // {{1,2,3}, {3,2,1}}
    pub density: f64,
    pub super_patterns: Vec<u32>,
    pub sub_patterns: Vec<u32>,
}

impl PartialEq for Pattern {
    fn eq(&self, other: &Self) -> bool {
        if self.dims_values == other.dims_values {
            return true;
        }

        return false;
    }
}

impl Eq for Pattern {}

impl Pattern {
    pub fn new(identifier: u32, pattern_str: String) -> Self {
        let extracted_values = Pattern::extractDimsAndDensity(pattern_str);
        let dims_values = extracted_values.0;
        let density = extracted_values.1;

        return Pattern {
            identifier: identifier,
            dims_values: dims_values,
            density: density,
            super_patterns: Vec::new(),
            sub_patterns: Vec::new(),
        };
    }

    fn extractDimsAndDensity(pattern_str: String) -> (Vec<Vec<u32>>, f64) {
        let mut dims_values: Vec<Vec<u32>> = Vec::new();
        let mut density: f64 = -1.0;

        let pattern_str: Vec<String> = pattern_str.split(" ").map(|i| i.to_owned()).collect();
        let vector_length = pattern_str.len();

        for (i, dim_values_str) in pattern_str.iter().enumerate() {
            if i == vector_length - 1 {
                density = dim_values_str.replace("\r", "").parse::<f64>().unwrap();
                break;
            }

            let dim_values: Vec<u32> = dim_values_str
                .split(",")
                .map(|i| i.parse::<u32>().unwrap())
                .collect();
            dims_values.push(dim_values);
        }

        return (dims_values, density);
    }

    fn cartesianProduct(set_a: &Vec<Vec<u32>>, set_b: &Vec<u32>) -> Vec<Vec<u32>> {
        let mut result: Vec<Vec<u32>> = Vec::new();
        for i in 0..set_a.len() {
            for j in 0..set_b.len() {
                let mut temp = set_a[i].clone();
                temp.push(set_b[j]);
                result.push(temp);
            }
        }
        return result;
    }

    pub fn getCells(&self) -> Vec<Vec<u32>> {
        let n = self.dims_values.len();
        let mut temp: Vec<Vec<u32>> = self.dims_values[0]
            .clone()
            .into_iter()
            .map(|i| vec![i])
            .collect();

        for i in 1..n {
            temp = Pattern::cartesianProduct(&temp, &self.dims_values[i]);
        }
        return temp;
    }

    pub fn selfRelationTo(&self, pattern: &Pattern) -> (Relation, f64) {
        // Relation of the actual pattern
        let self_cells = self.getCells();
        let other_cells = pattern.getCells();

        let self_cell_length = self_cells.len();
        let other_cell_length = other_cells.len();

        let self_unit_increase: f64 = 1.0 / self_cell_length as f64;
        let other_unit_increase: f64 = 1.0 / other_cell_length as f64;

        let mut self_overlap_percentage = 0.0;
        let mut other_overlap_percentage = 0.0;

        let mut counter = 0.0;
        for self_cell in self_cells.iter() {
            counter += 1.0;

            for other_cell in other_cells.iter() {
                if self_cell == other_cell {
                    self_overlap_percentage += self_unit_increase;
                    other_overlap_percentage += other_unit_increase;
                }
            }
        }

        // dbg!(self_overlap_percentage);
        // dbg!(other_overlap_percentage);

        if self_overlap_percentage > other_overlap_percentage {
            return (Relation::SubPattern, self_overlap_percentage);
        }

        if other_overlap_percentage > self_overlap_percentage {
            return (Relation::SuperPattern, self_overlap_percentage);
        }

        return (Relation::NotRelatable, self_overlap_percentage);
    }
}
