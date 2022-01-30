mod control_frame;
mod control_stack;
mod inst_builder;
mod value_stack;

pub use self::inst_builder::{
    BrTable,
    InstructionIdx,
    InstructionsBuilder,
    LabelIdx,
    RelativeDepth,
    Reloc,
};
use self::{control_frame::ControlFrame, control_stack::ControlFlowStack, value_stack::ValueStack};
use super::{DropKeep, Instruction, Target};
use crate::{
    module2::{BlockType, FuncIdx, FuncTypeIdx, GlobalIdx, MemoryIdx, ModuleResources, TableIdx},
    Engine,
    ModuleError,
};
use wasmi_core::{ValueType, F32, F64};

/// The interface to translate a `wasmi` bytecode function using Wasm bytecode.
#[derive(Debug)]
pub struct FunctionBuilder<'engine, 'parser> {
    /// The [`Engine`] for which the function is translated.
    engine: &'engine Engine,
    /// The function under construction.
    func: FuncIdx,
    /// The immutable `wasmi` module resources.
    res: ModuleResources<'parser>,
    /// The control flow frame stack that represents the Wasm control flow.
    control_frames: ControlFlowStack,
    /// The emulated value stack.
    value_stack: ValueStack,
    /// The instruction builder.
    ///
    /// # Note
    ///
    /// Allows to incrementally construct the instruction of a function.
    inst_builder: InstructionsBuilder,
    /// The amount of local variables of the currently compiled function.
    len_locals: usize,
    /// The maximum height of the emulated value stack of the translated function.
    ///
    /// # Note
    ///
    /// This does not include input parameters and local variables.
    max_stack_height: usize,
    /// This represents the reachability of the currently translated code.
    ///
    /// - `true`: The currently translated code is reachable.
    /// - `false`: The currently translated code is unreachable and can be skipped.
    ///
    /// # Note
    ///
    /// Visiting the Wasm `Else` or `End` control flow operator resets
    /// reachability to `true` again.
    reachable: bool,
}

impl<'engine, 'parser> FunctionBuilder<'engine, 'parser> {
    /// Creates a new [`FunctionBuilder`].
    pub fn new(engine: &'engine Engine, func: FuncIdx, res: ModuleResources<'parser>) -> Self {
        Self {
            engine,
            func,
            res,
            control_frames: ControlFlowStack::default(),
            value_stack: ValueStack::default(),
            inst_builder: InstructionsBuilder::default(),
            len_locals: 0,
            max_stack_height: 0,
            reachable: true,
        }
    }

    /// Try to resolve the given label.
    ///
    /// In case the label cannot yet be resolved register the [`Reloc`] as its user.
    fn try_resolve_label<F>(&mut self, label: LabelIdx, reloc_provider: F) -> InstructionIdx
    where
        F: FnOnce(InstructionIdx) -> Reloc,
    {
        let pc = self.inst_builder.current_pc();
        self.inst_builder
            .try_resolve_label(label, || reloc_provider(pc))
    }

    /// Translates the given local variables for the translated function.
    pub fn translate_locals(
        &mut self,
        amount: u32,
        _value_type: ValueType,
    ) -> Result<(), ModuleError> {
        self.len_locals += amount as usize;
        Ok(())
    }
}

