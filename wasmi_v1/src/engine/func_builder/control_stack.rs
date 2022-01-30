use super::ControlFrame;

/// The stack of control flow frames.
#[derive(Debug, Default)]
pub struct ControlFlowStack {
    frames: Vec<ControlFrame>,
}

impl ControlFlowStack {
    /// Returns the current depth of the stack of the [`ControlFlowStack`].
    pub fn len(&self) -> usize {
        self.frames.len()
    }

    /// Pushes a new control flow frame to the [`ControlFlowStack`].
    pub fn push_frame<T>(&mut self, frame: T)
    where
        T: Into<ControlFrame>,
    {
        self.frames.push(frame.into())
    }

    /// Pops the last control flow frame from the [`ControlFlowStack`].
    ///
    /// # Panics
    ///
    /// If the [`ControlFlowStack`] is empty.
    pub fn pop_frame(&mut self) -> ControlFrame {
        self.frames
            .pop()
            .expect("tried to pop control flow frame from empty control flow stack")
    }

    /// Returns the first control flow frame on the control stack.
    ///
    /// # Note
    ///
    /// Usually the first control flow frame is the control flow block of the
    /// function body itself. Branching to it is equal to returning from the
    /// function following WebAssembly semantics.
    pub fn first(&self) -> &ControlFrame {
        self.frames.first().expect(
            "tried to return the first control flow \
                frame from empty control flow stack",
        )
    }

    /// Returns the last control flow frame on the control stack.
    pub fn last_mut(&mut self) -> &mut ControlFrame {
        self.frames.last_mut().expect(
            "tried to exclusively peek the last control flow \
            frame from an empty control flow stack",
        )
    }

    /// Returns an exclusive reference to the control flow frame at the given `depth`.
    ///
    /// A `depth` of 0 is equal to calling [`ControlFlowStack::last_mut`].
    ///
    /// # Panics
    ///
    /// If `depth` exceeds the length of the stack of control flow frames.
    pub fn nth_back_mut(&mut self, depth: u32) -> &mut ControlFrame {
        let len = self.len();
        self.frames
            .iter_mut()
            .nth_back(depth as usize)
            .unwrap_or_else(|| {
                panic!(
                    "tried to peek the {}-th control flow frame \
                    but there are only {} control flow frames",
                    depth, len
                )
            })
    }
}
