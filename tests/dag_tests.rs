#![allow(non_snake_case)]
mod dag_tests{
    use std::collections::HashMap;
    use std::fs;
    use boxcluster_visualization::*;
    use boxcluster_visualization::dag_creator::*;
    use boxcluster_visualization::pattern::*;

    #[test]
    fn testSimpleOverlap(){
        let path = "tests/test_data/simple-overlap.txt".to_owned();
        let patterns = getPatterns(path);

        let mut dag_creator = DagCreator::new();
        dag_creator.calculate(patterns, None);
        
        let mut expected_subs: HashMap<u32, Vec<u32>> = HashMap::new();
        expected_subs.insert(1, vec![2]);
        expected_subs.insert(2, vec![]);

        let mut expected_supers: HashMap<u32, Vec<u32>> = HashMap::new();
        expected_supers.insert(1, vec![]);
        expected_supers.insert(2, vec![1]);
        assert_eq!(dag_creator.pattern_subs, expected_subs);
        assert_eq!(dag_creator.pattern_supers, expected_supers);
    }

    #[test]
    fn testSimpleOverlap2(){
        let path = "tests/test_data/simple-overlap-2.txt".to_owned();
        let patterns = getPatterns(path);

        let mut dag_creator = DagCreator::new();
        dag_creator.calculate(patterns, None);
        
        let mut expected_subs: HashMap<u32, Vec<u32>> = HashMap::new();
        expected_subs.insert(1, vec![2]);
        expected_subs.insert(2, vec![]);
        expected_subs.insert(3, vec![]);

        let mut expected_supers: HashMap<u32, Vec<u32>> = HashMap::new();
        expected_supers.insert(1, vec![]);
        expected_supers.insert(2, vec![1]);
        expected_supers.insert(3, vec![]);

        assert_eq!(dag_creator.pattern_subs, expected_subs);
        assert_eq!(dag_creator.pattern_supers, expected_supers);
    }

    #[test]
    fn testDoubleDiffOverlap(){
        let path = "tests/test_data/double-diff-overlap.txt".to_owned();
        let patterns = getPatterns(path);

        let mut dag_creator = DagCreator::new();
        dag_creator.calculate(patterns, None);
        
        let mut expected_subs: HashMap<u32, Vec<u32>> = HashMap::new();
        expected_subs.insert(1, vec![2, 3]);
        expected_subs.insert(2, vec![]);
        expected_subs.insert(3, vec![]);

        let mut expected_supers: HashMap<u32, Vec<u32>> = HashMap::new();
        expected_supers.insert(1, vec![]);
        expected_supers.insert(2, vec![1]);
        expected_supers.insert(3, vec![1]);
        
        assert_eq!(dag_creator.pattern_subs, expected_subs);
        assert_eq!(dag_creator.pattern_supers, expected_supers);
    }

    #[test]
    fn testTripleDiffOverlap(){
        let path = "tests/test_data/triple-diff-overlap.txt".to_owned();
        let patterns = getPatterns(path);

        let mut dag_creator = DagCreator::new();
        dag_creator.calculate(patterns, None);
        
        let mut expected_subs: HashMap<u32, Vec<u32>> = HashMap::new();
        expected_subs.insert(1, vec![2, 3, 4]);
        expected_subs.insert(2, vec![]);
        expected_subs.insert(3, vec![]);
        expected_subs.insert(4, vec![]);

        let mut expected_supers: HashMap<u32, Vec<u32>> = HashMap::new();
        expected_supers.insert(1, vec![]);
        expected_supers.insert(2, vec![1]);
        expected_supers.insert(3, vec![1]);
        expected_supers.insert(4, vec![1]);
        
        assert_eq!(dag_creator.pattern_subs, expected_subs);
        assert_eq!(dag_creator.pattern_supers, expected_supers);
    }

