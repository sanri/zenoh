//
// Copyright (c) 2017, 2020 ADLINK Technology Inc.
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ADLINK zenoh team, <zenoh@adlink-labs.tech>
//
use std::collections::VecDeque;

pub(crate) struct StackBuffer<T> {
    buffer: VecDeque<T>,
    capacity: usize,
    n: usize,
}

impl<T> StackBuffer<T> {
    pub(crate) fn new(capacity: usize) -> StackBuffer<T> {
        let buffer = VecDeque::<T>::with_capacity(capacity);
        StackBuffer {
            buffer,
            capacity,
            n: 0,
        }
    }

    #[inline]
    pub(crate) fn push(&mut self, elem: T) -> Option<T> {
        if self.n < self.capacity {
            self.buffer.push_front(elem);
            self.n += 1;
            return None;
        }
        Some(elem)
    }

    #[inline]
    pub(crate) fn pop(&mut self) -> Option<T> {
        let x = self.buffer.pop_front();
        if x.is_some() {
            self.n -= 1;
        }
        x
    }

    #[allow(dead_code)]
    #[inline]
    pub(crate) fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    #[inline]
    pub(crate) fn is_full(&self) -> bool {
        self.len() == self.capacity()
    }

    #[inline]
    pub(crate) fn len(&self) -> usize {
        self.n
    }

    #[inline]
    pub(crate) fn capacity(&self) -> usize {
        self.capacity
    }
}