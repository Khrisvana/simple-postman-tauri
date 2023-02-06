#[derive(Debug, Clone)]
struct Something {
    id: i32,
    name: String,
    parent_id: Option<i32>,
}

#[derive(Debug, Clone)]
struct MappedSomething {
    smt: Something,
    child: Option<Vec<MappedSomething>>,
}

fn main() {
    let mut something: Vec<Something> = vec![Something {
        id: 1,
        name: "parent".to_string(),
        parent_id: None,
    },
    Something {
        id: 2,
        name: "parent 2".to_string(),
        parent_id: None,
    }];

    let something2: Vec<Something> = vec![Something {
        id: 2,
        name: "child".to_string(),
        parent_id: Some(1),
    },
    Something {
        id: 3,
        name: "child 2".to_string(),
        parent_id: Some(1),
    }];

    let mut mapped_something: Vec<MappedSomething> = vec![];
    for item in something.into_iter() {
        mapped_something.push(recurse(mapped_something.clone(), &item, &something2));
    }
    
    println!("{:?}", mapped_something);
    // let mapped_something: Vec<MappedSomething> = something.into_iter();

    // println!("{:?}", zipped);
}

fn recurse(mut replacement: Vec<MappedSomething>, parent: &Something, child: &Vec<Something>) -> MappedSomething {
    
    let res: _ = child
        .clone()
        .into_iter()
        .filter(|x| &parent.id == &x.parent_id.unwrap_or(0))
        .collect::<Vec<Something>>();
        
    let mut parent_temp: MappedSomething = MappedSomething {
        smt: parent.clone(),
        child: None
    };
      
    let mut child_list: Vec<MappedSomething> = vec![];
    
    for each_result in res.into_iter() {
        let something3: Vec<Something> = vec![Something {
            id: 4,
            name: "child".to_string(),
            parent_id: Some(3),
        }];
        
        let current_res: MappedSomething = MappedSomething {
            smt: each_result.clone(),
            child: None
        } ;
        
        child_list.push(recurse(child_list.clone(), &each_result, &something3));
    } 
    parent_temp.child = Some(child_list);
    
    parent_temp 
} 