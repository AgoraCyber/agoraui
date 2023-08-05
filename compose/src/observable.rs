use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
    task::{Poll, Waker},
};

use futures::Future;

#[derive(Debug)]
pub struct Observable<T> {
    id: usize,
    raw: Rc<RefCell<_RawObservable<T>>>,
}

impl<T> Clone for Observable<T> {
    fn clone(&self) -> Self {
        let raw = Rc::clone(&self.raw);

        Self {
            id: Rc::strong_count(&raw),
            raw,
        }
    }
}

impl<T> Observable<T> {
    pub fn new(value: T) -> Self {
        Self {
            id: 1,
            raw: Rc::new(RefCell::new(_RawObservable {
                value: value,
                wakers: Default::default(),
                consumers: 0,
            })),
        }
    }
}

impl<T: Default> Default for Observable<T> {
    fn default() -> Self {
        Self {
            id: 1,
            raw: Rc::new(RefCell::new(_RawObservable {
                value: Default::default(),
                wakers: Default::default(),
                consumers: 0,
            })),
        }
    }
}

#[derive(Debug)]
struct _RawObservable<T> {
    value: T,
    wakers: HashMap<usize, Waker>,
    consumers: usize,
}

impl<T: Clone> _RawObservable<T> {
    fn set_value(&mut self, value: T) {
        assert!(self.consumers == 0, "call set_value when wakeup wakers");

        self.value = value;

        for (id, waker) in self.wakers.iter() {
            log::debug!("Wakeup waker({})", id);
            waker.wake_by_ref();
        }

        self.consumers = self.wakers.len();

        self.wakers.clear();

        log::debug!("Total wakeup wakers({})", self.consumers);
    }

    fn poll(&mut self, id: usize, waker: &Waker) -> Poll<T> {
        if self.consumers > 0 {
            log::debug!("Wakeup one waker");
            self.consumers -= 1;
            return Poll::Ready(self.value.clone());
        }

        log::debug!("waker({}) register", id);

        self.wakers.insert(id, waker.clone());

        return Poll::Pending;
    }
}

impl<T: Clone> Observable<T> {
    /// Set new value and notify all waiting listeners.
    pub fn set_value(&self, value: T) {
        self.raw.borrow_mut().set_value(value);
    }

    pub fn next(&self) -> ObservableNext<T> {
        ObservableNext {
            inner: self.clone(),
        }
    }
}

pub struct ObservableNext<T> {
    inner: Observable<T>,
}

impl<T: Clone> Future for ObservableNext<T> {
    type Output = T;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        self.inner.raw.borrow_mut().poll(self.inner.id, cx.waker())
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use async_std::task::sleep;
    use futures::future::join;

    use super::*;

    #[async_std::test]
    async fn test_observable_multi_consumer() {
        _ = pretty_env_logger::try_init();

        let ob = Observable::new(12);
        let ob_notifier = ob.clone();

        async_std::task::spawn_local(async move {
            sleep(Duration::from_secs(1)).await;
            ob_notifier.set_value(2);
        });

        assert_eq!(join(ob.next(), ob.next()).await, (2, 2));
    }

    #[async_std::test]
    async fn test_observable_one_consumer() {
        _ = pretty_env_logger::try_init();

        let ob = Observable::new(12);
        let ob_notifier = ob.clone();

        async_std::task::spawn_local(async move {
            sleep(Duration::from_secs(1)).await;
            ob_notifier.set_value(3);
        });

        assert_eq!(ob.next().await, 3);
    }
}