impl<'engine, 'parser> FunctionBuilder<'engine, 'parser> {
    /// Translates a Wasm `unreachable` instruction.
    pub fn translate_unreachable(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translates a Wasm `block` control flow operator.
    pub fn translate_block(&mut self, _block_type: BlockType) -> Result<(), ModuleError> {
        let end_label = self.inst_builder.new_label();
        self.control_frames
            .push_frame(ControlFrame::Block { end_label });
        Ok(())
    }

    /// Translates a Wasm `block` control flow operator.
    pub fn translate_loop(&mut self, _block_type: BlockType) -> Result<(), ModuleError> {
        let header = self.inst_builder.new_label();
        self.inst_builder.resolve_label(header);
        self.control_frames
            .push_frame(ControlFrame::Loop { header });
        Ok(())
    }

    /// Translates a Wasm `if` control flow operator.
    pub fn translate_if(&mut self, _block_type: BlockType) -> Result<(), ModuleError> {
        let else_label = self.inst_builder.new_label();
        let end_label = self.inst_builder.new_label();
        self.control_frames.push_frame(ControlFrame::If {
            else_label,
            end_label,
        });
        let dst_pc = self.try_resolve_label(else_label, |pc| Reloc::Br { inst_idx: pc });
        let branch_target = Target::new(dst_pc, DropKeep::new(0, 0));
        self.inst_builder
            .push_inst(Instruction::BrIfEqz(branch_target));
        Ok(())
    }

    /// Translates a Wasm `else` control flow operator.
    pub fn translate_else(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translates a Wasm `end` control flow operator.
    pub fn translate_end(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translates a Wasm `br` control flow operator.
    pub fn translate_br(&mut self, relative_depth: u32) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translates a Wasm `br_if` control flow operator.
    pub fn translate_br_if(&mut self, relative_depth: u32) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translates a Wasm `br_table` control flow operator.
    pub fn translate_br_table(&mut self, br_table: impl BrTable) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translates a Wasm `return` control flow operator.
    pub fn translate_return(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translates a Wasm `call` instruction.
    pub fn translate_call(&mut self, func_idx: FuncIdx) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translates a Wasm `call_indirect` instruction.
    pub fn translate_call_indirect(
        &mut self,
        func_type_idx: FuncTypeIdx,
        table_idx: TableIdx,
    ) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translates a Wasm `drop` instruction.
    pub fn translate_drop(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translates a Wasm `select` instruction.
    pub fn translate_select(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `local.get` instruction.
    pub fn translate_local_get(&mut self, local_idx: u32) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `local.set` instruction.
    pub fn translate_local_set(&mut self, local_idx: u32) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `local.tee` instruction.
    pub fn translate_local_tee(&mut self, local_idx: u32) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `global.get` instruction.
    pub fn translate_global_get(&mut self, global_idx: GlobalIdx) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `global.set` instruction.
    pub fn translate_global_set(&mut self, global_idx: GlobalIdx) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i32.load` instruction.
    pub fn translate_i32_load(
        &mut self,
        memory_idx: MemoryIdx,
        offset: u32,
    ) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i64.load` instruction.
    pub fn translate_i64_load(
        &mut self,
        memory_idx: MemoryIdx,
        offset: u32,
    ) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f32.load` instruction.
    pub fn translate_f32_load(
        &mut self,
        memory_idx: MemoryIdx,
        offset: u32,
    ) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f64.load` instruction.
    pub fn translate_f64_load(
        &mut self,
        memory_idx: MemoryIdx,
        offset: u32,
    ) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i32.load_i8` instruction.
    pub fn translate_i32_load_i8(
        &mut self,
        memory_idx: MemoryIdx,
        offset: u32,
    ) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i32.load_u8` instruction.
    pub fn translate_i32_load_u8(
        &mut self,
        memory_idx: MemoryIdx,
        offset: u32,
    ) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i32.load_i16` instruction.
    pub fn translate_i32_load_i16(
        &mut self,
        memory_idx: MemoryIdx,
        offset: u32,
    ) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i32.load_u16` instruction.
    pub fn translate_i32_load_u16(
        &mut self,
        memory_idx: MemoryIdx,
        offset: u32,
    ) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i64.load_i8` instruction.
    pub fn translate_i64_load_i8(
        &mut self,
        memory_idx: MemoryIdx,
        offset: u32,
    ) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i64.load_u8` instruction.
    pub fn translate_i64_load_u8(
        &mut self,
        memory_idx: MemoryIdx,
        offset: u32,
    ) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i64.load_i16` instruction.
    pub fn translate_i64_load_i16(
        &mut self,
        memory_idx: MemoryIdx,
        offset: u32,
    ) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i64.load_u16` instruction.
    pub fn translate_i64_load_u16(
        &mut self,
        memory_idx: MemoryIdx,
        offset: u32,
    ) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i64.load_i32` instruction.
    pub fn translate_i64_load_i32(
        &mut self,
        memory_idx: MemoryIdx,
        offset: u32,
    ) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i64.load_u32` instruction.
    pub fn translate_i64_load_u32(
        &mut self,
        memory_idx: MemoryIdx,
        offset: u32,
    ) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i32.store` instruction.
    pub fn translate_i32_store(
        &mut self,
        memory_idx: MemoryIdx,
        offset: u32,
    ) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i64.store` instruction.
    pub fn translate_i64_store(
        &mut self,
        memory_idx: MemoryIdx,
        offset: u32,
    ) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f32.store` instruction.
    pub fn translate_f32_store(
        &mut self,
        memory_idx: MemoryIdx,
        offset: u32,
    ) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f64.store` instruction.
    pub fn translate_f64_store(
        &mut self,
        memory_idx: MemoryIdx,
        offset: u32,
    ) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i32.store_i8` instruction.
    pub fn translate_i32_store_i8(
        &mut self,
        memory_idx: MemoryIdx,
        offset: u32,
    ) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i32.store_i16` instruction.
    pub fn translate_i32_store_i16(
        &mut self,
        memory_idx: MemoryIdx,
        offset: u32,
    ) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i64.store_i8` instruction.
    pub fn translate_i64_store_i8(
        &mut self,
        memory_idx: MemoryIdx,
        offset: u32,
    ) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i64.store_i16` instruction.
    pub fn translate_i64_store_i16(
        &mut self,
        memory_idx: MemoryIdx,
        offset: u32,
    ) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i64.store_i32` instruction.
    pub fn translate_i64_store_i32(
        &mut self,
        memory_idx: MemoryIdx,
        offset: u32,
    ) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `memory.size` instruction.
    pub fn translate_memory_size(&mut self, memory_idx: MemoryIdx) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `memory.grow` instruction.
    pub fn translate_memory_grow(&mut self, memory_idx: MemoryIdx) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i32.const` instruction.
    pub fn translate_i32_const(&mut self, value: i32) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i64.const` instruction.
    pub fn translate_i64_const(&mut self, value: i64) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f32.const` instruction.
    pub fn translate_f32_const(&mut self, value: F32) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f64.const` instruction.
    pub fn translate_f64_const(&mut self, value: F64) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i32_eqz` instruction.
    pub fn translate_i32_eqz(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i32_eq` instruction.
    pub fn translate_i32_eq(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i32_ne` instruction.
    pub fn translate_i32_ne(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i32_lt` instruction.
    pub fn translate_i32_lt(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `u32_lt` instruction.
    pub fn translate_u32_lt(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i32_gt` instruction.
    pub fn translate_i32_gt(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `u32_gt` instruction.
    pub fn translate_u32_gt(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i32_le` instruction.
    pub fn translate_i32_le(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `u32_le` instruction.
    pub fn translate_u32_le(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i32_ge` instruction.
    pub fn translate_i32_ge(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `u32_ge` instruction.
    pub fn translate_u32_ge(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i64_eqz` instruction.
    pub fn translate_i64_eqz(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i64_eq` instruction.
    pub fn translate_i64_eq(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i64_ne` instruction.
    pub fn translate_i64_ne(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i64_lt` instruction.
    pub fn translate_i64_lt(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `u64_lt` instruction.
    pub fn translate_u64_lt(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i64_gt` instruction.
    pub fn translate_i64_gt(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `u64_gt` instruction.
    pub fn translate_u64_gt(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i64_le` instruction.
    pub fn translate_i64_le(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `u64_le` instruction.
    pub fn translate_u64_le(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i64_ge` instruction.
    pub fn translate_i64_ge(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `u64_ge` instruction.
    pub fn translate_u64_ge(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f32_eq` instruction.
    pub fn translate_f32_eq(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f32_ne` instruction.
    pub fn translate_f32_ne(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f32_lt` instruction.
    pub fn translate_f32_lt(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f32_gt` instruction.
    pub fn translate_f32_gt(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f32_le` instruction.
    pub fn translate_f32_le(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f32_ge` instruction.
    pub fn translate_f32_ge(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f64_eq` instruction.
    pub fn translate_f64_eq(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f64_ne` instruction.
    pub fn translate_f64_ne(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f64_lt` instruction.
    pub fn translate_f64_lt(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f64_gt` instruction.
    pub fn translate_f64_gt(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f64_le` instruction.
    pub fn translate_f64_le(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f64_ge` instruction.
    pub fn translate_f64_ge(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i32_clz` instruction.
    pub fn translate_i32_clz(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i32_ctz` instruction.
    pub fn translate_i32_ctz(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i32_popcnt` instruction.
    pub fn translate_i32_popcnt(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i32_add` instruction.
    pub fn translate_i32_add(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i32_sub` instruction.
    pub fn translate_i32_sub(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i32_mul` instruction.
    pub fn translate_i32_mul(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i32_div` instruction.
    pub fn translate_i32_div(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `u32_div` instruction.
    pub fn translate_u32_div(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i32_remS` instruction.
    pub fn translate_i32_remS(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `u32_rem` instruction.
    pub fn translate_u32_rem(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i32_and` instruction.
    pub fn translate_i32_and(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i32_or` instruction.
    pub fn translate_i32_or(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i32_xor` instruction.
    pub fn translate_i32_xor(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i32_shl` instruction.
    pub fn translate_i32_shl(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i32_shr` instruction.
    pub fn translate_i32_shr(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `u32_shr` instruction.
    pub fn translate_u32_shr(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i32_rotl` instruction.
    pub fn translate_i32_rotl(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i32_rotr` instruction.
    pub fn translate_i32_rotr(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i64_clz` instruction.
    pub fn translate_i64_clz(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i64_ctz` instruction.
    pub fn translate_i64_ctz(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i64_popcnt` instruction.
    pub fn translate_i64_popcnt(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i64_add` instruction.
    pub fn translate_i64_add(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i64_sub` instruction.
    pub fn translate_i64_sub(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i64_mul` instruction.
    pub fn translate_i64_mul(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i64_div` instruction.
    pub fn translate_i64_div(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `u64_div` instruction.
    pub fn translate_u64_div(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i64_rem` instruction.
    pub fn translate_i64_rem(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `u64_rem` instruction.
    pub fn translate_u64_rem(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i64_and` instruction.
    pub fn translate_i64_and(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i64_or` instruction.
    pub fn translate_i64_or(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i64_xor` instruction.
    pub fn translate_i64_xor(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i64_shl` instruction.
    pub fn translate_i64_shl(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i64_shrS` instruction.
    pub fn translate_i64_shrS(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `u64_shr` instruction.
    pub fn translate_u64_shr(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i64_rotl` instruction.
    pub fn translate_i64_rotl(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i64_rotr` instruction.
    pub fn translate_i64_rotr(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f32_abs` instruction.
    pub fn translate_f32_abs(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f32_neg` instruction.
    pub fn translate_f32_neg(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f32_ceil` instruction.
    pub fn translate_f32_ceil(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f32_floor` instruction.
    pub fn translate_f32_floor(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f32_trunc` instruction.
    pub fn translate_f32_trunc(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f32_nearest` instruction.
    pub fn translate_f32_nearest(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f32_sqrt` instruction.
    pub fn translate_f32_sqrt(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f32_add` instruction.
    pub fn translate_f32_add(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f32_sub` instruction.
    pub fn translate_f32_sub(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f32_mul` instruction.
    pub fn translate_f32_mul(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f32_div` instruction.
    pub fn translate_f32_div(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f32_min` instruction.
    pub fn translate_f32_min(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f32_max` instruction.
    pub fn translate_f32_max(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f32_copysign` instruction.
    pub fn translate_f32_copysign(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f64_abs` instruction.
    pub fn translate_f64_abs(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f64_neg` instruction.
    pub fn translate_f64_neg(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f64_ceil` instruction.
    pub fn translate_f64_ceil(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f64_floor` instruction.
    pub fn translate_f64_floor(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f64_trunc` instruction.
    pub fn translate_f64_trunc(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f64_nearest` instruction.
    pub fn translate_f64_nearest(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f64_sqrt` instruction.
    pub fn translate_f64_sqrt(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f64_add` instruction.
    pub fn translate_f64_add(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f64_sub` instruction.
    pub fn translate_f64_sub(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f64_mul` instruction.
    pub fn translate_f64_mul(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f64_div` instruction.
    pub fn translate_f64_div(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f64_min` instruction.
    pub fn translate_f64_min(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f64_max` instruction.
    pub fn translate_f64_max(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f64_copysign` instruction.
    pub fn translate_f64_copysign(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i32_wrap_i64` instruction.
    pub fn translate_i32_wrap_i64(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i32_trunc_f32` instruction.
    pub fn translate_i32_trunc_f32(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `u32_trunc_f32` instruction.
    pub fn translate_u32_trunc_f32(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i32_trunc_f64` instruction.
    pub fn translate_i32_trunc_f64(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `u32_trunc_f64` instruction.
    pub fn translate_u32_trunc_f64(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i64_extend_i32` instruction.
    pub fn translate_i64_extend_i32(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `u64_extend_i32` instruction.
    pub fn translate_u64_extend_i32(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i64_trunc_F3` instruction.
    pub fn translate_i64_trunc_F3(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `u64_trunc_F3` instruction.
    pub fn translate_u64_trunc_F3(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i64_trunc_F6` instruction.
    pub fn translate_i64_trunc_F6(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `u64_trunc_F6` instruction.
    pub fn translate_u64_trunc_F6(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f32_convert_i32` instruction.
    pub fn translate_f32_convert_i32(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f32_convert_u32` instruction.
    pub fn translate_f32_convert_u32(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f32_convert_i64` instruction.
    pub fn translate_f32_convert_i64(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f32_convert_u64` instruction.
    pub fn translate_f32_convert_u64(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f32_demote_f64` instruction.
    pub fn translate_f32_demote_f64(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f64_convert_i32` instruction.
    pub fn translate_f64_convert_i32(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f64_convert_u32` instruction.
    pub fn translate_f64_convert_u32(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f64_convert_i64` instruction.
    pub fn translate_f64_convert_i64(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f64_convert_u64` instruction.
    pub fn translate_f64_convert_u64(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f64_promote_f32` instruction.
    pub fn translate_f64_promote_f32(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i32_reinterpret_f32` instruction.
    pub fn translate_i32_reinterpret_f32(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `i64_reinterpret_f64` instruction.
    pub fn translate_i64_reinterpret_f64(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f32_reinterpret_i32` instruction.
    pub fn translate_f32_reinterpret_i32(&mut self) -> Result<(), ModuleError> {
        todo!()
    }

    /// Translate a Wasm `f64_reinterpret_i64` instruction.
    pub fn translate_f64_reinterpret_i64(&mut self) -> Result<(), ModuleError> {
        todo!()
    }
}
