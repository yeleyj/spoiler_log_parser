use super::*;

// main. Cannot test because cannot fake the commandline args
#[test]
#[should_panic]
fn test_main_panics_because_no_args() {
    main();
}

// read_parse_output
#[test]
#[should_panic]
fn test_read_parse_output_panics_because_invalid_filename() {
    read_parse_output("");
}

#[test]
#[should_panic]
fn test_read_parse_output_panics_because_invalid_file_content() {
    read_parse_output("./test/invalid.json");
}

#[test]
fn test_read_parse_output_works_with_empty_json() {
    read_parse_output("./test/empty.json");
}

#[test]
fn test_read_parse_output_works_with_valid_file_and_locations() {
    read_parse_output("./test/v31_simple.json");
}

// string_to_json
#[test]
#[should_panic]
fn test_string_to_json_panics_because_invalid_json() {
    let _result = string_to_json("Ceci n'est pas du JSON").expect("Kaboom!");
}

#[test]
fn test_string_to_json_works_with_valid_json() {
    let _result = string_to_json("{}").expect("Kaboom!");
}

// insert_crystal_if_exists
#[test]
fn test_insert_crystal_if_exists_does_nothing_when_no_crystal_data() {
    let mut crystals: [String; 7] = [ String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new() ];

    insert_crystal_if_exists("Test", "notACrystal:1", &mut crystals);
    for i in 0..6 {
        assert!( 0 >= crystals[i].len() );
    }
}

#[test]
fn test_insert_crystal_if_exists_works_when_crystal_data() {
    let mut crystals: [String; 7] = [ String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new() ];

    insert_crystal_if_exists("Test", "Crystal7:1", &mut crystals);
    for i in 0..5 {
        assert!( 0 >= crystals[i].len() );
    }

    assert!( 0 < crystals[6].len() );
}

#[test]
fn test_insert_crystal_if_exists_does_nothing_with_bad_input() {
    let mut crystals: [String; 7] = [ String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new() ];

    insert_crystal_if_exists("Test", "asdf", &mut crystals);
    for i in 0..6 {
        assert!( 0 >= crystals[i].len() );
    }
}

#[test]
fn test_insert_crystal_if_exists_does_nothing_with_unhandled_crystal_low() {
    let mut crystals: [String; 7] = [ String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new() ];

    insert_crystal_if_exists("Test", "Crystal0:1", &mut crystals);
    for i in 0..6 {
        assert!( 0 >= crystals[i].len() );
    }
}

#[test]
fn test_insert_crystal_if_exists_does_nothing_with_unhandled_crystal_high() {
    let mut crystals: [String; 7] = [ String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new() ];

    insert_crystal_if_exists("Test", "Crystal8:1", &mut crystals);
    for i in 0..6 {
        assert!( 0 >= crystals[i].len() );
    }
}

// insert_pendant_if_exists
#[test]
fn test_insert_pendant_if_exists_does_nothing_when_no_pendant_data() {
    let mut pendants: [String; 3] = [ String::new(), String::new(), String::new() ];

    insert_pendant_if_exists("Test", "notAPendant:1", &mut pendants);
    for i in 0..3 {
        assert!( 0 >= pendants[i].len() );
    }
}

#[test]
fn test_insert_pendant_if_exists_works_when_pendant_data_courage() {
    let mut pendants: [String; 3] = [ String::new(), String::new(), String::new() ];

    insert_pendant_if_exists("Test", "PendantOfCourage:1", &mut pendants);
    assert!( 0 >= pendants[1].len() );
    assert!( 0 >= pendants[2].len() );
    assert!( 0 < pendants[0].len() );
}

#[test]
fn test_insert_pendant_if_exists_works_when_pendant_data_wisdom() {
    let mut pendants: [String; 3] = [ String::new(), String::new(), String::new() ];

    insert_pendant_if_exists("Test", "PendantOfWisdom:1", &mut pendants);
    assert!( 0 >= pendants[2].len() );
    assert!( 0 >= pendants[0].len() );
    assert!( 0 < pendants[1].len() );
}

