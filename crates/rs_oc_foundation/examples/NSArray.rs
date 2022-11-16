use rs_oc_basic::NIL;
use rs_oc_foundation::{NSArray, NSNumber};

fn main() {
    unsafe {
        let item = NSNumber::number_with_int(NIL, 1);
        let arr = NSArray::array_with_object(NIL, item);
        let list = NSArray::array_with_objects(
            NIL,
            &[
                NSNumber::number_with_int(NIL, 1),
                NSNumber::number_with_int(NIL, 1),
                NSNumber::number_with_int(NIL, 1),
                NSNumber::number_with_int(NIL, 1),
            ],
        );
        println!("item: {:?}", item);
        println!("count: {:?}", arr.count());
        println!("contains object: {:?}", arr.contains_object(item));
        println!("first object: {:?}", arr.first_object());
        println!("last object: {:?}", arr.last_object());
        println!("object at index: {:?}", arr.object_at_index(0));
    }
}
