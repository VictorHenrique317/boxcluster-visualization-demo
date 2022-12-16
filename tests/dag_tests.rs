#![allow(non_snake_case)]
mod dag_tests {
    use boxcluster_visualization::dag_creator::*;
    use boxcluster_visualization::pattern::*;
    use boxcluster_visualization::*;
    use itertools::Itertools;
    use rand::seq::SliceRandom;
    use std::collections::HashMap;
    use std::fs;
    use rand::thread_rng;

    fn sortHashMap(hashmap: &HashMap<u32, Vec<u32>>) -> HashMap<u32, Vec<u32>> {
        let mut sorted_hashmap: HashMap<u32, Vec<u32>> = HashMap::new();

        for key in hashmap.keys().sorted() {
            let sorted_value: Vec<u32> = hashmap
                .get(key)
                .unwrap()
                .into_iter()
                .sorted()
                .map(|i| i.clone())
                .collect();
            sorted_hashmap.insert(*key, sorted_value);
        }

        return sorted_hashmap;
    }

    #[test]
    fn testSimpleOverlap() {
        let path = "tests/test_data/simple-overlap.txt".to_owned();
        let mut patterns = getPatterns(path);
        patterns.shuffle(&mut thread_rng());

        let mut dag_creator = DagCreator::new(patterns);
        dag_creator.create();

        let mut expected_subs: HashMap<u32, Vec<u32>> = HashMap::new();
        expected_subs.insert(1, vec![2]);
        expected_subs.insert(2, vec![]);

        let mut expected_supers: HashMap<u32, Vec<u32>> = HashMap::new();
        expected_supers.insert(1, vec![]);
        expected_supers.insert(2, vec![1]);

        let r_subs = sortHashMap(&dag_creator.dag.getFlattenedSubs());
        let r_supers = sortHashMap(&dag_creator.dag.getFlattenedSupers());

        let e_subs = sortHashMap(&expected_subs);
        let e_supers = sortHashMap(&expected_supers);

        assert_eq!(r_subs, e_subs);
        assert_eq!(r_supers, e_supers);
    }

    #[test]
    fn testSimpleOverlap2() {
        let path = "tests/test_data/simple-overlap-2.txt".to_owned();
        let mut patterns = getPatterns(path);
        patterns.shuffle(&mut thread_rng());

        let mut dag_creator = DagCreator::new(patterns);
        dag_creator.create();

        let mut expected_subs: HashMap<u32, Vec<u32>> = HashMap::new();
        expected_subs.insert(1, vec![2]);
        expected_subs.insert(2, vec![]);
        expected_subs.insert(3, vec![]);

        let mut expected_supers: HashMap<u32, Vec<u32>> = HashMap::new();
        expected_supers.insert(1, vec![]);
        expected_supers.insert(2, vec![1]);
        expected_supers.insert(3, vec![]);

        let r_subs = sortHashMap(&dag_creator.dag.getFlattenedSubs());
        let r_supers = sortHashMap(&dag_creator.dag.getFlattenedSupers());

        let e_subs = sortHashMap(&expected_subs);
        let e_supers = sortHashMap(&expected_supers);

        assert_eq!(r_subs, e_subs);
        assert_eq!(r_supers, e_supers);
    }

    #[test]
    fn testDoubleDiffOverlap() {
        let path = "tests/test_data/double-diff-overlap.txt".to_owned();
        let mut patterns = getPatterns(path);
        patterns.shuffle(&mut thread_rng());

        let mut dag_creator = DagCreator::new(patterns);
        dag_creator.create();

        let mut expected_subs: HashMap<u32, Vec<u32>> = HashMap::new();
        expected_subs.insert(1, vec![2, 3]);
        expected_subs.insert(2, vec![]);
        expected_subs.insert(3, vec![]);

        let mut expected_supers: HashMap<u32, Vec<u32>> = HashMap::new();
        expected_supers.insert(1, vec![]);
        expected_supers.insert(2, vec![1]);
        expected_supers.insert(3, vec![1]);

        let r_subs = sortHashMap(&dag_creator.dag.getFlattenedSubs());
        let r_supers = sortHashMap(&dag_creator.dag.getFlattenedSupers());

        let e_subs = sortHashMap(&expected_subs);
        let e_supers = sortHashMap(&expected_supers);

        assert_eq!(r_subs, e_subs);
        assert_eq!(r_supers, e_supers);
    }

    #[test]
    fn testTripleDiffOverlap() {
        let path = "tests/test_data/triple-diff-overlap.txt".to_owned();
        let mut patterns = getPatterns(path);
        patterns.shuffle(&mut thread_rng());

        let mut dag_creator = DagCreator::new(patterns);
        dag_creator.create();

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

        let r_subs = sortHashMap(&dag_creator.dag.getFlattenedSubs());
        let r_supers = sortHashMap(&dag_creator.dag.getFlattenedSupers());

        let e_subs = sortHashMap(&expected_subs);
        let e_supers = sortHashMap(&expected_supers);

        assert_eq!(r_subs, e_subs);
        assert_eq!(r_supers, e_supers);
    }

    #[test]
    fn testQuadrupleDiffOverlap() {
        let path = "tests/test_data/quadruple-diff-overlap.txt".to_owned();
        let mut patterns = getPatterns(path);
        patterns.shuffle(&mut thread_rng());

        let mut dag_creator = DagCreator::new(patterns);
        dag_creator.create();

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

        let r_subs = sortHashMap(&dag_creator.dag.getFlattenedSubs());
        let r_supers = sortHashMap(&dag_creator.dag.getFlattenedSupers());

        let e_subs = sortHashMap(&expected_subs);
        let e_supers = sortHashMap(&expected_supers);

        assert_eq!(r_subs, e_subs);
        assert_eq!(r_supers, e_supers);
    }

