#[cfg(test)]
mod tests {
    extern crate algorithms;

    use algorithms::sort::{combosort, heapsort, quicksort, stoogesort, cocktailsort};

    #[test]
    fn it_heap_sort() {
        let mut numbers = [4, 65, 2, -31];
        heapsort::heap_sort(&mut numbers);
        assert_eq!(numbers, [-31, 2, 4, 65]);
    }

    #[test]
    fn it_stooge_sort() {
        let mut numbers = [4, 65, 2, -31];
        stoogesort::stooge_sort(&mut numbers);
        assert_eq!(numbers, [-31, 2, 4, 65]);
    }

    #[test]
    fn it_quick_sort() {
        let mut numbers = [4, 65, 2, -31];
        quicksort::quick_sort(&mut numbers);
        assert_eq!(numbers, [-31, 2, 4, 65]);
    }

    #[test]
    fn it_comb_sort() {
        let mut numbers = [4, 65, 2, -31];
        combosort::comb_sort(&mut numbers);
        assert_eq!(numbers, [-31, 2, 4, 65]);
    }

    #[test]
    fn it_cocktail_sort() {
        let mut numbers = [4, 65, 2, -31];
        cocktailsort::cocktail_sort(&mut numbers);
        assert_eq!(numbers, [-31, 2, 4, 65]);
    }
}
