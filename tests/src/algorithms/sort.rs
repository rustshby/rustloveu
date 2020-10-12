#[cfg(test)]
mod tests {
    extern crate algorithms;

    #[test]
    fn it_heap_sort() {
        let mut numbers = [4, 65, 2, -31];
        algorithms::sort::heapsort::heap_sort(&mut numbers);
        assert_eq!(numbers, [-31, 2, 4, 65]);
    }

    #[test]
    fn it_stooge_sort() {
        let mut numbers = [4, 65, 2, -31];
        algorithms::sort::stoogesort::stooge_sort(&mut numbers);
        assert_eq!(numbers, [-31, 2, 4, 65]);
    }

    #[test]
    fn it_quick_sort() {
        let mut numbers = [4, 65, 2, -31];
        algorithms::sort::quicksort::quick_sort(&mut numbers);
        assert_eq!(numbers, [-31, 2, 4, 65]);
    }
}
