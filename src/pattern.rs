#![allow(non_snake_case)]
use itertools::iproduct;

pub struct Cell {
    coordinates: Vec<u32>,
}

pub enum Relation {
    NotContained,
    SuperPattern,
    SubPattern,
}

pub struct Pattern {
    pub dims_values: Vec<Vec<u32>>, // {{1,2,3}, {3,2,1}}
    pub density: f64,
}

impl Pattern {
    pub fn new(pattern_str: String) -> Self {
        let extracted_values = Pattern::extractDimsAndDensity(pattern_str);
        let dims_values = extracted_values.0;
        let density = extracted_values.1;

        return Pattern {
            dims_values: dims_values,
            density: density,
        };
    }

    fn extractDimsAndDensity(pattern_str: String) -> (Vec<Vec<u32>>, f64) {
        let mut dims_values: Vec<Vec<u32>> = Vec::new();
        let mut density: f64 = -1.0;

        let pattern_str: Vec<String> = pattern_str.split(" ").map(|i| i.to_owned()).collect();
        let vector_length = pattern_str.len();

        for (i, dim_values_str) in pattern_str.iter().enumerate() {
            if i == vector_length - 1 {
                density = dim_values_str.parse::<f64>().unwrap();
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

    pub fn getCells(&self) -> Vec<Vec<u32>>{
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

    pub fn getRelation(pattern: Pattern) -> Relation {
        return Relation::NotContained;
    }
}
