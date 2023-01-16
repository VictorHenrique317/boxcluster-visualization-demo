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
    Overlaps,
    SuperPattern,
    SubPattern,
}

#[derive(Clone, Debug)]
pub struct Pattern {
    pub identifier: u32,
    pub dims_values: Vec<Vec<u32>>, // {{1,2,3}, {3,2,1}}
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

    fn extractDimsAndDensity(pattern_str: String) -> (Vec<Vec<u32>>, f64) {
        let mut dims_values: Vec<Vec<u32>> = Vec::new();
        let mut density: f64 = -1.0;

        let pattern_str: Vec<String> = pattern_str.split(" ").map(|i| i.to_owned()).collect();
        let vector_length = pattern_str.len();

        for (i, dim_values_str) in pattern_str.iter().enumerate() {
            if i == vector_length - 1 {
                // Tries to get the density
                density = match dim_values_str.replace("\r", "").parse::<f64>(){
                    Ok(d) => d,
                    Err(_error) => -1.0,
                };

                if density != -1.0{ // Pattern HAS density in file
                    break;
                }
            }

            let mut dim_values: Vec<u32> = dim_values_str.replace("\r", "")
                .split(",")
                .map(|i| i.parse::<u32>().unwrap())
                .collect();
            dim_values.sort();

            dims_values.push(dim_values);
        }

        if density == -1.0{ // Pattern HAS NO density in file
            density = 1.0;
        }

        return (dims_values, density);
    }

    fn getSize(dims_values: &Vec<Vec<u32>>) -> u32{
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

    fn intersectionPercentage(vector: &Vec<u32>, base: &Vec<u32>) -> f64 { // Only works for sorted vectors
        let reference_area = vector.len() as f64;
        let mut used_vector = vector;
        let mut used_base = base;

        if vector.len() > base.len(){
            // One dimension of possible sub 'vector' is larger than the corresponding dim on base, so its not contained in base
            used_vector = base;
            used_base = vector;
            // Switches the vectors of place so that vector is always smaller than base
            // panic!("Wrong use of intersection method");
        }

        let mut current_index = 0;
        let mut contained_values_sum = 0;
        let mut stop = false;

        for element in used_vector{
            while true{
                let base_element = used_base.get(current_index);
            
                if base_element.is_none(){ // Index out of boudaries
                    stop = true;
                    break;
                }

                let base_element = base_element.unwrap();

                if base_element > element { // If the vector is sorted the value will not be found anymore
                    break;
                }

                current_index += 1; // Element is lesser or equal than base element, can change index

                if element == base_element{
                    contained_values_sum += 1;
                    break;
                }
            }

            if stop{
                break;
            }

        }

        return contained_values_sum as f64 / reference_area; // Percetange of intersection on VECTOR
    }

    pub fn selfRelationTo(&self, pattern: &Pattern) -> Relation {
        debug_print!("    Comparing patterns {} to {}: ", &self.identifier, &pattern.identifier);
        if self.identifier == pattern.identifier{
            debug_println!("{:?} (Identical patterns)", Relation::NotRelatable);
            return Relation::NotRelatable;
        }  
        
        // Relation of the actual pattern
        let self_dims_values = self.dims_values.iter();
        let mut other_dims_values = pattern.dims_values.iter();

        for self_dims_value in self_dims_values{
            let other_dims_value = other_dims_values.next().unwrap();

            let mut intersection_percentage: f64;

            if self.size > pattern.size{ // Self is possible super
                intersection_percentage = Pattern::intersectionPercentage(other_dims_value, self_dims_value);
            }
            else if pattern.size > self.size{ // Pattern is possible super
                intersection_percentage = Pattern::intersectionPercentage(self_dims_value, other_dims_value);
            }
            else{ // No one is super but there may be an overlap
                intersection_percentage = Pattern::intersectionPercentage(other_dims_value, self_dims_value); // Doesn't matter the order
            }

            // intersection_percentage = Pattern::intersectionPercentage(self_dims_value, other_dims_value);

            if intersection_percentage == 0.0{
                debug_println!("{:?}", Relation::NotRelatable);
                return Relation::NotRelatable;
            }

            if intersection_percentage < 1.0{
                debug_println!("{:?}", Relation::Overlaps);
                return Relation::Overlaps;
            }
        }

        // Here all dimensions have 100% intersection

        if self.size > pattern.size{
            debug_println!("{:?}", Relation::SuperPattern);
            return Relation::SuperPattern;
        }

        if self.size < pattern.size{
            debug_println!("{:?}", Relation::SubPattern);
            return Relation::SubPattern;
        }

        // Its the same pattern if the execution reaches here, duplicated patterns exist in the input file
        panic!("Duplicated patterns detected in input file: {} and {}", &self.identifier, &pattern.identifier);
        
    }
}