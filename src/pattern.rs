#![allow(non_snake_case)]
use std::collections::HashSet;

use debug_print::{debug_println, debug_print};
use itertools::iproduct;

pub struct Cell {
    coordinates: Vec<u32>,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Relation {
    NotRelatable,
    SuperPattern,
    SubPattern,
}

#[derive(Clone, Debug)]
pub struct Pattern {
    pub identifier: u32,
    pub dims_values: Vec<HashSet<u32>>, // {{1,2,3}, {3,2,1}}
    pub density: f64,
    pub size: u32,
    pub supers: Vec<u32>,
    pub subs: Vec<u32>,
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
        let size = Pattern::getSize(&dims_values);

        return Pattern {
            identifier: identifier,
            dims_values: dims_values,
            density: density,
            size: size,
            supers: Vec::new(),
            subs: Vec::new(),
        };
    }

    fn extractDimsAndDensity(pattern_str: String) -> (Vec<HashSet<u32>>, f64) {
        let mut dims_values: Vec<HashSet<u32>> = Vec::new();
        let mut density: f64 = -1.0;

        let pattern_str: Vec<String> = pattern_str.split(" ").map(|i| i.to_owned()).collect();
        let vector_length = pattern_str.len();

        for (i, dim_values_str) in pattern_str.iter().enumerate() {
            if i == vector_length - 1 {
                // density = dim_values_str.replace("\r", "").parse::<f64>().unwrap();
                density = match dim_values_str.replace("\r", "").parse::<f64>(){
                    Ok(d) => d,
                    Err(_error) => 1.0,
                };

                break;
            }

            let dim_values: HashSet<u32> = dim_values_str
                .split(",")
                .map(|i| i.parse::<u32>().unwrap())
                .collect();
            dims_values.push(dim_values);
        }

        return (dims_values, density);
    }

    fn getSize(dims_values: &Vec<HashSet<u32>>) -> u32{
        let mut size: u32 = 1;

        for dims_value in dims_values{
            size *= dims_value.len() as u32;
        }
        return size;
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

    pub fn selfRelationTo(&self, pattern: &Pattern) -> Relation {
        debug_print!("    Comparing patterns {} to {}: ", &self.identifier, &pattern.identifier);
        if self.identifier == pattern.identifier{
            debug_println!("{:?}", Relation::NotRelatable);
            return Relation::NotRelatable;
        }
        
        // Relation of the actual pattern
        let self_dims_values = self.dims_values.iter();
        let mut other_dims_values = pattern.dims_values.iter();

        for self_dims_value in self_dims_values{
            let other_dims_value = other_dims_values.next().unwrap();

            let intersection = self_dims_value.intersection(other_dims_value).count();
            if intersection == 0{ // No physical contact
                debug_println!("{:?}", Relation::NotRelatable);
                return Relation::NotRelatable;
            }
        }

        if self.size > pattern.size{
            debug_println!("{:?}", Relation::SuperPattern);
            return Relation::SuperPattern;
        }

        if self.size < pattern.size{
            debug_println!("{:?}", Relation::SubPattern);
            return Relation::SubPattern;
        }

        // Equal sizes
        debug_println!("{:?}", Relation::NotRelatable);
        return Relation::NotRelatable;
        
    }
}