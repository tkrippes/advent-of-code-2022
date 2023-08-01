mod compartment;
mod item;

use compartment::Compartment;

struct Rucksack {
    first_compartment: Compartment,
    second_compartment: Compartment,
}
