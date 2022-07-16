pub struct Store<T: Clone + Default + PartialEq, A: Clone> {
    state: T,
    reducer: Box<dyn Fn(T, A) -> T>,
}

impl<T: Clone + Default + PartialEq, A: Clone> Store<T, A> {
    pub fn new(initial_state: T, reducer: Box<dyn Fn(T, A) -> T>) -> Store<T, A> {
        Store {
            state: initial_state,
            reducer,
        }
    }

    pub fn dispatch(&mut self, action: A) {
        let reducer_function = &self.reducer;

        self.state = reducer_function(self.state.clone(), action);
    }

    pub fn get_state(&self) -> T {
        self.state.clone()
    }
}
