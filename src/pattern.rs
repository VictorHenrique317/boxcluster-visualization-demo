struct DimValues {
    // {3,2,1}
    values: Vec<u32>,
}

impl DimValues {
    fn new(dim_values_str: String) -> Self {
        let dim_values: Vec<u32> = dim_values_str
            .split(",")
            .map(|i| i.parse::<u32>().unwrap())
            .collect();
        return DimValues { values: dim_values };
    }
}

pub struct Pattern {
    tuples: Vec<DimValues>, // {{1,2,3}, {3,2,1}}
    density: f64,
}

impl Pattern {
    fn new(pattern_str: String) -> Self {
        return Pattern {
            tuples: Pattern::extractDimsValues(pattern_str),
            density: Pattern::extractDensity(pattern_str),
        };
    }

    fn extractDimsValues(pattern_str: String) -> Vec<DimValues> {
        let dims_values: Vec<DimValues> = Vec::new();

        for dim_values_str in pattern_str.split(" ") {
            dims_values.push(DimValues::new(dim_values_str.to_owned()));
        }

        return dims_values;
    }

    fn extractDensity(pattern_str: String) -> f64 {
        return pattern_str
            .split(" ")
            .last()
            .map(|density| density.parse::<f64>().unwrap());
    }
}
