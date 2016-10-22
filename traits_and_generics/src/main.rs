use std::mem;

trait Processor {
    fn process(&self);
}

struct NotProcessor;

struct IdProcessor {
    id: u32
}

impl Processor for IdProcessor {
    fn process(&self) {
        println!("IdProcessor, id is {}", self.id);
    }
}

#[derive(Copy, Clone)]
struct CopyableProcessor {
    id: u32
}

impl Processor for CopyableProcessor {
    fn process(&self) {
        println!("CopyableProcessor, id is {}", self.id);
    }
}

fn main() {
    let not_processor = NotProcessor;
    let id_processor = IdProcessor { id: 20 };
    let copyable_processor = CopyableProcessor { id: 30 };

    use_anything(&not_processor);
    use_anything(&id_processor);
    use_anything(&copyable_processor);

    // The following line gives a compiler error:
    //use_processor(not_processor);
    use_processor(&id_processor);
    use_processor(&copyable_processor);

    // The following two lines gives compiler errors:
    //use_copyable_processor(not_processor);
    //use_copyable_processor(id_processor);
    use_copyable_processor(copyable_processor);

    // This won't work:
    /*
    let new_id_processor = id_processor;
    println!("id_processor.id is {}", id_processor.id);
    */

    let mut new_copyable_processor = copyable_processor;
    new_copyable_processor.id = 22;
    println!("copyable_processor.id is {}", copyable_processor.id);
    println!("new_copyable_processor.id is {}", new_copyable_processor.id);

    let mut mutable_copyable_processor = CopyableProcessor { id: 100 };
    println!("mutable_copyable_processor.id is {}", mutable_copyable_processor.id);

    {
        let new_mutable_copyable_processor = &mut mutable_copyable_processor;
        new_mutable_copyable_processor.id = 22;
        println!("new_mutable_copyable_processor.id is {}", new_copyable_processor.id);    
    }
    
    println!("mutable_copyable_processor.id is {}", mutable_copyable_processor.id);
}

fn use_anything<T>(anything: &T) {
    println!("Size of anything: {}", mem::size_of_val(anything));
}

fn use_processor<T>(processor: &T) where T: Processor {
    println!("In use_processor...");
    processor.process();
}

fn use_copyable_processor<T>(processor: T) where T: Processor + Copy {
    println!("In use_copyable_processor...");
    processor.process();
}