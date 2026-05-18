#![feature(proc_macro_hygiene)]

mod lowercase_letters;
use lowercase_letters::*;


#[skyline::from_offset(0x68d530)]
pub fn get_fighter_vtable(id: u32) -> *const usize;

#[skyline::from_offset(0x33be790)]
pub fn get_weapon_vtable(id: u32) -> *const usize;

fn  print_fighter_vtable_addresses() {
    unsafe{
        let mut output_fighter = String::new();
        for fighter_kind in 0..0x5E{
            let vtable = get_fighter_vtable(fighter_kind);
            let first_entry_ptr = *(vtable as *const u64) as *const usize;
            let name = LOWERCASE_FIGHTER_NAMES.get(fighter_kind as usize).unwrap();

            output_fighter.push_str(&format!("{}\n", name));
            for entry in 0..146 {
                let function = *first_entry_ptr.add(entry as usize) - skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as usize;
                output_fighter.push_str(&format!("{:03}:  71{:08x}\n", entry, function));
            }
            output_fighter.push_str(&format!("\n"));
        }
        std::fs::write("sd:/fighter_vtable.txt",output_fighter).unwrap();
    }
}



fn  print_weapon_vtable_addresses() {
    unsafe{
        let mut output_weapon = String::new();
        for weapon_kind in 0..0x267{
            let vtable = get_weapon_vtable(weapon_kind);
            let first_entry_ptr = *(vtable as *const u64) as *const usize;
            let name = LOWERCASE_WEAPON_NAMES.get(weapon_kind as usize).unwrap();
            let owner_name = LOWERCASE_WEAPON_OWNER_NAMES.get(weapon_kind as usize).unwrap();

            output_weapon.push_str(&format!("{}::{}\n",owner_name, name));
            for entry in 0..102 {
                let function = *first_entry_ptr.add(entry as usize) - skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as usize;
                output_weapon.push_str(&format!("{:03}:  71{:08x}\n", entry, function));
            }
            output_weapon.push_str(&format!("\n"));
        }
        std::fs::write("sd:/weapon_vtable.txt",output_weapon).unwrap();
    }
}

fn  print_fighter_vtable_pointer_addresses() {
    unsafe{
        let mut output_fighter = String::new();
        for fighter_kind in 0..0x5E{
            let vtable = get_fighter_vtable(fighter_kind);
            let first_entry_ptr = *(vtable as *const u64) as *const usize;
            let name = LOWERCASE_FIGHTER_NAMES.get(fighter_kind as usize).unwrap();

            output_fighter.push_str(&format!("{}\n", name));
            for entry in 0..146 {
                let function = first_entry_ptr.add(entry as usize) as usize - skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as usize;
                output_fighter.push_str(&format!("{:03}:  71{:08x}\n", entry, function));
            }
            output_fighter.push_str(&format!("\n"));
        }
        std::fs::write("sd:/fighter_vtable_pointers.txt",output_fighter).unwrap();
    }
}



fn  print_weapon_vtable_pointer_addresses() {
    unsafe{
        let mut output_weapon = String::new();
        for weapon_kind in 0..0x267{
            let vtable = get_weapon_vtable(weapon_kind);
            let first_entry_ptr = *(vtable as *const u64) as *const usize;
            let name = LOWERCASE_WEAPON_NAMES.get(weapon_kind as usize).unwrap();
            let owner_name = LOWERCASE_WEAPON_OWNER_NAMES.get(weapon_kind as usize).unwrap();

            output_weapon.push_str(&format!("{}::{}\n",owner_name, name));
            for entry in 0..102 {
                let function = first_entry_ptr.add(entry as usize) as usize - skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as usize;
                output_weapon.push_str(&format!("{:03}:  71{:08x}\n", entry, function));
            }
            output_weapon.push_str(&format!("\n"));
        }
        std::fs::write("sd:/weapon_vtable_pointers.txt",output_weapon).unwrap();
    }
}




#[skyline::main(name = "vtable")]
pub fn main() {
    print_fighter_vtable_addresses();
    print_weapon_vtable_addresses();
    print_fighter_vtable_pointer_addresses();
    print_weapon_vtable_pointer_addresses();
}