    #[test]
    fn testQuadrupleDiffOverlap(){
        let path = "tests/test_data/quadruple-diff-overlap.txt".to_owned();
        let patterns = getPatterns(path);

        let mut dag_creator = DagCreator::new();
        dag_creator.calculate(patterns, None);
        
        let mut expected_subs: HashMap<u32, Vec<u32>> = HashMap::new();
        expected_subs.insert(1, vec![2, 3, 4, 5]);
        expected_subs.insert(2, vec![]);
        expected_subs.insert(3, vec![]);
        expected_subs.insert(4, vec![]);
        expected_subs.insert(5, vec![]);

        let mut expected_supers: HashMap<u32, Vec<u32>> = HashMap::new();
        expected_supers.insert(1, vec![]);
        expected_supers.insert(2, vec![1]);
        expected_supers.insert(3, vec![1]);
        expected_supers.insert(4, vec![1]);
        expected_supers.insert(5, vec![1]);
        
        assert_eq!(dag_creator.pattern_subs, expected_subs);
        assert_eq!(dag_creator.pattern_supers, expected_supers);
    }

    #[test]
    fn testSimpleMSub(){
        let path = "tests/test_data/simple-msub.txt".to_owned();
        let patterns = getPatterns(path);

        let mut dag_creator = DagCreator::new();
        dag_creator.calculate(patterns, None);
        
        let mut expected_subs: HashMap<u32, Vec<u32>> = HashMap::new();
        expected_subs.insert(1, vec![2]);
        expected_subs.insert(2, vec![3]);
        expected_subs.insert(3, vec![]);

        let mut expected_supers: HashMap<u32, Vec<u32>> = HashMap::new();
        expected_supers.insert(1, vec![]);
        expected_supers.insert(2, vec![1]);
        expected_supers.insert(3, vec![2]);
        
        assert_eq!(dag_creator.pattern_subs, expected_subs);
        assert_eq!(dag_creator.pattern_supers, expected_supers);
    }

    #[test]
    fn testSimpleMSub2(){
        let path = "tests/test_data/simple-msub-2.txt".to_owned();
        let patterns = getPatterns(path);

        let mut dag_creator = DagCreator::new();
        dag_creator.calculate(patterns, None);
        
        let mut expected_subs: HashMap<u32, Vec<u32>> = HashMap::new();
        expected_subs.insert(1, vec![2, 3]);
        expected_subs.insert(2, vec![]);
        expected_subs.insert(3, vec![4]);
        expected_subs.insert(4, vec![]);

        let mut expected_supers: HashMap<u32, Vec<u32>> = HashMap::new();
        expected_supers.insert(1, vec![]);
        expected_supers.insert(2, vec![1]);
        expected_supers.insert(3, vec![1]);
        expected_supers.insert(4, vec![3]);
        
        assert_eq!(dag_creator.pattern_subs, expected_subs);
        assert_eq!(dag_creator.pattern_supers, expected_supers);
    }

    #[test]
    fn testComplexMSub(){
        let path = "tests/test_data/complex-msub.txt".to_owned();
        let patterns = getPatterns(path);

        let mut dag_creator = DagCreator::new();
        dag_creator.calculate(patterns, None);
        
        let mut expected_subs: HashMap<u32, Vec<u32>> = HashMap::new();
        expected_subs.insert(1, vec![2, 5, 6]);
        expected_subs.insert(2, vec![3, 4]);
        expected_subs.insert(3, vec![]);
        expected_subs.insert(4, vec![]);
        expected_subs.insert(5, vec![]);
        expected_subs.insert(6, vec![7]);
        expected_subs.insert(7, vec![]);

        let mut expected_supers: HashMap<u32, Vec<u32>> = HashMap::new();
        expected_supers.insert(1, vec![]);
        expected_supers.insert(2, vec![1]);
        expected_supers.insert(3, vec![2]);
        expected_supers.insert(4, vec![2]);
        expected_supers.insert(5, vec![1]);
        expected_supers.insert(6, vec![1]);
        expected_supers.insert(7, vec![6]);
        
        assert_eq!(dag_creator.pattern_subs, expected_subs);
        assert_eq!(dag_creator.pattern_supers, expected_supers);
    }

}