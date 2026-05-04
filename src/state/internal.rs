use derive_more::Debug;
use ossa_core::store::ecg::v0::{HeaderId, OperationId};
use ossa_core::time::{CausalTime, ConcretizeTime};
use ossa_core::util::Sha256Hash;
use ossa_crdt::map::twopmap::TwoPMapOp;
use ossa_crdt::{map::twopmap::TwoPMap, register::LWW, time::CausalState, CRDT};
use ossa_dioxus::{DefaultSetup, UseStore};
use ossa_typeable::Typeable;

use serde::{Deserialize, Serialize};

pub type Time = OperationId<HeaderId<Sha256Hash>>;

// pub struct RecipeId(Time); // TODO: Newtype wrap this. JP: How do we get this newtype wrapper to work? `Into` instance?
pub type RecipeId<Time> = Time;
#[derive(Clone, CRDT, Debug, PartialEq, Typeable, Serialize, Deserialize)]
#[crdt(bound = "Time: Ord", time = Time, concretize_time, concretize_time_op)]
#[crdt(bound_concretize_time = "Time::Serialized: Ord")]
pub struct Recipe<Time> {
    pub title: LWW<Time, String>,
    pub ingredients: LWW<Time, Vec<String>>, // Sequence<String>,
    pub instructions: LWW<Time, String>,     // RGA<String>,
                                             // pub image: Sequence<OssaRef<Image>>, // Sequence?
}

pub type CookbookId = usize; // TODO: Newtype wrap this.
#[derive(Clone, CRDT, Debug, Deserialize, Serialize, Typeable)]
#[crdt(bound = "Time: Clone + Ord", time = Time, concretize_time_op)]
#[crdt(bound_concretize_time = "Time::Serialized: Clone + Ord")]
#[crdt(bound_concretize_time = "__HeaderId: Clone")]
#[serde(bound = "Time: Clone + Ord + Serialize + for <'d> Deserialize<'d>")]
pub struct Cookbook<Time> { // : Clone + Ord> {
    pub title: LWW<Time, String>,
    pub recipes: TwoPMap<RecipeId<Time>, Recipe<Time>>,
}

pub type State = Vec<UseStore<DefaultSetup, Cookbook<Time>>>;

// impl<T, U> OperationFunctor<T, U> for RecipeOp<T> {
//     type Target<Time> = RecipeOp<Time>;
//
//     fn fmap(self, f: impl Fn(T) -> U) -> Self::Target<U> {
//         match self {
//             RecipeOp::Title(op) => RecipeOp::Title(op.fmap(f)),
//             RecipeOp::Ingredients (op) => RecipeOp::Ingredients(op.fmap(f)),
//             RecipeOp::Instructions(op) => RecipeOp::Instructions(op.fmap(f)),
//         }
//     }
// }
//
// impl<T, U> OperationFunctor<T, U> for CookbookOp<T> {
//     type Target<Time> = CookbookOp<Time>;
//
//     fn fmap(self, f: impl Fn(T) -> U) -> Self::Target<U> {
//         match self {
//             CookbookOp::Title(op) => CookbookOp::Title(op.fmap(f)),
//             CookbookOp::Recipes(op) => CookbookOp::Recipes(op.fmap(f)), // <TwoPMapOp<RecipeId<T>, Recipe<T>, RecipeOp<T>> as OperationFunctor<T, U>>::fmap(op, f)),
//         }
//     }
// }

// use std::marker::PhantomData;
// #[derive(Props)]
// pub struct State<'a> {
//     pub cookbooks: Vec<UseStore<DefaultSetup, Cookbook>>,
//     _phantom: PhantomData<'a, ()>,
// }
