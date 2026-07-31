use std::marker::PhantomData;

use crate::application_schema::{
    ApplicationEntityRef, ApplicationFieldCurrency, ApplicationFieldRef, ApplicationRelationRef,
    EqualityCapable, EqualityPredicate, TypedApplicationValue, WritePosture,
};

use super::ApplicationQueryRootPathGuard;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ApplicationQueryRootPathDirection {
    Forward,
    Reverse,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationQueryRootPathStep {
    relation: &'static str,
    from: &'static str,
    to: &'static str,
    direction: ApplicationQueryRootPathDirection,
}

impl ApplicationQueryRootPathStep {
    pub const fn relation(&self) -> &'static str {
        self.relation
    }

    pub const fn from(&self) -> &'static str {
        self.from
    }

    pub const fn to(&self) -> &'static str {
        self.to
    }

    pub const fn direction(&self) -> ApplicationQueryRootPathDirection {
        self.direction
    }

    pub const fn parent_entity(&self) -> &'static str {
        match self.direction {
            ApplicationQueryRootPathDirection::Forward => self.from,
            ApplicationQueryRootPathDirection::Reverse => self.to,
        }
    }

    pub const fn child_entity(&self) -> &'static str {
        match self.direction {
            ApplicationQueryRootPathDirection::Forward => self.to,
            ApplicationQueryRootPathDirection::Reverse => self.from,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationQueryRootPathMeaning {
    start_entity: &'static str,
    terminal_entity: &'static str,
    steps: Vec<ApplicationQueryRootPathStep>,
    guards: Vec<ApplicationQueryRootPathGuard>,
}

impl ApplicationQueryRootPathMeaning {
    pub const fn start_entity(&self) -> &'static str {
        self.start_entity
    }

    pub const fn terminal_entity(&self) -> &'static str {
        self.terminal_entity
    }

    pub fn steps(&self) -> &[ApplicationQueryRootPathStep] {
        &self.steps
    }

    pub fn guards(&self) -> &[ApplicationQueryRootPathGuard] {
        &self.guards
    }
}

pub struct ApplicationQueryRootPath<Schema, Start, Current> {
    start_entity: &'static str,
    current_entity: &'static str,
    steps: Vec<ApplicationQueryRootPathStep>,
    guards: Vec<ApplicationQueryRootPathGuard>,
    _marker: PhantomData<fn() -> (Schema, Start, Current)>,
}

impl<Schema, Start> ApplicationQueryRootPath<Schema, Start, Start> {
    pub fn from(start: ApplicationEntityRef<Schema, Start>) -> Self {
        Self {
            start_entity: start.name(),
            current_entity: start.name(),
            steps: Vec::new(),
            guards: Vec::new(),
            _marker: PhantomData,
        }
    }
}

impl<Schema, Start, Current> ApplicationQueryRootPath<Schema, Start, Current> {
    pub fn forward<Relation, Next>(
        mut self,
        relation: ApplicationRelationRef<Schema, Relation, Current, Next>,
    ) -> ApplicationQueryRootPath<Schema, Start, Next> {
        self.steps.push(ApplicationQueryRootPathStep {
            relation: relation.name(),
            from: relation.from(),
            to: relation.to(),
            direction: ApplicationQueryRootPathDirection::Forward,
        });
        ApplicationQueryRootPath {
            start_entity: self.start_entity,
            current_entity: relation.to(),
            steps: self.steps,
            guards: self.guards,
            _marker: PhantomData,
        }
    }

    pub fn reverse<Relation, Previous>(
        mut self,
        relation: ApplicationRelationRef<Schema, Relation, Previous, Current>,
    ) -> ApplicationQueryRootPath<Schema, Start, Previous> {
        self.steps.push(ApplicationQueryRootPathStep {
            relation: relation.name(),
            from: relation.from(),
            to: relation.to(),
            direction: ApplicationQueryRootPathDirection::Reverse,
        });
        ApplicationQueryRootPath {
            start_entity: self.start_entity,
            current_entity: relation.from(),
            steps: self.steps,
            guards: self.guards,
            _marker: PhantomData,
        }
    }

    pub fn where_equal<Aspect, Field, Value, Write, Currency>(
        mut self,
        field: ApplicationFieldRef<
            Schema,
            Current,
            Aspect,
            Field,
            Value,
            Write,
            EqualityPredicate,
            Currency,
        >,
        expected: Value,
    ) -> Self
    where
        Value: TypedApplicationValue,
        Write: WritePosture,
        EqualityPredicate: EqualityCapable,
        Currency: ApplicationFieldCurrency,
    {
        self.guards.push(ApplicationQueryRootPathGuard::new(
            self.steps.len(),
            field,
            expected,
        ));
        self
    }

    pub(crate) fn into_meaning(self) -> ApplicationQueryRootPathMeaning {
        ApplicationQueryRootPathMeaning {
            start_entity: self.start_entity,
            terminal_entity: self.current_entity,
            steps: self.steps,
            guards: self.guards,
        }
    }
}
