use std::collections::HashSet;

#[derive(Clone)]
enum Value {
    Number(i64),
    String(String),
    List(Vec<usize>),
}

struct Object {
    value: Value,
    marked: bool,
}

struct GC {
    heap: Vec<Option<Object>>,
    roots: HashSet<usize>,
}

impl GC {
    fn new() -> Self {
        Self { heap: Vec::new(), roots: HashSet::new() }
    }
    
    fn alloc(&mut self, value: Value) -> usize {
        if let Some(idx) = self.heap.iter().position(|s| s.is_none()) {
            self.heap[idx] = Some(Object { value, marked: false });
            idx
        } else {
            self.heap.push(Some(Object { value, marked: false }));
            self.heap.len() - 1
        }
    }
    
    fn root(&mut self, ptr: usize) {
        self.roots.insert(ptr);
    }
    
    fn collect(&mut self) -> usize {
        let mut work: Vec<_> = self.roots.iter().copied().collect();
        
        while let Some(i) = work.pop() {
            if let Some(obj) = &mut self.heap[i] {
                if !obj.marked {
                    obj.marked = true;
                    if let Value::List(kids) = &obj.value {
                        work.extend(kids);
                    }
                }
            }
        }
        
        let mut freed = 0;
        for slot in &mut self.heap {
            if let Some(obj) = slot {
                if !obj.marked {
                    *slot = None;
                    freed += 1;
                } else {
                    obj.marked = false;
                }
            }
        }
        freed
    }
    
    fn alive(&self) -> usize {
        self.heap.iter().filter(|s| s.is_some()).count()
    }
    
    fn get(&self, ptr: usize) -> Option<&Value> {
        self.heap.get(ptr).and_then(|s| s.as_ref()).map(|o| &o.value)
    }
}

fn main() {
    let mut gc = GC::new();
    
    let n1 = gc.alloc(Value::Number(42));
    let _n2 = gc.alloc(Value::Number(1337));
    let s1 = gc.alloc(Value::String("hello".into()));
    let _s2 = gc.alloc(Value::String("world".into()));
    let list = gc.alloc(Value::List(vec![n1, s1]));
    
    println!("allocated 5 objects, alive: {}", gc.alive());
    
    gc.root(n1);
    gc.root(list);
    
    let freed = gc.collect();
    println!("collected {} objects, alive: {}", freed, gc.alive());
    
    if let Some(Value::Number(n)) = gc.get(n1) {
        println!("n1 still accessible: {}", n);
    }
    
    let size_before = gc.heap.len();
    let n3 = gc.alloc(Value::Number(999));
    gc.root(n3);
    println!("allocated 1 more, heap size: {} (reused slot)", gc.heap.len());
    
    assert_eq!(size_before, gc.heap.len());
}