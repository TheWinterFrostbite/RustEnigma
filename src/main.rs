static ALPHABET: [char; 26] = 
[
    'a','b','c','d','e','f','g','h','i','j','k','l','m',
    'n','o','p','q','r','s','t','u','v','w','x','y','z',
];
fn main()
{
    let mut rotors: [i32; 3] = [0,0,0];
    //rotors = move_rotors_continuous(rotors);
    rotors = move_rotors(0,1,2,rotors);
    let mut permutable_alphabet = ALPHABET;

    struct plugboard
    {
        x: usize,
        y: usize,
    }

/*
    let permutator_1 = plugboard{x: 5, y: 13};
    let permutator_2 = plugboard{x: 23, y: 17};
    let permutator_3 = plugboard{x: 0, y: 2};
    let permutator_4 = plugboard{x: 5, y: 10};
    let permutator_5 = plugboard{x: 20, y: 19};

    permutable_alphabet[permutator_1.x] = ALPHABET[permutator_1.y];
    permutable_alphabet[permutator_1.y] = ALPHABET[permutator_1.x];
    permutable_alphabet[permutator_2.x] = ALPHABET[permutator_2.y];
    permutable_alphabet[permutator_2.y] = ALPHABET[permutator_2.x];
    permutable_alphabet[permutator_3.x] = ALPHABET[permutator_3.y];
    permutable_alphabet[permutator_3.y] = ALPHABET[permutator_3.x];
    permutable_alphabet[permutator_4.x] = ALPHABET[permutator_4.y];
    permutable_alphabet[permutator_4.y] = ALPHABET[permutator_4.x];
    permutable_alphabet[permutator_5.x] = ALPHABET[permutator_5.y];
    permutable_alphabet[permutator_5.y] = ALPHABET[permutator_5.x];
*/

    println!("{:?} {}", permutable_alphabet, permutable_alphabet.len());
    println!("{:?} {}", ALPHABET, ALPHABET.len());

    rotors_state(rotors, permutable_alphabet);

}
fn rotors_state(rotors: [i32; 3], characters: [char; 26])
{
    println!
    (
        "{} {} {}",
        rotors[0],
        rotors[1],
        rotors[2],
    );

    let rotor_a: usize = rotors[0] as usize;
    let rotor_b: usize = rotors[1] as usize;
    let rotor_c: usize = rotors[2] as usize; 

    println!
    (
        "{} {} {}",
        characters[rotor_a],
        characters[rotor_b],
        characters[rotor_c],
    );
}
fn move_rotors_continuous(mut rotors: [i32; 3]) -> [i32; 3]
{
    let max_value:i32 = 26;
    let reset_value:i32 = 26;

    while rotors[0] < max_value
    {
        rotors[0] += 1;
        if rotors[0] == max_value
        {
            rotors[0] -= reset_value;
            rotors[1] += 1;
            if rotors[1] == max_value
            {
                rotors[1] -= reset_value;
                rotors[2] += 1;
                if rotors[2] == max_value
                {
                    rotors[2] -= reset_value;
                }
            }
        }
    }
    rotors
}
fn move_rotors(rotor_a: i32, rotor_b: i32, rotor_c: i32, mut rotors: [i32; 3]) -> [i32; 3]
{
    let max_value:i32 = 25;
    let reset_value:i32 = 24;

    rotors[0] += rotor_a;
    rotors[1] += rotor_b;
    rotors[2] += rotor_c;

    if rotors[0] == max_value
    {
        rotors[0] -= reset_value
    }
    if rotors[1] == max_value
    {
        rotors[1] -= reset_value
    }
    if rotors[2] == max_value
    {
        rotors[2] -= reset_value
    } 
    rotors
}
