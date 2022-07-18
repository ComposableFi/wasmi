use super::super::{AsContext, AsContextMut, StoreContext, StoreContextMut};
use crate::{Engine, Extern, Instance};

/// Represents the caller’s context when creating a host function via [`Func::wrap`].
///
/// [`Func::wrap`]: struct.Func.html#method.wrap
pub struct Caller<'a, T, E> {
    pub(crate) store: StoreContextMut<'a, T, E>,
    /// The module instance associated to the call.
    /// This is `Some` if the host function was called from a Wasm function
    /// since all Wasm function are associated to a module instance.
    /// This usually is `None` if the host function was called from the host side.
    instance: Option<Instance>,
}

impl<'a, T, E> Caller<'a, T, E> {
    /// Creates a new [`Caller`] from the given store context and [`Instance`] handle.
    pub fn new<C>(ctx: &'a mut C, instance: Option<Instance>) -> Self
    where
        C: AsContextMut<UserState = T, Error = E>,
    {
        Self {
            store: ctx.as_context_mut(),
            instance,
        }
    }

    /// Queries the caller for an exported definition identifier by `name`.
    ///
    /// Returns `None` if there is no associated [`Instance`] of the caller
    /// or if the caller does not provide an export under the name `name`.
    pub fn get_export(&self, name: &str) -> Option<Extern> {
        self.instance
            .and_then(|instance| instance.get_export(self, name))
    }

    /// Returns a shared reference to the host provided data.
    pub fn host_data(&self) -> &T {
        self.store.store.state()
    }

    /// Returns an exclusive reference to the host provided data.
    pub fn host_data_mut(&mut self) -> &mut T {
        self.store.store.state_mut()
    }

    /// Returns a shared reference to the used [`Engine`].
    pub fn engine(&self) -> &Engine {
        self.store.store.engine()
    }
}

impl<T, E> AsContext for Caller<'_, T, E> {
    type UserState = T;
    type Error = E;

    fn as_context(&self) -> StoreContext<'_, Self::UserState, Self::Error> {
        self.store.as_context()
    }
}

impl<T, E> AsContextMut for Caller<'_, T, E> {
    fn as_context_mut(&mut self) -> StoreContextMut<'_, Self::UserState, Self::Error> {
        self.store.as_context_mut()
    }
}

impl<'a, T: AsContextMut> From<&'a mut T> for Caller<'a, T::UserState, T::Error> {
    fn from(ctx: &'a mut T) -> Self {
        Self {
            store: ctx.as_context_mut(),
            instance: None,
        }
    }
}