#[test]
fn test_insert_pendant_if_exists_works_when_pendant_data_power() {
    let mut pendants: [String; 3] = [ String::new(), String::new(), String::new() ];

    insert_pendant_if_exists("Test", "PendantOfPower:1", &mut pendants);
    assert!( 0 >= pendants[1].len() );
    assert!( 0 >= pendants[0].len() );
    assert!( 0 < pendants[2].len() );
}

#[test]
fn test_insert_pendant_if_exists_does_nothing_with_bad_input() {
    let mut pendants: [String; 3] = [ String::new(), String::new(), String::new() ];

    insert_pendant_if_exists("Test", "asdf", &mut pendants);
    for i in 0..3 {
        assert!( 0 >= pendants[i].len() );
    }
}

// parse_json
// TODO: more tests here for each part, maybe? Compare output of parser, compare output of JSON to expected as well?
#[test]
fn test_parse_json_works_with_valid_json() {
    let contents = fs::read_to_string("./test/v31_simple.json").expect("Could not read file!");

    let json = string_to_json(&contents).unwrap_or_else(|error| {
        panic!("Problem parsing the file: {:?}", error);
    });

    let _result = parse_json(&json).expect("No parse for you!");
}

#[test]
#[should_panic]
fn test_parse_json_fails_with_invalid_json() {
    let contents = fs::read_to_string("./test/invalid.json").expect("Could not read file!");

    let json = string_to_json(&contents).unwrap_or_else(|error| {
        panic!("Problem parsing the file: {:?}", error);
    });

    let _parse_result = parse_json(&json).unwrap_or_else(|error| {
        panic!("Could not [parse!]: {:?}", error);
    });
}

#[test]
fn test_parse_json_works_with_empty_json() {
    let contents = fs::read_to_string("./test/empty.json").expect("Could not read file!");

    let json = string_to_json(&contents).unwrap_or_else(|error| {
        panic!("Problem parsing the file: {:?}", error);
    });

    let _parse_result = parse_json(&json).unwrap_or_else(|error| {
        panic!("Could not [parse!]: {:?}", error);
    });
}


// unbox_json_str_or_return_empty_string

#[test]
fn test_unbox_json_str_or_return_empty_string_will_return_empty_string_on_null() {
    assert_eq!( String::from(""), unbox_json_str_and_remove_colon_one_or_return_empty_string(&serde_json::value::Value::Null) );
}

#[test]
fn test_unbox_json_str_or_return_empty_string_will_return_string_on_string() {
    let test_str = "potato";
    let test_val: serde_json::value::Value = serde_json::value::Value::String( String::from(test_str) );
    assert_eq!( String::from(test_str), unbox_json_str_and_remove_colon_one_or_return_empty_string(&test_val) );
}

#[test]
fn test_unbox_json_str_or_return_empty_string_will_return_repalced_string_on_string_when_colon_one() {
    let test_str = "potato:1";
    let expected = "potato";
    let test_val: serde_json::value::Value = serde_json::value::Value::String( String::from(test_str) );
    assert_eq!( String::from(expected), unbox_json_str_and_remove_colon_one_or_return_empty_string(&test_val) );
}

// unbox_json_str_or_return_empty_str

#[test]
fn test_unbox_json_str_or_return_empty_str_will_return_empty_string_on_null() {
    assert_eq!( "", unbox_json_str_or_return_empty_str(&serde_json::value::Value::Null) );
}

#[test]
fn test_unbox_json_str_or_return_empty_str_will_return_string_on_string() {
    let test_str = "potato";
    let test_val: serde_json::value::Value = serde_json::value::Value::String( String::from(test_str) );
    assert_eq!( test_str, unbox_json_str_or_return_empty_str(&test_val) );
}

// replace_final_colon_one_and_return_string

#[test]
fn test_replace_final_colon_one_and_return_string_will_return_same_when_no_colon_one() {
    let test_str = "potato";
    assert_eq!( test_str, replace_final_colon_one_and_return_string(&test_str) );
}

#[test]
fn test_replace_final_colon_one_and_return_string_will_return_removed_when_colon_one() {
    let test_str = "potato:1";
    let expected = "potato";
    assert_eq!( expected, replace_final_colon_one_and_return_string(&test_str) );
}