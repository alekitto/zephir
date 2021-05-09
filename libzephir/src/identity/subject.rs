use crate::identity::role::Role;
use crate::policy::policy::{CompletePolicy, ToJson};
use num_traits::cast::AsPrimitive;

pub trait Subject: Role + ToJson {
    /// Returns the inline policy associate with the subject.
    fn get_inline_policy(&self) -> Option<&CompletePolicy>;
}

pub(crate) struct SubjectIterator<'a, T: Subject> {
    current: isize,
    total: isize,

    subject: &'a T,
    linked_policies: &'a [CompletePolicy],
}

impl<'a, T: Subject> SubjectIterator<'a, T> {
    pub(crate) fn new(subject: &'a T) -> Self {
        let linked_policies = subject.linked_policies();
        let linked_policies = linked_policies.into_iter().as_slice();

        let total: usize = linked_policies.len();

        SubjectIterator {
            current: -1,
            total: total.as_(),
            linked_policies,
            subject,
        }
    }
}

impl<'a, T: Subject> Iterator for SubjectIterator<'a, T> {
    type Item = &'a CompletePolicy;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.total {
            return Option::None;
        }

        let element = match self.current {
            -1 => self.subject.get_inline_policy().or_else(|| {
                self.current += 1;
                self.next()
            }),
            _ => self.linked_policies.get(self.current.unsigned_abs())
        };

        self.current += 1;
        element
    }
}
