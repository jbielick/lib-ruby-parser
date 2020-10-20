// Stack that holds names of current arguments,
// i.e. while parsing
//   def m1(a = (def m2(b = def m3(c = 1); end); end)); end
//                                   ^
// stack is [:a, :b, :c]
//
// Emulates `p->cur_arg` in MRI's parse.y
//
// @api private
//
#[derive(Debug, Clone, Default)]
struct InnerCurrentArgStack {
    stack: Vec<Option<String>>,
}

impl InnerCurrentArgStack {
    pub(crate) fn new() -> Self {
        Self { stack: vec![] }
    }

    pub(crate) fn push(&mut self, value: Option<String>) {
        self.stack.push(value)
    }

    pub(crate) fn set(&mut self, value: Option<String>) {
        self.pop();
        self.push(value)
    }

    pub(crate) fn pop(&mut self) {
        self.stack.pop();
    }

    pub(crate) fn reset(&mut self) {
        self.stack.clear()
    }

    pub(crate) fn top(&self) -> Option<String> {
        match self.stack.last() {
            Some(Some(value)) => Some(value.clone()),
            _ => None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, Default)]
pub(crate) struct CurrentArgStack {
    inner: Rc<RefCell<InnerCurrentArgStack>>,
}

impl CurrentArgStack {
    pub(crate) fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(InnerCurrentArgStack::new())),
        }
    }
    pub(crate) fn push(&self, value: Option<String>) {
        self.inner.borrow_mut().push(value)
    }
    pub(crate) fn set(&self, value: Option<String>) {
        self.inner.borrow_mut().set(value)
    }
    pub(crate) fn pop(&self) {
        self.inner.borrow_mut().pop()
    }
    #[allow(dead_code)]
    pub(crate) fn reset(&self) {
        self.inner.borrow_mut().reset()
    }
    pub(crate) fn top(&self) -> Option<String> {
        self.inner.borrow().top()
    }
}
