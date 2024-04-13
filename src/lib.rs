#![allow(non_snake_case)]
use std::collections::HashMap;

fn anagramMapCreator(word: &String) -> HashMap<char, u32> {
    let mut anagramMap = HashMap::new();
    word.chars().for_each(|character| {
        let c = anagramMap.get(&character);
        let count = match c {
            Some(&count) => count + 1,
            None => 1,
        };
        anagramMap.insert(character, count);
    });
    anagramMap
}

pub fn anagramGrouper(list: &Vec<String>) -> HashMap<i32, Vec<&String>> {
    let mut indexInUniqueWords = 0;
    let mut uniqueWords: HashMap<i32, Vec<&String>> = HashMap::new();
    let mut hashmapCollection: Vec<HashMap<char, u32>>  = vec![];
    for word in list {
        let output = anagramMapCreator(word);
        let present = hashmapCollection.iter().position(|hash| output.eq(hash));
        match present {
            Some(anagramIndex) => {
                let entry = uniqueWords.get(&(anagramIndex.try_into().unwrap()));
                let new_entry: Vec<&String> = match entry {
                    Some(ent) => {
                        let mut newEnt = ent.clone();
                        newEnt.push(word);
                        newEnt
                    },
                    None => vec![word]
                };
                uniqueWords.insert(anagramIndex.try_into().unwrap(), new_entry);

                
            },
            None => {
                hashmapCollection.push(output);
                indexInUniqueWords += 1;
                uniqueWords.insert(indexInUniqueWords-1, vec![word]);
            }
        };
        // println!("{:?}", output);
    }
    uniqueWords
}
