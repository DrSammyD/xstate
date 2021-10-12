mod utils;
use std::any::{Any, TypeId};
use std::collections::BTreeMap;
use std::ops::Index;
use wasm_bindgen::prelude::*;

const ASSIGN_ACTION: &'static str = "xstate.assign";

#[wasm_bindgen]
pub fn createMachine(fsmConfig: JsValue, options: JsValue) {}

enum AssignmentsTypes<TContext, TEvent>
where
    TContext: Clone + Copy + Index<Box<dyn Any>>,
    TEvent: Clone + Copy,
{
    Fn(Box<dyn Fn(TContext, TEvent) -> Box<dyn Any>>),
    Val(Box<dyn Any>),
}
enum AssignmentTypes<TContext, TEvent>
where
    TContext: Clone + Copy + Index<Box<dyn Any>>,
    TEvent: Clone + Copy,
{
    Assignment(Box<dyn Fn(TContext, TEvent) -> TContext>),
    Assignments(BTreeMap<String, AssignmentsTypes<TContext, TEvent>>),
}

struct ActionObject<TContext, TEvent>
where
    TContext: Clone + Copy + Index<Box<dyn Any>>,
    TEvent: Clone + Copy,
{
    typ: String,
    exec: Box<dyn Fn(TContext, TEvent) -> ()>,
    assignment: Option<AssignmentTypes<TContext, TEvent>>,
}

fn handleActions<TContext, TEvent>(
    actions: Vec<&ActionObject<TContext, TEvent>>,
    context: TContext,
    eventObject: TEvent,
) where
    TContext: Clone + Copy + Index<Box<dyn Any>>,
    TEvent: Clone + Copy,
{
    let next_context = context;
    let mut tmp_context = next_context.clone();
    let mut assigned = false;
    let non_assign_actions = actions.into_iter().filter(|action| {
        if action.typ == ASSIGN_ACTION {
            assigned = true;
            if action.assignment.is_some() {
                tmp_context = match action.assignment.as_ref().unwrap() {
                    AssignmentTypes::Assignment(assignment) => {
                        assignment(next_context, eventObject)
                    }
                    AssignmentTypes::Assignments(assignments) => {
                        for (key, assignment) in assignments {
                            match assignment {
                                AssignmentsTypes::Val(val) => {
                                    tmp_context[key] = val;
                                }
                                AssignmentsTypes::Fn(func) => {
                                    tmp_context[key] = func(next_context, eventObject);
                                }
                            }
                        }
                        tmp_context
                    }
                };
            }
        } else {
            return true;
        }
        return false;
    });
    return ();
}
