pub fn group_option_vector<T>(vector: Vec<Option<T>>) -> Vec<Vec<T>> {
    let mut result: Vec<Vec<T>> = Vec::new();

    let mut current: Vec<T> = Vec::new();
    for element in vector {
        if let Some(content) = element {
            current.push(content);
        } else {
            result.push(current);
            current = Vec::new();
        }
    }

    return result;
} 
