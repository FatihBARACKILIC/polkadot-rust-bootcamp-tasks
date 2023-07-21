struct FilterCondition<T> {
    condition: T,
}

// koşulumuzu burada kontrol ediyoruz
// <T: PartialEq> Generic değişkenlerin PartialEq Trait'inden türemiş olmasını
//  zorunlu tutuyor bu sayede eşitliği kontrol ediyoruz. Bu olmasaydı hata verirdi.
impl<T: PartialEq> FilterCondition<T> {
    fn is_match(&self, item: &T) -> bool {
        item == &self.condition
    }
}

fn custom_filter<T>(collection: &[T], filter: &FilterCondition<T>) -> Vec<T>
where
    T: PartialEq + Copy,
    // Partial ile eşitliği kontrol edebiliyoruz.
    // Copy ile onu kendi değişkenimize kopyalayabiliyoruz.
{
    let mut result = Vec::new();
    for item in collection.iter() {
        if filter.is_match(item) {
            result.push(*item);
        }
    }
    result
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 52, 5, 6, 7, 12, 2, 13];
    let condition = FilterCondition { condition: 2 };
    let result = custom_filter(&numbers, &condition);
    println!("Result: {:?}", result);
    println!("Numbers: {:?}", numbers);
}
