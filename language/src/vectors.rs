pub fn vectors() {
    // Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory. Vectors can only store values of the same type
    let v: Vec<i32> = Vec::new(); // create vector

    let v = vec![1, 2, 3]; // call vec macro to create a vector and hold the values. Vector type is inferred from the given values.

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("{:?}", v); //print vector

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2); // get method gives us Option<T> value; return None if values overflows or underflows
    let fourth: Option<&i32> = v.get(3);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let v = vec![1, 2, 3, 4, 5];

    // let does_not_exist = &v[100]; // returns reference; causes error: index out of bounds
    let does_not_exist = v.get(100); // returns Option<T>
    
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    println!("{:?}",  v);

}