    #[test]
    fn testSimpleMSub() {
        let path = "tests/test_data/simple-msub.txt".to_owned();
        let mut patterns = getPatterns(path);
        patterns.shuffle(&mut thread_rng());

        let mut dag_creator = DagCreator::new(patterns);
        dag_creator.create();

        let mut expected_subs: HashMap<u32, Vec<u32>> = HashMap::new();
        expected_subs.insert(1, vec![2]);
        expected_subs.insert(2, vec![3]);
        expected_subs.insert(3, vec![]);

        let mut expected_supers: HashMap<u32, Vec<u32>> = HashMap::new();
        expected_supers.insert(1, vec![]);
        expected_supers.insert(2, vec![1]);
        expected_supers.insert(3, vec![2]);

        let r_subs = sortHashMap(&dag_creator.dag.getFlattenedSubs());
        let r_supers = sortHashMap(&dag_creator.dag.getFlattenedSupers());

        let e_subs = sortHashMap(&expected_subs);
        let e_supers = sortHashMap(&expected_supers);

        assert_eq!(r_subs, e_subs);
        assert_eq!(r_supers, e_supers);
    }

    #[test]
    fn testSimpleMSub2() {
        let path = "tests/test_data/simple-msub-2.txt".to_owned();
        let mut patterns = getPatterns(path);
        patterns.shuffle(&mut thread_rng());

        let mut dag_creator = DagCreator::new(patterns);
        dag_creator.create();

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

        let r_subs = sortHashMap(&dag_creator.dag.getFlattenedSubs());
        let r_supers = sortHashMap(&dag_creator.dag.getFlattenedSupers());

        let e_subs = sortHashMap(&expected_subs);
        let e_supers = sortHashMap(&expected_supers);

        assert_eq!(r_subs, e_subs);
        assert_eq!(r_supers, e_supers);
    }

    #[test]
    fn testComplexMSub() {
        let path = "tests/test_data/complex-msub.txt".to_owned();
        let mut patterns = getPatterns(path);
        patterns.shuffle(&mut thread_rng());

        let mut dag_creator = DagCreator::new(patterns);
        dag_creator.create();

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

        let r_subs = sortHashMap(&dag_creator.dag.getFlattenedSubs());
        let r_supers = sortHashMap(&dag_creator.dag.getFlattenedSupers());

        let e_subs = sortHashMap(&expected_subs);
        let e_supers = sortHashMap(&expected_supers);

        assert_eq!(r_subs, e_subs);
        assert_eq!(r_supers, e_supers);
    }

    #[test]
    fn testSimpleMSuper() {
        let path = "tests/test_data/simple-msuper.txt".to_owned();
        let mut patterns = getPatterns(path);
        patterns.shuffle(&mut thread_rng());

        let mut dag_creator = DagCreator::new(patterns);
        dag_creator.create();

        let mut expected_subs: HashMap<u32, Vec<u32>> = HashMap::new();
        expected_subs.insert(1, vec![2,4]);
        expected_subs.insert(2, vec![3,5]);
        expected_subs.insert(3, vec![]);
        expected_subs.insert(4, vec![5]);
        expected_subs.insert(5, vec![]);

        let mut expected_supers: HashMap<u32, Vec<u32>> = HashMap::new();
        expected_supers.insert(1, vec![]);
        expected_supers.insert(2, vec![1]);
        expected_supers.insert(3, vec![2]);
        expected_supers.insert(4, vec![1]);
        expected_supers.insert(5, vec![2, 4]);

        let r_subs = sortHashMap(&dag_creator.dag.getFlattenedSubs());
        let r_supers = sortHashMap(&dag_creator.dag.getFlattenedSupers());

        let e_subs = sortHashMap(&expected_subs);
        let e_supers = sortHashMap(&expected_supers);

        assert_eq!(r_subs, e_subs);
        assert_eq!(r_supers, e_supers);
    }

    #[test]
    fn testReal1() {
        let path = "tests/test_data/real1.txt".to_owned();
        let mut patterns = getPatterns(path);
        // patterns.shuffle(&mut thread_rng());

        let mut dag_creator = DagCreator::new(patterns);
        dag_creator.create();

        let mut expected_subs: HashMap<u32, Vec<u32>> = HashMap::new();
        expected_subs.insert(1, vec![]);
        expected_subs.insert(2, vec![3]);
        expected_subs.insert(3, vec![4]);
        expected_subs.insert(4, vec![1]);

        let mut expected_supers: HashMap<u32, Vec<u32>> = HashMap::new();
        expected_supers.insert(1, vec![4]);
        expected_supers.insert(2, vec![]);
        expected_supers.insert(3, vec![2]);
        expected_supers.insert(4, vec![3]);

        let r_subs = sortHashMap(&dag_creator.dag.getFlattenedSubs());
        let r_supers = sortHashMap(&dag_creator.dag.getFlattenedSupers());

        let e_subs = sortHashMap(&expected_subs);
        let e_supers = sortHashMap(&expected_supers);

        assert_eq!(r_subs, e_subs);
        assert_eq!(r_supers, e_supers);
    }

    // #[test]
    // fn exaustive_testing(){
    //     for i in 0..100{
    //         testSimpleOverlap();
    //         testSimpleOverlap2();
    //         testDoubleDiffOverlap();
    //         testTripleDiffOverlap();
    //         testQuadrupleDiffOverlap();
    //         testSimpleMSub();
    //         testSimpleMSub2();
    //         testComplexMSub();
    //         testSimpleMSuper();
    //         testReal1();
    //     }
    // }

}
