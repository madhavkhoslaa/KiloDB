use crate::traits::Store::Store;
use std::collections::BTreeMap;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct SortedSetStore {
    pub data: BTreeMap<f64, Vec<String>>, // score -> members with that score
    pub member_to_score: BTreeMap<String, f64>, // member -> score for quick lookups
}

impl SortedSetStore {
    pub fn new() -> Self {
        SortedSetStore {
            data: BTreeMap::new(),
            member_to_score: BTreeMap::new(),
        }
    }

    pub fn new_with_data(data: BTreeMap<f64, Vec<String>>, member_to_score: BTreeMap<String, f64>) -> Self {
        SortedSetStore { data, member_to_score }
    }

    pub fn add(&mut self, score: f64, member: String) -> bool {
        // Remove member from old score if it exists
        if let Some(old_score) = self.member_to_score.get(&member) {
            if let Some(members) = self.data.get_mut(old_score) {
                members.retain(|m| m != &member);
                if members.is_empty() {
                    self.data.remove(old_score);
                }
            }
        }

        // Add member to new score
        self.data.entry(score).or_insert_with(Vec::new).push(member.clone());
        self.member_to_score.insert(member, score);
        
        true
    }

    pub fn remove(&mut self, member: &str) -> bool {
        if let Some(score) = self.member_to_score.remove(member) {
            if let Some(members) = self.data.get_mut(&score) {
                members.retain(|m| m != member);
                if members.is_empty() {
                    self.data.remove(&score);
                }
            }
            true
        } else {
            false
        }
    }

    pub fn get_score(&self, member: &str) -> Option<f64> {
        self.member_to_score.get(member).copied()
    }

    pub fn range(&self, start: isize, stop: isize) -> Vec<String> {
        let all_members: Vec<String> = self.data
            .iter()
            .flat_map(|(_, members)| members.iter().cloned())
            .collect();
        
        let len = all_members.len() as isize;
        let start = if start < 0 { len + start } else { start };
        let stop = if stop < 0 { len + stop } else { stop };
        
        let start = start.max(0).min(len) as usize;
        let stop = stop.max(0).min(len) as usize;
        
        if start >= stop {
            return Vec::new();
        }
        
        all_members.into_iter().skip(start).take(stop - start).collect()
    }

    pub fn len(&self) -> usize {
        self.member_to_score.len()
    }

    pub fn is_empty(&self) -> bool {
        self.member_to_score.is_empty()
    }
}

impl Store for SortedSetStore {} 