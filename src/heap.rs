use crate::tvm::Tvm;

trait HeapHolder {
    fn get_heap(&self) -> &[i32];
    fn get_heap_size(&self) -> usize;
    fn allocate(&mut self, size: usize) -> usize;
    fn deallocate(&mut self, address: usize);
}

impl HeapHolder for Tvm {
    fn get_heap(&self) -> &[i32] {
        &self.memory[..self.heap_size]
    }

    fn get_heap_size(&self) -> usize {
        self.heap_size
    }

    fn allocate(&mut self, size: usize) -> usize {
        let address = self.heap_size;
        self.heap_size += size;
        address
    }

    fn deallocate(&mut self, address: usize) {
        self.heap_size = address;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_heap() {
        let mut tvm = Tvm::default();
        tvm.allocate(3);
        assert_eq!(tvm.get_heap(), &[0, 0, 0]);
    }

    #[test]
    fn test_get_heap_size() {
        let mut tvm = Tvm::default();
        tvm.allocate(3);
        assert_eq!(tvm.get_heap_size(), 3);
    }

    #[test]
    fn test_allocate() {
        let mut tvm = Tvm::default();
        let address = tvm.allocate(3);
        assert_eq!(address, 0);
        assert_eq!(tvm.get_heap(), &[0, 0, 0]);
    }

    #[test]
    fn test_deallocate() {
        let mut tvm = Tvm::default();
        let address = tvm.allocate(3);
        assert_eq!(address, 0);
        assert_eq!(tvm.get_heap(), &[0, 0, 0]);
        tvm.deallocate(address);
        assert_eq!(tvm.get_heap(), &[]);
    }
}