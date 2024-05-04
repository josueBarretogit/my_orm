/// The input will have the form: atribbute("atribbute to extract")
pub fn extract_string_atribute(input : String) -> String {

    let index_c1 = input.find('(').unwrap();
    let index_c2 = input.find(')').unwrap();

    input[(index_c1 + 1)..index_c2].replace("\"", "")
}
