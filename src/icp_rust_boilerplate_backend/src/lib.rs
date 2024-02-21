#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Fragrance {
    id: u64,
    name: String,
    description: String,
    mood_enhancing_properties: Vec<String>,
    created_at: u64,
    updated_at: Option<u64>,
}

impl Storable for Fragrance {
    // Serialization and deserialization methods for Fragrance
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Fragrance {
    // Constants related to storage size for Fragrance
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Thread-local storage for MemoryManager, ID counter, and Fragrance storage
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static FRAGRANCE_STORAGE: RefCell<StableBTreeMap<u64, Fragrance, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct FragrancePayload {
    name: String,
    description: String,
    mood_enhancing_properties: Vec<String>,
}

#[ic_cdk::query]
fn get_fragrance(id: u64) -> Result<Fragrance, Error> {
    // Retrieve a fragrance by ID from storage
    match _get_fragrance(&id) {
        Some(fragrance) => Ok(fragrance),
        None => Err(Error::NotFound {
            msg: format!("a fragrance with id={} not found", id),
        }),
    }
}

#[ic_cdk::update]
fn add_fragrance(fragrance: FragrancePayload) -> Option<Fragrance> {
    // Validate input
    if fragrance.name.is_empty() || fragrance.description.is_empty() {
        return Err(Error::InvalidInput {
            msg: "Empty fields are not allowed.".to_string(),
        });
    }

    // Increment ID counter and create a new Fragrance instance
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value +  1)
        })
        .map_err(|_| Error::Internal {
            msg: "Cannot increment id counter".to_string(),
        })?;
    
    let new_fragrance = Fragrance {
        id,
        name: fragrance.name,
        description: fragrance.description,
        mood_enhancing_properties: fragrance.mood_enhancing_properties,
        created_at: time(),
        updated_at: None,
    };

    // Insert the new fragrance into storage
    do_insert_fragrance(&new_fragrance);
    Some(new_fragrance)
}

#[ic_cdk::update]
fn update_fragrance(id: u64, payload: FragrancePayload) -> Result<Fragrance, Error> {
    // Validate input
    if payload.name.is_empty() || payload.description.is_empty() {
        return Err(Error::InvalidInput {
            msg: "Empty fields are not allowed.".to_string(),
        });
    }

    // Update an existing fragrance with new data
    match FRAGRANCE_STORAGE.with(|service| service.borrow().get(&id)) {
        Some(mut fragrance) => {
            fragrance.name = payload.name;
            fragrance.description = payload.description;
            fragrance.mood_enhancing_properties = payload.mood_enhancing_properties;
            fragrance.updated_at = Some(time());
            do_insert_fragrance(&fragrance);
            Ok(fragrance)
        }
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't update a fragrance with id={}. fragrance not found",
                id
            ),
        }),
    }
}

// helper method to perform insert.
fn do_insert_fragrance(fragrance: &Fragrance) {
    // Insert a fragrance into the storage
    FRAGRANCE_STORAGE.with(|service| service.borrow_mut().insert(fragrance.id, fragrance.clone()));
}

#[ic_cdk::update]
fn delete_fragrance(id: u64) -> Result<Fragrance, Error> {
    // Delete a fragrance from storage
    match FRAGRANCE_STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(fragrance) => Ok(fragrance),
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't delete a fragrance with id={}. fragrance not found.",
                id
            ),
        }),
    }
}

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
}

// a helper method to get a fragrance by id. used in get_fragrance/update_fragrance
fn _get_fragrance(id: &u64) -> Option<Fragrance> {
    // Retrieve a fragrance by ID from storage
    FRAGRANCE_STORAGE.with(|service| service.borrow().get(id))
}

#[ic_cdk::query]
fn list_fragrances() -> Vec<Fragrance> {
    // Retrieve a list of all fragrances from storage
    FRAGRANCE_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .map(|(_, fragrance)| fragrance.clone())
            .collect()
    })
}

#[ic_cdk::query]
fn search_fragrance_names(keyword: String) -> Result<Vec<String>, Error> {
    // Validate keyword
    if keyword.is_empty() {
        return Err(Error::InvalidInput {
            msg: "Keyword cannot be empty".to_string(),
        });
    }

    // Search for fragrances by name or description and return their names
    let matching_names: Vec<String> = FRAGRANCE_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .filter(|(_, fragrance)| {
                fragrance.name.contains(&keyword) || fragrance.description.contains(&keyword)
            })
            .map(|(_, fragrance)| fragrance.name.clone())
            .collect()
    });

    // Return the result or an error message if no matches are found
    if matching_names.is_empty() {
        Err(Error::NotFound {
            msg: format!("No fragrances found with the keyword '{}'", keyword),
        })
    } else {
        Ok(matching_names)
    }
}

#[ic_cdk::query]
fn get_recommendations(mood_keyword: String) -> Result<Vec<Fragrance>, Error> {
    // Validate keyword
    if mood_keyword.is_empty() {
        return Err(Error::InvalidInput {
            msg: "Mood Keyword cannot be empty".to_string(),
        });
    }
    
    let matching_fragrances: Vec<Fragrance> = FRAGRANCE_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .filter(|(_, fragrance)| {
                fragrance
                    .mood_enhancing_properties
                    .iter()
                    .any(|prop| prop.contains(&mood_keyword))
            })
            .map(|(_, fragrance)| fragrance.clone())
            .collect()
    });

    // Return the result or an error message if no matches are found
    if matching_fragrances.is_empty() {
        Err(Error::NotFound {
            msg: format!("No fragrances found with mood-enhancing properties related to '{}'", mood_keyword),
        })
    } else {
        Ok(matching_fragrances)
    }
}

#[ic_cdk::query]
fn filter_fragrances_by_mood(keyword: String) -> Result<Vec<Fragrance>, Error> {
    // Validate keyword
    if keyword.is_empty() {
        return Err(Error::InvalidInput {
            msg: "Keyword cannot be empty".to_string(),
        });
    }
    
    // Filter fragrances by mood-enhancing properties
    let matching_fragrances: Vec<Fragrance> = FRAGRANCE_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .filter(|(_, fragrance)| {
                fragrance
                    .mood_enhancing_properties
                    .iter()
                    .any(|prop| prop.contains(&keyword))
            })
            .map(|(_, fragrance)| fragrance.clone())
            .collect()
    });

    // Return the result or an error message if no matches are found
    if matching_fragrances.is_empty() {
        Err(Error::NotFound {
            msg: format!("No fragrances found with mood-enhancing properties related to '{}'", keyword),
        })
    } else {
        Ok(matching_fragrances)
    }
}

#[ic_cdk::query]
fn sort_fragrances_by_creation_date() -> Vec<Fragrance> {
    // Sort fragrances by creation date in descending order
    let mut fragrances: Vec<Fragrance> = FRAGRANCE_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .map(|(_, fragrance)| fragrance.clone())
            .collect()
    });

    fragrances.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    fragrances
}
// need this to generate candid
ic_cdk::export_candid!();
